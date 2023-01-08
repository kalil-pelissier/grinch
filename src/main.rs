mod graph;

use graph::vertex::Vertex;
use graph::Graph;
use uuid::Uuid;

fn main() {
    let mut g = Graph::new();
    g.add_v();
    // g.add_e("friends")
    //     .unwrap()
    //     .from(Uuid::new_v4())
    //     .to(Uuid::new_v4());

    print!("{:?}", g);
}
