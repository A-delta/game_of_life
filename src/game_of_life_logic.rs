use rayon::prelude::*;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Default for &Cell {
    fn default() -> Self {
        &Cell::Dead
    }
}

#[derive(crate::Resource)]
pub struct Universe {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Cell>,
}

impl Universe {
    pub fn new(height: usize, width: usize) -> Universe {
        let cells = vec![Cell::Dead; width * height];
        Universe {
            height,
            width,
            cells,
        }
    }

    pub fn iterate(&mut self) -> Self {
        let mut next_universe = Universe::new(self.height, self.width);
        let alive = self.count_alive_neighbors();
        next_universe.cells = next_universe
            .cells
            .par_iter_mut()
            .enumerate()
            .map(|(i, cell)| -> Cell {
                match (cell, alive[i]) {
                    (Cell::Alive, 2 | 3) | (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                }
            })
            .collect::<Vec<Cell>>();
        next_universe
    }

    pub fn get_cell(&self, (i, j): (usize, usize)) -> Cell {
        if i >= self.height || j >= self.width {
            Cell::Dead
        } else {
            *(self.cells.get(i * self.width + j).unwrap_or_default())
        }
    }

    pub fn edit_cell(&mut self, (i, j): (usize, usize)) {
        match self.cells[i * self.width + j] {
            Cell::Alive => self.cells[i * self.width + j] = Cell::Dead,
            Cell::Dead => self.cells[i * self.width + j] = Cell::Alive,
        }
    }

    pub fn set_cell(&mut self, (i, j): (usize, usize), cell: Cell) {
        self.cells[i * self.width + j] = cell;
    }
    pub fn coordinates_from_linear(&self, i: usize) -> (usize, usize) {
        let y = i % self.width;
        let x = i / self.width;

        (x, y)
    }

    pub fn count_alive_neighbors(&self) -> Vec<usize> {
        let mut counter = vec![0; self.height * self.width];
        for (index, value) in counter.iter_mut().enumerate() {
            let (i, j) = self.coordinates_from_linear(index);
            for i_offset in 1..=3 {
                for j_offset in 1..=3 {
                    if !((i == 0 && i_offset == 1)   // upper border
                        || (j == 0 && j_offset == 1) // lower border
                        || (j == 2 && i == 2))
                    // current cell
                    {
                        let i_offsetted = i + (i_offset) - 2;
                        let j_offsetted = j + (j_offset) - 2;
                        match self.get_cell((i_offsetted, j_offsetted)) {
                            Cell::Alive => {
                                *value += 1;
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
        counter
    }
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            match *cell {
                Cell::Alive => write!(f, "■ ").expect("An error occurred while writing to buffer"),
                Cell::Dead => write!(f, "□ ").expect("An error occurred while writing to buffer"),
            }
            if (i + 1) % self.width == 0 {
                writeln!(f).expect("An error occurred while writing to buffer");
            }
        }
        Ok(())
    }
}
