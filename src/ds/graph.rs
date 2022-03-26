use hashbrown::{HashMap, HashSet};
use rand::{thread_rng, Rng};
use std::ops::{Index, IndexMut};
use std::hash::Hash;
use std::hash::Hasher;
use std::fs::File;
use std::io::Write;
use std::fmt;
use std::io;
// use std::io::*;

#[derive(Clone, Debug, PartialEq)]
/// Graph operation error
pub enum GraphErr {
    /// There is no vertex with the given id in the graph
    NoSuchVertex,

    /// There is no such edge in the graph
    NoSuchEdge,

    /// Could not add an edge to the graph
    CannotAddEdge,

    /// The given weight is invalid
    InvalidWeight,

    /// The operation cannot be performed as it will
    /// create a cycle in the graph.
    CycleError,

    // #[cfg(feature = "dot")]
    // /// Could not render .dot file
    // CouldNotRender,

    // #[cfg(feature = "dot")]
    // /// The name of the graph is invalid. Check [this](https://docs.rs/dot/0.1.1/dot/struct.Id.html#method.new)
    // /// out for more information.
    // InvalidGraphName,
}

#[derive(Clone, Debug)]
pub struct GraphProperties { pub directed: bool}

#[derive(Clone, Debug)]
/// Edge internal struct
pub struct Edge<T: fmt::Display> {
    pub start: u64,
    pub end: u64,
    pub data: Option<T>,
}

#[derive(Clone, Debug)]
pub struct Node<T: fmt::Display> {
    id: u64,
    incoming: Vec<u64>,
    outgoing: Vec<u64>,
    pub data: T,
}

#[derive(Clone, Debug)]
/// Graph data-structure
pub struct Graph<N: fmt::Display, E: fmt::Display> {
    /// Mapping of vertex ids and vertex values
    nodes: HashMap<u64, Node<N>>,
    /// Mapping between edges and weights
    edges: HashMap<u64, Edge<E>>,
    /// Properties of the represented graph
    properties: GraphProperties,
    /// Id of root node
    root: Option<u64>,
}

impl<N: fmt::Display, E: fmt::Display + Clone> Graph<N, E> {

