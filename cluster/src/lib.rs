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
    pub fn new_boxed() -> Box<ClusterError> {
        Box::new(ClusterError {
            detail: String::from(""),
        })
    }

    pub fn detailled_boxed(message: &str) -> Box<ClusterError> {
        Box::new(ClusterError {
            detail: String::from(message),
        })
    }

    pub fn new() -> ClusterError {
        ClusterError {
            detail: String::from(""),
        }
    }

    pub fn detailled(message: &str) -> ClusterError {
        ClusterError {
            detail: String::from(message),
        }
    }

    pub fn message(&self) -> String {
        self.detail.clone()
    }
}

impl Default for ClusterError {
    fn default() -> Self {
        Self::new()
    }
}

trait Node<K> {
    fn adj(&self) -> &Vec<K>;
    fn adj_mut(&mut self) -> &mut Vec<K>;
}

/// Graph data structure trait.
/// Named Cluster to help diffenciate from the other implementation of graph data structure.
trait Cluster<K, N: Node<K>>
where
    K: PartialEq,
    K: Clone,
{
    /// Get a node from the graph
    /// # Parameter
    /// - key : the index of the node in the Graph.
    /// Returns An option containing the Node if present in the Graph, returns None otherwise.
    ///
    fn get(&self, key: &K) -> Option<&N>;

    /// Get the adjancy list of the node designed by it key given in parameter.
    /// # Parameter
    /// - key : the index of the node we want to get the adjacency list.
    fn get_adj<'clu, 'res>(&'clu self, key: &K) -> Option<&'res Vec<K>>
    where
        'clu: 'res,
        N: 'res,
    {
        self.get(key).map(|n| n.adj())
    }

    fn new_key(&self) -> K;

    fn contains_key(&self, key: &K) -> bool;

    fn add(&mut self, node: N) -> K;

    fn remove(&mut self, key: K) -> Result<()>;

    fn get_mut(&mut self, key: &K) -> Option<&mut N>;

    fn get_adj_mut<'clu, 'res>(&'clu mut self, key: &K) -> Option<&'res mut Vec<K>>
    where
        'clu: 'res,
        N: 'res,
    {
        self.get_mut(key).map(|n| n.adj_mut())
    }

    fn add_edge(&mut self, src: K, dst: K) -> Result<()> {
        let adj = self.get_adj_mut(&src).ok_or(ClusterError::detailled_boxed(
            "To add edge, both node must exists in the Cluster.",
        ))?;
        if !adj.contains(&dst) {
            adj.push(dst);
        }
        Ok(())
    }

    fn remove_edge(&mut self, src: K, dst: K) -> Result<()> {
        let adj = self
            .get_adj_mut(&src)
            .ok_or(ClusterError::detailled_boxed("<src> node does not exists."))?;
        if let Some(index) = adj.iter().position(|i| *i == dst) {
            adj.remove(index);
        }
        Ok(())
    }

    fn add_doubly_edge(&mut self, src: K, dst: K) -> Result<()> {
        self.add_edge(src.clone(), dst.clone())?;
        self.add_edge(dst, src)?;
        Ok(())
    }

    fn remove_doubly_edge(&mut self, src: K, dst: K) -> Result<()> {
        self.remove_edge(src.clone(), dst.clone())?;
        self.remove_edge(dst, src)?;
        Ok(())
    }
}
