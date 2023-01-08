use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct Edge {
    id: Uuid,
    label: String,
    from: Uuid,
    to: Uuid,
}

impl Edge {
    pub fn new(label: &str) -> Self {
       Self {
           id: Uuid::new_v4(),
           label: String::from(label),
          ..Default::default()
       }
    }

    pub fn from(mut self, id: Uuid) -> Self {
        self.from = id;
        self
    }

    pub fn to(mut self, id: Uuid) -> Self {
        self.to = id;
        self
    }
}
