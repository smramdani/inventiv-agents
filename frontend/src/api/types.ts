export type UserRole = "Owner" | "Admin" | "User";

export interface WhoAmI {
  user_id: string;
  org_id: string;
  role: UserRole;
}

export interface LlmProvider {
  id: string;
  organization_id: string;
  name: string;
  base_url: string;
  is_active: boolean;
}

export type SkillType = "MCP" | "Native";

export interface Skill {
  id: string;
  organization_id: string;
  name: string;
  description: string | null;
  skill_type: SkillType;
  endpoint_url: string | null;
  configuration: Record<string, unknown>;
  is_active: boolean;
}

export interface Agent {
  id: string;
  organization_id: string;
  llm_provider_id: string | null;
  name: string;
  mission: string;
  persona: string | null;
  is_active: boolean;
}
