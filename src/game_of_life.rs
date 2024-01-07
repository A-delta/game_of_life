pub mod game_of_life {

    #[derive(Clone, Copy, Debug)]
    pub enum Cell {
        Alive,
        Dead,
        //Outside,
    }

    impl Default for &Cell {
        fn default() -> Self {
            &Cell::Dead
        }
    }

    pub struct Universe {
        height: usize,
        width: usize,
        cells: Vec<Cell>,
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

        pub fn iterate(mut self) -> Self {
            let mut next_universe = Universe::new(self.height, self.width);
            let alive = self.count_alive_neighbors();
            //println!("{:?}", alive);
            for (i, cell) in self.cells.iter_mut().enumerate() {
                match (cell, alive[i]) {
                    (Cell::Alive, 2 | 3) | (Cell::Dead, 3) => next_universe.cells[i] = Cell::Alive,
                    _ => {}
                }
            }
            next_universe
        }

        fn get_cell(&self, (i, j): (usize, usize)) -> Cell {
            //println!("{},{}", i, j);
            if i >= self.height || j >= self.width {
                Cell::Dead
            } else {
                *(self.cells.get(i * self.width + j).unwrap_or_default())
            }
        }

        pub fn edit_cell(&mut self, (i, j): (usize, usize), cell: Cell) {
            self.cells[i * self.width + j] = cell;
        }
        fn coordinates_from_linear(&self, i: usize) -> (usize, usize) {
            let y = i % self.width;
            let x = i / self.width;

            (x, y)
        }
        fn count_alive_neighbors(&self) -> Vec<usize> {
            let mut counter = vec![0; self.height * self.width];
            for index in 0..self.height * self.width {
                let (i, j) = self.coordinates_from_linear(index);
                for i_offset in 1..=3 {
                    for j_offset in 1..=3 {
                        let (i_offsetted, j_offsetted) =
                            if (i == 0 && i_offset == 1) || (j == 0 && j_offset == 1) {
                                (0, 0)
                            } else {
                                (i + (i_offset) - 2, j + (j_offset) - 2)
                            };
                        match (
                            i_offset,
                            j_offset,
                            self.get_cell((i_offsetted, j_offsetted)),
                        ) {
                            (2, 2, _) => continue,
                            (_, _, Cell::Alive) => {
                                counter[index] += 1;
                            }
                            _ => continue,
                        }
                    }
                }
                //println!("{i}, {j} : {}", counter[index]);
            }
            counter
        }

        pub fn display(&self) {
            for (i, cell) in self.cells.iter().enumerate() {
                match *cell {
                    //match self.get_cell(self.coordinates_from_linear(i)) {
                    Cell::Alive => print!("■ "),
                    Cell::Dead => print!("□ "),
                }
                if (i + 1) % self.width == 0 {
                    println!("");
                }
            }
        }
    }
}
