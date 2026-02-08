use serde::{Serialize, Deserialize};
use std::collections::HashSet;

/// SelectionMode - How items can be selected
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    /// Only one item can be selected at a time
    Single,
    
    /// Multiple items can be selected
    Multiple,
    
    /// No selection allowed
    None,
}

/// SelectionState - Current selection state
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionState<T: Clone + PartialEq + Eq + std::hash::Hash> {
    /// No items selected
    Empty,
    
    /// Single item selected
    Single(T),
    
    /// Multiple items selected
    Multiple(HashSet<T>),
}

impl<T: Clone + PartialEq + Eq + std::hash::Hash> SelectionState<T> {
    pub fn new() -> Self {
        Self::Empty
    }
    
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
    
    pub fn is_selected(&self, item: &T) -> bool {
        match self {
            Self::Empty => false,
            Self::Single(id) => id == item,
            Self::Multiple(set) => set.contains(item),
        }
    }
    
    pub fn count(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Single(_) => 1,
            Self::Multiple(set) => set.len(),
        }
    }
    
    pub fn get_all(&self) -> Vec<T> {
        match self {
            Self::Empty => vec![],
            Self::Single(id) => vec![id.clone()],
            Self::Multiple(set) => set.iter().cloned().collect(),
        }
    }
    
    pub fn toggle(&mut self, item: T) {
        match self {
            Self::Empty => {
                *self = Self::Single(item);
            }
            Self::Single(current) => {
                if current == &item {
                    *self = Self::Empty;
                } else {
                    let mut set = HashSet::new();
                    set.insert(current.clone());
                    set.insert(item);
                    *self = Self::Multiple(set);
                }
            }
            Self::Multiple(set) => {
                if set.contains(&item) {
                    set.remove(&item);
                    if set.is_empty() {
                        *self = Self::Empty;
                    } else if set.len() == 1 {
                        *self = Self::Single(set.iter().next().unwrap().clone());
                    }
                } else {
                    set.insert(item);
                }
            }
        }
    }
    
    pub fn add(&mut self, item: T) {
        match self {
            Self::Empty => {
                *self = Self::Single(item);
            }
            Self::Single(current) => {
                if current != &item {
                    let mut set = HashSet::new();
                    set.insert(current.clone());
                    set.insert(item);
                    *self = Self::Multiple(set);
                }
            }
            Self::Multiple(set) => {
                set.insert(item);
            }
        }
    }
    
    pub fn remove(&mut self, item: &T) {
        match self {
            Self::Empty => {}
            Self::Single(current) => {
                if current == item {
                    *self = Self::Empty;
                }
            }
            Self::Multiple(set) => {
                set.remove(item);
                if set.is_empty() {
                    *self = Self::Empty;
                } else if set.len() == 1 {
                    *self = Self::Single(set.iter().next().unwrap().clone());
                }
            }
        }
    }
    
    pub fn clear(&mut self) {
        *self = Self::Empty;
    }
    
    pub fn set_single(&mut self, item: T) {
        *self = Self::Single(item);
    }
}

impl<T: Clone + PartialEq + Eq + std::hash::Hash> Default for SelectionState<T> {
    fn default() -> Self {
        Self::Empty
    }
}
