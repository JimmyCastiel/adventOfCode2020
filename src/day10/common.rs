use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

pub(crate) const FINAL: &str = "./inputs/exo10_final_input.txt";

#[cfg(test)]
pub(crate) const TEST: &str = "./inputs/exo10_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day10ErrorKind {
    Test,
    Parse,
}

impl Display for Day10ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg;
        match self {
            Self::Test => msg = "test",
            Self::Parse => msg = "parse",
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct Day10Error {
    kind: Day10ErrorKind,
    message: String
}

impl Display for Day10Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

pub struct Serie {
    numbers: Vec<u64>
}

impl Serie {
    pub fn get_differences(&mut self) -> HashMap<u64, u64> {
        let mut idx: usize = 0;
        let mut res = HashMap::new();
        self.numbers.sort();
        res.insert(self.numbers[idx], res.get(&self.numbers[idx]).unwrap_or(&0) + 1);
        loop {
            if (idx + 1) >= self.numbers.len(){
                break
            }
            let diff = self.numbers[idx + 1] - self.numbers[idx];
            res.insert(diff, res.get(&diff).unwrap_or(&0) + 1);
            idx += 1;
        }
        res.insert(3, res.get(&3).unwrap_or(&0) + 1);
        res
    }
}

#[cfg(test)]
pub(crate) fn parse_test() -> Result<Serie, Day10Error> { parse_file(TEST) }

pub fn parse_final() -> Result<Serie, Day10Error> { parse_file(FINAL) }

fn parse_file(path: &str) -> Result<Serie, Day10Error> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();
    let mut numbers = Vec::new();

    for line in file {
        match line {
            Ok(line) => {
                let number = line.as_str().parse::<u64>().unwrap_or(0);
                numbers.push(number);
            },
            _ => {}
        }
    }

    Ok(Serie { numbers })
}
