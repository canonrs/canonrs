//! @canon-level: strict
//! @canon-owner: primitives-team
//! ChartData — domínio de dados do Chart (separado da UI)

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChartSeries {
    pub name: String,
    pub data: Vec<f64>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub series: Vec<ChartSeries>,
}

impl ChartData {
    pub fn to_json(&self) -> String {
        let labels: Vec<String> = self.labels.iter()
            .map(|l| format!(r#""{}""#, l.replace('"', "\\\"")))
            .collect();
        let series: Vec<String> = self.series.iter().map(|s| {
            let data: Vec<String> = s.data.iter().map(|d| d.to_string()).collect();
            let color = s.color.as_deref()
                .map(|c| format!(r#","color":"{}""#, c))
                .unwrap_or_default();
            format!(r#"{{"name":"{}","data":[{}]{}}}"#,
                s.name.replace('"', "\\\""),
                data.join(","),
                color)
        }).collect();
        format!(r#"{{"labels":[{}],"series":[{}]}}"#, labels.join(","), series.join(","))
    }
}
