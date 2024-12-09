pub fn flatten_image(image: &[Vec<f64>]) -> Vec<f64> {
    image.iter().flat_map(|row| row.clone()).collect()
}
