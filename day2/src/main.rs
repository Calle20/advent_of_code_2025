fn main() {
    println!("Day 2");

    let mut sum = 0;

    //read from input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let line = input.lines().next().unwrap();

    line.split(',').for_each(|num_str| {
        let ids = num_str.split('-');
        let first_id: i64 = ids
            .clone()
            .next()
            .unwrap()
            .trim()
            .parse()
            .expect("Invalid number");
        let last_id: i64 = ids.last().unwrap().trim().parse().expect("Invalid number");

        for id in first_id..=last_id {
            // if has_sequence(id) {
            if has_any_sequence(&id.to_string()) {
                sum += id;
            }
        }
    });

    println!("Invalid IDs: {}", sum);
}

// p1
fn has_sequence(id: i64) -> bool {
    let digits = (id as f32).log10().floor() as u32 + 1;

    // split magnitude (half of digits)
    let split: i64 = 10_i64.pow(digits / 2);

    // split number
    let left = id / split;
    let right = id % split;
    if left == right {
        return true;
    }
    false
}

// p2
//expected sum: 4174379265
fn has_any_sequence(id: &str) -> bool {
    let len = id.len();

    // Try every possible pattern length
    for pat_len in 1..=len / 2 {
        if len % pat_len != 0 {
            continue;
        }

        let pattern = &id[..pat_len];
        let repeats = len / pat_len;

        if pattern.repeat(repeats) == id {
            return true;
        }
    }

    false
}
