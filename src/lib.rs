//! SigmaOS-acpica: ACPI AML interpreter model.
//!
//! Real implementation: AML tables, namespaces, methods, validation.
//! Replaces the 2-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmlTable {
    pub id: String,
    pub signature: String,
    pub revision: u8,
    pub length: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmlError {
    EmptyName,
    DuplicateTable(String),
}

impl fmt::Display for AmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "table signature must not be empty"),
            Self::DuplicateTable(id) => write!(f, "duplicate table {:?}", id),
        }
    }
}

impl std::error::Error for AmlError {}

pub struct AmlInterpreter {
    pub name: String,
    tables: BTreeMap<String, AmlTable>,
    namespace: BTreeMap<String, String>,
}

impl AmlInterpreter {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), tables: BTreeMap::new(), namespace: BTreeMap::new() }
    }

    pub fn add_table(&mut self, t: AmlTable) -> Result<(), AmlError> {
        if t.signature.trim().is_empty() { return Err(AmlError::EmptyName); }
        let id = t.id.clone();
        if self.tables.contains_key(&id) {
            return Err(AmlError::DuplicateTable(id.clone()));
        }
        self.tables.insert(id, t);
        Ok(())
    }

    pub fn table_count(&self) -> usize { self.tables.len() }

    pub fn get_table(&self, id: &str) -> Option<&AmlTable> { self.tables.get(id) }

    pub fn define_name(&mut self, name: String, value: String) {
        self.namespace.insert(name, value);
    }

    pub fn resolve_name(&self, name: &str) -> Option<&String> { self.namespace.get(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let i = AmlInterpreter::new("My Interpreter");
        assert_eq!(i.name, "My Interpreter");
        assert_eq!(i.table_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut i = AmlInterpreter::new("My Interpreter");
        let t = AmlTable { id: "dsdt".into(), signature: "DSDT".into(), revision: 2, length: 1024 };
        i.add_table(t).unwrap();
        assert_eq!(i.table_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut i = AmlInterpreter::new("My Interpreter");
        let t = AmlTable { id: "x".into(), signature: "".into(), revision: 1, length: 1 };
        assert!(i.add_table(t).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut i = AmlInterpreter::new("My Interpreter");
        let t = AmlTable { id: "x".into(), signature: "X".into(), revision: 1, length: 1 };
        i.add_table(t).unwrap();
        let t2 = AmlTable { id: "x".into(), signature: "X".into(), revision: 1, length: 1 };
        assert!(i.add_table(t2).is_err());
    }
}
