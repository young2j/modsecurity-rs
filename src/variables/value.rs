use std::collections::LinkedList;

use super::VariableOrigin;

pub struct VariableValue {
    m_orign: LinkedList<VariableOrigin>,
    m_collection: String,
    m_key: String,
    m_key_with_collection: String,
    m_value: String,
}

impl VariableValue {
    pub fn new(collection_name: &str, key: &str, value: &str) -> VariableValue {
        let key_with_collection = if collection_name.len() == 0 {
            key.to_string()
        } else {
            format!("{}:{}", collection_name, key)
        };
        VariableValue {
            m_key_with_collection: key_with_collection,
            m_key: key.to_string(),
            m_value: value.to_string(),
            m_collection: collection_name.to_string(),
            m_orign: LinkedList::new(),
        }
    }
    pub fn from_variable_value(&mut self, vv: VariableValue) {
        self.m_collection = vv.m_collection;
        self.m_key = vv.m_key;
        self.m_key_with_collection = vv.m_key_with_collection;
        self.m_value = vv.m_value;
        self.m_orign.extend(vv.m_orign.into_iter());
    }

    pub fn get_key(&self) -> &str {
        &self.m_key
    }

    pub fn get_key_with_collection(&self) -> &str {
        &self.m_key_with_collection
    }

    pub fn get_collection(&self) -> &str {
        &self.m_collection
    }

    pub fn get_value(&self) -> &str {
        &self.m_value
    }

    pub fn set_value(&mut self, value: &str) {
        self.m_value = value.to_string();
    }

    pub fn add_origin(&mut self, vo: VariableOrigin) {
        self.m_orign.push_back(vo);
    }

    pub fn get_origin(&self) -> &LinkedList<VariableOrigin> {
        &self.m_orign
    }
}
