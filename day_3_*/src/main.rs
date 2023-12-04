use std::fs;

#[derive(Debug, PartialEq)]
struct Coord(u32, u32);

impl Coord {
    fn new(line: u32, index: u32) -> Self {
        Self(line, index)
    }
}

struct Number {
    value: u32,
    pos: Vec<Coord>,
}

impl Number {
    fn new(value: u32, pos: Vec<Coord>) -> Self {
        Self { value, pos }
    }
}

struct Symbol {
    touching: [Coord; 9],
}

impl Symbol {
    fn new(touching: [Coord; 9]) -> Self {
        Self { touching }
    }
}

fn create_number(value: String, line_index: u32, index: u32) -> Number {
    let mut vec = Vec::new();
    for (i, _) in value.chars().collect::<Vec<_>>().iter().enumerate() {
        vec.push(Coord::new(line_index, index - i as u32 - 1))
    }
    Number::new(value.parse::<u32>().unwrap(), vec)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut total = 0;

    for (line_index, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut new_number: Option<String> = None;

        for (index, char) in chars.iter().enumerate() {
            let len = chars.len();
            match char {
                c if *c == '.' => match new_number {
                    None => (),
                    Some(value) => {
                        let number = create_number(value, line_index as u32, index as u32);
                        numbers.push(number);
                        new_number = None;
                    }
                },
                c if c.is_numeric() => {
                    match new_number {
                        None => new_number = Some(c.to_string()),
                        Some(value) => new_number = Some(value + &c.to_string()),
                    }
                    if index == len - 1 {
                        match new_number {
                            None => (),
                            Some(value) => {
                                let number = create_number(value, line_index as u32, index as u32);
                                numbers.push(number);
                                new_number = None;
                            }
                        }
                    }
                }
                _ => {
                    match new_number {
                        None => (),
                        Some(value) => {
                            let number = create_number(value, line_index as u32, index as u32);
                            numbers.push(number);
                            new_number = None;
                        }
                    }
                    let l: u32 = line_index as u32;
                    let i: u32 = index as u32;
                    let touching = [
                        Coord::new(l - 1, i - 1),
                        Coord::new(l - 1, i),
                        Coord::new(l - 1, i + 1),
                        Coord::new(l, i - 1),
                        Coord::new(l, i),
                        Coord::new(l, i + 1),
                        Coord::new(l + 1, i - 1),
                        Coord::new(l + 1, i),
                        Coord::new(l + 1, i + 1),
                    ];
                    symbols.push(Symbol::new(touching))
                }
            }
        }
    }

    'numbers: for number in &numbers {
        for symbol in &symbols {
            for p in &number.pos {
                for t in &symbol.touching {
                    if p.0 == t.0 && p.1 == t.1 {
                        total += number.value;
                        continue 'numbers;
                    }
                }
            }
        }
    }

    println!("total: {}", total)
}
