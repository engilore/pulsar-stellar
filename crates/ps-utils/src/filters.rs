pub fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 {
        return data.to_vec();
    }
    data.windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window.len() as f64)
        .collect()
}
