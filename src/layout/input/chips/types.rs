// src/layout/input/chips/types.rs

use std::collections::HashMap;
use super::chip_slot::ChipSlot;

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

#[derive(Clone, Debug)]
pub enum ChipItem {
    Simple {
        id: String,
        label: String,
    },
    Parametric {
        id: String,
        key: String,
        slot: ChipSlot,
        is_editing: bool,
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

    pub fn param_value(&self) -> Option<String> {
        match self {
            Self::Simple { .. } => None,
            Self::Parametric { slot, .. } => Some(slot.display_text()),
        }
    }

    pub fn convert_to_parametric(&mut self) {
        if let Self::Simple { id, label } = self {
            let slot = match label.as_str() {
                "Płeć" => ChipSlot::select("Kobieta", vec!["Kobieta".into(), "Mężczyzna".into()]),
                "Wiek" => ChipSlot::range(18.0, 65.0, 0.0, 120.0),
                _ => ChipSlot::text(""),
            };

            let is_editing = !matches!(slot, ChipSlot::Select(_));

            *self = Self::Parametric {
                id: id.clone(),
                key: label.clone(),
                slot,
                is_editing,
            };
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
    Added(String),
    Removed(String),
    Clicked(String),
    ParamChanged { id: String, display_value: String },
}