//! Ramify engine — PTX branch divergence management

/// Branch prediction outcome
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchOutcome {
    Taken,
    NotTaken,
    Divergent,
}

/// A branch point in the control flow
#[derive(Debug, Clone)]
pub struct BranchPoint {
    pub id: usize,
    pub address: usize,
    pub divergence_count: usize,
    pub total_branches: usize,
    pub last_outcome: BranchOutcome,
}

impl BranchPoint {
    pub fn divergence_rate(&self) -> f64 {
        if self.total_branches == 0 { return 0.0; }
        self.divergence_count as f64 / self.total_branches as f64
    }
}

/// Ramify engine for managing GPU branch divergence
pub struct RamifyEngine {
    pub branches: Vec<BranchPoint>,
    pub warp_size: usize,
    pub divergence_threshold: f64,
}

impl RamifyEngine {
    pub fn new(warp_size: usize) -> Self {
        RamifyEngine { branches: Vec::new(), warp_size, divergence_threshold: 0.3 }
    }

    /// Record a branch event
    pub fn record_branch(&mut self, address: usize, threads_taken: usize) -> BranchOutcome {
        let outcome = if threads_taken == 0 { BranchOutcome::NotTaken }
        else if threads_taken == self.warp_size { BranchOutcome::Taken }
        else { BranchOutcome::Divergent };

        // Find or create branch point
        let bp = self.branches.iter_mut()
            .find(|b| b.address == address);
        match bp {
            Some(bp) => {
                bp.total_branches += 1;
                bp.last_outcome = outcome;
                if outcome == BranchOutcome::Divergent { bp.divergence_count += 1; }
            }
            None => {
                self.branches.push(BranchPoint {
                    id: self.branches.len(),
                    address,
                    divergence_count: if outcome == BranchOutcome::Divergent { 1 } else { 0 },
                    total_branches: 1,
                    last_outcome: outcome,
                });
            }
        }
        outcome
    }

    /// Get branches with high divergence (optimization candidates)
    pub fn high_divergence_branches(&self) -> Vec<&BranchPoint> {
        self.branches.iter()
            .filter(|b| b.divergence_rate() > self.divergence_threshold)
            .collect()
    }

    /// Overall divergence score (0-1)
    pub fn overall_divergence(&self) -> f64 {
        if self.branches.is_empty() { return 0.0; }
        let total_div: usize = self.branches.iter().map(|b| b.divergence_count).sum();
        let total: usize = self.branches.iter().map(|b| b.total_branches).sum();
        total_div as f64 / total.max(1) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ramify_engine() {
        let mut engine = RamifyEngine::new(32);
        engine.record_branch(0x100, 32); // all taken
        engine.record_branch(0x100, 0);  // none taken
        engine.record_branch(0x100, 15); // divergent
        assert!(engine.overall_divergence() > 0.0);
        assert_eq!(engine.high_divergence_branches().len(), 1);
    }
}
