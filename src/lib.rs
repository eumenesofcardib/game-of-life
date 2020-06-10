mod utils;

use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

/*
extern crate web_sys;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer <'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
	console::time_with_label(name);
	Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop (&mut self) {
	console::time_end_with_label(self.name);
    }
}
*/

/*
macro_rules! log {
    ( $( $t:tt )* ) => {
	web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
*/

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
	(row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
	let mut count = 0;

	let north = if row == 0 {
            self.height - 1
	} else {
            row - 1
	};

	let south = if row == self.height - 1 {
            0
	} else {
            row + 1
	};

	let west = if column == 0 {
            self.width - 1
	} else {
            column - 1
	};

	let east = if column == self.width - 1 {
            0
	} else {
            column + 1
	};

	let nw = self.get_index(north, west);
	count += self.cells[nw] as u8;

	let n = self.get_index(north, column);
	count += self.cells[n] as u8;

	let ne = self.get_index(north, east);
	count += self.cells[ne] as u8;

	let w = self.get_index(row, west);
	count += self.cells[w] as u8;

	let e = self.get_index(row, east);
	count += self.cells[e] as u8;

	let sw = self.get_index(south, west);
	count += self.cells[sw] as u8;

	let s = self.get_index(south, column);
	count += self.cells[s] as u8;

	let se = self.get_index(south, east);
	count += self.cells[se] as u8;

	count
    }
}

/// Public methods, for the public eye of javascript

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
//	let _timer = Timer::new("Universe::tick");
	let mut next = self.cells.clone();

	for row in 0..self.height {
	    for col in 0..self.width {
		let idx = self.get_index(row, col);
		let cell = self.cells[idx];
		let live_neighbors = self.live_neighbor_count(row, col);

//		log!("cell[{}, {}] is initially {:?} and has {} live neighbors", row, col, cell, live_neighbors);
		
		next.set(idx, match (cell, live_neighbors) {
		    (true, x) if x < 2 => false,
		    (true, 2) | (true, 3) => true,
		    (true, x) if x > 3 => false,
		    (false, 3) => true,
		    (otherwise, _) => otherwise,
		});

//		if !next[idx] == self.cells[idx] {
//		    log!("cell[{}, {}] is initially {:?} and has {} live neighbors and transitions to {}", row, col, cell, live_neighbors, next[idx]);
//		}
		
	    }
	}

	self.cells = next;
    }

    pub fn new() -> Universe {
	utils::set_panic_hook();

	let width = 128;
	let height = 128;

	let size = (width * height) as usize;
	let mut cells = FixedBitSet::with_capacity(size);

	cells.clear();

	Universe {
	    width,
	    height,
	    cells,
	}
    }
    
/*    pub fn render(&self) -> String {
	self.to_string()
    }
*/
    pub fn width(&self) -> u32 {
	self.width
    }

    pub fn height(&self) -> u32 {
	self.height
    }

    pub fn cells(&self) -> *const u32 {
	self.cells.as_slice().as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
	self.width = width;
	self.cells.clear();
    }

    pub fn set_height(&mut self, height: u32) {
	self.height = height;
	self.cells.clear();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
	let idx = self.get_index(row, column);
	self.cells.toggle(idx);
    }

    pub fn randomize(&mut self) {
	let size = (self.width * self. height) as usize;
	for i in 0..size {
	    self.cells.set(i, js_sys::Math::random() < 0.5);
	}
    }

    pub fn clear(&mut self) {
	self.cells.clear();
    }
}

// not for the eyes of javascript
impl Universe {
    pub fn get_cells(&self) -> &[u32] {
	&self.cells.as_slice()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
	for (row, col) in cells.iter().cloned() {
	    let idx = self.get_index(row, col);
	    self.cells.set(idx, true);
	}
    }
}


/*
use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	for line in self.cells.as_slice().chunks(self.width as usize) {
	    for &cell in line {
		let symbol = if cell == 0 { '◻' } else { '◼' };
		write!(f, "{}", symbol)?;
	    }
	    write!(f, "\n");
	}

	Ok(())
    }
}
*/
