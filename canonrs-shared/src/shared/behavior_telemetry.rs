//! Telemetry & Observability Layer
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Níveis de log estruturado
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TelemetryLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

/// Evento de telemetria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub timestamp: i64,
    pub level: TelemetryLevel,
    pub behavior_name: String,
    pub event_type: TelemetryEventType,
    pub metadata: serde_json::Value,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TelemetryEventType {
    Attached { element_id: String },
    Detached { element_id: String },
    Error { error: String },
    Performance { operation: String },
    UserInteraction { action: String },
}

/// Coletor de métricas de performance
#[derive(Debug)]
pub struct PerformanceTracker {
    start: Instant,
    operation: String,
}

impl PerformanceTracker {
    pub fn start(operation: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.into(),
        }
    }
    
    pub fn finish(self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

/// Singleton global para coletar telemetria
pub struct TelemetryCollector {
    events: std::sync::Mutex<Vec<TelemetryEvent>>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self {
            events: std::sync::Mutex::new(Vec::new()),
        }
    }
    
    pub fn record(&self, event: TelemetryEvent) {
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
            
            if events.len() > 1000 {
                events.drain(0..500);
            }
        }
    }
    
    pub fn drain(&self) -> Vec<TelemetryEvent> {
        self.events.lock().ok().map(|mut e| e.drain(..).collect()).unwrap_or_default()
    }
    
    pub fn get_metrics(&self) -> TelemetryMetrics {
        let events = self.events.lock().ok().map(|e| e.clone()).unwrap_or_default();
        
        let total_events = events.len();
        let errors = events.iter().filter(|e| e.level == TelemetryLevel::Error).count();
        let avg_duration = events
            .iter()
            .filter_map(|e| e.duration_ms)
            .sum::<u64>() as f64
            / total_events.max(1) as f64;
        
        TelemetryMetrics {
            total_events,
            errors,
            avg_duration_ms: avg_duration,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TelemetryMetrics {
    pub total_events: usize,
    pub errors: usize,
    pub avg_duration_ms: f64,
}

thread_local! {
    pub static TELEMETRY: TelemetryCollector = TelemetryCollector::new();
}
