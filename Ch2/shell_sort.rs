fn shell_sort<T: PartialOrd + Copy>(s: &mut [T]) {
    fn insertion_sort<T: PartialOrd + Copy>(s: &mut [T], start: usize, step: usize) {
        for i_idx in ((start + step)..s.len()).step_by(step) {
            let key: T = s[i_idx];
            let mut j_idx: usize = i_idx - step;

            while j_idx >= start && s[j_idx] > key {
                s[j_idx + step] = s[j_idx];
                if start < step && j_idx == start {
                    break;
                }
                j_idx -= step;
            }
            if start < step && j_idx == start && s[start] > key {
                s[start] = key;
            } else {
                s[j_idx + step] = key;
            }
        }
    }

    let mut h: usize = s.len() / 2;
    while h > 0 {
        for i in 0..h {
            insertion_sort(s, i, h);
        }

        h /= 2;
    }
}
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        shell_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec!["a"];
        shell_sort(&mut res);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        shell_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut res = vec!["d", "a", "c", "b"];
        shell_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        shell_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        shell_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn reverse_order() {
        let mut arr = (0..100).rev().collect::<Vec<i32>>();
        shell_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
