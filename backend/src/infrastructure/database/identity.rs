use crate::domain::identity::group::{Group, GroupMemberRole};
use crate::domain::identity::user::{User, UserRole};
use crate::error::AppResult;
use crate::infrastructure::database::DatabasePool;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub struct IdentityRepository;

impl IdentityRepository {
    pub async fn create_group(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        group: &Group,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        sqlx::query(
            "INSERT INTO groups (id, organization_id, name, description) VALUES ($1, $2, $3, $4)",
        )
        .bind(group.id)
        .bind(org_id)
        .bind(&group.name)
        .bind(&group.description)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn add_group_member(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        group_id: Uuid,
        user_id: Uuid,
        role: GroupMemberRole,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let role_str = match role {
            GroupMemberRole::Member => "Member",
            GroupMemberRole::Organizer => "Organizer",
        };

        sqlx::query("INSERT INTO group_members (group_id, user_id, role) VALUES ($1, $2, $3::group_member_role)")
            .bind(group_id)
            .bind(user_id)
            .bind(role_str)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn create_user(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        user: &User,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let role_str = match user.role {
            UserRole::Owner => "Owner",
            UserRole::Admin => "Admin",
            UserRole::User => "User",
        };

        sqlx::query("INSERT INTO users (id, organization_id, email, role) VALUES ($1, $2, $3, $4::user_role)")
            .bind(user.id)
            .bind(org_id)
            .bind(&user.email)
            .bind(role_str)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
