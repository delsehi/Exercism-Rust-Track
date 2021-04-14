#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Character {
    Pipe, Space, Underline, None
}

#[derive(Debug, PartialEq)]
pub struct Number {
    r0: [Character; 3],
    r1: [Character; 3],
    r2: [Character; 3],
}

impl Number {
    fn to_string(&self) -> String {
        use Character::*;
        match self {
            Number {r0: [Space, Underline, Space], r1: [Pipe, Space, Pipe], r2: [Pipe, Underline, Pipe]} => return String::from("0"),
            Number {r0: [Space, Space, Space], r1: [Space, Space, Pipe], r2: [Space, Space, Pipe]} => return String::from("1"),
            Number {r0: [Space, Underline, Space], r1: [Space, Underline, Pipe], r2: [Pipe, Underline, Space]} => return String::from("2"),
            Number {r0: [Space, Underline, Space], r1: [Space, Underline, Pipe], r2: [Space, Underline, Pipe]} => return String::from("3"),
            Number {r0: [Space, Space, Space], r1: [Pipe, Underline, Pipe], r2: [Space, Space, Pipe]} => return String::from("4"),
            Number {r0: [Space, Underline, Space], r1: [Pipe, Underline, Space], r2: [Space, Underline, Pipe]} => return String::from("5"),
            Number {r0: [Space, Underline, Space], r1: [Pipe, Underline, Space], r2: [Pipe, Underline, Pipe]} => return String::from("6"),
            Number {r0: [Space, Underline, Space], r1: [Space, Space, Pipe], r2: [Space, Space, Pipe]} => return String::from("7"),
            Number {r0: [Space, Underline, Space], r1: [Pipe, Underline, Pipe], r2: [Pipe, Underline, Pipe]} => return String::from("8"),
            Number {r0: [Space, Underline, Space], r1: [Pipe, Underline, Pipe], r2: [Space, Underline, Pipe]} => return String::from("9"),
            _ => return String::from('?'),
        }
    }
}

fn parse_row(input: &str) -> Result<[Character; 3], Error> {
    use Character::*;
    let mut result = [None; 3];
    if input.len() != 3 {
        return Err(Error::InvalidColumnCount(input.len()))
    }
    for (i, character) in input.chars().enumerate() {
        match character {
            '|' => result[i] = Pipe,
            '_' => result[i] = Underline,
            x => { if x.is_whitespace() {
                result[i] = Space
            } else {
                result[i] = None
            }
            }
        }
    }
    Ok(result)
}

fn convert_number(input: &str) -> Result<String, Error> {
    use Character::*;
    let mut result = Number {
        r0: [None; 3], r1: [None; 3], r2: [None; 3] 
    };
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() != 4 {return Err(Error::InvalidRowCount(lines.len()))}
    for (i, line) in lines.iter().enumerate() {
        let string = parse_row(line);
        match string {
            Result::Ok(string) => {
                match i {
                    0 => result.r0 = string,
                    1 => result.r1 = string,
                    2 => result.r2 = string,
                    _ => (),
                }
            }
            Err(err) => return Result::Err(err)
        }
    }
    Ok(result.to_string())
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut result = String::new();
    let mut lines: Vec<&str> = input.lines().collect();
    // For the length of one line, take 3 characters at a time, push the character on each row
    for c in (0..lines.get(0).unwrap().len()).step_by(3) {
        let mut result = String::new();
        result.push_str();
    }
    convert_number(input)
}
