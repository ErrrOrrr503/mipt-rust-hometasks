#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }
    if arr.len() == 0 {
        return Vec::<Vec<i32>>::new();
    }
    if arr.len() == k {
        return vec![arr.to_vec()];
    }
    let mut res = Vec::<Vec<i32>>::new();
    for i in 0..=arr.len() - k {
        /*for mut subcomb in combinations(&arr[i+1..], k - 1) {
            subcomb.insert(0, arr[i]);
            res.push(subcomb);
        }*/
        res.append(&mut combinations(&arr[i+1..], k - 1).into_iter().map(|mut subcomb| {subcomb.insert(0, arr[i]); subcomb}).collect());
    }
    return res;
}
