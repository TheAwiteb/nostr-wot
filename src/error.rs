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

use std::collections::TryReserveError;

/// Graph serialization error
#[derive(Debug, thiserror::Error)]
pub enum GraphSerializationError {
    #[error("Invalid data: insufficient bytes (expected at least {0}, got less)")]
    InsufficientData(usize),
    #[error("Invalid format: missing separator between nodes and edges")]
    InvalidFormat,
    #[error("Node not found in graph: {0}")]
    NodeNotFound(u64),
}


/// General errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    GraphSerializationError(#[from] GraphSerializationError),
    #[error("Failed to allocate memory: {0}")]
    MemoryAllocation(#[from] TryReserveError),
}
