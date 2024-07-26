pub mod functions;
pub mod network;
pub mod node;

use functions::{dot, elu};
pub use network::process_node_row;

fn main() {
    let elu_1 = elu(2.0, 1.0);
    let elu_2 = elu(-1.0, 1.0);
    let elu_3 = elu(-10.0, 1.0);

    println!("{elu_1} {elu_2} {elu_3}");

    let dot_1 = dot(&[1.0, 2.0, 3.0], &[2.0, 3.0, 4.0]).unwrap();
    let dot_2 = dot(&[0.35, 1.23, 5.62], &[1.87, 3.45, 9.2]).unwrap();

    println!("{dot_1} {dot_2}");
}
