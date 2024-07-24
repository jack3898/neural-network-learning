use crate::{functions::step, node::Node};

pub fn process_node_row(bias: f64, nodes: &[Node]) -> Node {
    let node_sum = nodes
        .into_iter()
        .map(|node: &Node| -> f64 { node.mul() })
        .sum::<f64>();

    let calculation = bias + node_sum;
    let next_value = step(calculation);

    return Node::new(next_value, 1.0);
}

#[cfg(test)]
mod or_tests {
    use super::process_node_row;

    #[test]
    fn should_return_true_or() {
        assert_eq!(
            process_node_row(-1.0, &[1.0.into(), 1.0.into()]),
            1.0.into()
        );
        assert_eq!(
            process_node_row(-1.0, &[0.0.into(), 1.0.into()]),
            1.0.into()
        );
        assert_eq!(
            process_node_row(-1.0, &[1.0.into(), 0.0.into()]),
            1.0.into()
        );
    }

    #[test]
    fn should_return_false_or() {
        assert_eq!(
            process_node_row(-1.0, &[0.0.into(), 0.0.into()]),
            0.0.into()
        );
    }

    #[test]
    fn should_return_true_and() {
        assert_eq!(
            process_node_row(-2.0, &[1.0.into(), 1.0.into()]),
            1.0.into()
        );
    }

    #[test]
    fn should_return_false_and() {
        assert_eq!(
            process_node_row(-2.0, &[0.0.into(), 0.0.into()]),
            0.0.into()
        );
        assert_eq!(
            process_node_row(-2.0, &[1.0.into(), 0.0.into()]),
            0.0.into()
        );
        assert_eq!(
            process_node_row(-2.0, &[0.0.into(), 1.0.into()]),
            0.0.into()
        );
    }
}
