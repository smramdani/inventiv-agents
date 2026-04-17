use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupMemberRole {
    Member,
    Organizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Group {
    pub fn new(
        organization_id: Uuid,
        name: &str,
        description: Option<String>,
    ) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Group name cannot be empty"));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            organization_id,
            name: name.to_string(),
            description,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_group_valid() {
        let org_id = Uuid::new_v4();
        let group = Group::new(org_id, "Engineering", Some("Core team".into())).unwrap();
        assert_eq!(group.name, "Engineering");
        assert_eq!(group.organization_id, org_id);
    }

    #[test]
    fn test_create_group_invalid_name() {
        let org_id = Uuid::new_v4();
        let result = Group::new(org_id, "", None);
        assert!(result.is_err());
    }
}
