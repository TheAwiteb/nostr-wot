use nostr::key::Keys;
use petgraph::{Direction, graph::NodeIndex};

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

mod basic_operations {
    use super::*;

    #[test]
    fn neighbors_no_outgoing() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p2, p1, Relation::Follow);
        graph.add_edge(p2, p3, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);

        let p2_neig = graph.neighbors(p1, Relation::Follow, Direction::Outgoing);
        assert_eq!(p2_neig.count(), 0);
    }

    #[test]
    fn neighbors_outgoing() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p2, p1, Relation::Follow);
        graph.add_edge(p2, p3, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);

        let neighbors = graph
            .neighbors(p2, Relation::Follow, Direction::Outgoing)
            .collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&p1));
        assert!(neighbors.contains(&p3));
    }

    #[test]
    fn neighbors_incoming() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p2, p1, Relation::Follow);
        graph.add_edge(p3, p1, Relation::Follow);
        graph.add_edge(p1, p2, Relation::Follow);

        let neighbors = graph
            .neighbors(p1, Relation::Follow, Direction::Incoming)
            .collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&p2));
        assert!(neighbors.contains(&p3));
    }

    #[test]
    fn neighbors_no_incoming() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p1, p3, Relation::Follow);
        graph.add_edge(p2, p3, Relation::Follow);

        // p1 has no incoming 'Follow' relations
        let p1_incoming_neig = graph.neighbors(p1, Relation::Follow, Direction::Incoming);
        assert_eq!(p1_incoming_neig.count(), 0);
    }

    #[test]
    fn no_neighbors_in_hops() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();


        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p1, Relation::Follow);
        graph.add_edge(p3, p4, Relation::Follow);
        graph.add_edge(p4, p2, Relation::Follow);

        assert_eq!(
            graph.count_neighbors_in_hops(p1, p4, Relation::Follow, 2),
            0
        );
    }

    #[test]
    fn no_incoming_neighbors_in_hops() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();


        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p1, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);
        graph.add_edge(p4, p2, Relation::Follow);

        assert_eq!(
            graph.count_neighbors_in_hops(p1, p4, Relation::Follow, 2),
            0
        );
    }

    #[test]
    fn no_outgoing_neighbors_in_hops() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();


        graph.add_edge(p2, p1, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);
        graph.add_edge(p3, p1, Relation::Follow);
        graph.add_edge(p4, p3, Relation::Follow);

        assert_eq!(
            graph.count_neighbors_in_hops(p1, p4, Relation::Follow, 2),
            0
        );
    }

    #[test]
    fn one_neighbors_in_hops() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();


        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p4, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);
        graph.add_edge(p3, p4, Relation::Follow);

        assert_eq!(
            graph.count_neighbors_in_hops(p1, p4, Relation::Follow, 2),
            1
        );
    }

    #[test]
    fn two_hops_neighbors() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();


        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p4, Relation::Follow);
        graph.add_edge(p4, p3, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);

        assert_eq!(
            graph.count_neighbors_in_hops(p1, p3, Relation::Follow, 2),
            1
        );
        assert_eq!(
            graph.count_neighbors_in_hops(p1, p3, Relation::Follow, 1),
            0
        );
    }

    #[test]
    fn three_hops_neighbors() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p5 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        // p1 -> p2 -> p3 -> p4 -> p5
        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p3, Relation::Follow);
        graph.add_edge(p3, p4, Relation::Follow);
        graph.add_edge(p4, p5, Relation::Follow);

        // p1 -> p2 -> p5
        graph.add_edge(p2, p5, Relation::Follow);

        // p1 -> p2 -> p3 -> p5
        graph.add_edge(p3, p5, Relation::Follow);

        assert_eq!(
            graph.count_neighbors_in_hops(p1, p5, Relation::Follow, 1),
            1
        );
        assert_eq!(
            graph.count_neighbors_in_hops(p1, p5, Relation::Follow, 2),
            2
        );
        assert_eq!(
            graph.count_neighbors_in_hops(p1, p5, Relation::Follow, 3),
            3
        );
    }
}

mod dump_wot {
    use super::*;

