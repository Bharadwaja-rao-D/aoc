pub fn day2(lines: Vec<String>) {
    let score = lines.iter().fold(0, |acc, play| {
        let opp = play.split_whitespace().next().unwrap();
        let ur = play.split_whitespace().nth(1).unwrap();
        if ur == "X" {
            if opp == "A" {
                return acc + 0 + 3;
            } else if opp == "B" {
                return acc + 0 + 1;
            } else {
                return acc + 0 + 2;
            }
        } else if ur == "Y" {
            if opp == "A" {
                return acc + 3 + 1;
            } else if opp == "B" {
                return acc + 3 + 2;
            } else {
                return acc + 3 + 3;
            }
        } else {
            if opp == "A" {
                return acc + 6 + 2;
            } else if opp == "B" {
                return acc + 6 + 3;
            } else {
                return acc + 6 + 1;
            }
        }
    });

    println!("{}", score);
}
