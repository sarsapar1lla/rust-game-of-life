use crate::game::cell::Cell;
use crate::game::position::Position;
use crate::game::run_generation;
use std::rc::Rc;
use yew::prelude::*;

pub enum Action {
    Update,
}

pub struct State {
    cells: Vec<Cell>,
}

impl State {
    pub fn new(cells: Vec<Cell>) -> Self {
        State { cells }
    }

    pub fn positions(&self) -> Vec<Position> {
        self.cells.iter().map(|c| c.position()).collect()
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Update => {
                let cells = self.cells.clone();
                let new_cells = run_generation(&cells);

                State::new(new_cells).into()
            }
        }
    }
}
