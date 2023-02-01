pub fn encode(init: &str) -> Vec<Vec<char>> {
    let mut encoded = vec![];
    for line in init.lines() {
        let chars: Vec<char> = line.chars().collect();
        encoded.push(chars.chunks(4).fold(vec![], |mut arr, ele| {
            arr.push(ele[1]);
            arr
        }));
    }

    let stack_size = encoded.last().unwrap().len();
    let mut stack: Vec<Vec<char>> = vec![vec![]; stack_size];

    encoded.iter().for_each(|row| {
        let mut count = 0;
        for char in row {
            if *char == ' ' {
                count += 1;
            } else {
                stack[count].push(*char);
                count += 1;
            }
        }
    });

    stack.iter_mut().for_each(|row| row.reverse());
    return stack;
}

#[derive(Debug)]
pub struct Movement {
    pub from: usize,
    pub to: usize,
    pub count: usize,
}

impl TryFrom<&str> for Movement {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let moves: Vec<&str> = value.split_whitespace().collect();

        return Ok(Movement {
            from: moves[3].parse::<usize>().unwrap() - 1,
            to: moves[5].parse::<usize>().unwrap() - 1,
            count: moves[1].parse().unwrap(),
        });
    }
}

pub fn day5(lines: String) {
    let (init, movements) = lines.split_once("\n\n").unwrap();
    let mut init = encode(init);
    let movements: Vec<Movement> = movements
        .lines()
        .map(|movement| movement.try_into().unwrap())
        .collect();

    for movement in movements.iter() {
        let mut temp = vec![];
        for _ in 0..movement.count {
            let ele = init[movement.from].pop().unwrap();
            temp.push(ele);
        }

        temp.reverse();
        init[movement.to].append(&mut temp);
    }

    let ans = init.iter().fold(String::new(), |tops, ele| {
        let last = *ele.last().unwrap();
        tops + &last.to_string()
    });

    println!("{}", ans);
}
