extern crate rand;
#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate num_cpus;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use chan_signal::Signal;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use chrono::prelude::*;
use std::io;
use std::io::{Read, BufReader, BufWriter};
use std::path::Path;
use std::fs::{File, OpenOptions};

const MAX_VALUE: usize = 100;
const SIZE: usize = 3;
const STATE_FILENAME: &'static str = "magic.state";

fn main() {
    println!("Magic square!");

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (tx, rx) = chan::sync(0);
    let tries = Arc::new(AtomicUsize::new(0));
    let now = SystemTime::now();

    let cpu_count = num_cpus::get();
    let current_time = Local::now();
    println!("Started {} using {} cores", current_time, cpu_count);

    let arc_state = Arc::new(Mutex::new(restore_state(cpu_count)));

    {
        let a_state = arc_state.clone();
        let b_state = a_state.lock().unwrap();
        let mut i = 0;
        for arc_square in &b_state.state {
            i = i + 1;
            let (tries, tx) = (tries.clone(), tx.clone());
            let a_square = arc_square.clone();
            thread::spawn(move || {
                find_square_sequence(i, tries, a_square);
                tx.send(());
            });
        }
    }
    //io finish
    {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("Press ENTER to quit program");
            io::stdin().bytes().next();
            tx.send(());
        });
    }

    chan_select! {
        signal.recv() -> signal => {
            println!("\nInterrupted.");
            println!("received signal: {:?}", signal);
        },
        rx.recv() => {
            println!("Finished.");
        },
    }
    let mut b_state = arc_state.lock().unwrap();
    save_state(&mut b_state);
    let count = tries.load(Ordering::SeqCst);
    println!("Tries: {}", count);
    let time = get_time(now.elapsed().unwrap());
    println!("Time: {:.2}", time);
    let performance = count as f32 / time;
    println!("Tries: {:.2} tries / s", performance);
    println!("Done");
}


#[derive(Serialize, Deserialize, Debug)]
struct State {
    state: Vec<Square>
}

impl State {
    fn from(state: &ArcState) -> State {
        let mut vec = Vec::new();
        for square in &state.state {
            let a_square = square.clone();
            let b_square = a_square.lock().unwrap();
            vec.push(b_square.clone());
        }
        State {
            state: vec
        }
    }
}

struct ArcState {
    state: Vec<Arc<Mutex<Square>>>
}

impl ArcState {
    fn from(state: State) -> ArcState {
        let mut vec = Vec::new();
        for square in state.state {
            vec.push(Arc::new(Mutex::new(square)));
        }
        ArcState {
            state: vec
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Square {
    data: [[usize; SIZE]; SIZE],
}

impl Square {
    fn new() -> Square {
        Square {
            data: Square::empty(),
        }
    }

    fn empty() -> [[usize; SIZE]; SIZE] {
        [[0usize; SIZE]; SIZE]
    }

    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.data[x][y] = value;
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.data[x][y]
    }

    fn contains(&self, new_value: usize) -> bool {
        for row in self.data.iter() {
            for value in row {
                if *value == new_value {
                    return true;
                }
            }
        }
        false
    }

    fn power_of_2(&mut self) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let value = self.data[x][y];
                self.data[x][y] = value * value;
            }
        }
    }

    fn is_magic(&self) -> bool {
        let default_total = [0, 0];
        let mut last_total = [0, 0];
        let mut diagonals = [0, 0];
        let mut total = [0, 0];
        for x in 0..SIZE {
            total = [0, 0];
            for y in 0..SIZE {
                let value = self.get(x, y);
                total[0] += value;
                total[1] += self.get(y, x);
                if x == y {
                    diagonals[0] += value;
                }
                if SIZE - x - 1 == y {
                    diagonals[1] += value;
                }
            }
            if last_total != default_total && last_total != total {
                return false;
            }
            last_total = total;
        }
        if total[0] == total[1] && total[0] == diagonals[0] && diagonals[0] == diagonals[1] {
            println!("Found magic {:?}", self);
            return true;
        }
        false
    }
}

fn get_time(duration: Duration) -> f32 {
    let sec = duration.as_secs() as f32;
    let nanos = duration.subsec_nanos() as f32 / 1_000_000_000f32;
    sec + nanos
}

fn find_square_sequence(number: usize, tries: Arc<AtomicUsize>, mutex_square: Arc<Mutex<Square>>) -> () {
    loop {
        let mut square = mutex_square.lock().unwrap();
        tries.fetch_add(1, Ordering::SeqCst);
        if add_one(&mut square) {
            println!("no more values for {}", number);
            break;
        }
        let mut squared_square = square.clone();
        squared_square.power_of_2();
        if squared_square.is_magic() {
            break;
        }
    };
}

fn reset_to_minimum(square: &mut Square) {
    square.set(0, 0, 9);
    square.set(0, 1, 8);
    square.set(0, 2, 7);
    square.set(1, 0, 6);
    square.set(1, 1, 5);
    square.set(1, 2, 4);
    square.set(2, 0, 3);
    square.set(2, 1, 2);
    square.set(2, 2, 1);
}

fn add_one(square: &mut Square) -> bool {
    for x in 0..SIZE {
        for y in 0..SIZE {
            let mut is_maximum_reached = false;
            let mut value = square.get(x, y);
            //do-while
            while {
                value += 1;
                if value > MAX_VALUE {
                    value = 1;
                    is_maximum_reached = true;
                }
                square.contains(value) //until
            } {}
            square.set(x, y, value);
            if !is_maximum_reached {
                return false;
            }
        }
    }
    true
}

fn restore_state(max: usize) -> ArcState {
    let state = match File::open(STATE_FILENAME) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(state) => state,
                Err(..) => default_state(max),
            }
        },
        Err(..) => default_state(max),
    };
    println!("Starting state {:?}", state);
    ArcState::from(state)
}

fn default_state(max : usize) -> State {
    println!("Using default start state");
    let mut vec: Vec<Square> = Vec::new();
    for _ in 0..max {
        let mut square = Square::new();
        reset_to_minimum(&mut square);
        vec.push(square);
    }
    State {
        state: vec
    }
}

fn save_state(arc_state: &mut ArcState) -> () {
    let state = State::from(&arc_state);
    let path = Path::new(STATE_FILENAME);
    let mut options = OpenOptions::new();
    options.write(true)
        .truncate(true)
        .create(true);
    let file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("File cannot be created"),
    };
    let writer = BufWriter::new(&file);
    match serde_json::to_writer(writer, &state) {
        Ok(..) => println!("Saved state {:?}", state),
        Err(..) => println!("Saving failed"),
    }
}

#[test]
fn test_is_magic() {
    let mut square = Square::new();
    square.set(0, 0, 4);
    square.set(0, 1, 9);
    square.set(0, 2, 2);
    square.set(1, 0, 3);
    square.set(1, 1, 5);
    square.set(1, 2, 7);
    square.set(2, 0, 8);
    square.set(2, 1, 1);
    square.set(2, 2, 6);
    assert!(square.is_magic())
}

#[test]
fn test_serialization() {
    let mut square = Square::new();
    square.data[0][1] = 42;
    let serialized = serde_json::to_string(&square).unwrap();
    println!("Serialized {:?} or {}", square, serialized);
    let deserialized: Square = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized {:?} or {:?}", square, deserialized);
    let serialized2 = serde_json::to_string(&deserialized).unwrap();
    assert_eq!(serialized, serialized2);
}