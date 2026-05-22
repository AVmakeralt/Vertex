pub fn truncate_and_return<T>(vec: &mut Vec<T>, len: usize) -> Vec<T> {
    vec.split_off(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_and_return_base_case() {
        let mut data = vec![1,2,3,4];
        let split = truncate_and_return(&mut data, 2);

        assert_eq!(data, vec![1,2]); // the first part of the vector should be 1,2
        assert_eq!(split, vec![3,4]); // the second part of the vector should be 3,4
    }

    #[test]
    fn truncate_and_return_at_start() {
        let mut data = vec![1,2,3,4];
        let split = truncate_and_return(&mut data, 0);

        assert_eq!(data, vec![]); // the first part of the vector should be empty
        assert_eq!(split, vec![1,2,3,4]); // the second part of the vector should contain everything
    }

    #[test]
    fn truncate_and_return_at_end() {
        let mut data = vec![1,2,3,4];
        let split = truncate_and_return(&mut data, 4);

        assert_eq!(data, vec![1,2,3,4]); // the first part of the vector should contain everything
        assert_eq!(split, vec![]); // the second part of the vector should be enoty
    }
}
