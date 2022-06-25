// https://github.com/esnard/dragster/blob/master/dragster.c

mod dragster;
use dragster::*;

mod play;
use play::*;

const HUMAN: bool = true;

fn main() {
    match HUMAN {
        true => human_play(),
        false => rng_player(),
    }
}
