use std::thread;
use std::time::Duration;

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
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    fn to_index(&self) -> usize {
        return self.y * WIDTH + self.x;
    }

    fn to_coordinate(index: usize) -> Coordinate {
        Coordinate::new(
            index % WIDTH,
            index / WIDTH,
        )
    }

    fn adjacent_nowrap(&self, direction: usize) -> Coordinate {
        /*
        [1][2][3]
        [4][X][5]
        [6][7][8]
        */

        match direction {
            1 => Coordinate::new(self.x - 1, self.y - 1),
            2 => Coordinate::new(self.x, self.y - 1),
            3 => Coordinate::new(self.x + 1, self.y - 1),
            4 => Coordinate::new(self.x - 1, self.y),
            5 => Coordinate::new(self.x + 1, self.y),
            6 => Coordinate::new(self.x - 1, self.y + 1),
            7 => Coordinate::new(self.x, self.y + 1),
            8 => Coordinate::new(self.x + 1, self.y + 1),
            _ => panic!("Invalid direction"),
        }
    }

    fn adjacent_wrap(&self, direction: usize) -> Coordinate {
        let coordinate = self.adjacent_nowrap(direction);

        Coordinate::new(
            coordinate.x % (WIDTH),
            coordinate.y % (HEIGHT),
        )
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render_board(board: &[bool; PIXELS]) {
    clear_console();

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            if board[h * WIDTH + w] {
                print!("{}", ON);
            } else {
                print!("{}", OFF);
            }
        }

        print!("\n");
    }
}

fn board_wrap(index: usize) -> usize {
    return index % PIXELS;
}

fn main() {
    let mut board: [bool; PIXELS] = [false; PIXELS];
    let pause = Duration::from_millis(100);

    render_board(&board);
}