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

fn merge_sort<T: Copy + Ord>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    let mid: usize = a.len() / 2;

    merge_sort(&mut a[..mid]);
    merge_sort(&mut a[mid..]);
    merge(a, mid);
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        merge_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
