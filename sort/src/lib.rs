/// from: Github/TheAlgorithms
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

pub fn bubble_sort_iter<T: Ord + Clone>(arr: &mut [T]) {
    let mut n = arr.len();
    let mut v = Vec::new(); // because windows need immmutable parent
    arr.clone_into(&mut v);

    while n > 0 {
        let mut last_swapped = 0;

        // Using iterator to find the last swap position
        for (i, window) in v.windows(2).enumerate().take(n - 1) {
            if window[0] > window[1] {
                arr.swap(i, i + 1);
                last_swapped = i + 1;
            }
        }

        n = last_swapped;
    }
}

fn partition<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut idx = lo; // where we swapped last time

    for i in lo..hi {
        if arr[i] <= pivot {
            // swap because only smaller values are on left side
            arr.swap(idx, i);

            idx += 1;
        }
    }

    // now swap our pivot as well
    // because now greater values are at and beyond
    // out last swapped place
    // idx += 1;
    arr.swap(hi, idx);

    idx
}

fn sort<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(arr, lo, hi);

    if pivot_idx > 0 {
        sort(arr, lo, pivot_idx - 1);
    }

    sort(arr, pivot_idx + 1, hi);
}

pub fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    sort(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
    where
        // T: cmp::PartialOrd,
        // If HashSet is used
        T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
    {
        use std::collections::HashSet;

        match a.len() == b.len() {
            true => {
                // This is O(n^2) but performs better on smaller data sizes
                //b.iter().all(|item| a.contains(item))

                // This is O(n), performs well on larger data sizes
                let set_a: HashSet<&T> = a.iter().collect();
                let set_b: HashSet<&T> = b.iter().collect();
                set_a == set_b
            }
            false => false,
        }
    }

    pub fn is_sorted<T>(arr: &[T]) -> bool
    where
        T: cmp::PartialOrd,
    {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn descending_quicksort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        quick_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }

    #[test]
    fn descending_iter() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        bubble_sort_iter(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending_iter() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        bubble_sort_iter(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
    #[test]
    fn empty_iter() {
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
        bubble_sort_iter(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
