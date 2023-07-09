use common::*;
use gfx::{Commands};
use models::{Card, gen_card};
use platform_types::{command, unscaled, Input, Speaker, SFX};
use xs::{Xs, Seed};

pub struct State {
    pub rng: Xs,
    state: common::State,
    platform: Platform,
    events: Vec<Event>,
}

const TILE_SIZE: unscaled::Inner = 45;

impl State {
    pub fn new(seed: Seed) -> State {
        let rng = xs::from_seed(seed);

        State {
            rng,
            state: state_manipulation::new_state(Size::new(
                i32::from(TILE_SIZE),
                i32::from(TILE_SIZE)
            )),
            platform: Platform {
                print_xy: platform::print_xy,
                clear: platform::clear,
                size: platform::size,
                pick: platform::pick,
                mouse_position: platform::mouse_position,
                clicks: platform::clicks,
                key_pressed: platform::key_pressed,
                set_colors: platform::set_colors,
                get_colors: platform::get_colors,
                set_foreground: platform::set_foreground,
                get_foreground: platform::get_foreground,
                set_background: platform::set_background,
                get_background: platform::get_background,
                set_layer: platform::set_layer,
                get_layer: platform::get_layer,
            },
            events: Vec::with_capacity(1),
        }
    }

    pub fn update_and_render(
        commands: &mut Commands,
        state: &mut State,
        input: Input,
        speaker: &mut Speaker,
    ) {
        state.events.clear();
        match input {
            _ => {
                //state.events.push();
            }
        }

        let _ignored = state_manipulation::update_and_render(
            &state.platform,
            &mut state.state,
            &mut state.events
        );

        platform::push_commands(commands);

        platform::end_frame();
    }
}

mod platform {
    use super::*;
    use platform_types::{sprite};
    use std::{
        collections::HashMap,
        sync::{Mutex, OnceLock}
    };

    type X = unscaled::Inner;
    type Y = unscaled::Inner;

    struct State {
        chars: HashMap<(X, Y), &'static str>,
    }
    
    fn state() -> &'static Mutex<State> {
        static STATE: OnceLock<Mutex<State>> = OnceLock::new();
        STATE.get_or_init(|| Mutex::new(State {
            chars: HashMap::with_capacity(128),
        }))
    }

    macro_rules! state {
        () => {
            state().lock().expect("should not be poisoned")
        }
    }

    /// `Platform` function pointers

    pub fn print_xy(x_in: i32, y_in: i32, s: &'static str) {
        assert_eq!(s.chars().count(), 1, "{s}");

        match (X::try_from(x_in), Y::try_from(y_in)) {
            (Ok(x), Ok(y)) => {
                state!().chars.insert((x, y), s);
            },
            _ => {
                assert!(false, "bad (x, y): ({x_in}, {y_in})");
            }
        }
    }
    pub fn clear(rect: Option<Rect>) {

    }
    pub fn size() -> Size {
        Size::new(i32::from(TILE_SIZE), i32::from(TILE_SIZE))
    }
    pub fn pick(point: Point, _: i32) -> char {
        '\0'
    }
    pub fn mouse_position() -> Point {
        Point::default()
    }
    pub fn clicks() -> i32 {
        0
    }
    pub fn key_pressed(key: KeyCode) -> bool {
        false
    }
    pub fn set_colors(foreground: Color, background: Color) {
        
    }
    pub fn get_colors() -> (Color, Color) {
        (
            Color { red: 255, green: 0, blue: 255, alpha: 255 },
            Color { red: 255, green: 0, blue: 255, alpha: 255 },
        )
    }
    pub fn set_foreground(foreground: Color) {

    }
    pub fn get_foreground() -> (Color) {
        get_colors().0
    }
    pub fn set_background(background: Color) {

    }
    pub fn get_background() -> (Color) {
        get_colors().1
    }
    pub fn set_layer(layer: i32) {

    }
    pub fn get_layer() -> i32 {
        0
    }

    /// `platform` state management
    pub fn push_commands(commands: &mut Commands) {
        for ((x, y), s) in state!().chars.iter() {
            let (sx, sy) = match *s {
                "☒" => (1 * TILE_SIZE, 0),
                "\u{E010}" => (2 * TILE_SIZE, 0),
                "\u{E011}" => (3 * TILE_SIZE, 0),
                "\u{E012}" => (4 * TILE_SIZE, 0),
                "\u{E013}" => (5 * TILE_SIZE, 0),
                "\u{E014}" => (6 * TILE_SIZE, 0),
                "\u{E015}" => (7 * TILE_SIZE, 0),
                "\u{E016}" => (8 * TILE_SIZE, 0),
                "\u{E017}" => (9 * TILE_SIZE, 0),
                "\u{E018}" => (10 * TILE_SIZE, 0),
                "@" => (3 * TILE_SIZE, 1 * TILE_SIZE),
                "R" => (4 * TILE_SIZE, 1 * TILE_SIZE),
                "↑" => (3 * TILE_SIZE, 2 * TILE_SIZE),
                "←" => (4 * TILE_SIZE, 2 * TILE_SIZE),
                "↓" => (5 * TILE_SIZE, 2 * TILE_SIZE),
                "→" => (6 * TILE_SIZE, 2 * TILE_SIZE),
                "┌" => (3 * TILE_SIZE, 3 * TILE_SIZE),
                "─" => (4 * TILE_SIZE, 3 * TILE_SIZE),
                "╖" => (5 * TILE_SIZE, 3 * TILE_SIZE),
                "│" => (6 * TILE_SIZE, 3 * TILE_SIZE),
                "╘" => (7 * TILE_SIZE, 3 * TILE_SIZE),
                "┘" => (8 * TILE_SIZE, 3 * TILE_SIZE),
                "╔" => (3 * TILE_SIZE, 4 * TILE_SIZE),
                "═" => (4 * TILE_SIZE, 4 * TILE_SIZE),
                "╕" => (5 * TILE_SIZE, 4 * TILE_SIZE),
                "║" => (6 * TILE_SIZE, 4 * TILE_SIZE),
                "╙" => (7 * TILE_SIZE, 4 * TILE_SIZE),
                "╝" => (8 * TILE_SIZE, 4 * TILE_SIZE),
                _ => {
                    debug_assert!(false, "unknown tile str: \"{s}\"");
                    (0, 0)
                }
            };

            commands.sspr(
                sprite::XY {
                    x: sprite::X(sx),
                    y: sprite::Y(sy),
                },
                command::Rect::from_unscaled(unscaled::Rect {
                    x: unscaled::X((x * TILE_SIZE) as _),
                    y: unscaled::Y((y * TILE_SIZE) as _),
                    w: unscaled::W(TILE_SIZE),
                    h: unscaled::H(TILE_SIZE),
                })
            );
        }
    }
        
    pub fn end_frame() {
        state!().chars.clear();
    }
}
