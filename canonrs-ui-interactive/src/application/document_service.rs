const BASE_URL: &str = "http://localhost:8112";
const CDN_URL: &str = "http://localhost:8113";

#[derive(Debug)]
pub enum SaveError {
    Conflict { current_version: i64, sent_version: i64 },
    Other(String),
}

#[cfg(target_arch = "wasm32")]
pub async fn save_document(
    id: String,
    name: String,
    layout: String,
    version: i64,
    payload_json: String,
) -> Result<i64, SaveError> {
    use crate::infra::http_client::post_json;

    let body = serde_json::json!({
        "id": id, "name": name, "layout": layout,
        "version": version, "payload_json": payload_json,
    }).to_string();

    let (v, status) = post_json(&format!("{}/documents", BASE_URL), body)
        .await.map_err(SaveError::Other)?;

    if status == 200 || status == 201 {
        Ok(v["version"].as_i64().unwrap_or(1))
    } else if status == 409 {
        Err(SaveError::Conflict {
            current_version: v["current_version"].as_i64().unwrap_or(0),
            sent_version: v["sent_version"].as_i64().unwrap_or(0),
        })
    } else {
        Err(SaveError::Other(format!("HTTP {}", status)))
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn list_documents() -> Result<Vec<(String, String, String)>, String> {
    use crate::infra::http_client::get_json;

    let v = get_json(&format!("{}/documents", BASE_URL)).await?;
    let items = v.as_array().ok_or("not array")?.clone();
    Ok(items.into_iter().filter_map(|v| Some((
        v["id"].as_str()?.to_string(),
        v["name"].as_str()?.to_string(),
        v["updated_at"].as_str().unwrap_or("").to_string(),
    ))).collect())
}

#[cfg(target_arch = "wasm32")]
pub async fn get_document(id: String) -> Result<(String, String, i64), String> {
    use crate::infra::http_client::get_json;

    let v = get_json(&format!("{}/documents/{}", BASE_URL, id)).await?;
    let payload = v["payload_json"].as_str().ok_or("no payload_json")?.to_string();
    let layout = v["layout"].as_str().unwrap_or("Dashboard").to_string();
    let version = v["version"].as_i64().unwrap_or(1);
    Ok((payload, layout, version))
}

#[cfg(target_arch = "wasm32")]
pub async fn publish_document(id: String) -> Result<i64, String> {
    use crate::infra::http_client::post_json;

    let url = format!("{}/documents/{}/publish", BASE_URL, id);
    let (v, status) = post_json(&url, "{}".into()).await?;
    if status == 200 || status == 201 {
        Ok(v["version"].as_i64().unwrap_or(1))
    } else {
        Err(v["error"].as_str().unwrap_or("unknown").to_string())
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn invalidate_cache(document_id: String) -> Result<(), String> {
    use crate::infra::http_client::delete_json;
    delete_json(&format!("{}/cache/{}", CDN_URL, document_id)).await
}
