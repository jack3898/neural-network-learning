/// A basic step function
/// If the input is greater than or equal to 0 then 1.0 is returned. Otherwise, 0.0 is returned.
pub fn step(input: f64) -> f64 {
    if input >= 0.0 {
        1.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod step_tests {
    use super::step;

    #[test]
    fn should_step_when_0() {
        assert_eq!(step(0.0), 1.0);
    }

    #[test]
    fn should_not_step_when_lt_0() {
        assert_eq!(step(-0.1), 0.0);
    }

    #[test]
    fn should_step_when_gt_0() {
        assert_eq!(step(0.1), 1.0);
    }

    #[test]
    fn should_step_when_gt_1() {
        assert_eq!(step(1.1), 1.0);
    }
}
