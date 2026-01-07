use leptos::prelude::*;
use super::types::TokenRow;

#[server]
pub async fn fetch_tokens() -> Result<Vec<TokenRow>, ServerFnError> {
    use rusqlite::Connection;

    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut stmt = conn.prepare(
        "SELECT 
            t.id, t.scope, 
            COALESCE(t.domain, 'Other') as domain,
            CASE t.family_id 
                WHEN 1 THEN 'A'
                WHEN 2 THEN 'B'
                WHEN 3 THEN 'C'
                WHEN 4 THEN 'D'
                WHEN 5 THEN 'E'
                WHEN 6 THEN 'F'
                ELSE NULL
            END as family_code,
            f.name as family_name,
            t.category, t.name,
            t.category || '.' || t.name as full_name,
            t.value, t.status, t.created_at
        FROM tokens t
        LEFT JOIN component_families f ON t.family_id = f.id
        WHERE t.status = 'active'
        ORDER BY t.scope, t.domain, t.category, t.name"
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let tokens_iter = stmt.query_map([], |row| {
        Ok(TokenRow {
            id: row.get::<_, i32>(0)?.to_string(),
            scope: row.get(1)?,
            domain: row.get(2)?,
            family_code: row.get(3)?,
            family_name: row.get(4)?,
            category: row.get(5)?,
            name: row.get(6)?,
            full_name: row.get(7)?,
            value: row.get(8)?,
            status: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut tokens = Vec::new();
    for token in tokens_iter {
        tokens.push(token.map_err(|e| ServerFnError::new(e.to_string()))?);
    }

    Ok(tokens)
}
