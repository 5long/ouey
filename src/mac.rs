pub fn normalize_mac_str(s: &String) -> String {
    s.replace(|c:char| !c.is_ascii_hexdigit(), "").to_lowercase()
}

#[test]
fn normalize_mac_str_test() {
    assert_eq!("123456", normalize_mac_str(&"12:34:56".to_string()));
    assert_eq!("123456abcdef", normalize_mac_str(&"12-34-56-ab-cd-ef".to_string()));
    assert_eq!("ab", normalize_mac_str(&"Ab".to_string()));
}
