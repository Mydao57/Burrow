use vault_core::{generate_password, GeneratorOptions, VaultError};

fn has(s: &str, set: &str) -> bool {
    s.chars().any(|c| set.contains(c))
}

#[test]
fn respects_length_and_includes_every_selected_set() {
    for _ in 0..200 {
        let p = generate_password(&GeneratorOptions { length: 8, ..Default::default() }).unwrap();
        assert_eq!(p.chars().count(), 8);
        assert!(has(&p, "abcdefghijklmnopqrstuvwxyz"));
        assert!(has(&p, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(has(&p, "0123456789"));
        assert!(p.chars().any(|c| !c.is_ascii_alphanumeric()));
    }
}

#[test]
fn only_selected_sets_are_used() {
    let opts = GeneratorOptions { length: 64, lowercase: false, uppercase: false, digits: true, symbols: false, exclude_ambiguous: false };
    let p = generate_password(&opts).unwrap();
    assert!(p.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn ambiguous_characters_can_be_excluded() {
    let opts = GeneratorOptions { length: 200, exclude_ambiguous: true, ..Default::default() };
    for _ in 0..50 {
        let p = generate_password(&opts).unwrap();
        assert!(!has(&p, "O0Il1|"));
    }
}

#[test]
fn invalid_options_are_errors() {
    let none = GeneratorOptions { lowercase: false, uppercase: false, digits: false, symbols: false, ..Default::default() };
    assert!(matches!(generate_password(&none), Err(VaultError::InvalidGeneratorOptions(_))));
    let short = GeneratorOptions { length: 3, ..Default::default() };
    assert!(matches!(generate_password(&short), Err(VaultError::InvalidGeneratorOptions(_))));
    let long = GeneratorOptions { length: 10_000, ..Default::default() };
    assert!(matches!(generate_password(&long), Err(VaultError::InvalidGeneratorOptions(_))));
}

#[test]
fn passwords_differ() {
    let o = GeneratorOptions::default();
    assert_ne!(generate_password(&o).unwrap(), generate_password(&o).unwrap());
}
