use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct ClusterError {
    detail: String,
}

impl Display for ClusterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = self.source() {
            println!("Cluster Error : {}", self.message());
            writeln!(f, "Caused by {}", source)
        } else {
            println!("Cluster Error : {}", self.message());
            writeln!(f, "Unknown source.")
        }
    }
}
impl Error for ClusterError {}
impl ClusterError {

    #[allow(dead_code)]
    /// Create a new boxed Cluster error with no details.
    /// # Return
    /// The newly created Boxed ClusterError.
    pub fn new_boxed() -> Box<ClusterError> {
        Box::new(ClusterError {
            detail: String::from(""),
        })
    }

    #[allow(dead_code)]
    /// Create a new boxed Cluster error with the specified details in it.
    /// # Parameter
    /// - detail - The detail of the error.
    /// # Return
    /// The newly created Boxed ClusterError.
    pub fn detailled_boxed(detail: &str) -> Box<ClusterError> {
        Box::new(ClusterError {
            detail: String::from(detail),
        })
    }

    /// Create a Cluster error with no details
    /// # Return
    /// The newly created ClusterError
    pub fn new() -> ClusterError {
        ClusterError {
            detail: String::from(""),
        }
    }

    /// Create a ClusterError with the specified details in it.
    /// # Parameter
    /// - detail - The detail of the error.
    /// # Return
    /// The newly created ClusterError.
    pub fn detailled(message: &str) -> ClusterError {
        ClusterError {
            detail: String::from(message),
        }
    }

    /// Returs the details of the ClusterError
    /// # Returns
    /// The detail of the ClusterError.
    pub fn message(&self) -> String {
        self.detail.clone()
    }
}

impl Default for ClusterError {
    fn default() -> Self {
        Self::new()
    }
}


pub trait Mappable<K, V> {
    /// Get a value from the Mappable.
    /// # Parameter
    /// - key - the key of the node in the Mappable.
    ///
    /// # Return
    /// An option containing an immutable reference to the Value if present in the Mappable, returns None otherwise.
    fn get(&self, key: &K) -> Option<&V>;

    /// Get a value from the Mappable.
    /// # Parameter
    /// - key - the key of the value in the Mappable.
    ///
    /// # Return
    /// An option containing an mutable reference to the value if present in the Mappable, returns None otherwise.
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    /// Add a value to the Mappable
    /// # Parameters
    /// - key - The key where the value has to be stored in the Mappable.
    /// - value - The value to store in the Mappable.
    fn add(&mut self, key: K, value: V) -> Option<V>;

    /// Removes the designated value from the Mappable
    /// # Parameter
    /// - key - The key of the value to remove.
    /// # Return
    /// An Option containing the value if it exists, None otherwise
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Check if the Mappable contains a value at a given key.
    /// # Parameter
    /// - key - The key on we want to check the Mappable contains it or no.
    ///
    /// # Return
    /// True if the key is in the Mappable, false otherwise.
    fn contains_key(&self, key: &K) -> bool;
}

impl<K,V> Mappable<K,V> for HashMap<K,V> where K: core::hash::Hash + Eq {
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        HashMap::get_mut(self, key)
    }

    fn add(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        HashMap::remove(self, key)
    }

    fn contains_key(&self, key: &K) -> bool {
        HashMap::contains_key(self, key)
    }
}

pub trait Settable<V> {
    /// Adds a value in the Settable.
    /// # Parameter
    /// - val - The value to add in the Settable.
    fn add(&mut self, val: V);

    /// Removes a value in the Settable.
    /// # Parameter
    /// - val - The value to remove in the Settable.
    fn remove(&mut self, val:& V);

    /// Check if the Settable contains a given value.
    /// # Parameter
    /// - val - The value we want to check the Settable contains it or no.
    ///
    /// # Return
    /// True if the key is in the Settable, false otherwise.
    fn contains(&mut self, val: &V) -> bool;
}

impl<V> Settable<V> for Vec<V>
where
    V: PartialEq,
{
    fn add(&mut self, val: V) {
        if !self.contains(&val) {
            self.push(val);
        }
    }

    fn remove(&mut self, val: &V) {
        if let Some(index) = self.iter().position(|i| *i == *val) {
            self.remove(index);
        }
    }

    fn contains(&mut self, val: &V) -> bool {
        self.as_slice().contains(val)
    }
}

