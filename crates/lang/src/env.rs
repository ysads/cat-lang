use crate::{statement::Statement, val::Val};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum NamedInfo {
    Binding(Val),
    Func {
        params: Vec<String>,
        body: Statement,
    },
}

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    named_elements: HashMap<String, NamedInfo>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            named_elements: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn add_binding(&mut self, name: String, val: Val) {
        self.named_elements.insert(name, NamedInfo::Binding(val));
    }

    pub(crate) fn add_func(&mut self, name: String, params: Vec<String>, body: Statement) {
        self.named_elements
            .insert(name, NamedInfo::Func { params, body });
    }

    pub(crate) fn get_binding(&self, name: &str) -> Result<Val, String> {
        self.get_named_info(name)
            .and_then(|named_info| match named_info {
                NamedInfo::Binding(val) => Some(val),
                _ => None,
            })
            .ok_or_else(|| format!("Binding with name `{}` not found", name))
    }

    pub(crate) fn get_func(&self, name: &str) -> Result<(Vec<String>, Statement), String> {
        self.get_named_info(name)
            .and_then(|named_info| match named_info {
                NamedInfo::Func { params, body } => Some((params, body)),
                _ => None,
            })
            .ok_or_else(|| format!("Function with name `{}` not found", name))
    }

    fn get_named_info(&self, name: &str) -> Option<NamedInfo> {
        self.named_elements
            .get(name)
            .cloned()
            .or_else(|| self.parent.and_then(|parent| parent.get_named_info(name)))
    }
}
