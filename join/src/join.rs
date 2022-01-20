use crate::relations::{make_sorted, Relation};

fn compare_prefix(
    left: &Relation,
    left_index: usize,
    right: &Relation,
    right_index: usize,
    prefix: usize,
) -> i32 {
    for i in 0..prefix {
        let cmp = left.get(left_index, i) - right.get(right_index, i);
        if cmp != 0 {
            return cmp;
        }
    }
    0
}

fn write_match(
    left: &Relation,
    left_index: usize,
    right: &Relation,
    right_index: usize,
    data: &mut Vec<i32>,
    prefix: usize,
) {
    for i in 0..left.arity {
        data.push(left.get(left_index, i));
    }
    for i in prefix..right.arity {
        data.push(right.get(right_index, i));
    }
}

fn binary_search(relation: &Relation, low: usize, high: usize, prefix: usize) -> usize {
    let mut top = high;
    let mut bot = low;
    while bot < top {
        let mid = (bot + top) / 2;
        let cmp = compare_prefix(relation, mid, relation, mid, prefix);
        if cmp >= 0 {
            top = mid - 1;
        } else {
            bot = mid + 1;
        }
    }
    bot
}

pub fn do_join(left: &Relation, right: &Relation, prefix: usize) -> Relation {
    let arity = left.arity + right.arity - prefix;
    let mut data = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left.size && right_index < right.size {
        let mut cmp = compare_prefix(&left, left_index, &right, right_index, prefix);
        if cmp == 0 {
            let prev_right = right_index;
            while cmp == 0 {
                write_match(&left, left_index, &right, right_index, &mut data, prefix);
                right_index += 1;
                if right_index == right.size {
                    break;
                }
                cmp = compare_prefix(&left, left_index, &right, right_index, prefix);
            }
            right_index = prev_right;
            left_index += 1;
        } else if cmp < 0 {
            left_index = binary_search(&left, left_index + 1, left.size - 1, prefix);
        } else {
            right_index = binary_search(&right, right_index + 1, right.size - 1, prefix);
        }
    }
    let size = data.len() / arity;
    make_sorted(data, size, arity)
}

#[cfg(test)]
mod tests {
    use crate::relations::Relation;

    fn unary_relation_a() -> Relation {
        Relation {
            data: vec![1, 2, 3, 4, 5],
            size: 5,
            arity: 1,
        }
    }

    fn unary_relation_b() -> Relation {
        Relation {
            data: vec![3, 5, 8],
            size: 3,
            arity: 1,
        }
    }

    fn relation_arity_3() -> Relation {
        Relation {
            data: vec![
                0, 1, 5, // Tuple 1
                0, 1, 7, // Tuple 2
                2, 3, 5, // Tuple 3
            ],
            size: 3,
            arity: 3,
        }
    }

    fn relation_arity_4() -> Relation {
        Relation {
            data: vec![
                0, 0, 0, 0, // Tuple 1
                0, 1, 0, 0, // Tuple 2
                0, 1, 5, 5, // Tuple 3
                1, 6, 6, 2, // Tuple 4
                2, 3, 6, 6, // Tuple 5
                6, 6, 6, 6, // Tuple 6
            ],
            size: 6,
            arity: 4,
        }
    }

    #[test]
    fn test_unary_join() {
        assert_eq!(
            super::do_join(&unary_relation_a(), &unary_relation_b(), 1),
            Relation {
                data: vec![3, 5],
                size: 2,
                arity: 1,
            }
        );
    }

    #[test]
    fn test_unary_with_ternary_join() {
        assert_eq!(
            super::do_join(&unary_relation_a(), &relation_arity_3(), 1),
            Relation {
                data: vec![2, 3, 5],
                size: 1,
                arity: 3,
            }
        );
    }

    #[test]
    fn test_complex_join() {
        assert_eq!(
            super::do_join(&relation_arity_3(), &relation_arity_4(), 2),
            Relation {
                data: vec![
                    0, 1, 5, 0, 0, // Tuple 1
                    0, 1, 5, 5, 5, // Tuple 2
                    0, 1, 7, 0, 0, // Tuple 3
                    0, 1, 7, 5, 5, // Tuple 4
                    2, 3, 5, 6, 6, // Tuple 5
                ],
                size: 5,
                arity: 5,
            }
        );
    }
}
