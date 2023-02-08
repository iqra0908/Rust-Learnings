use std::collections::HashSet;

fn next_closest_time(time: String) -> String {
    let (hour, minute) = time.split_at(2);

    let mut nums = HashSet::new();
    nums.insert(hour);
    nums.insert(minute);

    let two_digits: Vec<String> = nums.iter().flat_map(|a| nums.iter().map(move |b| format!("{}{}", a, b))).collect();

    let minute_idx = two_digits.iter().position(|x| x == minute).unwrap();
    if minute_idx + 1 < two_digits.len() && two_digits[minute_idx+1] < "60".to_string() {
        return format!("{}:{}", hour, two_digits[minute_idx+1]);
    }

    let hour_idx = two_digits.iter().position(|x| x == hour).unwrap();
    if hour_idx + 1 < two_digits.len() && two_digits[hour_idx+1] < "24".to_string() {
        return format!("{}:{}", two_digits[hour_idx+1], two_digits[0]);
    }

    format!("{}:{}", two_digits[0], two_digits[0])
}

fn main() {
    println!("Hello, world!");
    println!("{}", next_closest_time("19:34".to_string()));
}

