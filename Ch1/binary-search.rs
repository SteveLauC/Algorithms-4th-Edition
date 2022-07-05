use std::cmp::Ordering;

/// arguments:
///     * `key`: key to search
///     * `a`: a sorted array
///
/// return: index of `key` in `a`
fn binary_search<T: Ord>(key: T, a: &[T], is_ascending: bool) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = a.len() - 1;
    let mut mid: usize;
    while low <= high {
        mid = low + (high - low) / 2;

        if is_ascending {
            match a[mid].cmp(&key) {
                Ordering::Greater => high = mid - 1,
                Ordering::Less => low = mid + 1,
                Ordering::Equal => return Some(mid),
            }
        } else {
            match a[mid].cmp(&key) {
                Ordering::Greater => low = mid + 1,
                Ordering::Less => high = mid - 1,
                Ordering::Equal => return Some(mid),
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search("a", &vec![], true);
        assert_eq!(index, None);


        let index = binary_search("a", &vec![], false);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search("a", &vec!["a"], true);
        assert_eq!(index, Some(0));
        let index = binary_search("a", &vec!["a"], false);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search("a", &vec!["a", "b", "c", "d", "google", "zoo"], true);
        assert_eq!(index, Some(0));

        let index = binary_search("google", &vec!["a", "b", "c", "d", "google", "zoo"], true);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn search_strings_desc() {
        let index = binary_search("a", &vec!["zoo", "google", "d", "c", "b", "a"], false);
        assert_eq!(index, Some(5));

        let index = binary_search("zoo", &vec!["zoo", "google", "d", "c", "b", "a"], false);
        assert_eq!(index, Some(0));

        let index = binary_search("google", &vec!["zoo", "google", "d", "c", "b", "a"], false);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn search_ints_asc() {
        let index = binary_search(4, &vec![1, 2, 3, 4], true);
        assert_eq!(index, Some(3));

        let index = binary_search(3, &vec![1, 2, 3, 4], true);
        assert_eq!(index, Some(2));

        let index = binary_search(2, &vec![1, 2, 3, 4], true);
        assert_eq!(index, Some(1));

        let index = binary_search(1, &vec![1, 2, 3, 4], true);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints_desc() {
        let index = binary_search(4, &vec![4, 3, 2, 1], false);
        assert_eq!(index, Some(0));

        let index = binary_search(3, &vec![4, 3, 2, 1], false);
        assert_eq!(index, Some(1));

        let index = binary_search(2, &vec![4, 3, 2, 1], false);
        assert_eq!(index, Some(2));

        let index = binary_search(1, &vec![4, 3, 2, 1], false);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn not_found() {
        let index = binary_search(5, &vec![1, 2, 3, 4], true);
        assert_eq!(index, None);

        let index = binary_search(5, &vec![4, 3, 2, 1], true);
        assert_eq!(index, None);

        let index = binary_search(5, &vec![1, 2, 3, 4], false);
        assert_eq!(index, None);

        let index = binary_search(5, &vec![4, 3, 2, 1], false);
        assert_eq!(index, None);
    }
}
