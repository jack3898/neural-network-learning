pub fn elu(x: f64, alpha: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        alpha * (x.exp() - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::elu;

    #[test]
    fn elu_minus_1() {
        assert_eq!(elu(-1.0, 1.0), -0.6321205588285577);
    }

    #[test]
    fn should_plateau() {
        assert_eq!(elu(-10.0, 1.0), -0.9999546000702375);
        assert_eq!(elu(-100.0, 1.0), -1.0);
        assert_eq!(elu(-1000.0, 1.0), -1.0);
    }

    #[test]
    fn should_linear_after_0() {
        assert_eq!(elu(0.0, 1.0), 0.0);
        assert_eq!(elu(1.0, 1.0), 1.0);
        assert_eq!(elu(2.0, 1.0), 2.0);
        assert_eq!(elu(3.0, 1.0), 3.0);
    }
}
