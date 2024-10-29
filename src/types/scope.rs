use std::collections::HashMap;

use crate::{constant::Constant, var::VarName};

// pub type VarScope = HashMap<VarName, Constant>;
pub type VarScope<'a> = VarValue<'a, Constant>;

pub struct VarValue<'a, T> {
    pub mappings: HashMap<VarName, T>,
    pub fallback: Option<&'a Self>,
}

impl<'a, T> VarValue<'a, T> {
    pub fn get(&self, key: &VarName) -> Option<&T> {
        self.mappings.get(key).or_else(|| {
            if let Some(fallback) = &self.fallback {
                fallback.get(key)
            } else {
                None
            }
        })
    }

    pub fn get_key_value(&self, key: &VarName) -> Option<(&VarName, &T)> {
        self.mappings.get_key_value(key).or_else(|| {
            if let Some(fallback) = &self.fallback {
                fallback.get_key_value(key)
            } else {
                None
            }
        })
    }
}
