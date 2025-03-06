pub fn is_double(value: &str) -> bool {
    let re = regex::Regex::new(r"^\d+(\.\d+)?$").unwrap();
    re.is_match(value)
}