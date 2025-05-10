use rand::prelude::*;
use raylib::prelude::*;

const SPACE_CELLS_X: usize = 200;
const SPACE_CELLS_Y: usize = 200;
const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 600;

type Cell = bool;
type Grid = [[Cell; SPACE_CELLS_Y]; SPACE_CELLS_X];

#[derive(Debug)]
struct Space {
    cells: Grid,
}

struct CellIter<'a> {
    space: &'a Space,
    x: usize,
    y: usize,
}

impl<'a> Iterator for CellIter<'a> {
    type Item = (usize, usize, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.space.cells[self.x][self.y];
        let current = (self.x, self.y, val);

        self.x += 1;

        if self.x >= SPACE_CELLS_X {
            self.y += 1;
            self.x = 0;
        }

        if self.y >= SPACE_CELLS_Y {
            return None;
        }

        Some(current)
    }
}

impl Space {
    fn generate() -> Self {
        let mut rng = rand::rng();
        let mut cells = [[false; SPACE_CELLS_Y]; SPACE_CELLS_X];
        for x in 0..SPACE_CELLS_X {
            for y in 0..SPACE_CELLS_Y {
                if rng.random_bool(0.15) {
                    cells[x][y] = true;
                }
            }
        }
        Space { cells }
    }

    fn iter(&self) -> CellIter {
        CellIter {
            space: self,
            x: 0,
            y: 0,
        }
    }

    fn simulate(&mut self) {
        const DIRECTIONS: [(isize, isize); 8] = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        let mut next_generation = self.cells.clone();
        for (x, y, cell) in self.iter() {
            let mut alive_neighbours = 0;

            for (dx, dy) in DIRECTIONS {
                let rx = x as isize + dx;
                let ry = y as isize + dy;

                if rx >= 0 && rx < SPACE_CELLS_X as isize && ry >= 0 && ry < SPACE_CELLS_Y as isize
                {
                    if self.cells[rx as usize][ry as usize] {
                        alive_neighbours += 1;
                    }
                }
            }

            next_generation[x][y] = match (cell, alive_neighbours) {
                // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                (true, x) if x < 2 => false,
                // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
                (true, 2) | (true, 3) => true,
                // Rule 3: Any live cell with more than three live neighbours dies, as if by overpopulation.
                (true, x) if x > 3 => false,
                // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                (false, 3) => true,

                (current, _) => current,
            };
        }

        self.cells = next_generation;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Game Of Life")
        .build();

    rl.set_target_fps(30);

    let mut space = Space::generate();
    let camera = Camera2D {
        target: Vector2::zero(),
        offset: Vector2::zero(),
        rotation: 0.0,
        zoom: 3.0,
    };

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            space = Space::generate();
        }

        space.simulate();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_mode2D(camera, |mut dd, _| {
            for (x, y, val) in space.iter() {
                dd.draw_pixel(
                    x as i32,
                    y as i32,
                    match val {
                        true => Color::WHITE,
                        false => Color::BLACK,
                    },
                );
            }
        });
    }
}
