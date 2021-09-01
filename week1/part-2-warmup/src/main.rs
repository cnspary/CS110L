/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut ret_vec: Vec<i32> = Vec::new();

    for num in v.iter() {
        ret_vec.push(num + n);
    }

    ret_vec
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for x in v.iter_mut() {
        *x += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut set: HashSet<i32> = HashSet::new();

    let mut idx: usize = 0;
    while idx < v.len() {
        if set.contains(&v[idx]) {
            v.remove(idx);
            idx -= 1;
        } else {
            set.insert(v[idx]);
        }
        idx += 1;
    }
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