    pub fn new(props: GraphProperties) -> Graph<N, E> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            properties: props,
            root: None,
        }
    }

    pub fn add_node(&mut self, data: N) -> u64 {
        let mut rng = thread_rng();
        let nid = rng.gen_range(0..10000000000);
        // println!("NID: {}", nid);

        if self.nodes.contains_key(&nid) {
            println!("AHAHAHA");
        }

        self.nodes.insert(nid, Node{ id: nid, data: data, incoming: vec![], outgoing: vec![],});
        nid
    }   

    pub fn add_edge(&mut self, a: &u64, b: &u64, data: Option<E>) -> Result<u64, GraphErr> {
        
        let e = self.has_edge(a, b);
        if  e.is_ok() {
            return e;
        };

        let id1 = if self.nodes.get(a).is_some() {
            *a
        } else {
            return Err(GraphErr::NoSuchVertex);
        };

        let id2 = if self.nodes.get(b).is_some() {
            *b
        } else {
            return Err(GraphErr::NoSuchVertex);
        };

        let mut rng = thread_rng();
        
        let mut eid = rng.gen_range(0..10000000000);
        // println!("EID: {}", eid);
        self.edges.insert(eid, Edge{start: id1, end: id2, data: data.clone()});
        self.nodes.get_mut(&id1).unwrap().outgoing.push(eid);
        self.nodes.get_mut(&id2).unwrap().incoming.push(eid);

        Ok(eid)
    }

    pub fn get_node(&self, id: &u64) -> & Node<N> {
        self.nodes.get(id).unwrap()
    }

    pub fn get_node_mut(&mut self, id: &u64) -> &mut Node<N> {
        self.nodes.get_mut(id).unwrap()
    }

    pub fn get_root(&self) -> Option<u64> {
        self.root
    }

    pub fn set_root(&mut self, id: u64) {
        self.root = Some(id);
    }

    pub fn get_edge(&self, id: &u64) -> & Edge<E> {
        // println!("ID: id");
        self.edges.get(id).unwrap()
    }    

    pub fn get_edge_mut(&mut self, id: &u64) -> &mut Edge<E> {
        self.edges.get_mut(id).unwrap()
    }

    pub fn has_edge(&self, a: &u64, b: &u64) -> Result<u64, GraphErr> {
        let node = self.nodes.get(a);
        let edge_id = match self.nodes.get(a) {
            Some(x) => x.outgoing.iter().find(|idx| self.edges.get(idx).unwrap().end == *b),
            _ => None
        };
        let result = match edge_id {
            Some(id) => Ok(*id),
            _ => Err(GraphErr::NoSuchEdge)
        };
        result
    }    

    pub fn remove_edge(&mut self, id: &u64) -> Option<Edge<E>> {

        // Check if edge exists in graph
        if !self.edges.contains_key(id) {
            return None;
        }

        // println!("ID: {}", id);

        // Remove outgoing adjacency in corresponding node
        let out_node_id = self.get_edge(id).start;
        let out_node_neighbours = &mut self.get_node_mut(&out_node_id).outgoing;
        let index_out = out_node_neighbours.iter().position(|x| x == id).unwrap();
        out_node_neighbours.remove(index_out);  

        // Remove incoming adjacency in corresponding node
        let in_node_id = self.get_edge(id).end;
        let in_node_neighbours = &mut self.get_node_mut(&in_node_id).incoming;
        let index_in = in_node_neighbours.iter().position(|x| x == id).unwrap();
        in_node_neighbours.remove(index_in);        

        // Remove edge from storage container
        self.edges.remove(id)
    }    

    pub fn edges(&self) -> impl Iterator<Item=&u64> {
        self.edges.keys()
    }

    pub fn nodes(&self) -> impl Iterator<Item=&u64> {
        self.nodes.keys()
    }
    
    // Obtain Iterator over neigbouring nodes or adjacent edges
    pub fn out_neighbors<'a>(&'a self, id: u64) -> Box<dyn Iterator<Item=&u64>+'a> {
        match self.nodes.get(&id) {
            Some(x) => Box::new(x.outgoing.iter().map(|idx| &self.edges.get(idx).unwrap().end)),
            _ => Box::new(::std::iter::empty())
        }
    }

    pub fn out_edges<'a>(&'a self, id: u64) -> Box<dyn Iterator<Item=&u64>+'a> {
        match self.nodes.get(&id) {
            Some(x) => Box::new(x.outgoing.iter()),
            _ => Box::new(::std::iter::empty())
        }
    }

    pub fn in_neighbors<'a>(&'a self, id: u64) -> Box<dyn Iterator<Item=&u64>+'a> {
        match self.nodes.get(&id) {
            Some(x) => Box::new(x.incoming.iter().map(|idx| &self.edges.get(idx).unwrap().end)),
            _ => Box::new(::std::iter::empty())
        }
    }

    pub fn in_edges<'a>(&'a self, id: u64) -> Box<dyn Iterator<Item=&u64>+'a> {
        match self.nodes.get(&id) {
            Some(x) => Box::new(x.incoming.iter()),
            _ => Box::new(::std::iter::empty())
        }
    }    

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn write_dot(&self, f: &str) {
        let mut w = File::create(f).unwrap();
        if self.properties.directed {
            writeln!(&mut w, "digraph sample {{").unwrap();
        }
        else {
            writeln!(&mut w, "strict graph sample {{").unwrap();
        }
        for (id, edge) in &self.edges {
            match (self.properties.directed, edge.data.as_ref()) {
                (true, Some(x)) =>  writeln!(&mut w, "  {} -> {} [ label = \"{}\" ];", edge.start, edge.end, x).unwrap(),
                (true, None) => writeln!(&mut w, "  {} -> {};", edge.start, edge.end).unwrap(),
                (false, Some(x)) => writeln!(&mut w, "  {} -- {} [ label = \"{}\" ];", edge.start, edge.end, x).unwrap(),
                (false, None) => writeln!(&mut w, "  {} -- {};", edge.start, edge.end).unwrap(),
            };  
        }
        for (id, node) in &self.nodes {
            writeln!(&mut w, "  {} [ label = \"{}\" ];", id, node.data).unwrap();
        }

        writeln!(&mut w, "}}").unwrap();
    }
}


