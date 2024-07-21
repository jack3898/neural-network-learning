use crate::node::Node;

pub fn step(bias: f32, nodes: &[Node]) -> Node {
    let node_sum = nodes
        .into_iter()
        .map(|node: &Node| -> f32 { node.mul() })
        .sum::<f32>();

    let calculation = bias + node_sum;
    let next_value = if calculation >= 0.0 { 1.0 } else { 0.0 };

    return Node::new(next_value, 1.0);
}

#[cfg(test)]
mod or_tests {
    use super::step;

    #[test]
    fn should_return_true_or() {
        assert_eq!(step(-1.0, &[1.0.into(), 1.0.into()]), 1.0.into());
        assert_eq!(step(-1.0, &[0.0.into(), 1.0.into()]), 1.0.into());
        assert_eq!(step(-1.0, &[1.0.into(), 0.0.into()]), 1.0.into());
    }

    #[test]
    fn should_return_false_or() {
        assert_eq!(step(-1.0, &[0.0.into(), 0.0.into()]), 0.0.into());
    }

    #[test]
    fn should_return_true_and() {
        assert_eq!(step(-2.0, &[1.0.into(), 1.0.into()]), 1.0.into());
    }

    #[test]
    fn should_return_false_and() {
        assert_eq!(step(-2.0, &[0.0.into(), 0.0.into()]), 0.0.into());
        assert_eq!(step(-2.0, &[1.0.into(), 0.0.into()]), 0.0.into());
        assert_eq!(step(-2.0, &[0.0.into(), 1.0.into()]), 0.0.into());
    }
}
