use crate::engine::GameEngine;
use crate::model::Position;
use gloo::timers::callback::Interval;
use log::info;
use wasm_bindgen::JsCast;
use yew::prelude::*;

static ALIVE_COLOUR: &str = "gold";
static DEAD_COLOUR: &str = "lightcyan";
const GRID_SIZE: usize = 80;

#[derive(Debug)]
pub enum Msg {
    Reset,
    Start,
    Stop,
    Tick,
    Toggle { row: i32, col: i32 },
}

pub struct App {
    engine: GameEngine,
    cells: Vec<Position>,
    active: bool,
    refresh_interval: Option<Interval>,
}

impl App {
    fn generate_button(
        &self,
        button_text: String,
        click_callback: Callback<MouseEvent, ()>,
        disable_when_active: bool,
    ) -> Html {
        let mut button_class: String = String::from("controller-button");
        if (self.active && disable_when_active) || !(self.active || disable_when_active) {
            button_class = format!("{} disabled", button_class);
        }
        html! {
            <button class={ button_class } onclick={ click_callback }>{ button_text }</button>
        }
    }

    fn generate_grid(&self, on_click: Callback<MouseEvent, ()>) -> Html {
        let cell_count = &self.cells.len();
        info!("Cell count: {}", cell_count);
        let mut squares: Vec<(i32, i32, bool)> = Vec::new();
        for row in 0..GRID_SIZE as i32 {
            for col in 0..GRID_SIZE as i32 {
                let alive = self.cells.contains(&Position::new(row, col));
                squares.push((row, col, alive));
            }
        }
        squares
            .iter()
            .map(|s| self.item_from_cell(s.0, s.1, s.2, on_click.clone()))
            .collect::<Html>()
    }

    fn item_from_cell(
        &self,
        row: i32,
        col: i32,
        alive: bool,
        on_click: Callback<MouseEvent, ()>,
    ) -> Html {
        let colour = if alive { ALIVE_COLOUR } else { DEAD_COLOUR };
        let id = format!("{}_{}", row, col);
        let style = format!(
            "grid-row:{}; grid-column:{}; background-color:{}",
            row + 1,
            col + 1,
            colour
        );
        html! {
            <div class="item" id={ id } style={ style } onclick={ on_click }></div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            engine: GameEngine::new(),
            cells: Vec::new(),
            active: false,
            refresh_interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.active = false;
                self.cells.clear();
                info!("Cells cleared");
                true
            }
            Msg::Start => {
                let callback = ctx.link().callback(|_| Msg::Tick);
                self.refresh_interval = Some(Interval::new(500, move || callback.emit(())));
                self.active = true;
                info!("Started simulation");
                false
            }
            Msg::Stop => {
                self.active = false;
                self.refresh_interval = None;
                info!("Stopped simulation");
                true
            }
            Msg::Tick => {
                if !self.active {
                    return false;
                }
                self.cells = self.engine.run_generation(&self.cells);
                info!("Cell count: {}", self.cells.len());

                if self.cells.is_empty() {
                    info!("No cells left. Stopping simulation");
                    self.active = false;
                }
                true
            }
            Msg::Toggle { row, col } => {
                if self.active {
                    return false;
                }
                let position = Position::new(row, col);
                match self.cells.iter().position(|p| *p == position) {
                    Some(index) => {
                        self.cells.remove(index);
                        info!("Deactivated cell {}", position);
                    }
                    None => {
                        self.cells.push(position);
                        info!("Activated cell {}", position);
                    }
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_reset = ctx.link().callback(|_| Msg::Reset);
        let on_start = ctx.link().callback(|_| Msg::Start);
        let on_stop = ctx.link().callback(|_| Msg::Stop);

        let on_click = ctx.link().callback(|event: MouseEvent| {
            info!("Received mouse event");
            let (row, col) = match extract_square_coordinates(event) {
                Some((row, col)) => (row, col),
                None => panic!("Could not extract square coordinates from event"),
            };
            info!("Row: {}, col: {}", row, col);
            Msg::Toggle { row, col }
        });

        html! {
            <>
                <div class="text-container">
                    <h1>{ "Game of Life" }</h1>
                    <p>{ "This is an implementation of Conway's 'Game of Life' in Rust" }</p>
                    <p>{ "The game has four rules which are applied with every iteration" }</p>
                    <ol>
                        <li>{ "Any living cell with fewer than two neighbours dies, as if by under-population" }</li>
                        <li>{ "Any living cell with two or three neighbours survives to the next generation" }</li>
                        <li>{ "Any living cell with more than three neighbours dies, as if by overcrowding" }</li>
                        <li>{ "Any dead cell with exactly three living neighbours becomes a living cell, as if by reproduction" }</li>
                    </ol>
                    <p>{ "Select cells by clicking on them. Once you're happy with your configuration, click 'Start' to begin the simulation." }</p>
                    <p>{ "Stop the simulation by clicking 'Stop' and clear the grid by clicking 'Reset'." }</p>
                </div>
                <div class="controller-container">
                    { self.generate_button(String::from("Start"), on_start, true) }
                    { self.generate_button(String::from("Stop"), on_stop, false) }
                    { self.generate_button(String::from("Reset"), on_reset, true)}
                </div>
                <div class="grid">
                    { self.generate_grid(on_click) }
                </div>
            </>
        }
    }
}

fn extract_square_coordinates(event: MouseEvent) -> Option<(i32, i32)> {
    let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;
    let coordinates: Vec<String> = target
        .get_attribute("id")?
        .split("_")
        .map(|s| s.to_string())
        .collect();

    let row = coordinates.get(0)?.parse().ok()?;
    let col = coordinates.get(1)?.parse().ok()?;
    Some((row, col))
}
