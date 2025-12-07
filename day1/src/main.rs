fn main() {
    println!("Day 1");

    //read from input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();

    let mut current_position = 50;
    let mut zero_count = 0;

    for line in lines{
        let char = line.chars().next().unwrap();
        let distance = line[1..].parse::<i32>().unwrap();

        println!("Char: {}, Distance: {}", char, distance);

        match char {
            'L' => {
                for _ in 0..distance {
                    current_position -= 1;
                    if current_position < 0 {
                        current_position = 99;
                    }
                    if current_position == 0 {
                        zero_count += 1;
                    }
                }
            },
            'R' => 
            {
                for _ in 0..distance {
                    current_position += 1;
                    if current_position > 99 {
                        current_position = 0;
                    }
                    if current_position == 0 {
                        zero_count += 1;
                    }
                }
            },
            _ => panic!("Invalid character in input"),
        }
        //p1 
        println!("Current Position: {}, Zero Count: {}", current_position, zero_count);
    }
    println!("Current Position: {}, Zero Count: {}", current_position, zero_count);  
}
