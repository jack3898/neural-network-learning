pub mod functions;
pub mod network;
pub mod node;

pub use network::process_node_row;

fn main() {
    // This is the start of my neural network learning.
    // I learned that the core of neural networks lie in the fundamentals of calculus mathematics.
    // A network processes inputs, and calculates probability of a result as its output.
    // It does so using a weighting system, which can influence the outcomes of our node calculations.
    // When a network is being "trained", in short what's happening is the weights passed into our calculations are being adjusted.

    // Below, we are running a function `step` which is a simple step function.
    // How `step()` works is it sums multiplying the nodes with the bias and if the result is greater than or equal to 0, the return value is 1.
    // The first parameter is a bias. The bias affects the total outcome of the function regardless of what the values of parameters 2 and 3 are.
    // Parameters 2 and 3 are nodes. The nodes contain a value and a weight. Using Rust's .into() method, it can convert a float into a node with a default weight of 1.
    // So for example, the first one is -1 + node (0 * 1) + node (1 * 1) which is equal to 0.
    // The above result is 0. So the step function will be 1. 1 is true, so that is the result.
    // Using variance in the weighting and bias, can influence the outcome of the network.
    // Training this specific network is all about balancing the weights you pass in to the step() function to influence its outcome.
    // Of course, the step function is one of, if not the most basic probability function neural networks can use. There are many others that I have yet to code up and try out.

    // Using -1 as a bias and 1 for the node weights results in an OR condition, you can pretend that this super basic network went through training to reach this conclusion.
    println!(
        "OR - 0 and 1 = {}",
        process_node_row(-1.0, &[0.0.into(), 1.0.into()])
    );
    println!(
        "OR - 0 and 0 = {}",
        process_node_row(-1.0, &[0.0.into(), 0.0.into()])
    );
    println!(
        "OR - 1 and 0 = {}",
        process_node_row(-1.0, &[1.0.into(), 0.0.into()])
    );
    println!(
        "OR - 1 and 1 = {}",
        process_node_row(-1.0, &[1.0.into(), 0.0.into()])
    );

    // Likewise, pretend we trained our basic network for AND conditions. After training, the bias weight was adjusted to be -2 and the weights of each node remain unchanged.
    println!(
        "AND - 0 and 1 = {}",
        process_node_row(-2.0, &[0.0.into(), 0.0.into()])
    );
    println!(
        "AND - 0 and 0 = {}",
        process_node_row(-2.0, &[0.0.into(), 0.0.into()])
    );
    println!(
        "AND - 1 and 0 = {}",
        process_node_row(-2.0, &[1.0.into(), 0.0.into()])
    );
    println!(
        "AND - 1 and 1 = {}",
        process_node_row(-2.0, &[1.0.into(), 1.0.into()])
    );

    // This is the first step in my neural network journey!
    // REFERENCE: https://youtu.be/J1QD9hLDEDY?si=2Q-T_5HlbhgwH6PB
}