/// DFS

pub struct Dfs<'a, N: fmt::Display, E: fmt::Display + Clone> {
    current: u64,
    stack: Vec<u64>,
    visited: HashSet<u64>,
    graph: &'a Graph<N,E>
}

impl<'a, N: fmt::Display, E: fmt::Display + Clone> Dfs<'a, N, E> {

    pub fn new(graph: &'a Graph<N, E>) -> Dfs<'a, N, E> {
        let mut stack = Vec::<u64>::new();
        let current = *graph.nodes().last().unwrap();
        stack.push(current);
        Dfs {
            current: current,
            stack: stack,
            visited: HashSet::new(),
            graph: graph
        }
    }

    pub fn init(&mut self, id: &u64) {
        self.current = *id;
        self.stack.clear();
        self.stack.push(*id);
    }

    pub fn process_node(&mut self) -> Result<u64, GraphErr> {
        // Obtain node from stack. Search is done, when stack empty.
        self.current = match self.stack.pop() {
            Some(x) => x,
            _ => return Err(GraphErr::NoSuchVertex)
        };
        for node in self.graph.out_neighbors(self.current) {
            if !self.visited.contains(node) {
                self.stack.push(*node);
            }
        }
        self.visited.insert(self.current);
        Ok(self.current)
    }
}


/// Ukonen Node
pub struct UkonenNode {
    link: u64,
    suffix_edge_ids: HashMap<u8, u64>
}

impl UkonenNode {
    pub fn new() -> UkonenNode {
        UkonenNode {
            link: 0,
            suffix_edge_ids: HashMap::new(),
        }
    }
}

impl fmt::Display for UkonenNode  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UN: Link {:?}", self.link)
    }
}

// Ukonen Edge
#[derive(Clone)]
pub struct UkonenEdge {
    pub suffix_start: usize,
    pub suffix_stop: i64
}

impl UkonenEdge {
    pub fn new() -> UkonenEdge {
        UkonenEdge {
            suffix_start: 0,
            suffix_stop: 0,
        }
    }
}

impl fmt::Display for UkonenEdge  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.suffix_start, self.suffix_stop)
    }
}

// Ukonen Algorithm
pub struct Ukonen<T:fmt::Display + Index<usize, Output=u8> + IntoIterator> {
    // Sequence to be processed
    seq: T, 
    idx: usize,
    // State indicators
    remaining : usize,
    active_node_id : u64, 
    active_edge_seq_idx : usize,
    active_length : usize,
    end: u8,
    // Graph 
    root_id : u64,
    graph: Graph<UkonenNode, UkonenEdge>,
    // Resolved
    previous_new_node : u64,

}

impl<T: fmt::Display + Clone + Index<usize, Output=u8> + IntoIterator> Ukonen<T> {

    pub fn new(sequence: T) -> Ukonen<T> {

        let mut graph = Graph::<UkonenNode, UkonenEdge>::new(GraphProperties{directed: true});
        let root_id = graph.add_node(UkonenNode::new());
        graph.set_root(root_id);

        Ukonen {
            seq : sequence, 
            idx : 0,
            remaining : 0,
            active_node_id : root_id, 
            active_edge_seq_idx : 0,
            active_length : 0,
            end : 0,
            root_id : root_id,
            graph :  graph,
            previous_new_node : root_id,
        }
    }    

