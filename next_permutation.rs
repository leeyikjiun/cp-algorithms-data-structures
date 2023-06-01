fn next_permutation<T: PartialOrd>(mut v: &mut Vec<T>) -> bool {
    let n = v.len();
    if n <= 1 { return false }

    let mut j = n-2;
    while j >= 0 && v[j] >= v[j+1] {
        if j == 0 { return false }
        j -= 1;
    }

    let mut l = n-1;
    while v[j] >= v[l] {
        l -= 1;
    }
    v.swap(j, l);

    v[j+1..=n-1].reverse();
    true
}
