// https://github.com/esnard/dragster/blob/master/dragster.c

use std::collections::HashSet;

type Int = i32;
type Input = u8;
type Time = f32;

const MAX_FRAMES: Int = 167;

const INITIAL_GEAR: Input = 0;
// const INITIAL_SPEED: Int = 0; // lmao

const INPUT_CLUTCH: Input = 1;
const INPUT_SHIFT: Input = 2;

const MIN_WINNING_DISTANCE: Int = 97 * 256;

const MAX_TACHOMETER: Int = 32;
const MAX_FRAME_COUNTER: Int = 16;
const MAX_GEAR: Input = 4;
const MAX_SPEED: Int = 256;

const MAX_STATES: Int = MAX_TACHOMETER * MAX_SPEED * (MAX_GEAR as Int + 1) * 2 * 2;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct GameState {
    timer: Int,
    frame_counter: Int,
    tachometer: Int,
    tachometer_diff: Int,
    distance: Int,
    speed: Int,
    gear: Input,
    initial_tachometer: Int,
    initial_frame_counter: Int,
    inputs: [Input; MAX_FRAMES as usize + 1],
}

impl GameState {
    fn new(tachometer: Int, frame_counter: Int, clutch: Input, shift: Input) -> GameState {
        let mut state = GameState {
            timer: 1,
            frame_counter: frame_counter,
            tachometer: tachometer,
            tachometer_diff: 0,
            distance: 0,
            speed: 0,
            gear: INITIAL_GEAR,
            initial_tachometer: tachometer,
            initial_frame_counter: frame_counter,
            inputs: [0; MAX_FRAMES as usize + 1 ],
        };
        state.inputs[0] = (clutch * INPUT_CLUTCH) as Input | (shift * INPUT_SHIFT) as Input;
        state
    }
    fn state_timer(&self) -> Time {
        (self.timer as Time * 3.34).trunc() / 100.0
    }
    /* not needed
    fn hash_state(&self) -> Int {
        return
            if self.inputs[self.timer as usize - 1] & INPUT_SHIFT != 0 { 1 } else { 0 }
            + 2 * self.gear
            + 2 * (MAX_GEAR + 1) * self.speed
            + 2 * (MAX_GEAR + 1) * MAX_GEAR * self.tachometer
            + 2 * (MAX_GEAR + 1) * MAX_GEAR * MAX_TACHOMETER * self.tachometer_diff
    }
    */
    fn game_step(&mut self, clutch: Input, shift: Input) {
        self.inputs[self.timer as usize] = (clutch * INPUT_CLUTCH) | (shift * INPUT_SHIFT);
        self.timer += 1;
        self.frame_counter = (self.frame_counter + 2) % MAX_FRAME_COUNTER;

        // Update gear and tachometer.
        if self.inputs[self.timer as usize - 2] & INPUT_SHIFT != 0 {
            self.gear = if self.gear >= MAX_GEAR { MAX_GEAR } else { self.gear + 1 };
            self.tachometer -= self.tachometer_diff + if clutch != 0 { -3 } else { 3 };
        } else {
            if self.frame_counter % 2_i32.pow(self.gear as u32) == 0 {
                self.tachometer -= self.tachometer_diff + if clutch > 0 { -1 } else { 1 };
            } else {
                self.tachometer -= self.tachometer_diff
            }
        }

        self.tachometer = self.tachometer.max(0);

        // Compute the speed limit.
        let speed_limit: Int = self.tachometer * 2_i32.pow(self.gear as u32 - 1)
            + if self.tachometer >= 20 && self.gear > 0 { 2_i32.pow(self.gear as u32 - 2) } else { 0 };

        // Update tachometer difference, which post_tachometer - tachometer.
        if self.inputs[self.timer as usize - 2] & INPUT_SHIFT != 0 {
            self.tachometer_diff = 0;
        } else {
            self.tachometer_diff = if speed_limit - self.speed >= 16 { 1 } else { 0 }
        }

        // Update speed
        if self.gear > 0 && self.inputs[self.timer as usize - 1] & INPUT_SHIFT != 0 {
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
            let clutch = (self.inputs[frame] & INPUT_CLUTCH != 0) as Input;
            let shift = (self.inputs[frame] & INPUT_SHIFT != 0) as Input;

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
    let clutch = 0;
    let shift = 0;

    let mut best_state = GameState::new(tachometer, frame_counter, clutch, shift);
    best_state.timer = MAX_FRAMES;
    best_state.distance = 0;

    let mut states: HashSet<GameState> = HashSet::new();
    let current_state = GameState::new(tachometer, frame_counter, clutch, shift);
    let mut total_simulations: u128 = 0;

    for frame_counter in (0..MAX_FRAME_COUNTER).step_by(2) {
        // let state = GameState::new(tachometer, frame_counter, clutch, shift);
        // let next_state = GameState::new(tachometer, frame_counter, clutch, shift);

        println!("Now testing all configurations with an initial frame counter equal to {}.", frame_counter);

        // Generating initial states, based on OmniGamer's model.
        for tachometer in (0..MAX_TACHOMETER).step_by(3) {
            for clutch in 0..=1 {
                for shift in 0..=1 {
                    let initial_state = GameState::new(tachometer, frame_counter, clutch, shift);
                    states.insert(initial_state);
                }
            }
        }
    }

    let mut next_state = GameState::new(tachometer, frame_counter, clutch, shift);
    let mut stop_configuration = false;

    /*
        This is the main loop: we generate all possible states from previous
        generated ones, dropping those who won't be able to finish, and using
        deduplication to greatly reduce the search space.
    */
    for frame in 1..=MAX_FRAMES {
        if stop_configuration { break }

        for _index in 0..MAX_STATES {
            if current_state.timer == frame {
                for clutch in 0..=1 {
                    for shift in 0..=1 {
                        states.insert(next_state);
                        next_state.game_step(clutch, shift);
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
                        }
                    }
                }
            }

            // current_state += 1; // (???????)
        }
        // memcpy(states, next_states, MAX_STATES * sizeof(GameState));
        // bzero(next_states, MAX_STATES * sizeof(GameState));
        // ??????
    }

    println!();

    if 0 == best_state.distance {
        println!("It's not possible to do the race under {}s.", best_state.state_timer());
        println!("{} simulations were performed.", total_simulations);

        best_state.debug_state(false);
    }
}
