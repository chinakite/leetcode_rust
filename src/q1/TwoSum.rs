use std::collections::HashMap;

pub fn twosum(arr: &[i32], target: i32) {
    let len = arr.len();
    let mut hmap:HashMap<i32, usize> = HashMap::new();

    for i in 0..len as usize {
        let minus = target - arr[i];
        if hmap.contains_key(&minus) {
            println!("Result : [{:?}, {}]", hmap.get(&minus).unwrap(), i);
            println!("Explanation : {} + {} = {}", minus, arr[i], target);
        }else{
            hmap.insert(arr[i], i);
        }
    }
}
