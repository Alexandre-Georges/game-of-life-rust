use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::console;
use random::Source;

const CELL_SIZE: f64 = 10.0;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
  pub width: u32,
  height: u32,
  cells: Vec<Cell>,
  context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Universe {
  pub fn new(width: u32, height: u32, alive_odds: f64) -> Universe {
    let d = js_sys::Date::now();
    let mut rng = random::default().seed([d as u64, 0]);

    // console::log_1(&JsValue::from_f64(self.height as f64 * CELL_SIZE));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(width * CELL_SIZE as u32);
    canvas.set_height(height * CELL_SIZE as u32);

    let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    let cells = (0..width * height)
      .map(|_| {
        let random: f64 = rng.read::<f64>();
        if random < alive_odds {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells,
      context,
    }
  }

  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (column + delta_col) % self.width;
        let idx = self.get_index(neighbor_row, neighbor_col);
        count += self.cells[idx] as u8;
      }
    }
    count
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        let next_cell = match (cell, live_neighbors) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (otherwise, _) => otherwise,
        };

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn render(&self) {
    self.context.set_fill_style(&JsValue::from("white"));
    self.context.clear_rect(0 as f64, 0 as f64, self.width as f64 * CELL_SIZE, self.height as f64 * CELL_SIZE);

    self.context.set_fill_style(&JsValue::from("black"));
    for (index, cell) in self.cells.iter().enumerate() {
      if *cell == Cell::Alive {
        let x = index as u32 % self.width;
        let y = index as u32 / self.width;
        self.context.fill_rect(
          x as f64 * CELL_SIZE as f64,
          y as f64 * CELL_SIZE as f64,
          CELL_SIZE as f64,
          CELL_SIZE as f64
        );
      }
    }
  }

  pub fn click(&mut self, x: u32, y: u32) {
    let index = (x + y * self.width) as usize;
    let cell = self.cells[index];
    let mut next_cell = Cell::Alive;
    if cell == Cell::Alive {
      next_cell = Cell::Dead;
    }
    self.cells[index] = next_cell;
  }

  pub fn get_cell_size(&self) -> f64 {
    CELL_SIZE
  }
}
