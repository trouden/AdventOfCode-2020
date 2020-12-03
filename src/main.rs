mod advent;

fn main() {
    println!("Executing Challenge 1!");

    let challenge1 = advent::Challenge1::new();

    println!("part 1: {}", challenge1.part1());
    println!("part 2: {}", challenge1.part2());

    println!("Executing Challenge 2!");

    let challenge2 = advent::Challenge2::new();

    println!("part 1: {}", challenge2.part1());
    println!("part 2: {}", challenge2.part2());

    println!("Executing Challenge 3!");

    let challenge3 = advent::Challenge3::new();

    println!("part 1: {}", challenge3.part1());
    println!("part 2: {}", challenge3.part2());
}
