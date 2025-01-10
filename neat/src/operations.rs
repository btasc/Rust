use std::collections::HashSet;

pub fn remove_duplicates(vec: Vec<i32>) -> Vec<i32>{
    let mut seen = HashSet::new();
    vec.into_iter()
    .filter(|x| seen.insert(*x)).collect()
}