#[macro_use] mod utils;

use wasm_bindgen::prelude::*;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!("cell[{}, {}] is initially {:?} and has {} live neighbors",row,col,cell,live_neighbors);
                next.set(index, Universe::get_next_tick_cell_state(cell, live_neighbors));
                // log!("    it becomes {:?}", next[index]);
            }
        }
        self.cells = next;
    }

    /// Set the width of the universe.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_cells();
    }

    /// Set the height of the universe.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_cells();
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let index = self.get_index(row, col);
            self.cells.set(index, true);
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (self.width * row + column) as usize
    }

    fn live_neighbor_count(&self, row:u32, column: u32) -> u8 {
        let mut count = 0;
        // [self.height - 1, 0, 1].iter().for_each() TODO rewrite functionally
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col  in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }

        count
    }

    fn get_next_tick_cell_state(cell: bool, live_neighbors: u8) -> bool {
        match (cell, live_neighbors) {
            // Rule 1: Any live cell with fewer than two live neighbours
            // dies, as if caused by underpopulation.
            (true, x) if x < 2 => false,
            // Rule 2: Any live cell with two or three live neighbours
            // lives on to the next generation.
            (true, 2) | (true, 3) => true,
            // Rule 3: Any live cell with more than three live
            // neighbours dies, as if by overpopulation.
            (true, x) if x > 3 => false,
            // Rule 4: Any dead cell with exactly three live neighbours
            // becomes a live cell, as if by reproduction.
            (false, 3) => true,
            // All other cells remain in the same state.
            (otherwise, _) => otherwise
        }
    }

    fn reset_cells(&mut self) {
        self.cells.clear();
    }
}
