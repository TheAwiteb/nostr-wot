use petgraph::graph::NodeIndex;

use crate::{WotGraph, relations::Relation};

mod export_import {
    use super::*;

    #[test]
    fn empty() {
        let graph = WotGraph::new();
        let exported_graph = graph.export().unwrap();
        drop(graph);

        let graph = WotGraph::import(&exported_graph).unwrap();
        let capacity = graph.inner.capacity();
        assert_eq!(capacity.0, 0);
        assert_eq!(capacity.1, 0);
    }

    #[test]
    fn empty_gzip() {
        let graph = WotGraph::new();
        let exported_graph = graph.export_gzip().unwrap();
        drop(graph);

        let graph = WotGraph::import_gzip(&exported_graph).unwrap();
        let capacity = graph.inner.capacity();
        assert_eq!(capacity.0, 0);
        assert_eq!(capacity.1, 0);
    }

    #[test]
    fn single() {
        let mut graph = WotGraph::new();
        graph.add_node(1).unwrap();
        let exported_graph = graph.export().unwrap();
        drop(graph);

        let graph = WotGraph::import(&exported_graph).unwrap();
        assert_eq!(graph.inner.raw_nodes().len(), 1);
        assert_eq!(graph.inner.raw_edges().len(), 0);
    }

    #[test]
    fn multi_node() {
        let mut graph = WotGraph::new();
        graph.add_node(1).unwrap();
        graph.add_node(2).unwrap();
        graph.add_node(3).unwrap();
        let exported_graph = graph.export().unwrap();
        drop(graph);

        let graph = WotGraph::import(&exported_graph).unwrap();
        assert_eq!(graph.inner.raw_nodes().len(), 3);
        assert_eq!(graph.inner.raw_edges().len(), 0);
    }

    #[test]
    fn with_edges() {
        let mut graph = WotGraph::new();
        graph.add_node(1).unwrap();
        graph.add_node(2).unwrap();
        graph
            .add_edge(node_idx(&graph, 1), node_idx(&graph, 2), Relation::Follow)
            .unwrap();
        let exported_graph = graph.export().unwrap();
        drop(graph);

        let graph = WotGraph::import(&exported_graph).unwrap();
        assert_eq!(graph.inner.raw_nodes().len(), 2);
        assert_eq!(graph.inner.raw_edges().len(), 1);
    }

    #[test]
    fn invalid_data() {
        let invalid_data = [3; 30]; // Not a valid exported graph
        assert!(WotGraph::import(&invalid_data).is_err());
    }

    #[test]
    fn invalid_gzip() {
        let invalid_data = [7; 60]; // Not a valid gzipped graph
        assert!(WotGraph::import_gzip(&invalid_data).is_err());
    }

    #[test]
    fn roundtrip_complex_graph() {
        let mut graph = WotGraph::new();
        // Add multiple nodes and edges
        for i in 1..=10 {
            graph.add_node(i).unwrap();
        }
        for i in 1..10 {
            graph
                .add_edge(
                    node_idx(&graph, i),
                    node_idx(&graph, i + 1),
                    Relation::Follow,
                )
                .unwrap();
        }

        let exported = graph.export().unwrap();
        let imported = WotGraph::import(&exported).unwrap();

        assert_eq!(
            graph.inner.raw_nodes().len(),
            imported.inner.raw_nodes().len()
        );
        assert_eq!(
            graph.inner.raw_edges().len(),
            imported.inner.raw_edges().len()
        );
    }

    #[test]
    fn roundtrip_gzip_complex_graph() {
        let mut graph = WotGraph::new();
        // Add multiple nodes and edges
        for i in 1..=10 {
            graph.add_node(i).unwrap();
        }
        for i in 1..10 {
            graph
                .add_edge(
                    node_idx(&graph, i),
                    node_idx(&graph, i + 1),
                    Relation::Follow,
                )
                .unwrap();
        }

        let exported = graph.export_gzip().unwrap();
        let imported = WotGraph::import_gzip(&exported).unwrap();

        assert_eq!(
            graph.inner.raw_nodes().len(),
            imported.inner.raw_nodes().len()
        );
        assert_eq!(
            graph.inner.raw_edges().len(),
            imported.inner.raw_edges().len()
        );
    }
}

fn node_idx(graph: &WotGraph, number: u64) -> NodeIndex {
    let inner = &graph.inner;
    inner.node_indices().find(|i| inner[*i] == number).unwrap()
}
