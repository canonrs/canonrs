//! Runtime kernel — centralized ownership for all interaction resources
//!
//! Todos os crates de interação devem usar este módulo.
//! Zero cb.forget() fora deste módulo.
//!
//! Architecture:
//!   context   — RuntimeContext: typed mount interface
//!   lifecycle — formal lifecycle states (Mount/Hydrate/Active/Replay/Suspend/Destroy)
//!   ownership — resource ownership tree with cascading cleanup
//!   scheduler — RAF queue, observer batching, event coalescing
//!   listeners — owned event listener registry
//!   timers    — owned timeout/RAF handles
//!   bootstrap — centralized subtree init and replay
//!   cleanup   — component lifecycle destroy
//!   globals   — runtime-level global state
//!   subtree   — safe subtree operations

pub mod context;
pub mod lifecycle;
pub mod ownership;
pub mod scheduler;
pub mod listeners;
pub mod timers;
pub mod bootstrap;
pub mod cleanup;
pub mod globals;
pub mod subtree;

pub mod scrollspy;
