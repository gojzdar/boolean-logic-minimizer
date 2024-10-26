use std::collections::HashMap;

use crate::{constant::Constant, var::VarName};

// pub type VarScope = HashMap<VarName, Constant>;

pub struct VarScope<'a> {
    pub mappings: HashMap<VarName, Constant>,
    pub fallback: Option<&'a VarScope<'a>>,
}
