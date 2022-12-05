use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut total = 0;
    for line in &lines {
        let len = line.len();
        let comp1 = &line[..len / 2];
        let comp2 = &line[len / 2..];

        for c in comp2.chars() {
            if comp1.contains(c) {
                total += match c.is_lowercase() {
                    true => (c as u32) % 97 + 1,
                    false => (c as u32) % 65 + 27,
                };
                break;
            }
        }
    }

    println!("Part 1: {total}");

    let mut total = 0;
    let mut i = 0;
    while i < lines.len() {
        let l1 = lines.get(i).unwrap();
        let l2 = lines.get(i + 1).unwrap();
        let l3 = lines.get(i + 2).unwrap();

        for c in l3.chars() {
            if l1.contains(c) && l2.contains(c) {
                total += match c.is_lowercase() {
                    true => (c as u32) % 97 + 1,
                    false => (c as u32) % 65 + 27,
                };
                break;
            }
        }

        i += 3;
    }

    println!("Part 2: {total}");
}
