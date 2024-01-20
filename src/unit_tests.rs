#[cfg(test)]
mod tests {
    use crate::game_of_life_logic::{Cell, Universe};

    #[test]
    fn test_edit_cell() {
        let mut universe = Universe::new(10, 10);
        universe.edit_cell((1, 1));
        assert_eq!(universe.get_cell((1, 1)), Cell::Alive);
    }

    fn set_cells_alive(universe: &mut Universe, positions: &[(usize, usize)]) {
        for pos in positions {
            universe.edit_cell(*pos);
        }
    }

    #[test]
    fn test_underpopulation() {
        let mut universe = Universe::new(3, 3);
        set_cells_alive(&mut universe, &[(1, 1)]);
        universe = universe.iterate();
        assert_eq!(universe.get_cell((1, 1)), Cell::Dead);
    }

    #[test]
    fn test_survival() {
        let mut universe = Universe::new(3, 3);
        set_cells_alive(&mut universe, &[(0, 1), (1, 1), (2, 1)]);
        println!("{universe}");
        universe = universe.iterate();
        println!("{:?}", universe.count_alive_neighbors());
        println!("{universe}");

        assert_eq!(universe.get_cell((1, 1)), Cell::Alive);
    }

    #[test]
    fn test_overpopulation() {
        let mut universe = Universe::new(3, 3);
        set_cells_alive(&mut universe, &[(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)]);
        universe = universe.iterate();
        assert_eq!(universe.get_cell((1, 1)), Cell::Dead);
    }

    #[test]
    fn test_reproduction() {
        let mut universe = Universe::new(3, 3);
        set_cells_alive(&mut universe, &[(0, 1), (1, 0), (1, 2)]);
        universe = universe.iterate();
        assert_eq!(universe.get_cell((1, 1)), Cell::Alive);
    }
}
