//! DataTable Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/data_table.rs


#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataTableIslandColumn {
    pub key:   String,
    pub label: String,
}


