//! SystemState — estado global do pipeline

#[derive(Default, Clone)]
pub struct SystemState {
    pub tokens: String,
    pub wasm:   String,
    pub leptos: String,
}

impl SystemState {
    pub fn print(&self) {
        println!("\n┌─ CANON SYSTEM STATE ─────────────────");
        println!("│  tokens : {}", self.tokens);
        println!("│  wasm   : {}", self.wasm);
        println!("│  leptos : {}", self.leptos);
        println!("└──────────────────────────────────────\n");
    }
}
