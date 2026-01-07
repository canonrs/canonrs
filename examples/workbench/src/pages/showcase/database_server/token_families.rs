use leptos::prelude::*;
use super::types::TokenFamilyDetailRow;

#[server]
pub async fn fetch_token_families() -> Result<Vec<TokenFamilyDetailRow>, ServerFnError> {
    use rusqlite::Connection;

    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut stmt = conn.prepare(
        "SELECT family_code, family_name, token_name, token_full_name, description
        FROM token_families_detailed WHERE is_active = 1 ORDER BY family_code"
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let rows_iter = stmt.query_map([], |row| {
        Ok(TokenFamilyDetailRow {
            family_code: row.get(0)?,
            family_name: row.get(1)?,
            token_name: row.get(2)?,
            token_full_name: row.get(3)?,
            description: row.get(4)?,
        })
    }).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut results = Vec::new();
    for row in rows_iter {
        results.push(row.map_err(|e| ServerFnError::new(e.to_string()))?);
    }

    Ok(results)
}
