use crate::model::Position;
use std::collections::HashMap;

static NEIGHBOURING_POSITIONS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
pub struct GameEngine {
    cache: HashMap<Position, Vec<Position>>,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            cache: HashMap::new(),
        }
    }

    pub fn run_generation(&mut self, cells: &[Position]) -> Vec<Position> {
        let survivors: Vec<Position> = cells
            .iter()
            .filter(|c| self.survives(c, cells))
            .map(|c| c.to_owned())
            .collect();

        let resurrected_cells = self.resurrect_cells(cells);

        [survivors, resurrected_cells].concat()
    }

    fn resurrect_cells(&mut self, positions: &[Position]) -> Vec<Position> {
        let adjacent_positions: Vec<Position> = positions
            .iter()
            .flat_map(|p| self.adjacent_positions(p))
            .filter(|p| !positions.contains(p))
            .map(|p| p.to_owned())
            .collect();

        let occurances = count_occurances(&adjacent_positions);
        occurances
            .iter()
            .filter(|i| *i.1 == 3)
            .map(|i| Position::new(i.0.x, i.0.y))
            .collect()
    }

    fn survives(&mut self, position: &Position, cell_positions: &[Position]) -> bool {
        let adjacent_positions = self.adjacent_positions(position);
        let neighbours = count_adjacent_cells(&adjacent_positions, cell_positions);
        (2..=3).contains(&neighbours)
    }

    fn adjacent_positions(&mut self, position: &Position) -> Vec<Position> {
        self.cache
            .entry(*position)
            .or_insert_with(|| compute_adjacent_positions(position))
            .to_owned()
    }
}

fn count_adjacent_cells(adjacents: &[Position], cell_positions: &[Position]) -> usize {
    cell_positions
        .iter()
        .filter(|c| adjacents.contains(c))
        .count()
}

fn compute_adjacent_positions(position: &Position) -> Vec<Position> {
    NEIGHBOURING_POSITIONS
        .iter()
        .map(|p| Position {
            x: position.x + p.0,
            y: position.y + p.1,
        })
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

    #[test]
    fn count_neighbours() {
        let adjacent_positions = vec![Position::new(1, 0), Position::new(0, -1)];
        let cell_positions = vec![
            Position::new(1, 0),
            Position::new(0, -1),
            Position::new(1, 2),
        ];
        assert_eq!(
            count_adjacent_cells(&adjacent_positions, &cell_positions),
            2
        )
    }

    #[test]
    fn cell_dies_if_less_than_2_neighbours() {
        let position = Position::new(0, 0);
        let cell_positions = vec![Position::new(1, 0)];
        assert!(!GameEngine::new().survives(&position, &cell_positions))
    }

    #[test]
    fn cell_dies_if_more_than_3_neighbours() {
        let position = Position::new(0, 0);
        let cell_positions = vec![
            Position::new(-1, 0),
            Position::new(1, 1),
            Position::new(0, -1),
            Position::new(-1, -1),
        ];
        assert!(!GameEngine::new().survives(&position, &cell_positions))
    }

    #[test]
    fn cell_lives_if_2_neighbours() {
        let position = Position::new(0, 0);
        let cell_positions = vec![Position::new(-1, 0), Position::new(1, 0)];
        assert!(GameEngine::new().survives(&position, &cell_positions))
    }

    #[test]
    fn cell_lives_if_3_neighbours() {
        let position = Position::new(0, 0);
        let cell_positions = vec![
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(1, 1),
        ];
        assert!(GameEngine::new().survives(&position, &cell_positions))
    }

    #[test]
    fn returns_adjacent_positions() {
        let position = Position::new(1, 1);
        let expected = vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(1, 0),
            Position::new(1, 2),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ];
        assert_eq!(compute_adjacent_positions(&position), expected)
    }

    #[test]
    fn no_cells_resurrect_if_less_than_3_cells_share_a_common_neighbour() {
        let cells = vec![Position::new(0, 0), Position::new(1, 0)];
        assert!(GameEngine::new().resurrect_cells(&cells).is_empty())
    }

    #[test]
    fn cells_resurrect_if_3_cells_share_a_common_neighbour() {
        let cells = vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
        ];
        assert_eq!(
            GameEngine::new().resurrect_cells(&cells),
            vec![Position::new(1, 1)]
        )
    }

    #[test]
    fn no_cells_resurrect_if_more_than_3_cells_share_a_common_neighbour() {
        let cells = vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 2),
        ];
        assert!(GameEngine::new().resurrect_cells(&cells).is_empty())
    }

    #[test]
    fn runs_generation() {
        let cells = vec![
            Position::new(-1, 0),
            Position::new(0, 0),
            Position::new(1, 0),
        ];
        let expected = vec![
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
        ]
        .sort();
        assert_eq!(GameEngine::new().run_generation(&cells).sort(), expected)
    }
}
