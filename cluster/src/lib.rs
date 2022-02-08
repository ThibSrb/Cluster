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
    /// Create a new boxed Cluster error with no details.
    /// # Return
    /// The newly created Boxed ClusterError.
    pub fn new_boxed() -> Box<ClusterError> {
        Box::new(ClusterError {
            detail: String::from(""),
        })
    }

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

pub trait AdjUnicity<V> {
    fn push_unique(&mut self, val: V);
}

impl<V> AdjUnicity<V> for Vec<V> where V: PartialEq {
    fn push_unique(&mut self, val: V) {
        if !self.contains(&val) {
            self.push(val);
        }
    }
}

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
pub trait Cluster<K, N: Node<K>>
where
    K: PartialEq,
    K: Clone,
{
    /// Get a node from the graph
    /// **Parameter**
    /// - key - the index of the node in the Graph.
    /// 
    /// # Return
    /// An option containing an immutable reference to the Node if present in the Graph, returns None otherwise.
    fn get(&self, key: &K) -> Option<&N>;

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

    /// Check if the Cluster contains a node at a given key.
    /// # Parameter
    /// - key - The key on we want to check the Cluster contains it or no.
    /// 
    /// # Return
    /// True if the key is in the Cluster, false otherwise.
    fn contains_key(&self, key: &K) -> bool;

    /// Add a node in the Cluster.
    /// # Return
    /// The index at which the node has been stored in the graph.
    fn add(&mut self, node: N) -> K;

    /// Removes the designated Node from the graph
    /// # Parameter
    /// - key - The key of the Node to remove.
    /// # Return
    /// An error if the node doesn't exist nothing otherwise.
    fn remove(&mut self, key: K) -> Result<()>;

    /// Get a node from the graph
    /// # Parameter
    /// - key - the index of the node in the Graph.
    /// 
    /// # Return
    /// An option containing an mutable reference to the Node if present in the Graph, returns None otherwise.
    fn get_mut(&mut self, key: &K) -> Option<&mut N>;

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
        if !adj.contains(&dst) {
            adj.push(dst);
        }
        Ok(())
    }

    /// Remove the edge between src and dst in the Cluster.
    /// # Parameters
    /// - src - The key of the source node
    /// - dst - The key of the destination node.
    /// 
    /// # Return
    /// Nothing if everithing gone well, an error otherwise.
    /// 
    fn remove_edge(&mut self, src: K, dst: K) -> Result<()> {
        let adj = self
            .get_adj_mut(&src)
            .ok_or(ClusterError::detailled("<src> node does not exists."))?;
        if let Some(index) = adj.iter().position(|i| *i == dst) {
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

    /// Reùove the edges between src and dst in the Cluster in both directions.
    /// # Parameters
    /// - src - The key of the source node
    /// - dst - The key of the destination node.
    /// 
    /// # Return
    /// Nothing if everithing gone well, an error otherwise.
    /// 
    fn remove_doubly_edge(&mut self, src: K, dst: K) -> Result<()> {
        self.remove_edge(src.clone(), dst.clone())?;
        self.remove_edge(dst, src)?;
        Ok(())
    }
}
