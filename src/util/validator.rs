use regex::Regex;
use std::path::Path;

pub fn validate_uri_opt(v: String) -> Result<(), String> {
    let re = Regex::new(r"https?://[\w!\?/\+\-_~=;\.,\*&@#\$%\(\)'\[\]]+").unwrap();
    if !re.is_match(&v) {
        return Err(String::from("Please specify uri"));
    }
    Ok(())
}

pub fn validate_domain_opt(v: String) -> Result<(), String> {
    let re = Regex::new(r"[\w\-._]+\.[A-Za-z]+").unwrap();
    if !re.is_match(&v) {
        return Err(String::from("Please specify domain name"));
    }
    Ok(())
}

pub fn validate_filepath(v: String) -> Result<(), String> {
    if !Path::new(&v).exists() {
        return Err(format!("File {} does not exist", v));
    }
    Ok(())
}

pub fn validate_timeout(v: String) -> Result<(), String> {
    let timeout_v = v.parse::<u64>();
    match timeout_v {
        Ok(timeout) => {
            if timeout <= 0 {
                return Err(String::from("Invalid timeout value"));
            }
        },
        Err(_) => {
            return Err(String::from("Invalid timeout value"));
        },
    }
    Ok(())
}

pub fn validate_request_method(v: String) -> Result<(), String> {
    let valid_methods: Vec<String> = vec![String::from("GET"),String::from("POST")];
    if !valid_methods.contains(&v.to_uppercase()) {
        return Err(format!("Invalid request method"));
    }
    Ok(())
}