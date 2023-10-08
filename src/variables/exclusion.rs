use std::collections::VecDeque;

pub trait KeyExclusion {
    fn m_match(&self, key: &str) -> bool;
}

//
pub struct KeyExclusionRegex {}

impl KeyExclusion for KeyExclusionRegex {
    fn m_match(&self, key: &str) -> bool {
        todo!()
    }
}

//
pub struct KeyExclusionString {}

impl KeyExclusion for KeyExclusionString {
    fn m_match(&self, key: &str) -> bool {
        todo!()
    }
}

pub struct KeyExclusions(VecDeque<Box<dyn KeyExclusion>>);
impl KeyExclusions {
    pub fn new() -> KeyExclusions {
        KeyExclusions(VecDeque::new())
    }
    pub fn to_omit(&self, key: &str) -> bool {
        self.0.iter().any(|ke| ke.m_match(key))
    }
}
