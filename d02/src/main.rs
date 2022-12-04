use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in lines {
        let score_1 = match line {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => 0,
        };

        total_1 += score_1;

        let score_2 = match line {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => 0,
        };

        total_2 += score_2;
    }

    println!("Part 1: {total_1}");
    println!("Part 2: {total_2}");
}
