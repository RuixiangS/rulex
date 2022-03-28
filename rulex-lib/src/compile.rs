use std::collections::{HashMap, HashSet};

use crate::{error::CompileError, regex::Regex, repetition::RegexQuantifier, Rulex};

pub(crate) type CompileResult<'i> = Result<Regex<'i>, CompileError>;

#[derive(Debug, Clone)]
pub(crate) struct CompileState<'c, 'i> {
    pub(crate) next_idx: u32,
    pub(crate) used_names: HashMap<String, u32>,
    pub(crate) groups_count: u32,

    pub(crate) default_quantifier: RegexQuantifier,
    pub(crate) variables: Vec<(&'i str, &'c Rulex<'i>)>,
    pub(crate) current_vars: HashSet<usize>,
}
