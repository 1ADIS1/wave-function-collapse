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
    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown {
            key: geng::Key::Space,
        } = event
        {
            self.grid.generate_next();
        }
    }

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

        for (cell, x, y) in self.grid.cells() {
            let cell_type = match cell.cell_type {
                None => continue,
                Some(cell_type) => cell_type,
            };
            let pos = vec2(x, y).map(|x| x as f32) * TILE_SIZE;
            let aabb = AABB::point(pos).extend_positive(TILE_SIZE);
            draw_2d::Quad::new(aabb, Color::GRAY).draw_2d(&self.geng, framebuffer, &camera);
            match cell_type {
                CellType::Vertical => draw_2d::Quad::new(
                    aabb.extend_symmetric(vec2(-TILE_SIZE.x / 3.0, 0.0)),
                    Color::BLUE,
                )
                .draw_2d(&self.geng, framebuffer, &camera),
                CellType::Horizontal => draw_2d::Quad::new(
                    aabb.extend_symmetric(vec2(0.0, -TILE_SIZE.y / 3.0)),
                    Color::BLUE,
                )
                .draw_2d(&self.geng, framebuffer, &camera),
                CellType::Empty => {}
            }
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
        let cell = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.cell_type.is_none())
            .choose(&mut rng);
        match cell {
            None => {
                println!("There are no more cells to generate");
            }
            Some((index, cell)) => {
                let mut possible_options = HashMap::new();
                possible_options.insert("Vertical", CellType::Vertical);
                possible_options.insert("Horizontal", CellType::Horizontal);
                possible_options.insert("Empty", CellType::Empty);
                let (cell_x, cell_y) = self.index_to_position(index);

                for neighbour_index in self.get_neighbours(index) {
                    let neighbour = self.cells.get(neighbour_index).unwrap();
                    let (neighbour_x, neighbour_y) = self.index_to_position(neighbour_index);

                    match neighbour.cell_type {
                        Some(cell_type) => {
                            match cell_type {
                                CellType::Vertical => {
                                    possible_options.remove("Horizontal");

                                    // Check if vertical tile is only on the top or bottom
                                    if neighbour_x as isize + 1 == cell_x as isize
                                        || neighbour_x as isize - 1 == cell_x as isize
                                    {
                                        possible_options.remove("Vertical");
                                    }
                                }
                                CellType::Horizontal => {
                                    possible_options.remove("Vertical");

                                    // // Check if horizontal is only on the left or right
                                    if neighbour_y as isize + 1 == cell_y as isize
                                        || neighbour_y as isize - 1 == cell_y as isize
                                    {
                                        possible_options.remove("Horizontal");
                                    }
                                }
                                CellType::Empty => (),
                            }

                            println!("Possible options: {:?}", possible_options);
                        }
                        None => (),
                    }
                }

                let cell_type = possible_options.into_iter().choose(&mut rng).unwrap();

                let cell = self.cells.get_mut(index).unwrap();
                cell.cell_type = Some(cell_type.1);
                println!("Type of self to put: {:?}", cell.cell_type.unwrap());
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

    pub fn cells(&self) -> impl Iterator<Item = (&Cell, usize, usize)> {
        self.cells.iter().enumerate().map(|(index, cell)| {
            let (x, y) = self.index_to_position(index);
            (cell, x, y)
        })
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

        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .into_iter()
            .filter(|(x, y)| {
                *x >= 0 && *x < self.width as isize && *y >= 0 && *y < self.height as isize
            })
            .map(|(x, y)| self.position_to_index(dbg!(x) as usize, dbg!(dbg!(y) as usize)))
            .collect()
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