    pub fn process(&mut self) -> &mut Graph<UkonenNode, UkonenEdge>{
        for s in (self.seq.clone()).into_iter() {
            self.step();
            self.idx += 1;
        }
        &mut self.graph
    }

    fn split_suffix_edge(graph: &mut Graph<UkonenNode, UkonenEdge>, edge_id: u64, split_index: usize, value_index: usize, seq: &T) -> u64 {
        
        // Get indizes of nodes in graph that are connected to edge
        let edge_start = graph.get_edge(&edge_id).start;
        let edge_end = graph.get_edge(&edge_id).end;

        // Get indizes of suffix that is encoded by the given edge
        let suffix_start = graph.get_edge(&edge_id).data.as_ref().unwrap().suffix_start;
        let suffix_stop = graph.get_edge(&edge_id).data.as_ref().unwrap().suffix_stop;

        // Split edge
        let split_node_id = graph.add_node(UkonenNode::new());
        let split_edge_pre = graph.add_edge(
            &edge_start, 
            &split_node_id, 
            Some(UkonenEdge{suffix_start: suffix_start, suffix_stop: (suffix_start + split_index - 1) as i64})
        ).unwrap();
        let split_edge_succ = graph.add_edge(
            &split_node_id, 
            &edge_end, 
            Some(UkonenEdge{suffix_start: suffix_start + split_index, suffix_stop: suffix_stop as i64})
        ).unwrap();

        // Add value into node created at split
        let leaf_node = graph.add_node(UkonenNode::new());
        let value_edge = graph.add_edge(
            &split_node_id, 
            &leaf_node, 
            Some(UkonenEdge{suffix_start: value_index, suffix_stop: -1})
        ).unwrap();

        // Remove old edge that was split
        graph.remove_edge(&edge_id);
        graph.get_node_mut(&edge_start).data.suffix_edge_ids.remove(&seq[suffix_start]);

        // Update mapping for outgoing suffix edges for the involved nodes
        graph.get_node_mut(&edge_start).data.suffix_edge_ids.insert(seq[suffix_start], split_edge_pre);
        graph.get_node_mut(&split_node_id).data.suffix_edge_ids.insert(seq[suffix_start + split_index], split_edge_succ);
        graph.get_node_mut(&split_node_id).data.suffix_edge_ids.insert(seq[value_index], value_edge);

        split_node_id
    }

