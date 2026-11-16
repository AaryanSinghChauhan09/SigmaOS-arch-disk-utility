// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-arch-disk-utility: arch disk utility component model.
//!
//! Real implementation: disks, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disk {
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
pub enum ArchDiskUtilityError {
    EmptyDiskName,
    DuplicateDisk(String),
    EmptyEventSubject,
}

impl fmt::Display for ArchDiskUtilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDiskName => write!(f, "disk name must not be empty"),
            Self::DuplicateDisk(id) => write!(f, "duplicate disk {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArchDiskUtilityError {}

pub struct ArchDiskUtility {
    pub name: String,
    disks: BTreeMap<String, Disk>,
    events: BTreeMap<String, Event>,
}

impl ArchDiskUtility {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            disks: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_disk(&mut self, item: Disk) -> Result<(), ArchDiskUtilityError> {
        if item.name.trim().is_empty() {
            return Err(ArchDiskUtilityError::EmptyDiskName);
        }
        let id = item.id.clone();
        if self.disks.contains_key(&id) {
            return Err(ArchDiskUtilityError::DuplicateDisk(id.clone()));
        }
        self.disks.insert(id, item);
        Ok(())
    }

    pub fn disk_count(&self) -> usize {
        self.disks.len()
    }

    pub fn get_disk(&self, id: &str) -> Option<&Disk> {
        self.disks.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArchDiskUtilityError> {
        if event.subject.trim().is_empty() {
            return Err(ArchDiskUtilityError::EmptyEventSubject);
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

    fn sample_disk(id: &str, name: &str) -> Disk {
        Disk {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ArchDiskUtility::new("ArchDiskUtility");
        assert_eq!(m.name, "ArchDiskUtility");
        assert_eq!(m.disk_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_disk() {
        let mut m = ArchDiskUtility::new("ArchDiskUtility");
        m.add_disk(sample_disk("a1", "Alpha")).unwrap();
        assert_eq!(m.disk_count(), 1);
        assert!(m.get_disk("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ArchDiskUtility::new("ArchDiskUtility");
        assert!(m.add_disk(sample_disk("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ArchDiskUtility::new("ArchDiskUtility");
        m.add_disk(sample_disk("x", "One")).unwrap();
        assert!(m.add_disk(sample_disk("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ArchDiskUtility::new("ArchDiskUtility");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
