use std::collections::LinkedList;

use super::VariableOrigin;

pub struct VariableValue {
    m_origin: LinkedList<VariableOrigin>,
    m_collection: String,
    m_key: String,
    m_key_with_collection: String,
    m_value: String,
}

impl VariableValue {
    pub fn new(key: &str, value: Option<&str>) -> VariableValue {
        let m_value = if let Some(v) = value {
            v.to_string()
        } else {
            "".to_string()
        };

        VariableValue {
            m_origin: LinkedList::new(),
            m_collection: String::new(),
            m_key: String::from(key),
            m_key_with_collection: String::from(key),
            m_value,
        }
    }

    pub fn new_with_collection(collection_name: &str, key: &str, value: &str) -> VariableValue {
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
            m_origin: LinkedList::new(),
        }
    }

    pub fn new_from(vv: &VariableValue) -> VariableValue {
        let m_origin = LinkedList::from_iter(
            vv.m_origin
                .iter()
                .map(|o| VariableOrigin::new_with_props(o.m_length, o.m_offset)),
        );

        VariableValue {
            m_origin,
            m_collection: String::from(&vv.m_collection),
            m_key: String::from(&vv.m_key),
            m_key_with_collection: String::from(&vv.m_key_with_collection),
            m_value: String::from(&vv.m_value),
        }
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

    pub fn add_origin(&mut self, origin: VariableOrigin) {
        self.m_origin.push_back(origin);
    }

    pub fn get_origin(&self) -> &LinkedList<VariableOrigin> {
        &self.m_origin
    }
}
