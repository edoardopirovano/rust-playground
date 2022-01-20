mod join;
mod relations;

use join::do_join;
use rand::Rng;
use relations::random_sorted_relation;
use std::cmp::min;

fn main() {
    let mut rng = rand::thread_rng();
    let left = random_sorted_relation();
    let right = random_sorted_relation();
    let join_prefix = rng.gen_range(2..min(left.arity, right.arity) + 1);
    println!("Left: {}", left);
    println!("Right: {}", right);
    println!("Joining on prefix {}", join_prefix);
    let result = do_join(left, right, join_prefix);
    println!("Result: {}", result);
}
