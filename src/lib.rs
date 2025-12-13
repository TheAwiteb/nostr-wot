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
use std::path::Path;

use nostr::key::PublicKey;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};

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
    pub fn import_gzip(data: &[u8]) -> Result<Self, error::Error> {
        // It's usually half of the size, why not allocate it first
        let mut decompressed_graph = Vec::with_capacity(data.len() * 2);
        utils::gzip_decompress(data, &mut decompressed_graph)?;

        Self::import(&decompressed_graph)
    }

    /// Import a graph from a file. Must be exported using
    /// [`WotGraph::export`] or [`WotGraph::export_to_file`].
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn import_from_file<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        use std::{
            fs::File,
            io::{self, BufReader},
        };

        let file = File::open(path)?;
        let mut imported_graph = if let Ok(metadata) = file.metadata() {
            Vec::with_capacity(metadata.len().try_into().unwrap_or(usize::MAX))
        } else {
            Vec::new()
        };

        io::copy(&mut BufReader::new(file), &mut imported_graph)?;
        Self::import(&imported_graph)
    }

    /// Import a gzip compressed graph from a file. Must be exported using
    /// [`WotGraph::export_gzip`] or [`WotGraph::export_to_file_gzip`].
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn import_from_file_gzip<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        use std::{fs::File, io::BufReader};

        let file = File::open(path)?;
        let mut decompressed_graph = if let Ok(metadata) = file.metadata() {
            Vec::with_capacity(metadata.len().try_into().unwrap_or(usize::MAX))
        } else {
            Vec::new()
        };

        utils::gzip_decompress(BufReader::new(file), &mut decompressed_graph)?;
        Self::import(&decompressed_graph)
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
        let capacity =
            32 + (self.inner.raw_nodes().len() * 8) + (self.inner.raw_edges().len() * 17);
        let mut buffer = Vec::with_capacity(capacity);

        parser::export_graph(&self.inner, &mut buffer)?;
        Ok(buffer)
    }

    /// Compresses the graph nodes and edges using gzip and returns the result.
    ///
    /// The output is a compressed version of the data from
    /// [`WotGraph::export`].
    pub fn export_gzip(&self) -> Result<Vec<u8>, error::Error> {
        let exported_graph = self.export()?;
        // It's usually half of the size, why not allocate it first
        let mut compressed_graph = Vec::with_capacity(exported_graph.len() / 2);
        utils::gzip_compress(exported_graph.as_slice(), &mut compressed_graph)?;

        Ok(compressed_graph)
    }

    /// Export the graph to a file.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), error::Error> {
        use std::{
            fs::File,
            io::{BufWriter, Write},
        };

        let mut writer = BufWriter::new(File::create(path)?);
        parser::export_graph(&self.inner, &mut writer)?;
        writer.flush()?;

        Ok(())
    }

    /// Export a gzip compressed graph to file.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub fn export_to_file_gzip<P: AsRef<Path>>(&self, path: P) -> Result<(), error::Error> {
        use std::{
            fs::File,
            io::{BufWriter, Write},
        };

        let mut file = BufWriter::new(File::create(path)?);
        let mut encoder = flate2::write::GzEncoder::new(&mut file, utils::COMPRESSION_LEVEL);

        parser::export_graph(&self.inner, &mut encoder)?;
        encoder.finish()?;
        file.flush()?;

        Ok(())
    }
}
