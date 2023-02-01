pub struct Range {
    pub start: i32,
    pub end: i32,
}

impl TryFrom<&str> for Range {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (start, end) = value.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        return Ok(Self { start, end });
    }
}

pub fn day4(lines: Vec<String>){
    let count = lines.iter().fold(0, |count, ele| {
        let (first, second) = ele.split_once(',').unwrap();
        let first: Range = first.try_into().unwrap();
        let second: Range = second.try_into().unwrap();

        if (first.end >= second.start && first.start <= second.end)|| (second.end >= first.start && second.start <= first.end)
        {
            count + 1
        } else {
            count
        }
    });

    println!("Count: {}", count);
}
