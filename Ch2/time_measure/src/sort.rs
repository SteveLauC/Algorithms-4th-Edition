pub fn insertion_sort<T: PartialOrd + Copy>(s: &mut [T]) {
    for i_idx in 1..s.len() {
        let key: T = s[i_idx];
        let mut j_idx: usize = i_idx - 1;

        while s[j_idx] > key {
            s[j_idx + 1] = s[j_idx];
            if j_idx == 0 {
                break;
            }
            j_idx -= 1;
        }
        if j_idx == 0 && s[0] > key {
            s[0] = key;
        } else {
            s[j_idx + 1] = key;
        }
    }
}

pub fn selection_sort<T: PartialOrd>(s: &mut [T]) {
    for i_idx in 0..s.len() {
        let mut min_idx: usize = i_idx;
        for j_idx in (i_idx + 1)..s.len() {
            if s[j_idx] < s[min_idx] {
                min_idx = j_idx;
            }
        }

        s.swap(i_idx, min_idx);
    }
}
