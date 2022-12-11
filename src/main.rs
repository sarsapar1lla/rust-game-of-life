use game::cell::Cell;
use game::position::Position;
use log::info;
use state::{Action, State};
use yew::prelude::*;

mod game;
mod state;

#[function_component(App)]
fn app() -> Html {
    let cells = vec![
        Cell::new(50, 50),
        Cell::new(50, 51),
        Cell::new(51, 49),
        Cell::new(51, 50),
        Cell::new(52, 50),
    ];
    let state = use_reducer(|| State::new(cells));

    let on_click = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Update))
    };

    html! {
        <>
        <h1>{ "Game of Life" }</h1>
        <div class="grid" onclick={ on_click.clone() }>
            { generate_grid(&state.positions()) }
        </div>
        </>
    }
}

fn generate_grid(cell_positions: &[Position]) -> Html {
    let cell_count = cell_positions.len();
    info!("Cell count: {}", cell_count);
    let mut squares: Vec<(i32, i32, bool)> = Vec::new();
    for row in 0..80 {
        for col in 0..80 {
            let alive = cell_positions.contains(&Position::new(row, col));
            squares.push((row, col, alive));
        }
    }
    squares
        .iter()
        .map(|s| item_from_cell(s.0, s.1, s.2))
        .collect::<Html>()
}

fn item_from_cell(row: i32, col: i32, alive: bool) -> Html {
    let colour = if alive { "gold" } else { "silver" };
    let style = format!(
        "grid-row:{}; grid-column:{};background-color:{}",
        row + 1,
        col + 1,
        colour
    );
    html! {
        <div class="item" style={ style }></div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
