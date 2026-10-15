// src/layout/input/chips/types.rs

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ChipContainerMode {
    #[default]
    SingleLine,
    MultiLine,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChipOverflow {
    Wrap,
    Scroll,
    Popup { max_visible: usize },
}

impl Default for ChipOverflow {
    fn default() -> Self {
        Self::Wrap
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChipItem {
    Simple {
        id: String,
        label: String,
    },
    Parametric {
        id: String,
        key: String,
        display_value: String,
    },
}

impl ChipItem {
    pub fn id(&self) -> &str {
        match self {
            Self::Simple { id, .. } | Self::Parametric { id, .. } => id,
        }
    }

    pub fn key_or_label(&self) -> &str {
        match self {
            Self::Simple { label, .. } => label,
            Self::Parametric { key, .. } => key,
        }
    }

    pub fn param_value(&self) -> Option<&str> {
        match self {
            Self::Simple { .. } => None,
            Self::Parametric { display_value, .. } => Some(display_value),
        }
    }

    /// Przypisuje parametr do chipa (zamienia Simple na Parametric lub aktualizuje istniejący)
    pub fn set_param_value(&mut self, val: String) {
        match self {
            Self::Simple { id, label } => {
                *self = Self::Parametric {
                    id: id.clone(),
                    key: label.clone(),
                    display_value: val,
                };
            }
            Self::Parametric { display_value, .. } => {
                *display_value = val;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChipLimits {
    pub max_total: Option<usize>,
    pub max_per_key: HashMap<String, usize>,
    pub allow_custom: bool,
}

impl Default for ChipLimits {
    fn default() -> Self {
        Self {
            max_total: None,
            max_per_key: HashMap::new(),
            allow_custom: true,
        }
    }
}

impl ChipLimits {
    pub fn can_add(&self, key: &str, current_items: &[ChipItem]) -> bool {
        if let Some(total_limit) = self.max_total {
            if current_items.len() >= total_limit {
                return false;
            }
        }

        if let Some(&per_key_limit) = self.max_per_key.get(key) {
            let count = current_items
                .iter()
                .filter(|item| item.key_or_label() == key)
                .count();
            if count >= per_key_limit {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChipAction {
    Added(ChipItem),
    Removed(String),
    Clicked(String),
    ParamEditRequested(String),
}