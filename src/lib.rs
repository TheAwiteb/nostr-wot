// Copyright (c) 2026, Awiteb <a@4rs.nl>
//     lightweight nostr Web of Trust library
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use nostr::key::PublicKey;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};

pub(crate) const COMPRESSION_LEVEL: Compression = Compression::new(4);

/// Library errors
pub mod error;
/// Graph serialization and deserialization
mod parser;
/// Graph relations
pub mod relations;
/// Utils
pub mod utils;

/// WoT graph. storing public key hashes and their relations.
pub(crate) type GraphType = DiGraph<u64, u8>;

/// A directed graph representing a Web of Trust.
#[derive(Default)]
pub struct WotGraph {
    /// The underlying directed graph.
    inner: GraphType,
}

impl WotGraph {
    /// Creates a new empty graph.
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: DiGraph::new(),
        }
    }

    /// Creates a new empty graph with preallocated capacity for nodes and
    /// edges.
    #[inline]
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            inner: DiGraph::with_capacity(nodes, edges),
        }
    }

    /// Imports a graph from bytes. The graph should be previously exported
    /// using [`WotGraph::export`].
    #[inline]
    pub fn import(data: &[u8]) -> Result<Self, error::Error> {
        Ok(Self {
            inner: parser::import_graph(data)?,
        })
    }

    /// Imports a graph from a gzip-compressed bytes. The graph should be
    /// previously exported using [`WotGraph::export_gzip`].
    #[inline]
    pub fn import_gzip(data: &[u8]) -> Result<Self, error::Error> {
        Ok(Self {
            inner: parser::import_graph(GzDecoder::new(data))?,
        })
    }

    /// Import a graph from a file. Must be exported using
    /// [`WotGraph::export`] or [`WotGraph::export_to_file`].
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    #[inline]
    pub fn import_from_file<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        Ok(Self {
            inner: parser::import_graph(BufReader::new(File::open(path)?))?,
        })
    }

    /// Import a gzip compressed graph from a file. Must be exported using
    /// [`WotGraph::export_gzip`] or [`WotGraph::export_to_file_gzip`].
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    #[inline]
    pub fn import_from_file_gzip<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        Ok(Self {
            inner: parser::import_graph(GzDecoder::new(File::open(path)?))?,
        })
    }

    /// Add a new node.
    ///
    /// Returns `None` if the graph is full.
    #[inline]
    pub fn add_node(&mut self, node: u64) -> Option<NodeIndex> {
        self.inner.try_add_node(node).ok()
    }

    /// Add a new node from public key.
    ///
    /// This will create the node if the it's not exists.
    ///
    /// Returns `None` if the graph is full.
    pub fn add_node_pkey(&mut self, pkey: &PublicKey) -> Option<NodeIndex> {
        let pkey_hash = utils::hash_bytes(pkey.as_bytes());
        if let Some(idx) = self
            .inner
            .node_indices()
            .find(|idx| self.inner[*idx] == pkey_hash)
        {
            return Some(idx);
        }

        self.add_node(pkey_hash)
    }

    /// Adds an edge between `source` and `target` nodes with the given
    /// relation.
    ///
    /// Returns `None` if the graph is full or if either node doesn't exist.
    #[inline]
    pub fn add_edge(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        relation: relations::Relation,
    ) -> Option<EdgeIndex> {
        self.inner
            .try_update_edge(source, target, relation as u8)
            .ok()
    }

    /// Calculates the total number of bytes needed for exporting the graph.
    fn export_capacity(&self) -> usize {
        32 + (self.inner.raw_nodes().len() * 8) + (self.inner.raw_edges().len() * 17)
    }

    /// Export the graph nodes and edges in a binary format (little-endian).
    ///
    /// Format:
    /// - 8 bytes: nodes capacity
    /// - 8 bytes: edges capacity
    /// - 8 bytes: number of nodes
    /// - 8 bytes: number of edges
    /// - N * 8 bytes: node weights
    /// - E * 17 bytes: edges (8 bytes source, 1 byte relation, 8 bytes target)
    #[inline]
    pub fn export(&self) -> Result<Vec<u8>, error::Error> {
        let mut buffer = Vec::with_capacity(self.export_capacity());
        parser::export_graph(&self.inner, &mut buffer)?;
        Ok(buffer)
    }

    /// Compresses the graph nodes and edges using gzip and returns the result.
    ///
    /// The output is a compressed version of the data from
    /// [`WotGraph::export`].
    pub fn export_gzip(&self) -> Result<Vec<u8>, error::Error> {
        let mut compressed_graph = Vec::with_capacity(self.export_capacity() / 2);
        let mut encoder = GzEncoder::new(&mut compressed_graph, COMPRESSION_LEVEL);
        parser::export_graph(&self.inner, &mut encoder)?;
        encoder.finish()?;

        Ok(compressed_graph)
    }

    /// Export the graph to a file.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), error::Error> {
        let mut writer = BufWriter::new(File::create(path)?);
        parser::export_graph(&self.inner, &mut writer)?;
        writer.flush()?;

        Ok(())
    }

    /// Export a gzip compressed graph to file.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_to_file_gzip<P: AsRef<Path>>(&self, path: P) -> Result<(), error::Error> {
        let mut file = BufWriter::new(File::create(path)?);
        let mut encoder = GzEncoder::new(&mut file, COMPRESSION_LEVEL);

        parser::export_graph(&self.inner, &mut encoder)?;
        encoder.finish()?;
        file.flush()?;

        Ok(())
    }
}
