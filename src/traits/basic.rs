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

use std::collections::HashSet;

use petgraph::{Direction, graph::NodeIndex, visit::EdgeRef};

use crate::relations::Relation;

#[easy_ext::ext(BasicOperationsExt)]
pub impl crate::GraphType {
    /// Finds the neighboring nodes of `source` based on the given `relation`
    /// and `direction`.
    ///
    /// For [`Direction::Outgoing`], returns nodes that `source` has the
    /// relation **to**. For [`Direction::Incoming`], returns nodes that
    /// have the relation **to** `source`.
    fn get_matches_neighbors(
        &self,
        source: NodeIndex,
        relation: Relation,
        direction: Direction,
    ) -> impl Iterator<Item = NodeIndex> {
        self.edges_directed(source, direction)
            .filter_map(move |edge| {
                if edge.weight() == &(relation as u8) {
                    if direction == Direction::Outgoing {
                        Some(edge.target())
                    } else {
                        Some(edge.source())
                    }
                } else {
                    None
                }
            })
    }

    /// Counts how many nodes in the source's following hops (up to `max_hops`)
    /// have the given `relation` with the target.
    ///
    /// For each hop, checks if any node in that hop has the specified relation
    /// to the target. Each node is only counted once even if it appears in
    /// multiple hops.
    ///
    /// # Time Complexity
    /// O(V + E) where V is reachable vertices and E is their edges
    ///
    /// # Space Complexity
    /// O(V) for visited set and current level storage
    fn count_matches_in_hops(
        &self,
        source: NodeIndex,
        target: NodeIndex,
        relation: Relation,
        max_hops: u8,
    ) -> usize {
        // Early return if either node doesn't exist in the graph
        if self.raw_nodes().get(source.index()).is_none()
            || self.raw_nodes().get(target.index()).is_none()
        {
            return 0;
        }

        // Collect all nodes that have the specified relation pointing TO the target.
        // Using HashSet for O(1) lookup performance during the BFS traversal.
        // Example: if relation=Follow, this contains all nodes that follow the target.
        let target_incoming: HashSet<NodeIndex> = self
            .get_matches_neighbors(target, relation, Direction::Incoming)
            .collect();

        // if no nodes have this relation to target, there's nothing to count
        if target_incoming.is_empty() {
            return 0;
        }

        if max_hops == 0 {
            return usize::from(target_incoming.contains(&source));
        }

        // track visited nodes to Prevent counting the same node multiple times
        let mut visited = HashSet::new();

        // BFS frontier: nodes at the current hop level
        let mut current_level = vec![source];
        let mut count = 0;

        // Traverse up to max_hops levels (inclusive of hop 0 which is the source)
        // Hop 0: source node
        // Hop 1: nodes directly followed by source
        // Hop N: nodes N steps away from source via Follow edges
        for hop in 0..=max_hops {
            for node in &current_level {
                if visited.insert(*node) && target_incoming.contains(node) {
                    count += 1;
                }
            }

            // Skip building next level on final iteration
            if hop == max_hops {
                break;
            }

            // Build the next level of the BFS frontier by collecting all outgoing
            // Follow edges from current level nodes.
            // Filter out already-visited nodes to prevent cycles and redundant work.
            current_level = current_level
                .iter()
                .flat_map(|idx| {
                    self.get_matches_neighbors(*idx, Relation::Follow, Direction::Outgoing)
                })
                .filter(|idx| !visited.contains(idx))
                .collect();

            // if no more nodes to explore, exit early
            if current_level.is_empty() {
                break;
            }
        }

        count
    }
}
