fn max_dist_to_closest(seats: &[i32]) -> i32 {
    let ts: Vec<usize> = seats
        .iter()
        .enumerate()
        .filter(|(_, &seat)| seat == 1)
        .map(|(i, _)| i)
        .collect();

    let mut max_dist = 0;
    for i in 1..ts.len() {
        let dist = (ts[i] - ts[i - 1]) as i32 / 2;
        max_dist = max_dist.max(dist);
    }

    if ts[0] != 0 {
        max_dist = max_dist.max(ts[0] as i32);
    }
    if *ts.last().unwrap() != seats.len() - 1 {
        max_dist = max_dist.max((seats.len() - 1 - ts.last().unwrap()) as i32);
    }

    max_dist
}

fn main() {
    let seats = vec![1, 0, 0, 0];
    println!("{}", max_dist_to_closest(&seats));
}
