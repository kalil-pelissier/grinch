use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub id: Uuid,
    properties: HashMap<String, String>,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }
    pub fn property(mut self, k: &str, v: &str) -> Self {
        self.properties.insert(String::from(k), String::from(v));
        self
    }

    // pub fn has(k: &str, v: &str) -> bool {}
}
