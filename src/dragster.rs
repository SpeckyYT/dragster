pub type Int = i128;
pub type Time = f32;
pub type Inputs = Vec<Input>;

pub const INPUT_PROBABILITY: f64 = 0.5;

pub const INITIAL_GEAR: Int = 0;
pub const INITIAL_SPEED: Int = 0;

pub const MIN_WINNING_DISTANCE: Int = 97 * 256;

pub const MAX_TACHOMETER: Int = 32;
pub const MAX_FRAME_COUNTER: Int = 16;
pub const MAX_GEAR: Int = 4;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Input {
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
pub struct GameState {
    pub timer: Int,
    pub frame_counter: Int,
    pub tachometer: Int,
    pub tachometer_diff: Int,
    pub distance: Int,
    pub speed: Int,
    pub gear: Int,
    pub blown: bool,
    pub initial_tachometer: Int,
    pub initial_frame_counter: Int,
    pub inputs: Inputs,
}

impl GameState {
    pub fn new() -> Self {
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
    pub fn default() -> Self {
        Self::new()
    }
    pub fn state_timer(&self) -> Time {
        (self.timer as Time * 3.34).trunc() / 100.0
    }
    pub fn game_step(&mut self, clutch: bool, shift: bool) {
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
    pub fn debug_state(&self, mode: bool) {
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
    pub fn is_arrived(&self) -> bool {
        self.distance >= MIN_WINNING_DISTANCE
    }
}
