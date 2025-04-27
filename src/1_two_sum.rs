use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let diff: i32 = target - *value;

        if let Some(&diff_index) = hash_map.get(&diff) {
            return vec![diff_index, index as i32];
        }

        hash_map.insert(*value, index as i32);
    }

    return vec![];
}

pub fn print_two_number_vec(vec: Vec<i32>) {
    if vec.len() == 2 {
        println!("Vec: ({}, {})", vec[0], vec[1]);
    } else {
        println!("Vector must contain exactly 2 elements");
    }
}

fn main() {
    let input_vec: Vec<i32> = vec![3, 2, 4];
    let target: i32 = 6;

    let result_vec: Vec<i32> = two_sum(input_vec, target);

    print_two_number_vec(result_vec);
}
