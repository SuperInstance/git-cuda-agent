//! Muscle fibers — SIMD-parallel compute paths

/// Fiber types for different compute patterns
#[derive(Debug, Clone, Copy)]
pub enum FiberType {
    Scalar,      // Single value operations
    Simd4,       // 4-wide SIMD
    Simd8,       // 8-wide SIMD
    Simd16,      // 16-wide SIMD
    TensorCore,  // Matrix multiply (4x4 chunks)
    Memory,      // Async memory transfer
}

/// A muscle fiber representing a parallel compute path
#[derive(Debug, Clone)]
pub struct MuscleFiber {
    pub fiber_type: FiberType,
    pub width: usize,          // SIMD width
    pub throughput_ops_per_cycle: f64,
    pub active: bool,
    pub current_task: String,
}

impl MuscleFiber {
    pub fn new(fiber_type: FiberType) -> Self {
        let (width, ops) = match fiber_type {
            FiberType::Scalar => (1, 1.0),
            FiberType::Simd4 => (4, 4.0),
            FiberType::Simd8 => (8, 8.0),
            FiberType::Simd16 => (16, 16.0),
            FiberType::TensorCore => (16, 64.0), // 4x4x4 MACs
            FiberType::Memory => (1, 0.5),
        };
        MuscleFiber { fiber_type, width, throughput_ops_per_cycle: ops, active: false, current_task: String::new() }
    }
    pub fn assign(&mut self, task: &str) { self.active = true; self.current_task = task.into(); }
    pub fn release(&mut self) { self.active = false; self.current_task.clear(); }
}

/// Muscle manager — orchestrates all fibers
pub struct MuscleManager {
    pub fibers: Vec<MuscleFiber>,
}

impl MuscleManager {
    pub fn new() -> Self {
        let fibers = vec![
            MuscleFiber::new(FiberType::TensorCore),
            MuscleFiber::new(FiberType::Simd16),
            MuscleFiber::new(FiberType::Simd8),
            MuscleFiber::new(FiberType::Memory),
        ];
        MuscleManager { fibers }
    }
    pub fn total_throughput(&self) -> f64 {
        self.fibers.iter().filter(|f| f.active).map(|f| f.throughput_ops_per_cycle).sum()
    }
    pub fn available_count(&self) -> usize {
        self.fibers.iter().filter(|f| !f.active).count()
    }
    pub fn acquire_best(&mut self) -> Option<&mut MuscleFiber> {
        self.fibers.iter_mut()
            .filter(|f| !f.active)
            .max_by_key(|f| f.throughput_ops_per_cycle as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muscle_manager() {
        let mut mgr = MuscleManager::new();
        assert_eq!(mgr.available_count(), 4);
        if let Some(fiber) = mgr.acquire_best() {
            fiber.assign("inference");
        }
        assert_eq!(mgr.available_count(), 3);
    }
}
