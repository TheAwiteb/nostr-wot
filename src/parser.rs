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

use std::{
    collections::HashMap,
    io::{Cursor, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::GraphSerializationError;

/// Export the graph nodes and edges in a binary format (little-endian)
///
/// Format:
/// - 8 bytes: nodes capacity
/// - 8 bytes: edges capacity
/// - 8 bytes: number of nodes
/// - 8 bytes: number of edges
/// - N * 8 bytes: node weights
/// - E * 17 bytes: edges (8 bytes source, 1 byte relation, 8 bytes target)
pub fn export_graph<W: Write>(
    graph: &crate::GraphType,
    writer: &mut W,
) -> Result<(), crate::error::Error> {
    let nodes = graph.raw_nodes();
    let edges = graph.raw_edges();
    let (nodes_capacity, edges_capacity) = graph.capacity();

    // Write header
    writer.write_u64::<LittleEndian>(nodes_capacity as u64)?;
    writer.write_u64::<LittleEndian>(edges_capacity as u64)?;
    writer.write_u64::<LittleEndian>(nodes.len() as u64)?;
    writer.write_u64::<LittleEndian>(edges.len() as u64)?;

    // Write nodes
    for node in nodes {
        writer.write_u64::<LittleEndian>(node.weight)?;
    }

    // Write edges
    for edge in edges {
        writer.write_u64::<LittleEndian>(graph[edge.source()])?;
        writer.write_u8(edge.weight)?;
        writer.write_u64::<LittleEndian>(graph[edge.target()])?;
    }

    Ok(())
}

/// Import the graph from binary format
pub fn import_graph(data: &[u8]) -> Result<crate::GraphType, crate::error::Error> {
    if data.len() < 32 {
        return Err(GraphSerializationError::InsufficientData(data.len()).into());
    }

    let mut cursor = Cursor::new(data);

    // Read header
    let nodes_capacity = cursor.read_u64::<LittleEndian>()? as usize;
    let edges_capacity = cursor.read_u64::<LittleEndian>()? as usize;
    let num_nodes = cursor.read_u64::<LittleEndian>()? as usize;
    let num_edges = cursor.read_u64::<LittleEndian>()? as usize;

    // Validate expected data size
    let expected_size = 32 + (num_nodes * 8) + (num_edges * 17);
    if data.len() < expected_size {
        return Err(GraphSerializationError::InsufficientData(data.len()).into());
    }

    // Create graph with appropriate capacity
    let mut graph = crate::GraphType::with_capacity(nodes_capacity, edges_capacity);

    // Build a map for fast node lookup
    let mut node_map = HashMap::with_capacity(num_nodes);

    // Read nodes
    for _ in 0..num_nodes {
        let weight = cursor.read_u64::<LittleEndian>()?;
        let idx = graph.add_node(weight);
        node_map.insert(weight, idx);
    }

    // Read edges
    for _ in 0..num_edges {
        let source_weight = cursor.read_u64::<LittleEndian>()?;
        let relation = cursor.read_u8()?;
        let target_weight = cursor.read_u64::<LittleEndian>()?;

        let source_idx = node_map
            .get(&source_weight)
            .ok_or(GraphSerializationError::NodeNotFound(source_weight))?;
        let target_idx = node_map
            .get(&target_weight)
            .ok_or(GraphSerializationError::NodeNotFound(target_weight))?;

        graph.add_edge(*source_idx, *target_idx, relation);
    }

    Ok(graph)
}
