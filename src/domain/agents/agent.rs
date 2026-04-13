use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub llm_provider_id: Option<Uuid>,
    pub name: String,
    pub mission: String,
    pub persona: Option<String>,
    pub is_active: bool,
}

impl Agent {
    pub fn new(
        organization_id: Uuid,
        name: &str,
        mission: &str,
        llm_provider_id: Option<Uuid>,
    ) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Agent name cannot be empty"));
        }
        if mission.trim().is_empty() {
            return Err(anyhow::anyhow!("Agent mission cannot be empty"));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            organization_id,
            llm_provider_id,
            name: name.to_string(),
            mission: mission.to_string(),
            persona: None,
            is_active: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_agent_valid() {
        let org_id = Uuid::new_v4();
        let agent = Agent::new(org_id, "Support Agent", "Help users with tech issues", None).unwrap();
        assert_eq!(agent.name, "Support Agent");
        assert_eq!(agent.mission, "Help users with tech issues");
    }

    #[test]
    fn test_create_agent_invalid_mission() {
        let org_id = Uuid::new_v4();
        let result = Agent::new(org_id, "Lazy Agent", "", None);
        assert!(result.is_err());
    }
}
