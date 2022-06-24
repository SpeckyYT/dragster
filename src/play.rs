use crate::dragster::*;

use rand::{ thread_rng, Rng };

pub fn spawn_rng_player() -> Time {
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
