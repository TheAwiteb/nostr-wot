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

use std::io::{self, Read, Write};

use flate2::Compression;

pub(crate) const COMPRESSION_LEVEL: Compression = Compression::new(4);

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

/// Compresses data from a reader and writes it to a writer using gzip.
pub(crate) fn gzip_compress<R, W>(mut from: R, to: &mut W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    let mut encoder = flate2::write::GzEncoder::new(to, COMPRESSION_LEVEL);
    io::copy(&mut from, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

/// Decompresses gzip data from a reader and writes it to a writer.
pub(crate) fn gzip_decompress<R, W>(from: R, to: &mut W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    let mut decoder = flate2::read::GzDecoder::new(from);
    io::copy(&mut decoder, to)?;
    Ok(())
}
