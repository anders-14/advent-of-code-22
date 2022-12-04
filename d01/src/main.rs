use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let inventories = contents.split("\n\n");

    let mut splits = inventories
        .map(|inventory| {
            inventory
                .split("\n")
                .filter(|cal| *cal != "")
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();

    splits.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!("Part 1: {}", splits[0]);

    println!("Part 2: {}", splits[0] + splits[1] + splits[2]);
}
