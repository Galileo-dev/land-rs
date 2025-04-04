pub fn fast_flatten_vecs<T: Copy>(vecs: Vec<Vec<T>>) -> Vec<T> {
    // This is faster than vecs.into_iter().flatten().collect()
    // because it doesn't need to allocate a new Vec
    // (we take ownership of the first Vec and add the rest to it)
    let size: usize = vecs.iter().map(|v| v.len()).sum();
    let mut iter = vecs.into_iter();
    let mut result = if let Some(v) = iter.next() {
        v
    } else {
        return Vec::new();
    };
    result.reserve_exact(size - result.len());
    for v in iter {
        result.extend_from_slice(&v);
    }
    result
}
