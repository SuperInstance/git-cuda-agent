# Future Integration: git-cuda-agent

## Current State
A template repository combining the cudaclaw GPU compute framework with the Cocapn fleet protocol. Provides GPU-accelerated agent execution with Cell Agents (48-byte GPU structs), Muscle Fibers (named kernel configs), and a Ramify Engine for runtime kernel specialization via NVRTC. "Hello world" of GPU-native agents.

## Integration Opportunities

### With forgemaster
git-cuda-agent IS the Forgemaster's agent template. The Forgemaster manages the GPU fleet; git-cuda-agent provides the per-agent CUDA kernel structure. The Cell Agent struct (`#[repr(C)]`, 48 bytes) becomes the standard unit of GPU-resident computation in the Forgemaster's simulation grid. The Ramify Engine's runtime kernel specialization (NVRTC, 10-50ms compile) becomes the Forgemaster's adaptive optimization layer.

### With ternary-cell
Cell Agents on GPU map directly to ternary cells in the CellGrid. Each ternary cell's 6-phase tick (predict → perceive → surprise → vibe → gc → conservation) becomes a GPU kernel launch. The Muscle Fiber system (named kernel configs with block size, shared memory, register budgets) maps to different tick strategies: `cell_update` for predict/perceive, `crdt_merge` for vibe, `formula_eval` for surprise computation.

### With room-as-codespace
When a room needs GPU acceleration (e.g., the "simulation" room running 1M ternary cells), it spins up a Codespace with git-cuda-agent as the template. The Codespace inherits the CUDA framework, connects to PLATO for LLM proxy calls, and begins ticking. This is the GPU room pattern.

## Dormant Ideas Now Unlockable
The DNA Layer (`.claw-dna` files defining agent personality) was conceptually interesting but had no runtime. Now construct-core's `SkillSpec` and ternary-registry's capability declarations provide the runtime. Agent personality becomes a combination of loaded skills and ternary strategy vectors — the DNA is real.

## Potential in Mature Systems
Every GPU in the fleet (DGX, Jetson, RTX 4050) runs git-cuda-agent as the base layer. The Forgemaster orchestrates which agents run on which GPU, the Ramify Engine specializes kernels per-hardware, and the Cocapn protocol coordinates between GPU islands. This is the fleet's GPU backbone.

## Cross-Pollination Ideas
- **ptx-bench**: Benchmarks inform Ramify Engine's kernel specialization decisions
- **cudaclaw-1**: git-cuda-agent is built on cudaclaw; both evolve together
- **tile-cuda**: Tile CUDA kernels complement Cell Agent kernels for different workloads
- **JetsonClaw1-vessel**: Edge GPU deployment pattern from git-cuda-agent

## Dependencies for Next Steps
- Integration with ternary-cell's tick cycle as GPU kernel
- Forgemaster orchestration layer
- Room-as-codespace GPU room template
