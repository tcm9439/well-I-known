use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
enum ApiOperation {
    #[strum(ascii_case_insensitive)]
    Create,
    #[strum(ascii_case_insensitive)]
    Delete,
    #[strum(ascii_case_insensitive)]
    Update,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_api_operation_from_str() {
        assert_eq!(ApiOperation::from_str("create"), Ok(ApiOperation::Create));
        assert_eq!(ApiOperation::from_str("delete"), Ok(ApiOperation::Delete));
        assert_eq!(ApiOperation::from_str("update"), Ok(ApiOperation::Update));
    }

    #[test]
    fn test_api_operation_from_str_case_insensitive() {
        assert_eq!(ApiOperation::from_str("Create"), Ok(ApiOperation::Create));
        assert_eq!(ApiOperation::from_str("Delete"), Ok(ApiOperation::Delete));
        assert_eq!(ApiOperation::from_str("Update"), Ok(ApiOperation::Update));
    }

    #[test]
    fn test_api_operation_from_str_invalid() {
        assert!(ApiOperation::from_str("invalid").is_err());
    }

    #[test]
    fn test_api_operation_to_str() {
        assert_eq!(ApiOperation::Create.to_string(), "Create");
        assert_eq!(ApiOperation::Delete.to_string(), "Delete");
        assert_eq!(ApiOperation::Update.to_string(), "Update");
    }
}