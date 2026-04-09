//! Cell agent implementation — repr(C) GPU-compatible structs

use crate::AgentState;

/// GPU-compatible cell agent state (repr(C) for CUDA interop)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CellAgent {
    pub id: u32,
    pub state: u32,       // 0=idle, 1=active, 2=waiting, 3=done
    pub confidence: f32,
    pub input_ptr: u64,   // GPU memory pointer
    pub output_ptr: u64,
    pub task_type: u32,   // 0=none, 1=inference, 2=reasoning, 3=coordination
    pub result_code: i32,
}

impl CellAgent {
    pub fn new(id: u32) -> Self {
        CellAgent { id, state: 0, confidence: 1.0, ..Default::default() }
    }
    pub fn is_active(&self) -> bool { self.state == 1 }
    pub fn assign_task(&mut self, task_type: u32, input_ptr: u64, output_ptr: u64) {
        self.state = 1; self.task_type = task_type;
        self.input_ptr = input_ptr; self.output_ptr = output_ptr;
    }
    pub fn complete(&mut self, result_code: i32) {
        self.state = 3; self.result_code = result_code;
    }
}

/// Agent pool managing multiple cell agents
pub struct AgentPool {
    pub agents: Vec<CellAgent>,
    pub next_id: u32,
}

impl AgentPool {
    pub fn new(capacity: usize) -> Self {
        let agents = (0..capacity).map(CellAgent::new).collect();
        AgentPool { agents, next_id: capacity as u32 }
    }
    pub fn acquire(&mut self) -> Option<&mut CellAgent> {
        self.agents.iter_mut().find(|a| a.state == 0)
    }
    pub fn active_count(&self) -> usize {
        self.agents.iter().filter(|a| a.is_active()).count()
    }
    pub fn avg_confidence(&self) -> f32 {
        let active: Vec<&CellAgent> = self.agents.iter().filter(|a| a.is_active()).collect();
        if active.is_empty() { return 0.0; }
        active.iter().map(|a| a.confidence).sum::<f32>() / active.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_agent_lifecycle() {
        let mut agent = CellAgent::new(0);
        assert!(!agent.is_active());
        agent.assign_task(1, 100, 200);
        assert!(agent.is_active());
        agent.complete(0);
        assert_eq!(agent.result_code, 0);
    }

    #[test]
    fn test_agent_pool() {
        let mut pool = AgentPool::new(64);
        let a = pool.acquire();
        assert!(a.is_some());
        assert_eq!(pool.active_count(), 0); // not assigned yet
    }
}
