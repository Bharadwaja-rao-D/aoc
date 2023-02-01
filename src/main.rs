use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub struct FileInfo{
    pub file: String,
    pub size: usize,
}

pub enum Input{
    Command(String),
    Info(FileInfo),
}

pub struct Node{
    pub node_info: FileInfo,
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
}

impl Node{
    pub fn new(node_info: FileInfo) -> Self{
        return Self { node_info , left: Box::new(None), right: Box::new(None) }
    }
}

fn main() {
    let lines = fs::read_to_string("./input").unwrap();
}

fn _input(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let input = reader.lines().map(|ele| ele.unwrap()).collect();

    return input;
}
