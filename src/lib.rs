mod utils;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct World {
    ctx: CanvasRenderingContext2d,
    world: Box<[Cell]>,
    width: usize,
    height: usize,
    cell_w: usize,
    cell_h: usize,
    wrap_around: bool,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ctx: CanvasRenderingContext2d,
        width: usize,
        height: usize,
        cell_w: usize,
        cell_h: usize,
        wrap_around: bool,
    ) -> Self {
        let world = vec![Cell::Dead; width * height].into_boxed_slice();

        let s = Self {
            ctx,
            world,
            width,
            height,
            cell_w,
            cell_h,
            wrap_around,
        };
        s.init_canvas();
        s
    }

    fn get_cell_state(&mut self, x: usize, y: usize) -> u8 {
        let Some(x) = self.wrap(x, self.width) else {
            return 0;
        };

        let Some(y) = self.wrap(y, self.height) else {
            return 0;
        };

        self.world[y * self.width + x].get_state()
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.world[y * self.width + x]
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.world[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        *self.get_cell_mut(x, y) = cell;
    }

    fn wrap(&self, value: usize, max: usize) -> Option<usize> {
        if value == usize::MAX {
            if self.wrap_around {
                Some(max - 1)
            } else {
                None
            }
        } else if value == max {
            if self.wrap_around { Some(0) } else { None }
        } else {
            Some(value)
        }
    }

    fn calculate_neighbors(&mut self, x: usize, y: usize) -> u8 {
        let neighbors = 0
            + self.get_cell_state(x.wrapping_add_signed(1), y.wrapping_add_signed(1))
            + self.get_cell_state(x.wrapping_add_signed(1), y.wrapping_add_signed(0))
            + self.get_cell_state(x.wrapping_add_signed(1), y.wrapping_add_signed(-1))
            + self.get_cell_state(x.wrapping_add_signed(0), y.wrapping_add_signed(-1))
            + self.get_cell_state(x.wrapping_add_signed(-1), y.wrapping_add_signed(-1))
            + self.get_cell_state(x.wrapping_add_signed(-1), y.wrapping_add_signed(0))
            + self.get_cell_state(x.wrapping_add_signed(-1), y.wrapping_add_signed(1))
            + self.get_cell_state(x.wrapping_add_signed(0), y.wrapping_add_signed(1));
        neighbors
    }

    fn init_canvas(&self) {
        let total_width = (self.cell_w * self.width - 1) as f64;
        let total_height = (self.cell_h * self.height - 1) as f64;
        let canvas = self.ctx.canvas().unwrap();
        canvas.set_width(total_width as u32);
        canvas.set_height(total_height as u32);
        self.ctx.clear_rect(0.0, 0.0, total_width, total_height);
        self.ctx.set_stroke_style_str("lightgray");

        for i in 1..self.width {
            let x = (self.cell_w * i) as f64 - 0.5;
            self.ctx.move_to(x, 0.0);
            self.ctx.line_to(x, total_height);
        }
        for i in 1..self.height {
            let y = (self.cell_h * i) as f64 - 0.5;
            self.ctx.move_to(0.0, y);
            self.ctx.line_to(total_width, y);
        }
        self.ctx.stroke();
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                match cell {
                    Cell::Dead => self.ctx.set_fill_style_str("white"),
                    Cell::Alive => self.ctx.set_fill_style_str("black"),
                }
                self.ctx.fill_rect(
                    (x * self.cell_w) as f64,
                    (y * self.cell_h) as f64,
                    (self.cell_w - 1) as f64,
                    (self.cell_h - 1) as f64,
                );
            }
        }
    }

    pub fn update(&mut self) {
        let mut changed_cells = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbours = self.calculate_neighbors(x, y);
                let cell = self.get_cell(x, y);
                if cell.is_alive() && (neighbours < 2 || neighbours > 3) {
                    changed_cells.push((x, y, Cell::Dead));
                } else if cell.is_dead() && neighbours == 3 {
                    changed_cells.push((x, y, Cell::Alive));
                }
            }
        }
        for (x, y, cell) in changed_cells {
            self.change_cell(x, y, cell);
        }
    }

    fn change_cell(&mut self, x: usize, y: usize, cell: Cell) {
        *self.get_cell_mut(x, y) = cell;
        match cell {
            Cell::Dead => self.ctx.set_fill_style_str("white"),
            Cell::Alive => self.ctx.set_fill_style_str("black"),
        }
        self.ctx.fill_rect(
            (x * self.cell_w) as f64,
            (y * self.cell_h) as f64,
            (self.cell_w - 1) as f64,
            (self.cell_h - 1) as f64,
        );
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if self.get_cell(x, y).is_alive() {
            self.change_cell(x, y, Cell::Dead);
        } else {
            self.change_cell(x, y, Cell::Alive);
        }
    }
}

impl_getter_setter!(World, width: usize, width);
impl_getter_setter!(World, height: usize, height);
impl_getter_setter!(World, wrap_around: bool, wrap_around, set_wrap_around);

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, width: usize) {
        self.change_size(width, self.height);
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, height: usize) {
        self.change_size(self.width, height);
    }

    fn change_size(&mut self, width: usize, height: usize) {
        let mut new_world = vec![Cell::Dead; width * height].into_boxed_slice();
        let x = usize::min(width, self.width);
        for y in 0..usize::min(height, self.height) {
            new_world[y * width..][..x].copy_from_slice(&self.world[y * self.width..][..x]);
        }
        self.world = new_world;
        self.width = width;
        self.height = height;
        self.init_canvas();
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn get_state(&self) -> u8 {
        *self as u8 & 1
    }

    pub fn is_alive(&self) -> bool {
        self.get_state() == 1
    }

    pub fn is_dead(&self) -> bool {
        self.get_state() == 0
    }
}
