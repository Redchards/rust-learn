use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{ Rect, Point };
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::convert::TryInto;

struct Simulator {}

impl Simulator {
    fn new() -> Simulator {
        Simulator{}
    }

    fn step(&mut self, board: &mut Board) -> Board {
        let mut new_board = board.clone();

        for i in 0..board.cells.len() {
            new_board.cells[i] = self.new_state(board, i);
        }

        new_board
    }

    fn generate_neighborhood(&self, board: &Board, coords: Point) -> Vec<bool> {
        let y = coords.x();
        let x = coords.y();

        vec![
            board.get((x - 1) * 20, (y - 1) * 20).unwrap_or(false),
            board.get((x - 1) * 20, (y) * 20).unwrap_or(false),
            board.get((x - 1) * 20, (y + 1) * 20).unwrap_or(false),
            board.get((x) * 20, (y - 1) * 20).unwrap_or(false),
            board.get((x) * 20, (y + 1) * 20).unwrap_or(false),
            board.get((x + 1) * 20, (y - 1) * 20).unwrap_or(false),
            board.get((x + 1) * 20, (y) * 20).unwrap_or(false),
            board.get((x + 1) * 20, (y + 1) * 20).unwrap_or(false),
        ]
    }

    fn new_state(&self, board: &Board, cell_idx: usize) -> bool {
        let coords = board.cell_idx_to_coords(cell_idx).unwrap_or(Point::new(0, 0));

        let live_neighbors = self.generate_neighborhood(board, coords).iter().filter(|&n| *n).count();

        if live_neighbors < 2 || live_neighbors > 3 {
            false
        }
        else if (live_neighbors == 2 && board.get(coords.y() * 20, coords.x() * 20).unwrap()) || live_neighbors == 3 {
            true
        }
        else {
            false
        }
    }
}

struct Board {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board { 
            cells: vec![false; height * width],
            width: width,
            height: height,
        }
    }

    fn clone(&self) -> Board {
        Board {
            cells: self.cells.clone(),
            width: self.width,
            height: self.height,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<bool> {
        Some(self.cells[self.coords_to_cell_idx(x, y)?])
    }

    fn set(&mut self, x: i32, y: i32, new_val: bool) {
        let idx = self.coords_to_cell_idx(x, y).unwrap();
        self.cells[idx] = new_val;
    }
    
    fn invert(&mut self, x: i32, y: i32) {
        self.set(x, y, !self.get(x, y).unwrap())
    }

    fn coords_to_cell_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x / 20 < self.width as i32 && y / 20 < self.height as i32 {
            (y / 20 + x / 20 * self.height as i32).try_into().ok()
        }
        else {
            None
        }
    }

    fn cell_idx_to_coords(&self, idx: usize) -> Option<Point> {
        if idx < self.cells.len() {
            let x: i32 = ((idx % self.height) as i32).try_into().unwrap();
            let y: i32 = ((idx / self.height) as i32).try_into().unwrap();
            Some(Point::new(x, y))
        } 
        else {
            None
        }
    }
}

fn draw_board(canvas: &mut Canvas<Window>, board: &Board) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    for(pos, e) in board.cells.iter().enumerate() {
        let coords = board.cell_idx_to_coords(pos).unwrap();
        if *e {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(Rect::new(coords.y() * 20 + 1, coords.x() * 20 + 1, 19, 19))?
        }
        else {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(coords.y() * 20 + 1, coords.x() * 20 + 1, 19, 19))?
        }
    }

    canvas.present();

    Ok(())
}

fn draw_grid(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    for i in 0..40 {
        canvas.draw_line(Point::new(i * 20, 0), Point::new(i * 20, 600))?
    }
    for i in 0..30 {
        canvas.draw_line(Point::new(0, i * 20), Point::new(800, i * 20))?
    }

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("game of life", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    draw_grid(&mut canvas)?;

    let mut board = Board::new(40, 30);
    let mut simulator = Simulator::new();

    let mut evt_pump = sdl_context.event_pump()?;

    'mainloop: loop {
        for evt in evt_pump.poll_iter() {
            match evt {
                Event::Quit{ .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::MouseButtonDown { x, y, .. } => {
                    board.invert(x, y);
                    draw_board(&mut canvas, &board)?
                }
                Event::KeyDown {
                    keycode: Option::Some(Keycode::S),
                    ..
                } => {
                    board = simulator.step(&mut board);
                    draw_board(&mut canvas, &board)?
                }
                _ => {}
            }
        }
    }

    Ok(())
}
