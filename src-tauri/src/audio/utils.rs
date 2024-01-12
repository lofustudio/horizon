use anyhow::anyhow;
use rtrb::Consumer;

/// Reads from a rtrb consumer into a buffer.
// Modified from https://github.com/mgeier/rtrb/blob/0.3.0/src/chunks.rs#L860-L875
// Apache License 2.0 OR MIT License
pub fn read_from_consumer<T: Clone + Copy>(
    buf_r: &mut Consumer<T>,
    buf: &mut [T],
) -> anyhow::Result<usize> {
    use rtrb::chunks::ChunkError::TooFewSlots;
    let chunk = match buf_r.read_chunk(buf.len()) {
        Ok(chunk) => chunk,
        Err(TooFewSlots(0)) => return Err(anyhow!("output buffer underrun")),
        Err(TooFewSlots(n)) => buf_r.read_chunk(n).unwrap(),
    };
    let (first, second) = chunk.as_slices();
    let mid = first.len();
    let end = chunk.len();
    // NB: If buf.is_empty(), chunk will be empty as well and the following are no-ops:
    buf[..mid].copy_from_slice(first);
    buf[mid..end].copy_from_slice(second);
    chunk.commit_all();
    Ok(end)
}
