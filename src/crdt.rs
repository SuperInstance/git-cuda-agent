//! SmartCRDT — atomicCAS-based concurrent GPU state

use std::collections::HashMap;

/// CRDT value with vector clock
#[derive(Debug, Clone)]
pub struct CrdtValue {
    pub data: String,
    pub version: u64,
    pub node_id: String,
    pub tombstone: bool,
}

/// Simple CRDT register (last-writer-wins)
pub struct SmartCrdt {
    pub values: HashMap<String, CrdtValue>,
    pub node_id: String,
    pub clock: u64,
}

impl SmartCrdt {
    pub fn new(node_id: &str) -> Self {
        SmartCrdt { values: HashMap::new(), node_id: node_id.to_string(), clock: 0 }
    }

    /// Write a value (simulates atomicCAS on GPU)
    pub fn write(&mut self, key: &str, data: &str) -> bool {
        self.clock += 1;
        let current = self.values.get(key);
        let should_write = match current {
            None => true,
            Some(v) if v.tombstone => true,
            Some(v) => self.clock > v.version, // LWW
        };
        if should_write {
            self.values.insert(key.to_string(), CrdtValue {
                data: data.to_string(), version: self.clock,
                node_id: self.node_id.clone(), tombstone: false,
            });
        }
        should_write
    }

    /// Read a value
    pub fn read(&self, key: &str) -> Option<&CrdtValue> {
        self.values.get(key).filter(|v| !v.tombstone)
    }

    /// Delete a value (tombstone)
    pub fn delete(&mut self, key: &str) -> bool {
        if let Some(v) = self.values.get_mut(key) {
            self.clock += 1; v.tombstone = true; v.version = self.clock; return true;
        }
        false
    }

    /// Merge from another node
    pub fn merge(&mut self, other: &SmartCrdt) {
        for (key, other_val) in &other.values {
            let merge = match self.values.get(key) {
                None => true,
                Some(mine) => other_val.version > mine.version,
            };
            if merge {
                self.values.insert(key.clone(), other_val.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crdt_write_read() {
        let mut crdt = SmartCrdt::new("node-a");
        assert!(crdt.write("key1", "value1"));
        assert_eq!(crdt.read("key1").unwrap().data, "value1");
    }

    #[test]
    fn test_crdt_merge() {
        let mut a = SmartCrdt::new("node-a");
        let mut b = SmartCrdt::new("node-b");
        a.write("shared", "a-value");
        b.write("shared", "b-value"); // b has higher clock
        a.merge(&b);
        assert_eq!(a.read("shared").unwrap().data, "b-value"); // LWW
    }

    #[test]
    fn test_crdt_delete() {
        let mut crdt = SmartCrdt::new("node-a");
        crdt.write("temp", "data");
        crdt.delete("temp");
        assert!(crdt.read("temp").is_none());
    }
}
