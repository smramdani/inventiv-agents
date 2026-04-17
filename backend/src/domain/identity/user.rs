use crate::domain::identity::organization::Organization;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Owner,
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub preferred_locale: Option<String>,
}

impl User {
    pub fn new(org: &Organization, email: &str, role: UserRole) -> anyhow::Result<Self> {
        if !email.contains('@') {
            return Err(anyhow::anyhow!("Invalid email address"));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            organization_id: org.id,
            email: email.to_string(),
            role,
            preferred_locale: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::identity::organization::Organization;

    fn mock_org() -> Organization {
        Organization::new("Test Org", "en_US").unwrap()
    }

    #[test]
    fn test_create_user_valid() {
        let org = mock_org();
        let user = User::new(&org, "test@example.com", UserRole::Admin).unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.role, UserRole::Admin);
        assert_eq!(user.organization_id, org.id);
    }

    #[test]
    fn test_create_user_invalid_email() {
        let org = mock_org();
        let result = User::new(&org, "invalid-email", UserRole::User);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid email address");
    }
}
