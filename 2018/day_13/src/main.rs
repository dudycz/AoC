use std::io::{BufRead, BufReader};
use std::fs::File;
use std::vec::Vec;

const TEST_MAP: &str =
"
/->-\\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/
";

enum NextTurn {
    Left,
    Straight,
    Right,
}

enum Direction {
    N,
    E,
    S,
    W,
}

struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: NextTurn,
}

impl Cart {
    fn step(&mut self, railroads: &Vec<Vec<char>>) -> bool {
        // If on curve or intersection than change direction
        // Move according to direction
        // Check for collision

        match railroads[self.x][self.y] {
        }

        match self.direction {
            Direction::N => self.y -= 1,
            Direction::S => self.y += 1,
            Direction::W => self.x -= 1,
            Direction::E => self.x += 1,
        }
        false
    }
}

fn main() {
    let file = File::open("src/input").expect("cannot open file");
    let file = BufReader::new(file);

    let mut railroads = vec![vec![' '; 150]; 150];
    let mut carts: Vec<Cart> = Vec::new();

    for (i, line) in file.lines().enumerate() {
        for (j, tile) in line.unwrap().chars().enumerate() {

            match tile {
                '<' => carts.push(Cart {
                    x: j,
                    y: i,
                    direction: Direction::W,
                    next_turn: NextTurn::Left,
                }),
                '>' => carts.push(Cart {
                    x: j,
                    y: i,
                    direction: Direction::E,
                    next_turn: NextTurn::Left,
                }),
                '^' => carts.push(Cart {
                    x: j,
                    y: i,
                    direction: Direction::N,
                    next_turn: NextTurn::Left,
                }),
                'v' => carts.push(Cart {
                    x: j,
                    y: i,
                    direction: Direction::S,
                    next_turn: NextTurn::Left,
                }),
                 _ => railroads[i][j] = tile,
            }
        }
    }

    let mut done = false;
    while !done {
        for cart in &mut carts {
            if !cart.step(&railroads) {
                done = true;
            }
        }
    }
}
