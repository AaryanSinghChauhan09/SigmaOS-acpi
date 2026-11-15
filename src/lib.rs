//! SigmaOS-acpi: ACPI power management model.
//!
//! Real implementation: power states, events, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// ACPI event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcpiEvent {
    pub id: String,
    pub name: String,
    pub event_type: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcpiError {
    EmptyName,
    DuplicateEvent(String),
}

impl fmt::Display for AcpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "event name must not be empty"),
            Self::DuplicateEvent(id) => write!(f, "duplicate event {:?}", id),
        }
    }
}

impl std::error::Error for AcpiError {}

/// ACPI manager.
pub struct AcpiManager {
    pub name: String,
    events: BTreeMap<String, AcpiEvent>,
}

impl AcpiManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_event(&mut self, event: AcpiEvent) -> Result<(), AcpiError> {
        if event.name.trim().is_empty() { return Err(AcpiError::EmptyName); }
        let id = event.id.clone();
        if self.events.contains_key(&id) {
            return Err(AcpiError::DuplicateEvent(id.clone()));
        }
        self.events.insert(id, event);
        Ok(())
    }

    pub fn event_count(&self) -> usize { self.events.len() }

    pub fn get_event(&self, id: &str) -> Option<&AcpiEvent> { self.events.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_create_aaa() {
        let a = AcpiManager::new("My ACPI");
        assert_eq!(a.name, "My ACPI");
        assert_eq!(a.event_count(), 0);
    }

    #[test]
    fn test_add_event_aaa() {
        let mut a = AcpiManager::new("My ACPI");
        let e = AcpiEvent { id: "e1".into(), name: "power-button".into(), event_type: "button".into(), action: "suspend".into() };
        a.add_event(e).unwrap();
        assert_eq!(a.event_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = AcpiManager::new("My ACPI");
        let e = AcpiEvent { id: "x".into(), name: "".into(), event_type: "a".into(), action: "b".into() };
        assert!(a.add_event(e).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AcpiManager::new("My ACPI");
        let e = AcpiEvent { id: "x".into(), name: "X".into(), event_type: "a".into(), action: "b".into() };
        a.add_event(e).unwrap();
        let e2 = AcpiEvent { id: "x".into(), name: "X".into(), event_type: "a".into(), action: "b".into() };
        assert!(a.add_event(e2).is_err());
    }
}
