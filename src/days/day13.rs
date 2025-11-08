use std::{thread, time};

use crate::aoc_result::AoCResult;
use crate::shared::intcode::{InstructionResult, IntcodeProgram};

make_day!(Day13);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    prgm.run();

    let tiles : Vec<_> = prgm.output.chunks(3).map(|chunk| {
        Tile {
            x: chunk[0],
            y: chunk[1],
            tile_id: TileId::from(chunk[2] as u64)
        }
    })
    .collect();

    let block_tiles = tiles.iter()
        .filter(|tile| tile.tile_id == TileId::Block)
        .count();

    AoCResult::Num(block_tiles as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);

    // Set memory location 0:
    prgm.write(0, 2);

    let mut prgm_state = prgm.run();

    let mut screen = Option::<Screen>::None;
    let mut ball_x = 0i64;
    let mut paddle_x = 0i64;

    while prgm_state == InstructionResult::WAIT_FOR_INPUT {
        let tiles : Vec<_> = prgm.output.chunks(3).map(|chunk| {
            Tile {
                x: chunk[0],
                y: chunk[1],
                tile_id: if chunk[0] >= 0 { TileId::from(chunk[2] as u64) } else { TileId::Score(chunk[2] as u64) }
            }
        })
        .collect();
        prgm.output.clear();

        if let Some(old_screen) = screen.as_mut() {
            old_screen.update(&tiles);
        } else {
            let new_screen = Screen::new(&tiles);
            screen = Some(new_screen);
        }

        // From tiles: get info where paddle and ball are:
        ball_x = tiles.iter().find(|t| t.tile_id == TileId::Ball).map_or(ball_x, |t| t.x);
        paddle_x = tiles.iter().find(|t| t.tile_id == TileId::Paddle).map_or(paddle_x, |t| t.x);

        if ball_x > paddle_x {
            prgm.input.push_back(1);
        } else if ball_x < paddle_x {
            prgm.input.push_back(-1);
        } else {
            prgm.input.push_back(0);
        }

        // screen.as_mut().unwrap().print_to_console();
        // thread::sleep(time::Duration::from_millis(50));

        prgm_state = prgm.run_continue();
    }


    // Last update:
    let tiles : Vec<_> = prgm.output.chunks(3).map(|chunk| {
            Tile {
                x: chunk[0],
                y: chunk[1],
                tile_id: if chunk[0] >= 0 { TileId::from(chunk[2] as u64) } else { TileId::Score(chunk[2] as u64) }
            }
        })
        .collect();
    
    screen.as_mut().unwrap().update(&tiles);
    AoCResult::Num(screen.unwrap().score)
}

#[derive(PartialEq)]
enum TileId {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(u64)
}

impl From<u64> for TileId {
    fn from(res: u64) -> Self {
        match res {
            1 => TileId::Wall,
            2 => TileId::Block,
            3 => TileId::Paddle,
            4 => TileId::Ball,
            _ => TileId::Empty
        }
    }
}
struct Tile {
    pub x: i64,
    pub y: i64,
    pub tile_id: TileId
}

struct Screen {
    width: i64,
    score: u64,
    drawing_area: Vec<char>
}

impl Screen {
    fn new(tiles: &Vec<Tile>) -> Self {
        // Dimensions of the painting area:
        let mut width = 0;
        let mut height = 0;
        let mut score = 0;
        for t in tiles.iter() {
            if t.x == -1 {
                // score
                score = if let TileId::Score(s) = t.tile_id { s } else { 0 };
            } else {
                if t.x >= width { width = t.x + 1; }
                if t.y >= height { height = t.y + 1; }
            }
        }

        let mut drawing = vec!['.'; (width * height) as usize];

        for t in tiles.iter().filter(|t| t.x >= 0) {
            drawing[(t.y * width + t.x) as usize] = match t.tile_id {
                TileId::Wall => '#',
                TileId::Block => 'X',
                TileId::Ball => 'o',
                TileId::Paddle => '=',
                _ => ' '
            }
        }

        Screen {
            width: width,
            score: score,
            drawing_area: drawing
        }
    }

    fn update(&mut self, tiles: &Vec<Tile>) {
        for t in tiles.iter(){
            if let TileId::Score(s) = t.tile_id {
                self.score = s;
            } else {
                self.drawing_area[(t.y * self.width + t.x) as usize] = match t.tile_id {
                    TileId::Wall => '#',
                    TileId::Block => 'X',
                    TileId::Ball => 'o',
                    TileId::Paddle => '=',
                    _ => ' '
                }
            }
        }
    }

    fn print_to_console(&self) {
        println!("Score: {}", self.score);
        for line in self.drawing_area.chunks(self.width as usize) {
            println!("{}", line.iter().collect::<String>());
        }
    }
}