fn merge<T: Copy + Ord>(a: &mut [T], mid: usize) {
    let aux: Vec<T> = a.to_vec();
    let len: usize = a.len();
    let mut i: usize = 0;
    let mut j: usize = mid;
    let mut p: usize = 0;

    while i < mid && j < len {
        if aux[i] < aux[j] {
            a[p] = aux[i];
            i += 1;
        } else {
            a[p] = aux[j];
            j += 1;
        }
        p += 1;
    }

    if i < mid {
        a[p..len].clone_from_slice(&aux[i..mid]);
    }
    if j < len {
        a[p..len].clone_from_slice(&aux[j..len]);
    }
}

fn top_down_merge_sort<T: Copy + Ord>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    let mid: usize = a.len() / 2;

    top_down_merge_sort(&mut a[..mid]);
    top_down_merge_sort(&mut a[mid..]);
    merge(a, mid);
}

fn bottom_up_merge_sort<T: Copy + Ord>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    let len: usize = a.len();
    let mut sub_array_size: usize = 1;
    while sub_array_size < len {
        let mut start_index: usize = 0;
        // still have more than one sub-arrays to sort
        while len - start_index > sub_array_size {
            let end_idx: usize = if start_index + 2 * sub_array_size > len {
                len
            } else {
                start_index + 2 * sub_array_size
            };
            // merge a[start_index..start_index+sub_array_size] and a[start_index+sub_array_size, end_idx]
            // NOTE: mid is a relative start_index number starting from `start_index`
            merge(&mut a[start_index..end_idx], sub_array_size);
            // update `start_index` to merge the next sub-arrays
            start_index = end_idx;
        }
        sub_array_size *= 2;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod top_down {
        use super::super::*;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec!["a", "bb", "cc", "d"]);
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![]);
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1]);
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }
    }

    #[cfg(test)]
    mod bottom_up {
        use super::super::*;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec!["a", "bb", "cc", "d"]);
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![]);
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1]);
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }
    }
}
