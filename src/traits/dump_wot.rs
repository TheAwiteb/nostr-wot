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

use petgraph::graph::NodeIndex;

use crate::{relations::Relation, traits::basic::BasicOperationsExt};

#[easy_ext::ext(DumpWotExt)]
pub impl crate::GraphType {
    /// Counts the trust score between source and target within max_hops
    /// distance. The score is calculated as (follow_count - mute_count),
    /// where follow_count is the number of nodes following the target
    /// within `max_hops` from source, and mute_count is the number
    /// of nodes muting the target within the same distance.
    #[inline]
    fn dump_wot(&self, source: NodeIndex, target: NodeIndex, max_hops: u8) -> isize {
        isize::try_from(self.count_matches_in_hops(source, target, Relation::Follow, max_hops))
            .unwrap_or(isize::MAX)
            .checked_sub(
                isize::try_from(self.count_matches_in_hops(
                    source,
                    target,
                    Relation::Mute,
                    max_hops,
                ))
                .unwrap_or(isize::MAX),
            )
            .unwrap_or(isize::MIN)
    }
}
