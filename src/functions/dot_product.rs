/// Get the dot product of two vectors.
/// This version multiplies the corresponding values between two vectors, and sums the result.
pub fn dot(vec1: &[f64], vec2: &[f64]) -> Result<f64, &'static str> {
    if vec1.len() != vec2.len() {
        // Returning a static string is not recommended for error handling. For now this will do.
        return Err("Cannot get dot product of two vectors with different lengths.");
    }

    // Probably best to use a crate to calculate the dot product later. This is actually not the fastest implementation.
    let dot = vec1.iter().zip(vec2).map(|(x, y)| x * y).sum();

    Ok(dot)
}

#[cfg(test)]
mod dot_tests {
    use super::dot;

    #[test]
    fn should_err_when_mismatched_vectors() {
        let dot_product = dot(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0]);

        assert!(dot_product.is_err());
    }

    #[test]
    fn should_calculate_correct_dot_product() {
        let dot_product = dot(&vec![1.0, 2.0, 3.0], &vec![2.0, 3.0, 4.0]).unwrap();

        assert_eq!(dot_product, 20.0);
    }
}
