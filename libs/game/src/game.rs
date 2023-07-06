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

const TILE_SIZE: i32 = 45;

impl State {
    pub fn new(seed: Seed) -> State {
        let rng = xs::from_seed(seed);

        State {
            rng,
            state: state_manipulation::new_state(Size::new(TILE_SIZE, TILE_SIZE)),
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
    }
}

mod platform {
    use super::*;

    /// TODO use a static std::sync::OnceLock<Mutex<...>> to implement these properly.
    pub fn print_xy(x: i32, y: i32, s: &str) {
        
    }
    pub fn clear(rect: Option<Rect>) {

    }
    pub fn size() -> Size {
        Size::new(TILE_SIZE, TILE_SIZE)
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
}
