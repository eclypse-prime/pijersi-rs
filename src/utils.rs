/// Returns a vector of sorted indices
pub fn argsort<T: Ord>(data: &[T], reverse: bool) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    if reverse {
        indices.reverse();
    }
    indices
}

/// Reverse operation of argsort, uses an array of sorted indices to create the original unsorted vector
pub fn reverse_argsort<T: Clone>(original: &[T], indices: &[usize]) -> Vec<T> {
    let mut sorted = Vec::from(original);

    for (index_original, &index) in indices.iter().enumerate() {
        // println!("{index}");
        sorted[index] = original[index_original].clone();
    }

    sorted
}
