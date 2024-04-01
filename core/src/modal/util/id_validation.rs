/// Validate the ID in this application, including username & config key.
/// Rules:
/// 1. ID must be a string with length between 4 and 30.
/// 2. Only allow \[a-z,A-Z,0-9,_\] in the ID.
/// 3. ID must start with a letter.
pub fn validate_id(id: &str) -> Result<(), String> {
    if id.len() < 4 || id.len() > 30 {
        return Err("ID must be between 4 and 30 characters.".to_string());
    }

    if !id.chars().all(|c| c.is_numeric() || c == '_' || c.is_ascii_alphabetic()) {
        return Err("ID can only contain alphanumeric characters and underscores.".to_string());
    }

    if !id.chars().next().unwrap().is_ascii_alphabetic() {
        return Err("ID must start with a letter.".to_string());
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_id() {
        assert!(validate_id("aB_1").is_err());
        assert!(validate_id("a".repeat(31).as_str()).is_err());
        assert!(validate_id("a1##**3!").is_err());
        assert!(validate_id("1a21sae_").is_err());
        assert_eq!(validate_id("a23ZZX31"), Ok(()));
        assert_eq!(validate_id("a_1_bc"), Ok(()));
    }
}