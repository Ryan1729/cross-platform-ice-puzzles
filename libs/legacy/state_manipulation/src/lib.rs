extern crate common;
extern crate rand;

use common::*;
use common::Cell::*;
use common::Motion::*;

use std::collections::HashMap;

use rand::{StdRng, SeedableRng, Rng};

//NOTE(Ryan1729): debug_assertions only appears to work correctly when the
//crate is not a dylib. Assuming you make this crate *not* a dylib on release,
//these configs should work
//#[cfg(debug_assertions)]
//#[no_mangle]
//pub fn new_state(size: Size) -> State {
    ////skip the title screen
    //println!("debug {}",
             //if cfg!(debug_assertions) { "on" } else { "off" });
//
    //let seed: &[_] = &[42];
    //let rng: StdRng = SeedableRng::from_seed(seed);
//
    //next_level(size, rng, 4)
//}
//
//#[cfg(not(debug_assertions))]
#[no_mangle]
pub fn new_state(size: Size) -> State {
    //show the title screen
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|dur| dur.as_secs())
        .unwrap_or(42);

    println!("{}", timestamp);
    let seed: &[_] = &[timestamp as usize];
    let rng: StdRng = SeedableRng::from_seed(seed);

    let mut cells = HashMap::new();

    cells.insert((5, 2), Wall);
    cells.insert((4, 3), Wall);
    cells.insert((6, 3), Wall);

    cells.insert((4, 6), Wall);
    cells.insert((5, 7), Wall);

    cells.insert((10, 7), Wall);
    cells.insert((11, 6), Wall);

    cells.insert((11, 3), Wall);
    cells.insert((10, 2), Wall);

    let player_pos = (5, 3);

    State {
        player_pos: player_pos,
        initial_player_pos: player_pos,
        cells: cells,
        rng: rng,
        title_screen: true,
        frame_count: 0,
        motion: Stopped,
        max_steps: 4,
    }
}

const START_POS: (i32, i32) = (7, 3);

#[no_mangle]
//returns true if quit requested
pub fn update_and_render(platform: &Platform, state: &mut State, events: &mut Vec<Event>) -> bool {
    state.frame_count = state.frame_count.overflowing_add(1).0;

    if state.title_screen {
        for event in events {
            cross_mode_event_handling(platform, state, event);

            match *event {
                Event::Close |
                Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => return true,
                _ => (),
            }
        }

        if state.player_pos == START_POS {
            *state = next_level((platform.size)(), state.rng, state.max_steps);
        } else {
            move_player((platform.size)(), state);
        }

        print_tuple(platform, START_POS, goal_string(state.frame_count));

        draw(platform, state);

        draw_button(platform,
                    5,
                    9,
                    3,
                    3,
                    "↑",
                    (platform.key_pressed)(KeyCode::Up));
        draw_button(platform,
                    2,
                    12,
                    3,
                    3,
                    "←",
                    (platform.key_pressed)(KeyCode::Left));
        draw_button(platform,
                    5,
                    12,
                    3,
                    3,
                    "↓",
                    (platform.key_pressed)(KeyCode::Down));
        draw_button(platform,
                    8,
                    12,
                    3,
                    3,
                    "→",
                    (platform.key_pressed)(KeyCode::Right));

        draw_button(platform,
                    12,
                    12,
                    3,
                    3,
                    "R",
                    (platform.key_pressed)(KeyCode::R));

        false
    } else {
        game_update_and_render(platform, state, events)
    }
}

fn draw_button(platform: &Platform, x: i32, y: i32, w: i32, h: i32, label: &'static str, pressed: bool) {

    if pressed {
        draw_pressed_button_rect(platform, x, y, w, h);
    } else {
        draw_unpressed_button_rect(platform, x, y, w, h);
    }

    (platform.print_xy)(x + 1, y + 1, label);
}

