pub mod cell;
pub mod position;

use std::collections::{HashMap, HashSet};

use crate::game::cell::Cell;
use crate::game::position::Position;

pub fn run_generation(cells: &[Cell]) -> Vec<Cell> {
    let survivors: Vec<Cell> = cells
        .iter()
        .filter(|c| c.survives(cells))
        .map(|c| c.to_owned())
        .collect();

    let resurrected_cells = resurrect_cells(cells);

    [survivors, resurrected_cells].concat()
}

fn resurrect_cells(cells: &[Cell]) -> Vec<Cell> {
    let current_cell_positions: HashSet<Position> = cells.iter().map(|c| c.position).collect();

    let adjacent_positions: Vec<Position> = cells
        .iter()
        .flat_map(|c| c.adjacent.clone())
        .filter(|p| !current_cell_positions.contains(p))
        .collect();

    let occurances = count_occurances(&adjacent_positions);
    occurances
        .iter()
        .filter(|i| *i.1 == 3)
        .map(|i| Cell::new(i.0.x, i.0.y))
        .collect()
}

fn count_occurances(positions: &[Position]) -> HashMap<&Position, usize> {
    let mut occurances = HashMap::new();
    for position in positions.iter() {
        *occurances.entry(position).or_insert(0) += 1;
    }
    occurances
}

#[cfg(test)]
mod tests {

    use super::*;

    // TODO
    // #[test]
    // fn counts_occurances_in_list() {
    //     let positions = vec![
    //         Position::new(0, 0),
    //         Position::new(0, 0),
    //         Position::new(0, 1),
    //     ];
    //     let map = count_occurances(&positions);
    //     let mut expected: HashMap<&Position, usize> = HashMap::new();
    //     expected.insert(&Position::new(0, 0), 2 as usize);
    //     expected.insert(&Position::new(0, 1), 1 as usize);

    //     assert_eq!(map, expected)
    // }

    #[test]
    fn no_cells_resurrect_if_less_than_3_cells_share_a_common_neighbour() {
        let cells = vec![Cell::new(0, 0), Cell::new(1, 0)];
        assert!(resurrect_cells(&cells).is_empty())
    }

    #[test]
    fn cells_resurrect_if_3_cells_share_a_common_neighbour() {
        let cells = vec![Cell::new(0, 0), Cell::new(1, 0), Cell::new(0, 1)];
        assert_eq!(resurrect_cells(&cells), vec![Cell::new(1, 1)])
    }

    #[test]
    fn no_cells_resurrect_if_more_than_3_cells_share_a_common_neighbour() {
        let cells = vec![
            Cell::new(0, 0),
            Cell::new(1, 0),
            Cell::new(0, 1),
            Cell::new(1, 2),
        ];
        assert!(resurrect_cells(&cells).is_empty())
    }

    #[test]
    fn runs_generation() {
        let cells = vec![Cell::new(-1, 0), Cell::new(0, 0), Cell::new(1, 0)];
        let expected = vec![Cell::new(0, -1), Cell::new(0, 0), Cell::new(0, 1)].sort();
        assert_eq!(run_generation(&cells).sort(), expected)
    }
}
