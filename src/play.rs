use crate::dragster::*;

mod game;
use game::DragsterGame;

use std::sync::{ Arc, Mutex };
use threadpool::Builder;
use rand::{ thread_rng, Rng };
use ggez::event;
use ggez::conf;
use ggez::ContextBuilder;

const WINDOW_SCALING: f32 = 5.0;
const WINDOW_WIDTH: f32 = 192.0 * WINDOW_SCALING; 
const WINDOW_HEIGHT: f32 = 160.0 * WINDOW_SCALING;

fn spawn_rng_player() -> Time {
    let mut state = GameState::default();
            
    loop {
        let clutch = thread_rng().gen_bool(INPUT_PROBABILITY);
        let shift = thread_rng().gen_bool(INPUT_PROBABILITY);
        state.game_step(clutch, shift);
        if state.is_arrived() || state.blown { break }
    }

    if state.blown {
        Time::INFINITY
    } else {
        state.state_timer()
    }
}

pub fn rng_player() {
    let pool = Builder::new().build();
    let best_time = Arc::new(Mutex::new(Time::INFINITY));
    let attempts = Arc::new(Mutex::new(0));

    loop {
        let best_time = best_time.clone();
        let attempts = attempts.clone();

        pool.execute(move || {
            let time = spawn_rng_player();
            let mut best_time = best_time.lock().unwrap();
            let mut attempts = attempts.lock().unwrap();
            if &time < &best_time {
                *best_time = time;
                println!("{} | {:?}", best_time, attempts);
            }
            *attempts += 1;
        });
    }
}

pub fn human_play() {
    let (ctx, event_loop) = ContextBuilder::new("Dragster", "SpeckyYT")
        .window_mode(conf::WindowMode {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            min_width: WINDOW_WIDTH,
            min_height: WINDOW_HEIGHT,
            max_width: WINDOW_WIDTH,
            max_height: WINDOW_HEIGHT,
            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            resizable: false,
            visible: true,
            resize_on_scale_factor_change: false
        })
        .window_setup(conf::WindowSetup::default().title("Dragster"))
        .build()
        .expect("Failed to initialize ggez");

    event::run(ctx, event_loop, DragsterGame::new().unwrap())
}
