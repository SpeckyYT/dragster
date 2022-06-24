// https://github.com/esnard/dragster/blob/master/dragster.c

use std::sync::{ Arc, Mutex };
use threadpool::Builder;

mod dragster;
use dragster::*;

mod play;
use play::*;

fn main() {
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