/// Trait that ensure that a structure can become a vertice for a Cluster.
pub trait Node<K> {
    /// Get the adjacency of the current Node.
    /// # Return
    /// A immutable reference to the adjacency list of the current Node.
    fn adj(&self) -> &Vec<K>;
    /// Get the adjacency of the current Node.
    /// # Return
    /// A mutable reference to the adjacency list of the current Node.
    fn adj_mut(&mut self) -> &mut Vec<K>;
}

/// Graph data structure trait.
/// Named Cluster to help diffenciate from the other implementation of graph data structure.
pub trait Cluster<K, N: Node<K>>: Mappable<K, N>
where
    K: PartialEq,
    K: Clone,
{
    /// Get the adjancy list of the node designed by it key given in parameter.
    /// # Parameter
    /// - key - the index of the node we want to get the adjacency list.
    /// # Return
    /// An immutable reference to the adjacency list of the desgnated node of the Cluster or None if there is no such Node.
    fn get_adj<'clu, 'res>(&'clu self, key: &K) -> Option<&'res Vec<K>>
    where
        'clu: 'res,
        N: 'res,
    {
        self.get(key).map(|n| n.adj())
    }

    /// Generate a key that is not already used in the graph and returns it.
    /// # Return
    /// The key newly generated.
    fn new_key(&self) -> K;

    /// Add a node in the Cluster.
    /// # Return
    /// The index at which the node has been stored in the graph.
    fn add(&mut self, node: N) -> K {
        let key = self.new_key();
        Mappable::add(self, key.clone(), node);
        key
    }

    /// Get the adjancy list of the node designed by it key given in parameter.
    /// # Parameter
    /// - key - the index of the node we want to get the adjacency list.
    /// # Return
    /// A mutable reference to the adjacency list of the desgnated node of the Cluster or None if there is no such Node.
    fn get_adj_mut<'clu, 'res>(&'clu mut self, key: &K) -> Option<&'res mut Vec<K>>
    where
        'clu: 'res,
        N: 'res,
    {
        self.get_mut(key).map(|n| n.adj_mut())
    }

    /// Add an edge between src and dst in the Cluster.
    /// # Parameters
    /// - src - The key of the source node
    /// - dst - The key of the destination node.
    ///
    /// # Return
    /// Nothing if everithing gone well, an error otherwise.
    ///
    fn add_edge(&mut self, src: K, dst: K) -> Result<()> {
        let adj = self.get_adj_mut(&src).ok_or(ClusterError::detailled(
            "To add edge, both node must exists in the Cluster.",
        ))?;
        adj.add(dst);
        Ok(())
    }

    /// Remove the edge between src and dst in the Cluster.
    /// # Parameters
    /// - src - The key of the source node
    /// - dst - The key of the destination node.
    ///
    /// # Return
    /// Nothing if everithing gone well, an error otherwise.
    ///
    fn remove_edge(&mut self, src: &K, dst: &K) -> Result<()> {
        let adj = self
            .get_adj_mut(&src)
            .ok_or(ClusterError::detailled("<src> node does not exists."))?;
        if let Some(index) = adj.iter().position(|i| *i == *dst) {
            adj.remove(index);
        }
        Ok(())
    }

    /// Add an edge between src and dst in the Cluster in both directions.
    /// # Parameters
    /// - src - The key of the source node
    /// - dst - The key of the destination node.
    ///
    /// # Return
    /// Nothing if everithing gone well, an error otherwise.
    ///
    fn add_doubly_edge(&mut self, src: K, dst: K) -> Result<()> {
        self.add_edge(src.clone(), dst.clone())?;
        self.add_edge(dst, src)?;
        Ok(())
    }

    /// Remove the edges between src and dst in the Cluster in both directions.
    /// # Parameters
    /// - src - The key of the source node
    /// - dst - The key of the destination node.
    ///
    /// # Return
    /// Nothing if everithing gone well, an error otherwise.
    ///
    fn remove_doubly_edge(&mut self, src: &K, dst: &K) -> Result<()> {
        self.remove_edge(src, dst)?;
        self.remove_edge(dst, src)?;
        Ok(())
    }
}
