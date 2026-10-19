#!/usr/bin/env bash
# Publishes the DRAFT GitHub release of a tag, after checking it.
#
# Environment: TAG (e.g. v0.1.0), GITHUB_REPOSITORY (owner/name), GH_TOKEN,
#              DRY_RUN=true to run the checks without publishing.
# Needs: gh, jq, git (history and tags fetched, origin/release present), sha256sum.
set -euo pipefail

: "${TAG:?TAG is required}"
: "${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}"
DRY_RUN="${DRY_RUN:-false}"

fail() { echo "::error::$*"; exit 1; }
note() { echo "$*"; [ -z "${GITHUB_STEP_SUMMARY:-}" ] || echo "$*" >> "$GITHUB_STEP_SUMMARY"; }

[[ "$TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]] || fail "Invalid tag name: $TAG"

# 1. The tag exists and points to a commit of the release branch.
commit="$(git rev-parse --verify --quiet "refs/tags/$TAG^{commit}")" || fail "Tag $TAG does not exist"
git merge-base --is-ancestor "$commit" origin/release || fail "Tag $TAG is not on the release branch"

# 2. Exactly one release exists for the tag, and it is still a draft.
releases="$(gh api "repos/$GITHUB_REPOSITORY/releases?per_page=100" --paginate \
  --jq '.[] | select(.tag_name == env.TAG) | {id, draft, prerelease, assets: [.assets[] | {id, name, size}]}')"
count="$(printf '%s\n' "$releases" | grep -c . || true)"
[ "$count" -ge 1 ] || fail "No release found for $TAG (did the Release workflow finish?)"
[ "$count" -eq 1 ] || fail "$count releases exist for $TAG: delete the extra drafts first"
[ "$(jq -r .draft <<<"$releases")" = true ] || fail "The release for $TAG is already published"
id="$(jq -r .id <<<"$releases")"
prerelease="$(jq -r .prerelease <<<"$releases")"

# 3. The expected files are attached.
names="$(jq -r '.assets[].name' <<<"$releases")"
has() { grep -Eq "$1" <<<"$names"; }
has '^SHA256SUMS\.txt$'           || fail "SHA256SUMS.txt is missing from the release"
has '\.dmg$'                      || fail "No macOS installer (.dmg) attached"
has '(\.msi|-setup\.exe)$'        || fail "No Windows installer (.msi or -setup.exe) attached"
has '(\.deb|\.rpm|\.AppImage)$'   || fail "No Linux package (.deb, .rpm or .AppImage) attached"

# 4. The files match SHA256SUMS.txt, and every file is listed in it.
dir="$(mktemp -d)"
trap 'rm -rf "$dir"' EXIT
(
  cd "$dir"
  jq -r '.assets[] | "\(.id) \(.name)"' <<<"$releases" | while read -r asset_id name; do
    gh api -H "Accept: application/octet-stream" "repos/$GITHUB_REPOSITORY/releases/assets/$asset_id" > "$name"
  done
  sha256sum -c SHA256SUMS.txt || fail "Checksum mismatch: do not publish"
  while read -r name; do
    [ "$name" = SHA256SUMS.txt ] || grep -qF -- "  $name" SHA256SUMS.txt || fail "$name is not listed in SHA256SUMS.txt"
  done <<<"$names"
)

note "Release $TAG (draft id $id, prerelease=$prerelease) passed all checks:"
jq -r '.assets[] | "- \(.name) (\(.size) bytes)"' <<<"$releases" | while read -r line; do note "$line"; done

if [ "$DRY_RUN" = true ]; then
  note "Dry run: nothing was published."
  exit 0
fi

# 5. Publish. A pre-release is never marked as the latest release.
latest=true
[ "$prerelease" = true ] && latest=false
url="$(gh api -X PATCH "repos/$GITHUB_REPOSITORY/releases/$id" -F draft=false -f make_latest="$latest" --jq .html_url)"
note "Published: $url"
