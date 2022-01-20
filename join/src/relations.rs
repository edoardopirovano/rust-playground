use crc32fast::Hasher;
use rand::Rng;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Relation {
    pub data: Vec<i32>,
    pub size: usize,
    pub arity: usize,
}

#[derive(Debug)]
struct ChecksumError {
    expected: u32,
    actual: u32,
}

impl fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Checksum error: expected {}, actual {}",
            self.expected, self.actual
        )
    }
}

impl Error for ChecksumError {
    fn description(&self) -> &str {
        "Expected checksum does not match actual checksum"
    }
}

impl Relation {
    fn print_data(&self) -> String {
        let mut result = String::new();
        for i in 0..self.size {
            result.push('[');
            for j in 0..self.arity {
                result.push_str(&*self.data[(i * self.arity) + j].to_string());
                result.push(',');
            }
            if self.arity > 0 {
                result.pop();
            }
            result.push(']');
        }
        result
    }

    fn sort(&mut self) {
        if self.size == 0 {
            return;
        }
        self.quicksort(0, self.size - 1)
    }

    fn quicksort(&mut self, low: usize, high: usize) {
        if low >= high {
            return;
        }
        let p = self.quicksort_partition(low, high);
        if p > 0 {
            self.quicksort(low, p - 1);
        }
        self.quicksort(p + 1, high);
    }

    fn quicksort_partition(&mut self, low: usize, high: usize) -> usize {
        let mut i = low;
        for x in low..high {
            if self.compare(x, high) >= 0 {
                self.swap(i, x);
                i += 1;
            }
        }
        self.swap(i, high);
        i
    }

    fn compare(&self, a: usize, b: usize) -> i32 {
        for j in 0..self.arity {
            let left = self.get(a, j);
            let right = self.get(b, j);
            if left != right {
                return right - left;
            }
        }
        0
    }

    pub fn get(&self, i: usize, j: usize) -> i32 {
        self.data[(i * self.arity) + j]
    }

    fn set(&mut self, i: usize, j: usize, x: i32) {
        self.data[(i * self.arity) + j] = x;
    }

    fn swap(&mut self, a: usize, b: usize) {
        for j in 0..self.arity {
            let temp = self.get(a, j);
            self.set(a, j, self.get(b, j));
            self.set(b, j, temp);
        }
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut buffer = File::create(filename)?;
        let mut hasher = Hasher::new();
        buffer.write(&self.size.to_le_bytes())?;
        hasher.update(&self.size.to_le_bytes());
        buffer.write(&self.arity.to_le_bytes())?;
        hasher.update(&self.arity.to_le_bytes());
        for x in &self.data {
            buffer.write(&x.to_le_bytes())?;
            hasher.update(&x.to_le_bytes());
        }
        buffer.write(&hasher.finalize().to_le_bytes())?;
        Ok(())
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Relation of arity {} and size {}. Tuples: {}",
            self.arity,
            self.size,
            self.print_data()
        )
    }
}

pub fn make_sorted(data: Vec<i32>, size: usize, arity: usize) -> Relation {
    let mut relation = Relation { data, size, arity };
    relation.sort();
    relation
}

pub fn read_from_file(filename: &str) -> Result<Relation, Box<dyn Error>> {
    let mut buffer = File::open(filename).unwrap();
    let mut hasher = Hasher::new();
    let mut scratch = [0; 8];
    buffer.read(&mut scratch)?;
    let size = usize::from_le_bytes(scratch);
    hasher.update(&scratch);
    buffer.read(&mut scratch)?;
    let arity = usize::from_le_bytes(scratch);
    hasher.update(&scratch);
    let mut data = Vec::new();
    for _ in 0..(size * arity) {
        let mut scratch = [0; 4];
        buffer.read(&mut scratch)?;
        data.push(i32::from_le_bytes(scratch));
        hasher.update(&scratch);
    }
    let mut scratch = [0; 4];
    buffer.read(&mut scratch)?;
    let expected = u32::from_le_bytes(scratch);
    let actual = hasher.finalize();
    if expected != actual {
        return Err(Box::new(ChecksumError { expected, actual }));
    }
    Ok(Relation { data, size, arity })
}

pub fn random_sorted_relation() -> Relation {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(1..1000);
    let arity = rng.gen_range(2..5);
    let mut data = Vec::with_capacity(size * arity);
    for _x in 0..(size * arity) {
        data.push(rng.gen_range(0..50));
    }
    make_sorted(data, size, arity)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_sorted() {
        let relation = super::random_sorted_relation();
        for i in 0..(relation.size - 1) {
            assert_eq!(true, relation.compare(i, i + 1) >= 0);
        }
    }

    #[test]
    fn test_display() {
        let relation = super::Relation {
            data: vec![
                1, 2, // Tuple 1
                3, 4, // Tuple 2
                5, 6, // Tuple 3
            ],
            size: 3,
            arity: 2,
        };
        assert_eq!(
            format!("{}", relation),
            "Relation of arity 2 and size 3. Tuples: [1,2][3,4][5,6]"
        );
    }

    #[test]
    fn write_then_read() {
        let temp_file = "temp.bin";
        let relation = super::random_sorted_relation();
        relation.write_to_file(&temp_file).unwrap();
        let read_relation = super::read_from_file(&temp_file).unwrap();
        assert_eq!(relation, read_relation);
        std::fs::remove_file(temp_file).unwrap();
    }
}
