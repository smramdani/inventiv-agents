use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillType {
    MCP,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub skill_type: SkillType,
    pub endpoint_url: Option<String>,
    pub configuration: serde_json::Value,
    pub is_active: bool,
}

impl Skill {
    pub fn new(
        organization_id: Uuid,
        name: &str,
        skill_type: SkillType,
        endpoint_url: Option<String>,
    ) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Skill name cannot be empty"));
        }

        if skill_type == SkillType::MCP {
            let url = endpoint_url
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("MCP skill requires endpoint_url"))?;
            let u = url.trim();
            if u.is_empty() || !(u.starts_with("http://") || u.starts_with("https://")) {
                return Err(anyhow::anyhow!(
                    "MCP skill requires a valid http(s) endpoint_url"
                ));
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            organization_id,
            name: name.to_string(),
            description: None,
            skill_type,
            endpoint_url,
            configuration: serde_json::Value::Object(serde_json::Map::new()),
            is_active: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_skill_valid() {
        let org_id = Uuid::new_v4();
        let skill = Skill::new(
            org_id,
            "PDF Search",
            SkillType::MCP,
            Some("https://mcp.internal".into()),
        )
        .unwrap();
        assert_eq!(skill.name, "PDF Search");
        assert_eq!(skill.skill_type, SkillType::MCP);
    }

    #[test]
    fn test_mcp_requires_endpoint() {
        let org_id = Uuid::new_v4();
        let r = Skill::new(org_id, "No Endpoint", SkillType::MCP, None);
        assert!(r.is_err());
    }

    #[test]
    fn test_native_skill_without_endpoint() {
        let org_id = Uuid::new_v4();
        let skill = Skill::new(org_id, "Native Hook", SkillType::Native, None).unwrap();
        assert_eq!(skill.skill_type, SkillType::Native);
    }
}
