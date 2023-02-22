fn max_product(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_prod = nums[0];
    let mut min_prod = nums[0];
    let mut result = nums[0];

    for &curr in &nums[1..] {
        let temp_max = curr.max(max_prod * curr).max(min_prod * curr);
        let temp_min = curr.min(max_prod * curr).min(min_prod * curr);
        max_prod = temp_max;
        min_prod = temp_min;
        result = result.max(max_prod);
    }

    result
}


fn main() {
    let nums = vec![2, 3, -2, 4, -1, 5, 0, 7];
    println!("{}", max_product(&nums));
}