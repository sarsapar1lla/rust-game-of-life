use crate::game::position::Position;
use core::fmt::Display;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell {
    pub(super) position: Position,
    pub(super) adjacent: Vec<Position>,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.position)
    }
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        let position = Position::new(x, y);
        Cell {
            position,
            adjacent: position.adjacent(),
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub(super) fn survives(&self, cells: &[Cell]) -> bool {
        let neighbours = self.count_neighours(cells);
        (2..=3).contains(&neighbours)
    }

    fn is_neighbour(&self, other: &Cell) -> bool {
        self.adjacent.contains(&other.position)
    }

    fn count_neighours(&self, cells: &[Cell]) -> usize {
        cells.iter().filter(|c| self.is_neighbour(c)).count()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn identifies_neighbour() {
        let cell = Cell::new(0, 0);
        assert!(cell.is_neighbour(&Cell::new(0, 1)))
    }

    #[test]
    fn identifies_not_neighbour() {
        let cell = Cell::new(0, 0);
        assert!(!cell.is_neighbour(&Cell::new(2, 0)))
    }

    #[test]
    fn count_neighbours() {
        let cell = Cell::new(0, 0);
        let cells = vec![Cell::new(1, 0), Cell::new(0, -1), Cell::new(1, 2)];
        assert_eq!(cell.count_neighours(&cells), 2)
    }

    #[test]
    fn cell_dies_if_less_than_2_neighbours() {
        let cell = Cell::new(0, 0);
        let cells = vec![Cell::new(1, 0)];
        assert!(!cell.survives(&cells))
    }

    #[test]
    fn cell_dies_if_more_than_3_neighbours() {
        let cell = Cell::new(0, 0);
        let cells = vec![
            Cell::new(-1, 0),
            Cell::new(1, 1),
            Cell::new(0, -1),
            Cell::new(-1, -1),
        ];
        assert!(!cell.survives(&cells))
    }

    #[test]
    fn cell_lives_if_2_neighbours() {
        let cell = Cell::new(0, 0);
        let cells = vec![Cell::new(-1, 0), Cell::new(1, 0)];
        assert!(cell.survives(&cells))
    }

    #[test]
    fn cell_lives_if_3_neighbours() {
        let cell = Cell::new(0, 0);
        let cells = vec![Cell::new(-1, 0), Cell::new(1, 0), Cell::new(1, 1)];
        assert!(cell.survives(&cells))
    }
}