pub fn game_update_and_render(platform: &Platform,
                              state: &mut State,
                              events: &mut Vec<Event>)
                              -> bool {
    for event in events {
        cross_mode_event_handling(platform, state, event);

        match *event {
            Event::Close |
            Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => return true,
            _ => (),
        }
    }

    move_player((platform.size)(), state);

    if let Some(&Goal) = state.cells.get(&state.player_pos) {
        state.max_steps += 1;
        *state = next_level((platform.size)(), state.rng, state.max_steps);
    }

    draw(platform, state);

    false
}

fn move_player(size: Size, state: &mut State) {
    match state.motion {
        Stopped => {}
        dir => {
            let target = add(state.player_pos, dir_to_tuple(dir));
            if can_go(size, &state.cells, target) {
                state.player_pos = target;
            } else {
                state.motion = Stopped;
            }
        }
    }
}

fn cross_mode_event_handling(platform: &Platform, state: &mut State, event: &Event) {
    match *event {
        Event::KeyPressed { key: KeyCode::W, ctrl: _, shift: _ } |
        Event::KeyPressed { key: KeyCode::Up, ctrl: _, shift: _ } => {
            if state.motion == Stopped {
                state.motion = Up;
            }
        }
        Event::KeyPressed { key: KeyCode::D, ctrl: _, shift: _ } |
        Event::KeyPressed { key: KeyCode::Right, ctrl: _, shift: _ } => {
            if state.motion == Stopped {
                state.motion = Right;
            }
        }
        Event::KeyPressed { key: KeyCode::S, ctrl: _, shift: _ } |
        Event::KeyPressed { key: KeyCode::Down, ctrl: _, shift: _ } => {
            if state.motion == Stopped {
                state.motion = Down;
            }
        }
        Event::KeyPressed { key: KeyCode::A, ctrl: _, shift: _ } |
        Event::KeyPressed { key: KeyCode::Left, ctrl: _, shift: _ } => {
            if state.motion == Stopped {
                state.motion = Left;
            }
        }
        Event::KeyPressed { key: KeyCode::R, ctrl: false, shift: _ } => {
            println!("reset level");
            state.player_pos = state.initial_player_pos;
        }
        Event::KeyPressed { key: KeyCode::R, ctrl: true, shift: _ } => {
            println!("reset");
            *state = new_state((platform.size)());
        }
        _ => (),
    }
}

fn can_go(size: Size, cells: &Cells, (x, y): (i32, i32)) -> bool {
    if x >= 0 && y >= 0 && x < size.width && y < size.height {

        match cells.get(&(x, y)) {
            None => true,
            Some(&Goal) => true,
            Some(&Wall) => false,
        }
    } else {
        false
    }
}

fn goal_string(frame_count: u32) -> &'static str {
    match frame_count & 31 {
        1 => "\u{E010}",
        2 => "\u{E011}",
        3 => "\u{E011}",
        4 => "\u{E012}",
        5 => "\u{E012}",
        6 => "\u{E013}",
        7 => "\u{E013}",
        8 => "\u{E014}",
        9 => "\u{E014}",
        10 => "\u{E015}",
        11 => "\u{E015}",
        12 => "\u{E016}",
        13 => "\u{E016}",
        14 => "\u{E017}",
        15 => "\u{E017}",
        16 => "\u{E018}",
        17 => "\u{E017}",
        18 => "\u{E017}",
        19 => "\u{E016}",
        20 => "\u{E016}",
        21 => "\u{E015}",
        22 => "\u{E015}",
        23 => "\u{E014}",
        24 => "\u{E014}",
        25 => "\u{E013}",
        26 => "\u{E013}",
        27 => "\u{E012}",
        28 => "\u{E012}",
        29 => "\u{E011}",
        30 => "\u{E011}",
        31 => "\u{E010}",
        _ => "\u{E010}",
    }
}

fn print_tuple(platform: &Platform, (x, y): (i32, i32), text: &'static str) {
    if x >= 0 && y >= 0 {
        (platform.print_xy)(x, y, text);
    }
}

macro_rules! with_layer {
    ($platform:expr, $layer:expr, $code:block) => {
        {
            let current = ($platform.get_layer)();
            ($platform.set_layer)($layer);

            $code

            ($platform.set_layer)(current);
        }
    }
}

