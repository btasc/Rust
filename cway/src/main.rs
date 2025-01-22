use std::thread;
use std::time::{Duration, Instant};

const ON: &'static str = "[#]";
const OFF: &'static str = "[ ]";

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

const PIXELS: usize = WIDTH * HEIGHT;

struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn to_index(&self) -> usize {
        return self.y * WIDTH + self.x;
    }

    fn to_coordinate(index: usize) -> Coordinate {
        Coordinate {
            x: index % WIDTH,
            y: index / WIDTH,
        }
    }

    fn adjacent_nowrap(&self, direction: usize) -> Coordinate {
        /*
        [0][1][2]
        [3][X][4]
        [5][6][7]
        */

        let mut change: [isize; 2] = [0; 2];

        match direction {
            0 =>  change = [-1, -1],// --
            1 =>  change[1] = -1, // 0-
            2 =>  change = [1, -1], // +-
            3 =>  change[0] = -1, // -0
            4 =>  change[0] = 1, // +0
            5 =>  change = [-1, 1], // -+
            6 =>  change[1] = 1, // 0+
            7 =>  change = [1, 1], // ++
            _ => panic!("Invalid direction"),
        }

        Coordinate {
            x: (self.x as isize + change[0]) as usize,
            y: (self.y as isize + change[1]) as usize,
        }
    }

    fn adjacent_wrap(&self, direction: usize) -> Coordinate {
        let coordinate = self.adjacent_nowrap(direction);

        Coordinate {
            x: coordinate.x % (WIDTH),
            y: coordinate.y % (HEIGHT),
        }
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render_board(board: &[bool; PIXELS]) {
    clear_console();
    let mut print_board: String = String::new();

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            if board[h * WIDTH + w] {
                print_board.push_str(ON);
            } else {
                print_board.push_str(OFF);
            }
        }

        print_board.push_str("\n");
    }

    print!("{}", print_board);
}

fn board_wrap(index: usize) -> usize {
    return index % PIXELS;
}

fn get_adjacents(index: usize) -> [usize; 8] {
    let mut adjacents = [0; 8];
    let index_coordinate = Coordinate::to_coordinate(index);

    for i in 0..8 {
        adjacents[i] = index_coordinate.adjacent_wrap(i).to_index();
    }

    adjacents
}

fn main() {
    let start = Instant::now();
    let mut board: [bool; PIXELS] = [false; PIXELS];

    // Glider Coordinates
    let c1 = Coordinate { x: 1, y: 1 };
    let c2 = Coordinate { x: 2, y: 2 };
    let c3 = Coordinate { x: 2, y: 3 };
    let c4 = Coordinate { x: 1, y: 3 };
    let c5 = Coordinate { x: 0, y: 3 };
    board[c1.to_index()] = true;
    board[c2.to_index()] = true;
    board[c3.to_index()] = true;
    board[c4.to_index()] = true;
    board[c5.to_index()] = true;

    let pause = Duration::from_millis(10);

    

    for i in 0..10000 {
        //thread::sleep(pause);
        //render_board(&board);
        
        let mut death_marked: Vec<usize> = Vec::new();
        let mut life_marked: Vec<usize> = Vec::new();

        for i in 0..PIXELS {
            let adjacent_indexes: [usize; 8] = get_adjacents(i);

            let mut adjacent_values: [bool; 8] = [false; 8];

            for j in 0..8 {
                adjacent_values[j] = board[adjacent_indexes[j]];
            }

            let mut alive_count: usize = 0;

            for j in 0..8 {
                if adjacent_values[j] {
                    alive_count += 1;
                }
            }

            if board[i] {
                if alive_count < 2 || alive_count > 3 {
                    death_marked.push(i);
                }
            } else {
                if alive_count == 3 {
                    life_marked.push(i);
                }
            }
        }

        for index in death_marked {
            board[index] = false;
        }
        for index in life_marked {
            board[index] = true;
        }
    }

    render_board(&board);
    println!("Time elapsed: {:?}", start.elapsed());
}