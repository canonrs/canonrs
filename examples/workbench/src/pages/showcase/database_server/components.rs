use leptos::prelude::*;
use super::types::ComponentRow;

#[server]
pub async fn fetch_components() -> Result<Vec<ComponentRow>, ServerFnError> {
    use rusqlite::Connection;

    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut stmt = conn.prepare(
        "SELECT c.id, c.tipo, c.name,
            COALESCE(cf.name, 'Unknown') as familia,
            COALESCE(can.name, 'Unknown') as canonical_type,
            c.status,
            COALESCE(c.tokens_canonicos_percent, 0),
            COALESCE(c.tokens_familia_c_percent, 0),
            c.familias_aplicadas,
            COALESCE(c.has_readme, 0),
            COALESCE(c.folder_structure_correct, 0),
            COALESCE(c.canon_score, 0)
        FROM components c
        LEFT JOIN component_families cf ON c.component_family_id = cf.id
        LEFT JOIN canonical_types can ON c.canonical_type_id = can.id
        WHERE c.is_active = 1
        ORDER BY c.tokens_canonicos_percent DESC, c.name"
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let components_iter = stmt.query_map([], |row| {
        Ok(ComponentRow {
            id: row.get::<_, i32>(0)?.to_string(),
            tipo: row.get(1)?,
            name: row.get(2)?,
            familia: row.get(3)?,
            canonical_type: row.get(4)?,
            status: row.get(5)?,
            tokens_canonicos_percent: row.get(6)?,
            tokens_familia_c_percent: row.get(7)?,
            familias_aplicadas: row.get(8)?,
            has_readme: row.get(9)?,
            folder_structure_correct: row.get(10)?,
            canon_score: row.get(11)?,
        })
    }).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut components = Vec::new();
    for component in components_iter {
        components.push(component.map_err(|e| ServerFnError::new(e.to_string()))?);
    }

    Ok(components)
}

#[server]
pub async fn update_component_status(
    component_name: String,
    tokens_canonicos: i32,
    tokens_familia_c: i32,
    familias_aplicadas: String,
    has_readme: bool,
    folder_structure_correct: bool,
    status: String,
) -> Result<(), ServerFnError> {
    use rusqlite::Connection;

    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;

    conn.execute(
        "UPDATE components SET 
            tokens_canonicos_percent = ?1,
            tokens_familia_c_percent = ?2,
            familias_aplicadas = ?3,
            has_readme = ?4,
            folder_structure_correct = ?5,
            status = ?6,
            canonicos_aplicados = CASE WHEN ?1 = 100 THEN 1 ELSE 0 END,
            familia_aplicada = CASE WHEN ?2 = 100 THEN 1 ELSE 0 END,
            documento_feito = CASE WHEN ?4 = 1 THEN 1 ELSE 0 END,
            canon_score = (?1 + ?2 + CASE WHEN ?4 THEN 25 ELSE 0 END + CASE WHEN ?5 THEN 25 ELSE 0 END) / 4,
            updated_at = CURRENT_TIMESTAMP
        WHERE name = ?7",
        rusqlite::params![
            tokens_canonicos, tokens_familia_c, familias_aplicadas,
            has_readme as i32, folder_structure_correct as i32,
            status, component_name
        ],
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server]
pub async fn mark_component_100(component_name: String) -> Result<(), ServerFnError> {
    use rusqlite::Connection;

    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;

    conn.execute(
        "UPDATE components SET
            tokens_canonicos_percent = 100, tokens_familia_c_percent = 100,
            has_readme = 1, folder_structure_correct = 1,
            canonicos_aplicados = 1, familia_aplicada = 1, documento_feito = 1,
            canon_score = 100, status = 'stable', updated_at = CURRENT_TIMESTAMP
        WHERE name = ?1",
        rusqlite::params![component_name],
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
