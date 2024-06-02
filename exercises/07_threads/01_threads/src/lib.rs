// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawn threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // todo!()

    let mut first_half = vec![];
    first_half.extend_from_slice(&v[..(v.len()) / 2]);
    let mut second_half = vec![];
    second_half.extend_from_slice(&v[(v.len()) / 2..]);

    let first_handle = thread::spawn(move || {
        let first_sum = first_half.iter().sum::<i32>();
        first_sum
    });

    let second_handle = thread::spawn(move || {
        let second_sum = second_half.iter().sum::<i32>();
        second_sum
    });

    first_handle.join().unwrap() + second_handle.join().unwrap()

    //  // alt (better?) method:
    // let mid = v.len() / 2;
    // let (first_half, second_half) = v.split_at(mid);
    // let first = thread::spawn(move || first_half.into_iter().sum()).join().unwrap();
    // let second = thread::spawn(move || second_half.into_iter().sum()).join().unwrap();
    // return first + second;
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
