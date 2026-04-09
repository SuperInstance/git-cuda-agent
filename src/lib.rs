//! git-cuda-agent — GPU-accelerated agent template
//!
//! Combines cudaclaw patterns (cell agents, muscle fibers, Ramify,
//! SmartCRDT, DNA) with Cocapn fleet protocol (A2A, A2UI, vessel).

pub mod agent;
pub mod commands;
pub mod muscle;
pub mod ramify;
pub mod dna;
pub mod fleet;
pub mod crdt;
pub mod feedback;

use serde::{Deserialize, Serialize};

/// Agent state shared between CPU and GPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub name: String,
    pub vessel_class: String,
    pub confidence: f64,
    pub context_tokens: usize,
    pub active_tasks: usize,
    pub uptime_s: u64,
}

impl Default for AgentState {
    fn default() -> Self {
        AgentState {
            name: "agent".into(),
            vessel_class: "scout".into(),
            confidence: 1.0,
            context_tokens: 0,
            active_tasks: 0,
            uptime_s: 0,
        }
    }
}
