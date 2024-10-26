use std::collections::HashMap;

use crate::{constant::Constant, var::VarName};

pub type VarScope = HashMap<VarName, Constant>;
