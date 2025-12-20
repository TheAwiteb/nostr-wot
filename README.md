# nostr-wot

A lightweight nostr WoT (Web of Trust) library designed for simplicity and speed.

## Algorithms

Currently, it only supports **dump WoT**, which calculates the difference
between the number of public keys in the source's contact list that follow the
target and the number that mute it.

## Examples

It's straightforward. First, add the public keys and their
relationships, then run the algorithms on them to analyze the Web of Trust
(WoT).

### Dump WoT

```rust
use nostr_wot::WotGraph;
use nostr::key::Keys;

fn main() {
    let mut graph = WotGraph::new();
    
    // Generate and add public keys as nodes to the graph
    let node1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
    let node2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
    let node3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
    let node4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
    
    // Define the relationships between nodes
    graph.add_edge(node1, node2, Relation::Follow);
    graph.add_edge(node2, node3, Relation::Follow);
    graph.add_edge(node3, node4, Relation::Follow);
    
    // Evaluate the WoT score between node1 and node4 with different hop limits
    // With a hop limit of 1, node1 only follows node2 and doesn't reach node4, so the score is 0.
    assert_eq!(graph.dump_wot(node1, node4, 1), 0);
    // With a hop limit of 2, node2 follows node3, which follows node4, so the score is 1.
    assert_eq!(graph.dump_wot(node1, node4, 2), 1);
}
```

## Import and Export

`nostr-wot` is designed for speed by avoiding a database interface, instead, it
operates directly on the graph structure. Each node is compactly stored at 12
bytes (comprising a node ID of **4** bytes and a label of **8** bytes), while
each edge occupies just 13 bytes (with an edge ID of **4** bytes, source ID of
**4** bytes, target ID of **4** bytes, and a relation of **1** byte).

To use `nostr-wot`, you'll need to import your graph into it and export it
for future use. We provide a straightforward API for these operations. While
gzip-compressed bytes are recommended for efficiency, raw bytes are also
supported. The examples below focus on gzip compression for optimal performance.

Note that we leverage [`flate2`]'s Rust-based gzip implementation, ensuring you
don't have to deal with C dependencies, simply compile and run.

### Export

```rust
// ... (your graph somewhere in the scope)
// Export the graph as gzip-compressed bytes
let exported_graph = graph.export_gzip().unwrap();
// Export the graph to a gzip-compressed file
graph.export_to_file_gzip("filename.nostr_wot.gz").unwrap();
```

### Import

```rust
// ... (your exported graph somewhere in the scope)
// Import the graph from gzip-compressed bytes
let graph = WotGraph::import(&exported_graph).unwrap();
// Import the graph from a gzip-compressed file
let graph = WotGraph::import_from_file_gzip("filename.nostr_wot.gz").unwrap();
```

## License

Licensed under the MIT license for more details see `LICENSE` file or <http://opensource.org/licenses/MIT>.

[`flate2`]: https://github.com/rust-lang/flate2-rs
