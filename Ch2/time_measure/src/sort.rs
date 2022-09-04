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

pub fn shell_sort<T: PartialOrd + Copy>(s: &mut [T]) {
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
            if start < step && j_idx == start && s[0] > key {
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

pub fn top_down_merge_sort<T: Copy + Ord>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    let mid: usize = a.len() / 2;

    top_down_merge_sort(&mut a[..mid]);
    top_down_merge_sort(&mut a[mid..]);
    merge(a, mid);
}

pub fn bottom_up_merge_sort<T: Copy + Ord>(a: &mut [T]) {
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
            // merge a[start_index..start_index+sub_array_size] and 
            // a[start_index+sub_array_size, end_idx]
            //
            // NOTE: mid is a relative start_index number starting from `start_index`
            merge(&mut a[start_index..end_idx], sub_array_size);
            // update `start_index` to merge the next sub-arrays
            start_index = end_idx;
        }
        sub_array_size *= 2;
    }
}
