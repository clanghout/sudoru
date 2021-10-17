use std::fmt::{Display, Formatter};
use std::io::Stdin;
use std::path::Path;

fn main() {
    let mut board = Board::from_file("./sudoku1.txt");
    println!("Welcome to the sudoru game! This is your board:");
    println!("{}", board);

    // user input
    println!("Which tile do you want to read? Type as rownr -space- colnr");
    println!("To write; Type as rownr -space- colnr -space- new val");
    let mut buffer = String::new();
    let mut stdin = std::io::stdin(); // We get `Stdin` here.

    loop {
        let action = read_stdin_action(&mut buffer, &mut stdin);
        match action {
            Action::Read(coord) => println!("{:?} contains value {}", coord, board.get(&coord)),
            Action::Write(ref coord, val) => {
                let res = board.write(coord, val);
                match res {
                    Ok(_) => println!("{}", board),
                    Err(_) => println!("Could not write to this tile, part of the problem def :D"),
                }
            }
        }
    }
}

fn read_stdin_action(buffer: &mut String, stdin: &mut Stdin) -> Action {
    let action = match stdin.read_line(buffer) {
        Ok(_n) => parse_input_coords(buffer),
        Err(error) => panic!("error: {}", error),
    };
    buffer.clear();
    action
}

fn parse_input_coords(input: &str) -> Action {
    let ass = input.split(' ').map(str::trim).collect::<Vec<_>>();
    match ass.as_slice() {
        [r, c] => read_coord(r, c),
        [r, c, v] => write_coord(r, c, v),
        _ => {
            panic!("cna you not  read>???")
        }
    }
}

fn write_coord(row: &str, col: &str, val: &str) -> Action {
    let row = row.parse::<u8>();
    let col = col.parse::<u8>();
    let val = val.parse::<u8>().unwrap();
    Action::Write(Coord(row.unwrap(), col.unwrap()), val)
}

fn read_coord(r: &str, c: &str) -> Action {
    let row = r.parse::<u8>();
    let col = c.parse::<u8>();
    Action::Read(Coord(row.unwrap(), col.unwrap()))
}

// impl Debug shows what this does
// #[derive(Debug)]
struct Board {
    tiles: Vec<Tile>,
}

#[derive(Debug)]
struct Coord(u8, u8);

enum Action {
    Write(Coord, u8),
    Read(Coord),
}

enum Tile {
    Prefilled(u8),
    Filled(u8),
    Empty,
}

struct CannotWritePrefilledTileError;

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prefilled(i) => f.write_str(&format!("{}", i)),
            Self::Filled(i) => f.write_str(&format!("{}", i)),
            Self::Empty => f.write_str("."),
        }
    }
}

impl Board {
    fn new() -> Board {
        let grid_size = 9;
        let capacity = grid_size * grid_size;
        let mut vec = Vec::with_capacity(capacity);
        for _ in 0u8..capacity as u8 {
            vec.push(Tile::Empty);
        }
        Board { tiles: vec }
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Board {
        fn char_to_tile(char: char) -> Tile {
            if char == '*' {
                Tile::Empty
            } else {
                Tile::Prefilled(char.to_digit(10).unwrap() as u8)
            }
        }

        let path = path.as_ref();
        let result = std::fs::read_to_string(path).expect("Could not read file");

        let result: Vec<Tile> = result
            .lines()
            .flat_map(|line| line.chars().map(char_to_tile))
            .collect();

        Board { tiles: result }
    }

    fn index(row: u8, col: u8) -> u8 {
        (row - 1) * 9 + (col - 1)
    }

    fn get(&self, c: &Coord) -> &Tile {
        let row = c.0;
        let col = c.1;
        &self.tiles[Self::index(row, col) as usize]
    }

    fn write(&mut self, c: &Coord, val: u8) -> Result<(), CannotWritePrefilledTileError> {
        let row = c.0;
        let col = c.1;
        let index = Self::index(row, col) as usize;

        if let Tile::Prefilled(_) = self.tiles[index] {
            return Err(CannotWritePrefilledTileError);
        }
        self.tiles[index] = Tile::Filled(val);
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("  123 456 789\n")?;
        for (i, row) in self.tiles.chunks(9).enumerate() {
            if i % 3 == 0 {
                f.write_str("  --- --- ---\n")?;
            }
            f.write_str(&format!("{}|", i + 1))?;
            for (i, n) in row.iter().enumerate() {
                f.write_str(format!("{}", n).as_str())?;
                if i % 3 == 2 {
                    f.write_str("|")?;
                }
            }
            f.write_str("\n")?;
        }
        f.write_str(" --- --- ---")
    }
}
