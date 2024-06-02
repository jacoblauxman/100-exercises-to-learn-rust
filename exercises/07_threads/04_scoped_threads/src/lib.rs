// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    // todo!()
    let mid = v.len() / 2;
    let (first_half, second_half) = v.split_at(mid);

    std::thread::scope(|scope| {
        let first_handle = scope
            .spawn(|| first_half.iter().sum::<i32>())
            .join()
            .unwrap();
        let second_handle = scope
            .spawn(|| second_half.iter().sum::<i32>())
            .join()
            .unwrap();

        first_handle + second_handle
    })
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
