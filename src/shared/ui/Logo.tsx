/** Burrow mark: a mound with a keyhole-shaped entrance. Uses `currentColor`. */
export function Logo({ size = 28 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 32 32" fill="currentColor" aria-hidden="true" focusable="false">
      <path
        fillRule="evenodd"
        d="M2 28V17C2 9.3 8.3 4 16 4s14 5.3 14 13v11H2Zm14-16.5a3.6 3.6 0 0 0-1.9 6.7V23h3.8v-4.8A3.6 3.6 0 0 0 16 11.5Z"
      />
    </svg>
  );
}
