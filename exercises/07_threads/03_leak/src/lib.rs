// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // todo!()

    let box_v = Box::new(v);
    let static_v: &'static mut Vec<i32> = Box::leak(box_v);
    let mid = static_v.len() / 2;
    let (first_half, second_half) = static_v.split_at(mid);

    // shorter/better(?):
    // let v = v.leak();
    // let mid = v.len() / 2;
    // let (first_half, second_half) = v.split_at(mid);
    // ...

    let first = thread::spawn(move || first_half.into_iter().sum::<i32>())
        .join()
        .unwrap();
    let second = thread::spawn(move || second_half.into_iter().sum::<i32>())
        .join()
        .unwrap();

    first + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
