use geng::{prelude::*, Draw2d};

struct State {
    geng: Geng,
    grid: WaveFunctionCollapse,
}

impl State {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            grid: WaveFunctionCollapse::new(10, 10),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            fov: 30.0,
            rotation: 0.0,
        };

        const TILE_SIZE: Vec2<f32> = vec2(1.0, 1.0);
        const LINE_WIDTH: f32 = 0.1;
        const LINE_COLOR: Color<f32> = Color::GRAY;

        let line_width = self.grid.width as f32 * TILE_SIZE.x;
        let line_height = self.grid.height as f32 * TILE_SIZE.y;

        for y in 0..=self.grid.height {
            // Horizontal lines
            let y = y as f32 * TILE_SIZE.y;
            draw_2d::Segment::new(
                Segment::new(vec2(0.0, y), vec2(line_width, y)),
                LINE_WIDTH,
                LINE_COLOR,
            )
            .draw_2d(&self.geng, framebuffer, &camera);
        }
        for x in 0..=self.grid.width {
            // Vertical lines
            let x = x as f32 * TILE_SIZE.x;
            draw_2d::Segment::new(
                Segment::new(vec2(x, 0.0), vec2(x, line_height)),
                LINE_WIDTH,
                LINE_COLOR,
            )
            .draw_2d(&self.geng, framebuffer, &camera);
        }
    }
}

pub struct WaveFunctionCollapse {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl WaveFunctionCollapse {
    pub fn generate_next(&mut self) {
        let mut rng = global_rng(); // TODO: use seeded rng
        let cell = self.cells.iter_mut().filter(|cell| cell.cell_type.is_none()).choose(&mut rng);
        match cell {
            None => {
                // There are no cells left to generate
                return;
            }
            Some(cell) => {
                let options = vec![CellType::Vertical, CellType::Horizontal, CellType::Empty];
                cell.cell_type = Some(options.choose(&mut rng).unwrap().clone());
            }
        }
    }
}

impl WaveFunctionCollapse {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Cell { cell_type: None }; width * height],
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let index = self.position_to_index(x, y);
        self.cells.get(index)
    }

    /// Takes index of current cell and returns its neighbours.
    /// TODO: avoid vector usage
    pub fn get_neighbours(&self, index: usize) -> Vec<usize> {
        let (x, y) = self.index_to_position(index);
        let x = x as isize;
        let y = y as isize;

        let neighbours = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .into_iter()
            .filter(|(x, y)| *x > 0 && *x < self.width as isize && *y >= 0 && *y < self.height as isize)
            .map(|(x, y)| self.position_to_index(dbg!(x) as usize, dbg!(dbg!(y) as usize)))
            .collect();

        neighbours
    }

    pub fn index_to_position(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn position_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
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

    let mut state = State::new(&geng);
    state.grid.generate_next();

    geng::run(&geng, state)
}
