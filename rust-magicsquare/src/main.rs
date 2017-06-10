extern crate rand;
#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate num_cpus;
extern crate chrono;

use rand::Rng;
use chan_signal::Signal;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
use chrono::prelude::*;


fn main() {
    println!("Magic square!");

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (tx, rx) = chan::sync(0);
    let tries = Arc::new(AtomicUsize::new(0));
    let now = SystemTime::now();

    let cpu_count = num_cpus::get();
    let current_time = Local::now();
    println!("Started {} using {} cores", current_time, cpu_count);
    for _ in 0..cpu_count {
        let (tries, tx) = (tries.clone(), tx.clone());
        thread::spawn(move || {
            find_square(tries);
            tx.send(());
        });
    }

    let print_state = || {
        let count = tries.load(Ordering::SeqCst);
        println!("Tries: {}", count);
        let time = get_time(now.elapsed().unwrap());
        println!("Time: {:.2}", time);
        let performance = count as f32 / time;
        println!("Tries: {:.2} tries / s", performance);
    };

    // Wait for a signal or for work to be done.
    let on_interrupt = |signal| {
        println!("Interrupted.");
        println!("received signal: {:?}", signal);
        print_state();
    };
    let on_finish = || {
        println!("Finished.");
        print_state();
    };

    chan_select! {
        signal.recv() -> signal => on_interrupt(signal),
        rx.recv() => on_finish()
    }
}

fn get_time(duration: Duration) -> f32 {
    let sec = duration.as_secs() as f32;
    let nanos = duration.subsec_nanos() as f32 / 1_000_000_000f32;
    sec + nanos
}

fn find_square(tries: Arc<AtomicUsize>) {
    let mut rng = rand::thread_rng();
    let mut func = |_, _| rng.gen_range::<usize>(1, 1001);
    let mut square = [[0usize; 3]; 3];
    loop {
        tries.fetch_add(1, Ordering::SeqCst);
        populate(&mut square, &mut func);
        power_of_2(&mut square);
        if is_magic(&square) {
            break;
        }
    };
}

fn power_of_2(square: &mut [[usize; 3]; 3]) {
    for x in 0..3 {
        for y in 0..3 {
            square[x][y] *= square[x][y];
        }
    }
}

fn is_magic(square: &[[usize; 3]; 3]) -> bool {
    let default_total = [0, 0];
    let mut last_total = [0, 0];
    let mut diagonals = [0, 0];
    let mut total = [0, 0];
    for x in 0..3 {
        total = [0, 0];
        for y in 0..3 {
            total[0] += square[x][y];
            total[1] += square[y][x];
            if x == y {
                diagonals[0] += square[x][y];
            }
            if 3 - x - 1 == y {
                diagonals[1] += square[x][y];
            }
        }
        if last_total != default_total && last_total != total {
            return false
        }
        last_total = total;
    }
    if total[0] == total[1] && total[0] == diagonals[0] && diagonals[0] == diagonals[1] {
        println!("Found magic {:?}", square);
        return true
    }
    false
}

fn populate<F>(square: &mut [[usize; 3]; 3], func: &mut F) -> ()
    where F: FnMut(usize, usize) -> usize {
    for x in 0..3 {
        for y in 0..3 {
            square[x][y] = 0;
        }
    }
    for x in 0..3 {
        for y in 0..3 {
            let value = check_generate(&square, func, x, y);
            square[x][y] = value;
        }
    }
}

fn check_generate<F>(square: &[[usize; 3]; 3], func: &mut F, x: usize, y: usize) -> usize
    where F: FnMut(usize, usize) -> usize {
    let mut value = func(x, y);
    while contains(&square, value) {
        value = func(x, y);
    }
    return value
}

fn contains(square: &[[usize; 3]; 3], new_value: usize) -> bool {
    for row in square {
        for value in row {
            if *value == new_value {
                return true
            }
        }
    }
    false
}