    #[test]
    fn direct_follow_no_mute() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);

        assert_eq!(graph.dump_wot(p1, p2, 1), 1);
        assert_eq!(graph.dump_wot(p1, p2, 2), 1);
    }

    #[test]
    fn direct_mute_no_follow() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Mute);

        assert_eq!(graph.dump_wot(p1, p2, 1), -1);
        assert_eq!(graph.dump_wot(p1, p2, 2), -1);
    }

    #[test]
    fn direct_follow_and_mute_from_source_should_not_count() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p1, p2, Relation::Mute);

        assert_eq!(graph.dump_wot(p1, p2, 1), 0);
        assert_eq!(graph.dump_wot(p1, p2, 2), 0);
    }

    #[test]
    fn indirect_follow_no_mute() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p3, Relation::Follow);

        assert_eq!(graph.dump_wot(p1, p3, 0), 0);
        assert_eq!(graph.dump_wot(p1, p3, 1), 1);
        assert_eq!(graph.dump_wot(p1, p3, 2), 1);
    }

    #[test]
    fn indirect_mute_no_follow() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p3, Relation::Mute);

        assert_eq!(graph.dump_wot(p1, p3, 0), 0);
        assert_eq!(graph.dump_wot(p1, p3, 1), -1);
        assert_eq!(graph.dump_wot(p1, p3, 2), -1);
    }

    #[test]
    fn mixed_follow_and_mute_within_hops() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p5 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p5, Relation::Follow);

        graph.add_edge(p1, p3, Relation::Follow);
        graph.add_edge(p3, p5, Relation::Mute);

        graph.add_edge(p1, p4, Relation::Follow);
        graph.add_edge(p4, p5, Relation::Follow);

        assert_eq!(graph.dump_wot(p1, p5, 0), 0);
        assert_eq!(graph.dump_wot(p1, p5, 1), 1);
        assert_eq!(graph.dump_wot(p1, p5, 2), 1);
        assert_eq!(graph.dump_wot(p1, p5, 3), 1);
    }

    #[test]
    fn multiple_paths_to_same_intermediate_node() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p1, p3, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);
        graph.add_edge(p2, p4, Relation::Follow);

        assert_eq!(graph.dump_wot(p1, p4, 0), 0);
        assert_eq!(graph.dump_wot(p1, p4, 1), 1);
        assert_eq!(graph.dump_wot(p1, p4, 2), 1);
        assert_eq!(graph.dump_wot(p1, p4, 3), 1);
    }

    #[test]
    fn no_path_between_source_and_target() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p3, p2, Relation::Follow);
        graph.add_edge(p3, p1, Relation::Follow);
        graph.add_edge(p2, p1, Relation::Follow);

        assert_eq!(graph.dump_wot(p1, p2, 1), 0);
        assert_eq!(graph.dump_wot(p1, p2, 2), 0);
        assert_eq!(graph.dump_wot(p1, p2, 5), 0);
    }

    #[test]
    fn max_hops_zero() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p3, p2, Relation::Follow);

        assert_eq!(graph.dump_wot(p1, p2, 0), 1);
    }

    #[test]
    fn multiple_paths_different_relations() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p5 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p6 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p7 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p5, Relation::Follow); // +1
        graph.add_edge(p2, p3, Relation::Follow);
        graph.add_edge(p3, p5, Relation::Mute); // -1
        graph.add_edge(p3, p4, Relation::Follow);
        graph.add_edge(p4, p5, Relation::Follow); // +1
        graph.add_edge(p4, p6, Relation::Follow);
        graph.add_edge(p6, p5, Relation::Mute); // -1
        graph.add_edge(p4, p7, Relation::Follow);
        graph.add_edge(p7, p5, Relation::Mute); // -1

        assert_eq!(graph.dump_wot(p1, p5, 1), 1); // (+1)
        assert_eq!(graph.dump_wot(p1, p5, 2), 0); // (+1) (-1)
        assert_eq!(graph.dump_wot(p1, p5, 3), 1); // (+1) (-1) (+1)
        assert_eq!(graph.dump_wot(p1, p5, 4), -1); // (+1) (-1) (+1) (-2)
    }

    #[test]
    fn complex_multiple_hops() {
        let mut graph = WotGraph::new();

        let p1 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p2 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p3 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p4 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p5 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p6 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();
        let p7 = graph.add_node_pkey(&Keys::generate().public_key).unwrap();

        graph.add_edge(p1, p2, Relation::Follow);
        graph.add_edge(p2, p7, Relation::Follow); // +1

        graph.add_edge(p1, p3, Relation::Follow);
        graph.add_edge(p3, p4, Relation::Follow);
        graph.add_edge(p4, p7, Relation::Mute); // -1

        graph.add_edge(p1, p5, Relation::Follow);
        graph.add_edge(p5, p7, Relation::Follow); // +1
        graph.add_edge(p5, p6, Relation::Follow);
        graph.add_edge(p6, p7, Relation::Follow); // +1

        assert_eq!(graph.dump_wot(p1, p7, 1), 2);
        assert_eq!(graph.dump_wot(p1, p7, 2), 2);
        assert_eq!(graph.dump_wot(p1, p7, 3), 2);
    }
}
fn node_idx(graph: &WotGraph, number: u64) -> NodeIndex {
    let inner = &graph.inner;
    inner.node_indices().find(|i| inner[*i] == number).unwrap()
}
