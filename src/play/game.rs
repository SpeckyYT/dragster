use crate::*;
use std::time::Instant;
use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard;

const DELTA_PLAY_TIME: f64 = 1.0 / 30.0;
const DELTA_COUNTDOWN_TIME: f64 = 1.0 / 5.0;
const DEFAULT_COUNTDOWN: u8 = 10;
const DEBUG: bool = true;

pub struct DragsterGame {
    best_time: Time,
    delta_time: Instant,
    game_state: GameState,
    playing: bool,
    countdown: u8,
    start_countdown: bool,
}

impl DragsterGame {
    pub fn new() -> GameResult<Self> {
        Ok(Self {
            best_time: Time::INFINITY,
            game_state: GameState::default(),
            delta_time: Instant::now(),
            playing: true,
            countdown: DEFAULT_COUNTDOWN,
            start_countdown: false,
        })
    }
    fn check_delta(&mut self, delta: f64) -> bool {
        if self.delta_time.elapsed().as_secs_f64() >= delta {
            self.delta_time = self.delta_time + std::time::Duration::from_secs_f64(delta);
            return true
        }
        return false
    }
}

impl EventHandler for DragsterGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.playing {
            if self.check_delta(DELTA_PLAY_TIME) {
                let clutch = keyboard::is_key_pressed(ctx, keyboard::KeyCode::Left);
                let shift = keyboard::is_key_pressed(ctx, keyboard::KeyCode::Right);
                self.game_state.game_step(clutch, shift);

                if self.game_state.is_arrived() || self.game_state.blown {
                    self.playing = false;
                    if self.game_state.state_timer() < self.best_time && !self.game_state.blown {
                        self.best_time = self.game_state.state_timer();
                    }
                }
            }
        } else {
            if !self.start_countdown {
                self.start_countdown = keyboard::is_key_pressed(ctx, keyboard::KeyCode::Space);
            }

            if self.check_delta(DELTA_COUNTDOWN_TIME) {
                if self.countdown >= DEFAULT_COUNTDOWN {
                    if self.start_countdown {
                        self.countdown = DEFAULT_COUNTDOWN - 1;
                        self.start_countdown = false;
                    }
                } else if self.countdown > 0 {
                    self.countdown -= 1;
                } else {
                    self.playing = true;
                    self.game_state = GameState::new();
                    self.countdown = DEFAULT_COUNTDOWN;
                }
            }
        }
        
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));

        if DEBUG {
            let text = graphics::Text::new([
                "Debug Mode".to_string(),
                format!("Distance: {:?}", self.game_state.distance),
                format!("Tachometer: {:?}", self.game_state.tachometer),
                format!("Tachometer Diff: {:?}", self.game_state.tachometer_diff),
                format!("Speed: {:?}", self.game_state.speed),
                format!("Gear: {:?}", self.game_state.gear),
                format!("State Timer: {:?}", self.game_state.state_timer()),
                format!("Blown: {:?}", self.game_state.blown),
                format!("Best Time: {:?}", self.best_time),
                format!("Countdown: {:?}", self.countdown),
                format!("Playing: {:?}", self.playing),
            ].join("\n"));
            graphics::draw(ctx, &text, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)
    }
}
