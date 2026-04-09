//! DNA config parser — .claw-dna file handling

use std::collections::HashMap;

/// Parsed DNA configuration
#[derive(Debug, Clone)]
pub struct DnaConfig {
    pub name: String,
    pub vessel_class: String,
    pub personality: HashMap<String, f64>,
    pub capabilities: HashMap<String, usize>,
    pub gpu: GpuConfig,
    pub fleet: FleetConfig,
}

#[derive(Debug, Clone)]
pub struct GpuConfig {
    pub persistent_kernel: bool,
    pub cell_agent_count: usize,
    pub muscle_fiber_count: usize,
    pub ramify_enabled: bool,
    pub smart_crdt: bool,
}

#[derive(Debug, Clone)]
pub struct FleetConfig {
    pub a2a_enabled: bool,
    pub a2ui_enabled: bool,
    pub heartbeat_s: usize,
}

impl Default for DnaConfig {
    fn default() -> Self {
        let mut personality = HashMap::new();
        personality.insert("verbosity".into(), 0.7);
        personality.insert("curiosity".into(), 0.8);
        personality.insert("caution".into(), 0.6);
        personality.insert("persistence".into(), 0.9);
        let mut caps = HashMap::new();
        caps.insert("max_context_tokens".into(), 4096);
        caps.insert("max_output_tokens".into(), 1024);
        caps.insert("temperature_x100".into(), 70);
        caps.insert("top_p_x100".into(), 90);
        DnaConfig {
            name: "agent".into(), vessel_class: "scout".into(),
            personality, capabilities: caps,
            gpu: GpuConfig { persistent_kernel: true, cell_agent_count: 64,
                muscle_fiber_count: 4, ramify_enabled: true, smart_crdt: true },
            fleet: FleetConfig { a2a_enabled: true, a2ui_enabled: true, heartbeat_s: 30 },
        }
    }
}

/// Simple .claw-dna parser (TOML-like)
pub struct DnaParser;

impl DnaParser {
    pub fn parse(content: &str) -> DnaConfig {
        let mut config = DnaConfig::default();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() { continue; }
            if let Some(key) = line.strip_prefix("name = ") {
                config.name = key.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("vessel_class = ") {
                config.vessel_class = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("verbosity = ") {
                if let Ok(v) = val.parse() { config.personality.insert("verbosity".into(), v); }
            } else if let Some(val) = line.strip_prefix("curiosity = ") {
                if let Ok(v) = val.parse() { config.personality.insert("curiosity".into(), v); }
            } else if let Some(val) = line.strip_prefix("caution = ") {
                if let Ok(v) = val.parse() { config.personality.insert("caution".into(), v); }
            } else if let Some(val) = line.strip_prefix("cell_agent_count = ") {
                if let Ok(v) = val.parse() { config.gpu.cell_agent_count = v; }
            }
        }
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_parser() {
        let dna = r#"
name = "test-agent"
vessel_class = "captain"
verbosity = 0.3
curiosity = 0.95
cell_agent_count = 128
"#;
        let config = DnaParser::parse(dna);
        assert_eq!(config.name, "test-agent");
        assert_eq!(config.vessel_class, "captain");
        assert_eq!(config.gpu.cell_agent_count, 128);
    }
}
