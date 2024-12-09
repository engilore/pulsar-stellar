pub fn detect_edges(data: &[f64]) -> Vec<f64> {
    data.iter()
        .map(|&x| {
            if x > 0.5 {
                1.0
            } else {
                0.0
            }
        })
        .collect()
}
