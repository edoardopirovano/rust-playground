mod join;
mod relations;

use join::do_join;
use rand::Rng;
use relations::*;
use std::cmp::min;

fn write_random_relation(index: usize) {
    random_sorted_relation()
        .write_to_file(&format!("relations/relation{}.bin", index))
        .unwrap();
}

fn read_relation(index: usize) -> Relation {
    read_from_file(&format!("relations/relation{}.bin", index))
}

fn main() {
    std::fs::remove_dir_all("relations").unwrap();
    std::fs::create_dir("relations").unwrap();
    let mut rng = rand::thread_rng();
    let num_relations = rng.gen_range(3..20);
    for i in 0..num_relations {
        write_random_relation(i);
    }
    let num_joins = rng.gen_range(10..100);
    for i in 0..num_joins {
        let left = read_relation(rng.gen_range(0..num_relations));
        let right = read_relation(rng.gen_range(0..num_relations));
        let join_prefix = rng.gen_range(2..min(left.arity, right.arity) + 1);
        let result = do_join(&left, &right, join_prefix);
        result
            .write_to_file(&format!("relations/result{}.bin", i))
            .unwrap();
        println!("[{: >2}/{:0>2}] Did a prefix {} join of relations of size {} and {} to get an arity {} result of size {}.", i + 1, num_joins, join_prefix, left.size, right.size, result.arity, result.size);
    }
    std::fs::remove_dir_all("relations").unwrap();
}
