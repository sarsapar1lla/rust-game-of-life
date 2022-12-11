use core::fmt::Display;

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

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub(super) fn adjacent(&self) -> Vec<Position> {
        NEIGHBOURING_POSITIONS
            .iter()
            .map(|p| Position {
                x: self.x + p.0,
                y: self.y + p.1,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn returns_neighbouring_positions() {
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
        assert_eq!(position.adjacent(), expected)
    }
}
