// https://github.com/esnard/dragster/blob/master/dragster.c

use std::hash::{ Hash, Hasher };
use std::collections::HashSet;

type Int = i128;
type Time = f32;
type Inputs = [ Input; MAX_FRAMES as usize + 1 ];

const MAX_FRAMES: Int = 167; // 167 is sufficient but let's do 200 for safety

const INITIAL_GEAR: Int = 0;
const INITIAL_SPEED: Int = 0;

const MIN_WINNING_DISTANCE: Int = 97 * 256;

const MAX_TACHOMETER: Int = 32;
const MAX_FRAME_COUNTER: Int = 16;
const MAX_GEAR: Int = 4;
const MAX_SPEED: Int = 256;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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
    fn default() -> Self {
        Input::new(false, false)
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct GameState {
    timer: Int,
    frame_counter: Int,
    tachometer: Int,
    tachometer_diff: Int,
    distance: Int,
    speed: Int,
    gear: Int,
    initial_tachometer: Int,
    initial_frame_counter: Int,
    inputs: Inputs,
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inputs[self.timer as usize - 1].shift.hash(state);
        self.gear.hash(state);
        self.speed.hash(state);
        self.tachometer.hash(state);
        self.tachometer_diff.hash(state);
    }
}

impl GameState {
    fn new(tachometer: Int, frame_counter: Int, clutch: bool, shift: bool) -> Self {
        let mut state = Self {
            timer: 1,
            frame_counter: frame_counter,
            tachometer: tachometer,
            tachometer_diff: 0,
            distance: 0,
            speed: INITIAL_SPEED,
            gear: INITIAL_GEAR,
            initial_tachometer: tachometer,
            initial_frame_counter: frame_counter,
            inputs: [ Input::default(); MAX_FRAMES as usize + 1 ],
        };
        state.inputs[0] = Input::new(clutch, shift);
        state
    }
    fn default() -> Self {
        Self::new(0, 0, false, false)
    }
    fn state_timer(&self) -> Time {
        (self.timer as Time * 3.34).trunc() / 100.0
    }
    fn game_step(&mut self, clutch: bool, shift: bool) {
        if self.timer >= MAX_FRAMES { return }
        self.inputs[self.timer as usize] = Input::new(clutch, shift);
        self.timer += 1;
        self.frame_counter = (self.frame_counter + 2) % MAX_FRAME_COUNTER;

        // Update gear and tachometer.
        let gear_value: Int = if self.gear > 2 { (2 as Int).pow(self.gear as u32 - 1) } else { 1 };

        if self.inputs[self.timer as usize - 2].shift {
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

        // Compute the speed limit.
        let speed_limit: Int = self.tachometer * gear_value
            + if self.tachometer >= 20 && self.gear > 2 { (2 as Int).pow(self.gear as u32 - 2) } else { 0 };

        // Update tachometer difference, which post_tachometer - tachometer.
        if self.inputs[self.timer as usize - 2].shift {
            self.tachometer_diff = 0;
        } else {
            self.tachometer_diff = if speed_limit - self.speed >= 16 { 1 } else { 0 }
        }

        // Update speed
        if self.gear > 0 && self.inputs[self.timer as usize - 1].shift {
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
            initial_tachometer: self.initial_tachometer,
            initial_frame_counter: self.initial_frame_counter,
            inputs: self.inputs,
        };

        for frame in 0..=MAX_FRAMES as usize {
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
}

fn main() {
    let frame_counter = 0;
    let tachometer = 0;
    let clutch = false;
    let shift = false;

    let mut best_state = GameState::new(tachometer, frame_counter, clutch, shift);
    best_state.timer = MAX_FRAMES;
    best_state.distance = 0;

    let mut states: HashSet<GameState> = HashSet::new();
    let mut next_states: HashSet<GameState> = HashSet::new();

    let mut total_simulations: u128 = 0;

    for frame_counter in (0..MAX_FRAME_COUNTER).step_by(2) {
        states = HashSet::new();
        next_states = HashSet::new();

        println!("Now testing all configurations with an initial frame counter equal to {}.", frame_counter);

        // Generating initial states, based on OmniGamer's model.
        for tachometer in (0..MAX_TACHOMETER).step_by(3) {
            for clutch in 0..=1 {
                for shift in 0..=1 {
                    let initial_state = GameState::new(tachometer, frame_counter, clutch == 1, shift == 1);
                    states.insert(initial_state);
                }
            }
        }
    }

    let mut stop_configuration = false;

    /*
        This is the main loop: we generate all possible states from previous
        generated ones, dropping those who won't be able to finish, and using
        deduplication to greatly reduce the search space.
    */
    for frame in 1..=MAX_FRAMES {
        if stop_configuration { break }

        for current_state in states.iter() {
            if current_state.timer == frame {
                for clutch in 0..=1 {
                    for shift in 0..=1 {
                        let mut next_state = current_state.clone();
                        next_state.game_step(clutch == 1, shift == 1);
                        total_simulations += 1;

                        /*
                        * Dropping states which can't win anything.
                        *
                        * Todo: use bestState to detect which states won't
                        * be better than the best computed frame.
                        */
                        if next_state.tachometer < MAX_TACHOMETER && next_state.distance + MAX_SPEED * (MAX_FRAMES - frame) >= MIN_WINNING_DISTANCE {
                            if next_state.distance >= MIN_WINNING_DISTANCE {
                                if next_state.timer < best_state.timer || next_state.timer == best_state.timer && next_state.distance > best_state.distance {
                                    best_state = next_state;
                                }
                                stop_configuration = true;
                            }

                            /*
                            * If a state collision occurs, it's safe to
                            * keep the one which has the greatest distance.
                            */
                            if next_state.distance >= next_states.get(&next_state).unwrap_or(&GameState::default()).distance {
                                next_states.insert(next_state);
                            }
                        }
                    }
                }
            }
        }

        states = next_states.clone();
    }

    println!();

    if 0 == best_state.distance {
        println!("It's not possible to do the race under {}s.", best_state.state_timer());
        println!("{} simulations were performed.", total_simulations);

        best_state.debug_state(true);
    }
}
