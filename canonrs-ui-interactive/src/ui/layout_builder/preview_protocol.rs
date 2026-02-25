use leptos::prelude::*;
use rs_canonrs::domain::CanonDocument;
use super::application::build_tree;
use super::types::{Node, ActiveLayout};

pub const PROTOCOL_VERSION: u32 = 1;
pub const MAX_RETRIES: u32 = 3;

/// Estados do Preview Runtime
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PreviewState {
    Idle,
    WaitingReady,
    Sending { retries: u32 },
    Acked,
}

/// Mensagens do protocolo
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PreviewMessage {
    #[serde(rename = "READY")]
    Ready { version: u32 },
    #[serde(rename = "DOC")]
    Doc { id: uuid::Uuid, version: u32, payload: CanonDocument },
    #[serde(rename = "ACK")]
    Ack { id: uuid::Uuid, version: u32 },
    #[serde(rename = "VERSION_MISMATCH")]
    VersionMismatch { expected: u32, received: u32 },
}

fn log_protocol(event: &str, id: Option<uuid::Uuid>, state: &str, version: u32, payload_len: usize) {
    leptos::logging::log!(
        "[CANON PROTOCOL] event={} id={} state={} version={} payload_len={}",
        event,
        id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string()),
        state,
        version,
        payload_len,
    );
}

fn send_to_iframe(json: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        if let Some(window) = web_sys::window() {
            if let Some(iframe) = window.document()
                .and_then(|d| d.get_element_by_id("canon-preview-frame"))
                .and_then(|el| el.dyn_into::<web_sys::HtmlIFrameElement>().ok())
            {
                if let Some(cw) = iframe.content_window() {
                    return cw.post_message(
                        &leptos::wasm_bindgen::JsValue::from_str(json),
                        "*"
                    ).is_ok();
                }
            }
        }
    }
    false
}

/// Serializa e envia DOC para o iframe
/// Retorna Some(json) se enviado, None se falhou
pub fn send_preview(
    slots: RwSignal<Vec<Node>>,
    tree: RwSignal<Vec<Node>>,
    active_layout: ActiveLayout,
    preview_state: RwSignal<PreviewState>,
    last_doc_json: RwSignal<Option<String>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let all_nodes = {
            let s = slots.get();
            let t = tree.get();
            let mut combined = s.clone();
            combined.extend(t.clone());
            combined
        };

        let canon_nodes = build_tree(&all_nodes);
        let doc_id = uuid::Uuid::new_v4();
        let doc = CanonDocument {
            id: doc_id,
            layout: format!("{:?}", active_layout),
            version: 1,
            nodes: canon_nodes,
        };

        let msg = PreviewMessage::Doc {
            id: doc_id,
            version: PROTOCOL_VERSION,
            payload: doc,
        };

        if let Ok(json) = serde_json::to_string(&msg) {
            log_protocol("DOC_SEND", Some(doc_id), "Sending", PROTOCOL_VERSION, json.len());
            last_doc_json.set(Some(json.clone()));

            if send_to_iframe(&json) {
                preview_state.set(PreviewState::Sending { retries: 0 });
            } else {
                log_protocol("DOC_SEND_FAILED", Some(doc_id), "Idle", PROTOCOL_VERSION, 0);
                preview_state.set(PreviewState::WaitingReady);
            }
        }
    }
}

/// Retry do último DOC se ACK não chegou
pub fn retry_if_needed(
    preview_state: RwSignal<PreviewState>,
    last_doc_json: RwSignal<Option<String>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        if let PreviewState::Sending { retries } = preview_state.get() {
            if retries >= MAX_RETRIES {
                log_protocol("RETRY_EXHAUSTED", None, "Idle", PROTOCOL_VERSION, 0);
                preview_state.set(PreviewState::Idle);
                return;
            }
            if let Some(json) = last_doc_json.get() {
                log_protocol("DOC_RETRY", None, "Sending", PROTOCOL_VERSION, json.len());
                if send_to_iframe(&json) {
                    preview_state.set(PreviewState::Sending { retries: retries + 1 });
                }
            }
        }
    }
}

pub fn init_preview_listener(
    slots: RwSignal<Vec<Node>>,
    tree: RwSignal<Vec<Node>>,
    active_layout: RwSignal<ActiveLayout>,
    preview_state: RwSignal<PreviewState>,
    last_doc_json: RwSignal<Option<String>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;

        // Evita registrar listener duplicado
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if doc.get_element_by_id("canon-preview-listener-registered").is_some() {
                    return;
                }
                if let Ok(marker) = doc.create_element("meta") {
                    let _ = marker.set_attribute("id", "canon-preview-listener-registered");
                    if let Some(head) = doc.head() {
                        let _ = head.append_child(&marker);
                    }
                }
            }
        }

        let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            // Segurança: aceita só mesma origin
            let origin_ok = web_sys::window()
                .and_then(|w| w.location().origin().ok())
                .map(|o| o == event.origin())
                .unwrap_or(false);
            if !origin_ok { return; }

            if let Some(data) = event.data().as_string() {
                match serde_json::from_str::<PreviewMessage>(&data) {
                    Ok(msg) => match msg {
                        PreviewMessage::Ready { version } => {
                            // Validação de versão
                            if version != PROTOCOL_VERSION {
                                log_protocol("VERSION_MISMATCH", None, "Idle", version, 0);
                                let mismatch = PreviewMessage::VersionMismatch {
                                    expected: PROTOCOL_VERSION,
                                    received: version,
                                };
                                if let Ok(json) = serde_json::to_string(&mismatch) {
                                    send_to_iframe(&json);
                                }
                                return;
                            }
                            log_protocol("READY_RECEIVED", None, "WaitingReady", version, 0);
                            preview_state.set(PreviewState::WaitingReady);

                            // Reenvia último DOC se existir, senão serializa novo
                            if let Some(json) = last_doc_json.get() {
                                log_protocol("DOC_RESEND", None, "Sending", PROTOCOL_VERSION, json.len());
                                if send_to_iframe(&json) {
                                    preview_state.set(PreviewState::Sending { retries: 0 });
                                }
                            } else {
                                send_preview(slots, tree, active_layout.get(), preview_state, last_doc_json);
                            }
                        }
                        PreviewMessage::Ack { id, version } => {
                            if version != PROTOCOL_VERSION {
                                log_protocol("ACK_VERSION_MISMATCH", Some(id), "Acked", version, 0);
                                return;
                            }
                            log_protocol("ACK_RECEIVED", Some(id), "Acked", version, 0);
                            preview_state.set(PreviewState::Acked);
                        }
                        _ => {}
                    },
                    Err(_) => {} // Ignora mensagens não-protocolo
                }
            }
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);

        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "message",
                closure.as_ref().unchecked_ref(),
            );
        }
        closure.forget();
    }
}
