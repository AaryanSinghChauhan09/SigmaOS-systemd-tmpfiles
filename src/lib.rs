// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-tmpfiles: systemd tmpfiles component model.
//!
//! Real implementation: tmpfiless, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tmpfiles {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdTmpfilesError {
    EmptyTmpfilesName,
    DuplicateTmpfiles(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdTmpfilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTmpfilesName => write!(f, "tmpfiles name must not be empty"),
            Self::DuplicateTmpfiles(id) => write!(f, "duplicate tmpfiles {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdTmpfilesError {}

pub struct SystemdTmpfiles {
    pub name: String,
    tmpfiless: BTreeMap<String, Tmpfiles>,
    events: BTreeMap<String, Event>,
}

impl SystemdTmpfiles {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tmpfiless: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_tmpfiles(&mut self, item: Tmpfiles) -> Result<(), SystemdTmpfilesError> {
        if item.name.trim().is_empty() {
            return Err(SystemdTmpfilesError::EmptyTmpfilesName);
        }
        let id = item.id.clone();
        if self.tmpfiless.contains_key(&id) {
            return Err(SystemdTmpfilesError::DuplicateTmpfiles(id.clone()));
        }
        self.tmpfiless.insert(id, item);
        Ok(())
    }

    pub fn tmpfiles_count(&self) -> usize {
        self.tmpfiless.len()
    }

    pub fn get_tmpfiles(&self, id: &str) -> Option<&Tmpfiles> {
        self.tmpfiless.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdTmpfilesError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdTmpfilesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tmpfiles(id: &str, name: &str) -> Tmpfiles {
        Tmpfiles {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdTmpfiles::new("SystemdTmpfiles");
        assert_eq!(m.name, "SystemdTmpfiles");
        assert_eq!(m.tmpfiles_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_tmpfiles() {
        let mut m = SystemdTmpfiles::new("SystemdTmpfiles");
        m.add_tmpfiles(sample_tmpfiles("a1", "Alpha")).unwrap();
        assert_eq!(m.tmpfiles_count(), 1);
        assert!(m.get_tmpfiles("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdTmpfiles::new("SystemdTmpfiles");
        assert!(m.add_tmpfiles(sample_tmpfiles("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdTmpfiles::new("SystemdTmpfiles");
        m.add_tmpfiles(sample_tmpfiles("x", "One")).unwrap();
        assert!(m.add_tmpfiles(sample_tmpfiles("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdTmpfiles::new("SystemdTmpfiles");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
