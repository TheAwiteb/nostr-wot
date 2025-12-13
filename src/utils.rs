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

/// Hashes the given byte slice using the xxHash 64-bit algorithm with a
/// constant seed value.
pub fn hash_bytes(bytes: &[u8]) -> u64 {
    xxhash_rust::xxh64::xxh64(bytes, 0xC0FFEE)
}

// /// Removes duplicates from a vector
// pub(crate) fn dedub<I: PartialEq + Ord>(mut vector: Vec<I>) -> Vec<I> {
//     vector.sort_unstable();
//     vector.dedup();
//     vector
// }
