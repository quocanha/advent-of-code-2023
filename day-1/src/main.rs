use std::fs;

fn main() {
    let input = fs::read_to_string("./resources/input")
        .expect("Could not read input file.");

    let count = input.lines().count();
    let mut results = vec![0; count];

    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        // Get first digit
        for c in line.chars() {
            if c.is_digit(10) {
                first = Some(c);
                break;
            }
        }

        // Get last digit
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = Some(c);
                break;
            }
        }

        // Push result
        if first.is_some() && last.is_some() {
            results.push(
                [first.unwrap().to_string(), last.unwrap().to_string()]
                .concat().parse::<i32>().unwrap()
            );
        }
    }

    let mut sum: i32 = 0;
    for value in results.iter_mut() {
        sum = sum + *value;
    }
    println!("{}", sum);
}
