use fnv::FnvHashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<VId, E = (), V = ()> {
  pub(crate) vertices: FnvHashMap<VId, V>,
  pub(crate) adjacency: FnvHashMap<VId, Vec<(VId, E)>>,
}

impl<VId, E, V> Graph<VId, E, V>
where
  VId: Eq + Hash,
  V: Hash,
{
  pub fn new() -> Graph<VId, E, V> {
    Graph {
      vertices: FnvHashMap::default(),
      adjacency: FnvHashMap::default(),
    }
  }

  pub fn push_vertex(self: &mut Graph<VId, E, V>, vid: VId, vertex: V) {
    self.vertices.insert(vid, vertex);
  }

  pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }

  pub fn get_vertex(self: &Self, vid: &VId) -> Option<&V> {
    self.vertices.get(vid)
  }

  pub fn iter_vertices(&self) -> impl Iterator<Item = (&VId, &V)> {
    self.vertices.iter()
  }

  pub fn get_edge(self: &Self, from_vid: VId, to_vid: VId) -> Option<&E> {
    self.adjacency.get(&from_vid).and_then(|edges| {
      edges
        .iter()
        .find(|(curr_to_vid, _edge)| *curr_to_vid == to_vid)
        .map(|(_, edge)| edge)
    })
  }

  pub fn iter_edges(&self) -> impl Iterator<Item = (&VId, &Vec<(VId, E)>)> {
    self
      .adjacency
      .iter()
      .map(|(from_vid, incident)| (from_vid, incident))
  }

  pub fn iter_complete_edges(&self) -> impl Iterator<Item = (&VId, &VId, &E)> {
    self.iter_edges().flat_map(|(from_vid, incident)| {
      incident
        .iter()
        .map(move |(to_vid, e)| (from_vid, to_vid, e))
    })
  }

  pub fn incident_edges(self: &Self, vid: &VId) -> Option<&Vec<(VId, E)>> {
    self.adjacency.get(vid)
  }

  pub fn map_adjacent<F, R>(self: &Self, vid: VId, mut f: F) -> Vec<R>
  where
    F: FnMut(&(VId, E)) -> R,
  {
    let edges = self.adjacency.get(&vid);

    match edges {
      None => vec![],
      Some(edges) => edges.iter().map(|vid_and_e| f(vid_and_e)).collect(),
    }
  }
}

impl<VId, E> Graph<VId, E, ()>
where
  VId: Eq + Hash,
{
  pub fn push_vid(self: &mut Self, vid: VId) {
    self.vertices.insert(vid, ());
  }
}

impl<VId, E, V> Graph<VId, E, V>
where
  VId: Eq + Hash + Clone,
  V: Hash,
  E: Clone,
{
  pub fn push_undirected_edge(self: &mut Self, from: VId, to: VId, edge: E) {
    self.push_edge(from.clone(), to.clone(), edge.clone());
    self.push_edge(to, from, edge);
  }
}

