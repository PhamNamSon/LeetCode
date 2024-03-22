pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        println!("{}", num.to_string());
        if map.contains_key(&complement) {
            return vec![*map.get(&complement).unwrap(), i as i32];
        }
        map.insert(num, i as i32);
    }
    vec![]
}

fn main() {
}
