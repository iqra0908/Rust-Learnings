fn main() {
    let result = reverse(-123);
    println!("{0}", result);
}

fn reverse(x: i32) -> i32 {
    if x >= 2147483647 || x < -2147483648 {
        return 0;
    }
    println!("{}", x);

    let y = x.to_string();
    let mut signed = false;
    let mut z = String::new();
    if y.chars().nth(0).unwrap() == '-' {
        z = y[1..].to_string();
        signed = true;
    }

    for i in (0..y.len()).rev() {
        z.push(y.chars().nth(i).unwrap());
    }

    if signed {
        return -z.parse::<i32>().unwrap();

    } else {
        return z.parse::<i32>().unwrap();
    }
    
}
