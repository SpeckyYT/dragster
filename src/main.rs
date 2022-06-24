// https://github.com/esnard/dragster/blob/master/dragster.c

use rand::{ thread_rng, Rng };
use std::sync::{ Arc, Mutex };
use threadpool::Builder;

type Int = i128;
type Time = f32;
type Inputs = Vec<Input>;

const INPUT_PROBABILITY: f64 = 0.5;
// const MAX_FRAMES: Int = 167; // 167 is sufficient but let's do 200 for safety

const INITIAL_GEAR: Int = 0;
const INITIAL_SPEED: Int = 0;

const MIN_WINNING_DISTANCE: Int = 97 * 256;

const MAX_TACHOMETER: Int = 32;
const MAX_FRAME_COUNTER: Int = 16;
const MAX_GEAR: Int = 4;
// const MAX_SPEED: Int = 256;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Input {
    clutch: bool,
    shift: bool,
}

impl Input {
    fn new(clutch: bool, shift: bool) -> Self {
        Input {
            clutch,
            shift,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct GameState {
    timer: Int,
    frame_counter: Int,
    tachometer: Int,
    tachometer_diff: Int,
    distance: Int,
    speed: Int,
    gear: Int,
    blown: bool,
    initial_tachometer: Int,
    initial_frame_counter: Int,
    inputs: Inputs,
}

impl GameState {
    fn new() -> Self {
        Self {
            timer: 0,
            frame_counter: 0,
            tachometer: 0,
            tachometer_diff: 0,
            distance: 0,
            speed: INITIAL_SPEED,
            gear: INITIAL_GEAR,
            blown: false,
            initial_tachometer: 0,
            initial_frame_counter: 0,
            inputs: vec![],
        }
    }
    fn default() -> Self {
        Self::new()
    }
    fn state_timer(&self) -> Time {
        (self.timer as Time * 3.34).trunc() / 100.0
    }
    fn game_step(&mut self, clutch: bool, shift: bool) {
        self.inputs.push(Input::new(clutch, shift));
        self.timer += 1;
        self.frame_counter = (self.frame_counter + 2) % MAX_FRAME_COUNTER;

        // Update gear and tachometer.
        let gear_value: Int = if self.gear > 2 { (2 as Int).pow(self.gear as u32 - 1) } else { 1 };

        if self.inputs[self.inputs.len() - 1].shift {
            self.gear = if self.gear >= MAX_GEAR { MAX_GEAR } else { self.gear + 1 };
            self.tachometer -= self.tachometer_diff + if !clutch { -3 } else { 3 };
        } else {
            if self.frame_counter % gear_value == 0 {
                self.tachometer -= self.tachometer_diff + if clutch { -1 } else { 1 };
            } else {
                self.tachometer -= self.tachometer_diff
            }
        }

        self.tachometer = self.tachometer.max(0);

        if self.tachometer >= MAX_TACHOMETER {
            self.blown = true
        }

        // Compute the speed limit.
        let speed_limit: Int = self.tachometer * gear_value
            + if self.tachometer >= 20 && self.gear > 2 { (2 as Int).pow(self.gear as u32 - 2) } else { 0 };

        // Update tachometer difference, which post_tachometer - tachometer.
        if self.inputs[self.inputs.len() - 1].shift {
            self.tachometer_diff = 0;
        } else {
            self.tachometer_diff = if speed_limit - self.speed >= 16 { 1 } else { 0 }
        }

        // Update speed
        if self.gear > 0 && self.inputs[self.inputs.len() - 1].shift {
            if self.speed > speed_limit {
                self.speed -= 1;
            } else if self.speed < speed_limit {
                self.speed += 2;
            }
        }

        // Update distance
        self.distance += self.speed;
    }
    fn debug_state(&self, mode: bool) {
        let mut debug = GameState {
            timer: self.timer,
            frame_counter: self.frame_counter,
            tachometer: self.tachometer,
            tachometer_diff: self.tachometer_diff,
            distance: self.distance,
            speed: self.speed,
            gear: self.gear,
            blown: self.blown,
            initial_tachometer: self.initial_tachometer,
            initial_frame_counter: self.initial_frame_counter,
            inputs: self.inputs.clone(),
        };

        for frame in 0..self.inputs.len() {
            let clutch = self.inputs[frame].clutch;
            let shift = self.inputs[frame].shift;

            if frame > 0 {
                debug.game_step(clutch, shift)
            }

            if mode {
                // printf("%d: %d,%d | %d - %d - %d - %d - %d\n", frame, clutch, shift, debug_state.gear, debug_state.speed, debug_state.tachometer, debug_state.tachometer_diff, debug_state.distance);
                println!("{}: {:?} | {:?} - {:?} - {:?} - {:?} - {:?} - {:?}", frame, clutch, shift, debug.gear, debug.speed, debug.tachometer, debug.tachometer_diff, debug.distance);
            } else {
                println!("{}\t{}", shift, clutch);
            }
        }

        println!("Initial frame_counter: {}", self.initial_frame_counter);
        println!("Initial tachometer: {}", self.initial_tachometer);
    }
    fn is_arrived(&self) -> bool {
        self.distance >= MIN_WINNING_DISTANCE
    }
}

fn spawn_game() -> Time {
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

fn main() {
    let pool = Builder::new().build();
    let best_time = Arc::new(Mutex::new(Time::INFINITY));
    let attempts = Arc::new(Mutex::new(0));

    loop {
        let best_time = best_time.clone();
        let attempts = attempts.clone();

        pool.execute(move || {
            let time = spawn_game();
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
