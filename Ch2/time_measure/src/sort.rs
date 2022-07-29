pub fn insertion_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i - 1;

        while arr[j] > cur {
            arr[j + 1] = arr[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }

        // we exit the loop from that break statement
        if j == 0 && arr[0] > cur {
            arr[0] = cur;
        } else {
            // `arr[j] > cur` is not satsified, exit from condition judgement
            arr[j + 1] = cur;
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
