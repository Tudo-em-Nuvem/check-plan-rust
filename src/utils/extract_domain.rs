use crate::utils::is_double::is_double;
use regex::Regex;

pub fn extract_domain(domain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"\b(?:[a-zA-Z0-9][a-zA-Z0-9\-_@]*\.)+(?:xn--[a-zA-Z0-9]+|[a-zA-Z0-9]{2,}|[a-zA-Z0-9]{2}\.[a-zA-Z0-9]{2})\b").unwrap();
    
    let mut domains: Vec<String> = Vec::new();

    for cap in re.captures_iter(&domain.to_string()) {
        domains = cap.iter()
            .filter_map(|m| m.map(|m| m.as_str().to_string()))
            .collect();
    }

    if domains.len() > 1 {
        if is_double(&domains[0]){
            return Ok(domains[1].to_string())
        }
    } else {
        return Ok(domains[0].to_string())
    }

    Err("No domain found".into())
}
