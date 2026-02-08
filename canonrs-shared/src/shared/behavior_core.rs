//! Core types para behavior system - SEM dependências web

use std::fmt::Debug;

/// Configuração de behavior (pode ser usada em SSR e CSR)
#[derive(Debug, Clone)]
pub struct BehaviorConfig {
    pub auto_init: bool,
    pub retry_on_error: bool,
    pub max_retries: u32,
    pub debounce_ms: u32,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            auto_init: true,
            retry_on_error: false,
            max_retries: 3,
            debounce_ms: 100,
        }
    }
}

/// Builder pattern
#[derive(Default)]
pub struct BehaviorConfigBuilder {
    config: BehaviorConfig,
}

impl BehaviorConfigBuilder {
    pub fn auto_init(mut self, value: bool) -> Self {
        self.config.auto_init = value;
        self
    }
    
    pub fn retry_on_error(mut self, value: bool) -> Self {
        self.config.retry_on_error = value;
        self
    }
    
    pub fn max_retries(mut self, value: u32) -> Self {
        self.config.max_retries = value;
        self
    }
    
    pub fn debounce_ms(mut self, value: u32) -> Self {
        self.config.debounce_ms = value;
        self
    }
    
    pub fn build(self) -> BehaviorConfig {
        self.config
    }
}
