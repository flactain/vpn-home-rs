use async_trait::async_trait;
use sqlx::{PgPool, any::AnyQueryResult};

use crate::{
    entities::{approvals::ApprovalRequest, vpns::VpnOutline},
    repositories::vpns_repository::VpnsRepository,
};

pub struct PostgresVpnsRepository {
    pub pg_pool: PgPool,
}

impl PostgresVpnsRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresVpnsRepository { pg_pool }
    }
}

#[async_trait]
impl VpnsRepository for PostgresVpnsRepository {
    async fn find_all(&self) -> Result<Vec<VpnOutline>, sqlx::Error> {
        sqlx::query_as!(
            VpnOutline,
            r#"
                  WITH /* vpns.findAll */ clients_count AS (
                       SELECT 
                              c.vpn_id
                            , count(1) AS clients_count
                       FROM clients c 
                       WHERE 1 = 1
                         AND NOT c.is_deleted
                         AND c.approved_at IS NOT NULL
                       GROUP BY c.vpn_id 
                 )
                SELECT 
                       v.vpn_id 
                     , v.vpn_name
                     , v.owner_user_id 
                     , t.terminal_name AS server_name
                     , s.public_ip 
                     , s.private_ip 
                     , c.clients_count 
                     , v.approved_at IS NOT NULL AS is_approved
                     , v.created_at
                  FROM vpns v
            INNER JOIN servers s 
                    ON v.vpn_id =s.vpn_id 
            INNER JOIN terminals t 
                    ON s.terminal_id = t.terminal_id 
            INNER JOIN clients_count c
                    ON v.vpn_id = c.vpn_id 
                 WHERE 1 = 1
                   AND NOT v.is_deleted
                   AND NOT t.is_deleted
            ;
        "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }
    async fn find_requests(&self, user_id: &String) -> Result<Vec<ApprovalRequest>, sqlx::Error> {
        let rows: Vec<ApprovalRequest> = sqlx::query_as(
            r#"
                SELECT /* vpns.find_requests */
                       'CLIENT' AS resource_type
                     , 'CREATE' AS resource_handle
                     , v.vpn_id AS vpn_id
                     , c.terminal_id AS resource_id
                     , v.owner_user_id AS asignee_user_id
                     , t.owner_user_id AS request_user_id
                     , c.approved_at
                  FROM clients c 
            INNER JOIN vpns v 
                    ON c.vpn_id = v.vpn_id
            INNER JOIN terminals t 
                    ON c.terminal_id =t.terminal_id 
                 WHERE 1=1
                   AND v.owner_user_id =$1
                   AND c.approved_at IS NULL
               AND NOT v.is_deleted 
               AND NOT c.is_deleted 
               AND NOT t.is_deleted 
            
            UNION ALL
             
                SELECT /* vpns.find_requests */
                       'VPN' AS resource_type
                     , 'CREATE' AS resource_handle
                     , v.vpn_id AS vpn_id
                     , v.vpn_id AS resource_id
                     , 'flactain' AS asignee_user_id
                     , v.owner_user_id AS request_user_id
                     , v.approved_at
                  FROM vpns v 
                 WHERE 1=1
                   AND v.owner_user_id =$1
                   AND v.approved_at IS NULL
               AND NOT v.is_deleted 
        ;
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pg_pool)
        .await?;

        Ok(rows)
    }
    async fn approve_vpn(&self, approval_request: ApprovalRequest) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query(
            r#" 
            UPDATE vpns v
               SET approved_at = clock_timestamp()
                 , updated_at = clock_timestamp()
             WHERE 1=1
               AND v.vpn_id = $1
               AND v.approved_at IS NULL
           AND NOT v.is_deleted 
           ;
           "#,
        )
        .bind(approval_request.vpn_id())
        .execute(&self.pg_pool)
        .await;

        match result {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }
    async fn approve_client(
        &self,
        approval_request: ApprovalRequest,
    ) -> Result<AnyQueryResult, sqlx::Error> {
        let result = sqlx::query(
            r#" 
            UPDATE clients c
               SET approved_at = clock_timestamp()
                 , updated_at = clock_timestamp()
             WHERE 1=1
               AND c.vpn_id = $1
               AND c.terminal_id =$2
               AND c.approved_at IS NULL
           AND NOT c.is_deleted
           ;
           "#,
        )
        .bind(approval_request.vpn_id())
        .bind(approval_request.resource_id())
        .execute(&self.pg_pool)
        .await;

        match result {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }
}