fn draw(platform: &Platform, state: &State) {
    for (&coords, &cell) in state.cells.iter() {
        print_cell(platform, coords, cell, state.frame_count);
    }

    print_tuple(platform, state.initial_player_pos, "☐");

    with_layer!(platform, 1, {
        print_tuple(platform, state.player_pos, "@");
    })

}

fn draw_unpressed_button_rect(platform: &Platform, x: i32, y: i32, w: i32, h: i32) {
    draw_rect_with(platform,
                   x,
                   y,
                   w,
                   h,
                   ["┌", "─", "╖", "│", "║", "╘", "═", "╝"]);
}

fn draw_pressed_button_rect(platform: &Platform, x: i32, y: i32, w: i32, h: i32) {
    draw_rect_with(platform,
                   x,
                   y,
                   w,
                   h,
                   ["╔", "═", "╕", "║", "│", "╙", "─", "┘"]);
}

fn draw_rect_with(platform: &Platform, x: i32, y: i32, w: i32, h: i32, edges: [&'static str; 8]) {
    (platform.clear)(Some(Rect::from_values(x, y, w, h)));

    let right = x + w - 1;
    let bottom = y + h - 1;
    // top
    (platform.print_xy)(x, y, edges[0]);
    for i in (x + 1)..right {
        (platform.print_xy)(i, y, edges[1]);
    }
    (platform.print_xy)(right, y, edges[2]);

    // sides
    for i in (y + 1)..bottom {
        (platform.print_xy)(x, i, edges[3]);
        (platform.print_xy)(right, i, edges[4]);
    }

    //bottom
    (platform.print_xy)(x, bottom, edges[5]);
    for i in (x + 1)..right {
        (platform.print_xy)(i, bottom, edges[6]);
    }
    (platform.print_xy)(right, bottom, edges[7]);
}

fn print_cell(platform: &Platform, coords: (i32, i32), cell: Cell, frame_count: u32) {
    match cell {
        Goal => print_tuple(platform, coords, goal_string(frame_count)),
        _ => print_tuple(platform, coords, cell.to_static_str()),
    }
    // with_layer!(platform, CELL_LAYER, {
    // })
}

fn next_level(size: Size, mut rng: StdRng, max_steps: u8) -> State {
    let mut cells = HashMap::new();

    for y in 0..size.height {
        for x in 0..size.width {
            if rng.next_f32() > 0.9 {
                cells.insert((x, y), Wall);
            }
        }
    }

    let mut player_pos = gen_coord(size, &mut rng);

    if let Some(_) = cells.get(&player_pos) {
        let first_player_pos = player_pos;
        while let Some(_) = cells.get(&player_pos) {
            player_pos = next_coord(size, player_pos);

            if player_pos == first_player_pos {
                cells.remove(&player_pos);
            }
        }
    }

    let mut counts: HashMap<(i32, i32), u32> = HashMap::new();

    for dirs in DirsIter::new(max_steps) {
        let mut current_pos = player_pos;

        for &dir in dirs.iter() {
            loop {
                let target = add(current_pos, dir_to_tuple(dir));
                if can_go(size, &cells, target) {
                    current_pos = target;
                    increment_count(&mut counts, current_pos)
                } else {
                    break;
                }
            }
        }
    }

    let mut non_zero_minimum_count = std::u32::MAX;

    for &v in counts.values() {
        if v != 0 && v < non_zero_minimum_count {
            non_zero_minimum_count = v;
        }
    }

    //we do the sort so that the rng seed determines the puzzle,
    //not the hash ordering
    let mut goal_locations: Vec<(i32, i32)> = counts.iter()
        .filter(|&(_, &v)| v == non_zero_minimum_count)
        .map(|(&coord, _)| coord)
        .collect();

    goal_locations.sort_by_key(|&(coord, _)| coord);

    let mut len = goal_locations.len();
    if len > 0 {
        loop {
            let possible_goal = goal_locations.swap_remove(rng.gen_range(0, len));

            len = goal_locations.len();
            if not_on_edge(size, possible_goal) || len == 0 {
                cells.insert(possible_goal, Goal);
                break;
            }
        }
    } else {
        cells.insert(player_pos, Goal);
    }

    State {
        player_pos: player_pos,
        initial_player_pos: player_pos,
        cells: cells,
        rng: rng,
        title_screen: false,
        frame_count: 0,
        motion: Stopped,
        max_steps: max_steps,
    }
}

fn not_on_edge(size: Size, (x, y): (i32, i32)) -> bool {
    x != 0 && y != 0 && x != size.width - 1 && y != size.height - 1
}

struct DirsIter {
    index: u16,
    started: bool,
    max: u8,
    max_index: u16,
}

impl DirsIter {
    //if max_+index is not of the form (2 ^ 2n) - 1
    //certain directions will be favoured over others
    fn new(max: u8) -> Self {
        let max_index = if max < 8 {
            (1 << (2 * max)) - 1
        } else {
            std::u16::MAX
        };

        DirsIter {
            index: std::u16::MAX,
            started: false,
            max: max,
            max_index: max_index,
        }
    }
}

impl Iterator for DirsIter {
    type Item = Vec<Motion>;

    fn next(&mut self) -> Option<Vec<Motion>> {
        if self.started && (self.index == std::u16::MAX || self.index >= self.max_index) {
            None
        } else {
            self.started = true;
            self.index = self.index.overflowing_add(1).0;

            let mut result = Vec::new();

            for &mask in TwoBits::all_values().iter() {
                if result.len() as u8 >= self.max {
                    break;
                } else {
                    match extract_dir(mask, self.index) {
                        Stopped => {}
                        dir => result.push(dir),
                    }
                }
            }

            Some(result)
        }
    }
}

#[derive(Copy, Clone)]
enum TwoBits {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}
use TwoBits::*;

pub trait AllValues {
    fn all_values() -> Vec<Self> where Self: std::marker::Sized;
}

impl AllValues for TwoBits {
    fn all_values() -> Vec<TwoBits> {
        vec![First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth]
    }
}

fn extract_dir(mask: TwoBits, index: u16) -> Motion {
    let bits = match mask {
        First => index & 0b11,
        Second => (index & 0b1100) >> 2,
        Third => (index & 0b110000) >> 4,
        Fourth => (index & 0b11000000) >> 6,
        Fifth => (index & 0b1100000000) >> 8,
        Sixth => (index & 0b110000000000) >> 10,
        Seventh => (index & 0b11000000000000) >> 12,
        Eighth => (index & 0b1100000000000000) >> 14,
    };

    match bits {
        0b00 => Up,
        0b01 => Right,
        0b10 => Down,
        0b11 => Left,
        _ => Stopped,
    }
}

fn dir_to_tuple(dir: Motion) -> (i32, i32) {
    match dir {
        Up => (0, -1),
        Right => (1, 0),
        Down => (0, 1),
        Left => (-1, 0),
        Stopped => (0, 0),
    }
}



fn gen_coord(size: Size, rng: &mut StdRng) -> (i32, i32) {
    (rng.gen_range::<i32>(0, size.width), rng.gen_range::<i32>(0, size.height))
}

fn next_coord(size: Size, (x, y): (i32, i32)) -> (i32, i32) {
    debug_assert!(x >= 0 && y >= 0, "bad coord: ({}, {})", x, y);

    if x + 1 >= size.width {
        if y + 1 >= size.height {
            (0, 0)
        } else {
            (0, y + 1)
        }
    } else {
        (x + 1, y)
    }

}

use std::ops::Add;
fn add<T: Add<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 + x2, y1 + y2)
}

use std::ops::Sub;
fn sub<T: Sub<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 - x2, y1 - y2)
}

fn increment_count(counts: &mut HashMap<(i32, i32), u32>, key: (i32, i32)) {
    let count = counts.entry(key).or_insert(0);
    *count = count.saturating_add(1);
}
