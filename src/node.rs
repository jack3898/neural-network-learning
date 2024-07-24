use std::fmt::Display;

#[derive(Debug)]
pub struct Node {
    pub value: f64,
    pub weight: f64,
    pub inputs: Vec<Node>,
}

impl From<f64> for Node {
    fn from(item: f64) -> Self {
        Self {
            value: item,
            weight: 1.0,
            inputs: vec![],
        }
    }
}

impl From<(f64, f64)> for Node {
    fn from((value, weight): (f64, f64)) -> Self {
        Self {
            value,
            weight,
            inputs: vec![],
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node ({})", self.value)
    }
}

impl Node {
    pub fn new(value: f64, weight: f64) -> Self {
        return Self {
            value,
            weight,
            inputs: vec![],
        };
    }

    pub fn mul(&self) -> f64 {
        self.value * self.weight
    }
}

#[cfg(test)]
mod node_tests {
    use crate::node::Node;

    #[test]
    fn nodes_should_be_equal_with_different_weights() {
        assert_eq!(Node::new(1.0, 1.0), Node::new(1.0, 0.0));
    }

    #[test]
    fn should_be_constructed_from_float() {
        assert_eq!(
            Node {
                value: 0.5,
                weight: 1.0,
                inputs: vec![]
            },
            0.5.into()
        )
    }

    #[test]
    fn should_be_constructed_from_tuple() {
        assert_eq!(
            Node {
                value: 1.0,
                weight: 0.5,
                inputs: vec![]
            },
            (1.0, 0.5).into()
        )
    }
}
