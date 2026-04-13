use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProvider {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub base_url: String,
    pub is_active: bool,
}

impl LlmProvider {
    pub fn new(organization_id: Uuid, name: &str, base_url: &str) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Provider name cannot be empty"));
        }

        if !base_url.starts_with("http") {
            return Err(anyhow::anyhow!(
                "Invalid base URL: must start with http/https"
            ));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            organization_id,
            name: name.to_string(),
            base_url: base_url.to_string(),
            is_active: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider_valid() {
        let org_id = Uuid::new_v4();
        let provider = LlmProvider::new(org_id, "OVH AI", "https://api.ovh.com").unwrap();
        assert_eq!(provider.name, "OVH AI");
        assert_eq!(provider.organization_id, org_id);
    }

    #[test]
    fn test_create_provider_invalid_url() {
        let org_id = Uuid::new_v4();
        let result = LlmProvider::new(org_id, "Bad Provider", "ftp://invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_provider_invalid_url_without_scheme() {
        let org_id = Uuid::new_v4();
        let result = LlmProvider::new(org_id, "Bad", "example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_provider_whitespace_name_rejected() {
        let org_id = Uuid::new_v4();
        let result = LlmProvider::new(org_id, "   ", "https://api.example.com");
        assert!(result.is_err());
    }
}
