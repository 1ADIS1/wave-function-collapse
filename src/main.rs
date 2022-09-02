use geng::prelude::*;

struct State {
    geng: Geng,
}

impl State {
    pub fn new(geng: &Geng) -> Self {
        Self { geng: geng.clone() }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }
}


pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    length: usize,
}

impl Grid {
    pub fn new(width: usize, length: usize) -> Self {
        Self {
            cells: vec![Cell { cell_type: None }; width * length],
            width,
            length,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    /// Option here means whether the cell has been generated
    cell_type: Option<CellType>,
}

#[derive(Clone, Copy, Debug)]
pub enum CellType {
    Vertical,
    Horizontal,
    Empty,
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Wave Function Collapse");

    let state = State::new(&geng);

    geng::run(&geng, state)
}