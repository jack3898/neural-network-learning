use std::fmt::Display;

#[derive(Debug)]
pub struct Node {
    pub value: f32,
    pub weight: f32,
}

impl From<f32> for Node {
    fn from(item: f32) -> Self {
        Self {
            value: item,
            weight: 1.0,
        }
    }
}

impl From<(f32, f32)> for Node {
    fn from((value, weight): (f32, f32)) -> Self {
        Self { value, weight }
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
    pub fn new(value: f32, weight: f32) -> Self {
        return Self { value, weight };
    }

    pub fn mul(&self) -> f32 {
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
                weight: 1.0
            },
            0.5.into()
        )
    }

    #[test]
    fn should_be_constructed_from_tuple() {
        assert_eq!(
            Node {
                value: 1.0,
                weight: 0.5
            },
            (1.0, 0.5).into()
        )
    }
}
