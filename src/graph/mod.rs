pub mod edge;
pub mod vertex;

use edge::Edge;
use vertex::Vertex;

#[derive(Default, Debug)]
pub struct Graph {
    verticies: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_v(&mut self) {
        self.verticies.push(Vertex::new());
    }

    pub fn add_e(&mut self, label: &str) -> Option<&mut Edge> {
        let i = self.edges.len();
        self.edges.push(Edge::new(label));
        self.edges.get_mut(i)
    }

    pub fn V(self) -> Vec<Vertex> {
        self.verticies
    }

    pub fn E(self) -> Vec<Edge> {
        self.edges
    }
}
