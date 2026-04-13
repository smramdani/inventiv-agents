use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub default_locale: String,
}

impl Organization {
    pub fn new(name: &str, default_locale: &str) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Organization name cannot be empty"));
        }

        // Simplified locale validation for TDD start
        if !["en_US", "fr_FR", "ar_AR"].contains(&default_locale) {
            return Err(anyhow::anyhow!("Unsupported locale: {}", default_locale));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            default_locale: default_locale.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_organization_valid() {
        let org = Organization::new("My SME", "en_US").unwrap();
        assert_eq!(org.name, "My SME");
        assert_eq!(org.default_locale, "en_US");
    }

    #[test]
    fn test_create_organization_invalid_name() {
        let result = Organization::new("", "en_US");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Organization name cannot be empty"
        );
    }

    #[test]
    fn test_create_organization_unsupported_locale() {
        let result = Organization::new("My SME", "jp_JP");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported locale"));
    }
}
