// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    // todo!()

    let mid = slice.len() / 2;
    let (first_half, second_half): (&'static [i32], &'static [i32]) = slice.split_at(mid);

    let first: i32 = thread::spawn(move || first_half.into_iter().sum())
        .join()
        .unwrap();
    let second: i32 = thread::spawn(move || second_half.into_iter().sum())
        .join()
        .unwrap();

    first + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