fn main() {
  println!("Hello, world!");
  let mut g: Graph<&str, String> = Graph::new();

  let numeric_vids: Vec<u8> = (65..91).collect();
  let my_vids:Vec<String> = numeric_vids.iter().map(|my_char| (*my_char as char).to_string()).collect();

  let _these_vertices = my_vids.iter().map(|my_char| g.push_vid(&my_char));

  g.push_edge("E", "A", "E -> A".to_string());
  g.push_edge("K", "B", "K -> B".to_string());
  g.push_edge("V", "C", "V -> C".to_string());
  g.push_edge("Y", "D", "Y -> D".to_string());
  g.push_edge("P", "E", "P -> E".to_string());
  g.push_edge("V", "F", "V -> F".to_string());
  g.push_edge("S", "G", "S -> G".to_string());
  g.push_edge("H", "H", "H loop".to_string());
  g.push_edge("X", "I", "X -> I".to_string());
  g.push_edge("S", "J", "S -> J".to_string());
  g.push_edge("D", "K", "D -> K".to_string());
  g.push_edge("S", "L", "S -> L".to_string());
  g.push_edge("E", "M", "E -> M".to_string());
  g.push_edge("L", "N", "L -> N".to_string());
  g.push_edge("F", "O", "F -> O".to_string());
  g.push_edge("U", "P", "U -> P".to_string());
  g.push_edge("A", "Q", "A -> Q".to_string());
  g.push_edge("O", "R", "O -> R".to_string());
  g.push_edge("I", "S", "I -> S".to_string());
  g.push_edge("N", "T", "N -> T".to_string());
  g.push_edge("M", "U", "M -> U".to_string());
  g.push_edge("O", "V", "O -> V".to_string());
  g.push_edge("V", "W", "V -> W".to_string());
  g.push_edge("W", "X", "W -> X".to_string());
  g.push_edge("D", "Y", "D -> Y".to_string());
  g.push_edge("P", "Z", "P -> Z".to_string());
  g.push_edge("B", "G", "B -> G".to_string());
  g.push_edge("B", "A", "B -> A".to_string());
  g.push_edge("B", "B", "B loop".to_string());
  g.push_edge("B", "Y", "B -> Y".to_string());
  g.push_edge("Z", "Z", "Z loop".to_string());
  g.push_edge("Z", "O", "Z -> O".to_string());
  g.push_edge("Z", "M", "Z -> M".to_string());
  g.push_edge("Z", "B", "Z -> B".to_string());
  g.push_edge("Z", "I", "Z -> I".to_string());
  g.push_edge("Z", "E", "Z -> E".to_string());
  g.push_edge("T", "T", "T loop".to_string());
  g.push_edge("T", "O", "T -> O".to_string());
  g.push_edge("T", "N", "T -> N".to_string());
  g.push_edge("T", "A", "T -> A".to_string());
  g.push_edge("Q", "W", "Q -> W".to_string());
  g.push_edge("Q", "E", "Q -> E".to_string());
  g.push_edge("Q", "R", "Q -> R".to_string());
  g.push_edge("Q", "T", "Q -> T".to_string());
  g.push_edge("Q", "Y", "Q -> Y".to_string());
  g.push_edge("R", "M", "R -> M".to_string());
  g.push_edge("J", "C", "J -> C".to_string());
  g.push_edge("G", "L", "G -> L".to_string());
  g.push_edge("C", "A", "C -> A".to_string());
  g.push_edge("C", "T", "C -> T".to_string());
  
  let mut count = 65u8;

  println!("Let's generate the graph!");

  // Mapping Adjacent vertex
  loop {
    let my_char = (count as char).to_string();
    count += 1;
    println!("{:?}", g.map_adjacent(&my_char, |x| x.clone()));

    if count == 91 {
      println!("OK, that's enough");

      break;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_an_indexed_graph() {
    let mut g: Graph<&str, String> = Graph::new();
    g.push_vid("A");
    g.push_vid("B");
    g.push_vid("C");

    g.push_edge("A", "B", "A -> B".to_string());
    g.push_edge("B", "C", "B -> C".to_string());
    g.push_edge("C", "A", "C -> A".to_string());
    g.push_edge("A", "A", "A loop".to_string());

    assert_eq!(g.vertices.len(), 3);
    assert_eq!(
      g.adjacency.get("A").unwrap(),
      &[("B", "A -> B".to_string()), ("A", "A loop".to_string())]
    );
    assert_eq!(
      g.adjacency.get("B").unwrap(),
      &[("C", "B -> C".to_string())]
    );
    assert_eq!(
      g.adjacency.get("C").unwrap(),
      &[("A", "C -> A".to_string())]
    );

    assert_eq!(
      g.map_adjacent("A", |x| x.clone()),
      [("B", "A -> B".to_string()), ("A", "A loop".to_string())]
    );

    assert_eq!(
      g.map_adjacent("B", |x| x.clone()),
      [("C", "B -> C".to_string())]
    );
    assert_eq!(
      g.map_adjacent("C", |x| x.clone()),
      [("A", "C -> A".to_string())]
    );
    assert_eq!(g.get_vertex(&"A"), Some(&()));
    assert_eq!(g.get_vertex(&"B"), Some(&()));
    assert_eq!(g.get_vertex(&"Z"), None);
  }
}