    pub fn step(&mut self) {

        let mut cur_value = self.seq[self.idx];
        // Keeps track of the number of `suffixes` to be resolved after a new character is added 
        self.remaining += 1;
        // Set the current `end` character to new value
        self.end = cur_value;
        // Set previous new node for suffix links back to root 
        self.previous_new_node = self.root_id;

        // For given position `self.idx` in input sequence `self.seq`
        // Iterate as long as suffix after adding given letter is resolved

        // println!("\n{}", cur_value as char);



        while self.remaining > 0 { 
            
            // self.graph.write_dot("abc.dot");
            // let mut input = String::new();
            // io::stdin().read_line(&mut input).expect("error: unable to read user input");

            // println!("REMAIN {}", self.remaining);
            // println!("ACTIVE LENGTH {}", self.active_length);


            if self.active_length == 0 {
                self.active_edge_seq_idx = self.idx;
            }

            cur_value = self.seq[self.active_edge_seq_idx];
            // println!("CUR VAL {}", cur_value as char);

            // self.graph.write_dot("abc.dot");
            // let mut input = String::new();
            // io::stdin().read_line(&mut input).expect("error: unable to read user input");


            let cur_node  = self.graph.get_node(&self.active_node_id);

            // Check if there is a suffix corresponding to `cur_value` adjacent to current node, 
            // Add a new None/Edge to the graph, if there is no suffix corresponding
            // Activate the edge that corresponds to the suffix otherwise
            if !cur_node.data.suffix_edge_ids.contains_key(&cur_value) {

                // Create a new node and edge
                let nid = self.graph.add_node(UkonenNode::new());
                let sfx = UkonenEdge{suffix_start: self.idx, suffix_stop: -1};
                let eid = self.graph.add_edge(&self.active_node_id, &nid, Some(sfx)).unwrap();

                // Keep track of newly added edge (suffix) inside current node
                let cur_node_mut  = self.graph.get_node_mut(&self.active_node_id);
                cur_node_mut.data.suffix_edge_ids.insert(cur_value, eid);          
                
                if self.previous_new_node != self.root_id {
                    // println!("CREATE");

                    self.graph.get_node_mut(&self.previous_new_node).data.link = self.active_node_id;
                    self.previous_new_node = self.active_node_id;
                }
                // self.active_length += 1;

                self.remaining -= 1;
            }
            else {

                // Characters might match the suffix on the path from root downwards
                // This might introduce an update to the active node, if the current edge gets exhausted
                let cur_node  = self.graph.get_node(&self.active_node_id);
                let active_edge_identifier = cur_node.data.suffix_edge_ids[&self.seq[self.active_edge_seq_idx]];

                let active_edge = self.graph.get_edge(&(active_edge_identifier as u64)).clone();
                let lookup_idx = self.active_length + active_edge.data.as_ref().unwrap().suffix_start;

                // Check if an internal node has been reach, and update state accordingly
                let suffix_stop = active_edge.data.as_ref().unwrap().suffix_stop;
                let suffix_start = active_edge.data.as_ref().unwrap().suffix_start;

                if suffix_stop != -1 && lookup_idx > suffix_stop as usize {
                    self.active_node_id = active_edge.end;
                    self.active_length = lookup_idx - suffix_stop as usize - 1;
                    self.active_edge_seq_idx += (suffix_stop as usize - suffix_start) + 1; // +1?
                    continue;
                }

                // Check if next character on active edge matches the current value
                // If matches, we proceed along the edge and go to next character
                if self.seq[lookup_idx] == self.seq[self.idx] {
                    self.active_length += 1;
                    
                    if self.previous_new_node != self.root_id {
                        self.graph.get_node_mut(&self.previous_new_node).data.link = self.active_node_id;
                        self.previous_new_node = self.active_node_id;
                    }

                    break;
                }

                // println!("SPLIT");
                // Resolve all partial suffixes have been accumulated during transition along edge
                // Break the matching on the active edge and introduce new leaf node for new suffix
                let split_node_id = Ukonen::split_suffix_edge(
                    &mut self.graph, 
                    active_edge_identifier, 
                    self.active_length, 
                    self.idx, 
                    &self.seq
                );
                self.graph.get_node_mut(&split_node_id).data.link = self.root_id;

                // Update suffix links on previous node, if subsequent internal node created
                if self.previous_new_node != self.root_id {
                    self.graph.get_node_mut(&self.previous_new_node).data.link = split_node_id;
                }
                
                self.previous_new_node = split_node_id;
                self.remaining -= 1;
                
            }

            // println!("ASD");


            // Jump to next edge if are on an edge from root and length > 0, 
            // Otherwise we are at some internal node, jump to next internal node as per suffix link resolution rules
            if self.active_length > 0 && self.active_node_id == self.root_id {
                // Current character was processed, reduce length
                self.active_length -= 1;
                self.active_edge_seq_idx = self.idx - self.remaining + 1;
            }
            else if self.active_node_id != self.root_id {      
                self.active_node_id = self.graph.get_node(&self.active_node_id).data.link;
                // self.active_edge_seq_idx = self.idx - self.remaining + 1;
                // println!("JUMP to {}", self.active_edge_seq_idx);
            }
            // println!("\n\n");
        }  
    }
}