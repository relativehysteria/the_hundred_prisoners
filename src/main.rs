#![allow(unreachable_code)]
#![allow(dead_code)]
use core::arch::x86_64::_rdtsc;

mod xorshift;
use xorshift::Rng;

/// Number of prisoners and boxes
const NUM_PRISONERS: usize = 100;

/// Number of attempts to find the prisoners number
const NUM_ATTEMPTS: usize = 50;

fn main() {
    attempt(42988879486144);
    std::process::exit(0);

    search_for_valid_seeds();

    search_for_interesting_seeds();
}

#[derive(Debug)]
enum SearchResult {
    Found(usize),
    NotFound,
    Looped,
}

/// Attempt to solve the riddle given a certain seed.
fn attempt(seed: u64) {
    let mut rng = Rng::new(seed);

    // Create the boxes and randomize them
    let mut boxes = (0..NUM_PRISONERS).collect::<Vec<usize>>();
    rng.shuffle(&mut boxes);

    // Let the prisoners search for their numbers
    println!("{boxes:#?}");
    for i in 0..NUM_PRISONERS {
        println!("{i}: {:?}", search_number(&boxes, i, NUM_ATTEMPTS));
    }
}

/// Looks for seeds where each prisoner finds their number.
fn search_for_valid_seeds() -> ! {
    'main: loop {
        // Create the RNG
        let seed = unsafe { _rdtsc() };
        let mut rng = Rng::new(seed);

        // Create and randomize the boxes
        let mut boxes = (0..NUM_PRISONERS).collect::<Vec<usize>>();
        rng.shuffle(&mut boxes);

        // Search for the prisoners' numbers. If one of them loops or can't find
        // their number, this is an invalid seed.
        for i in 0..NUM_PRISONERS {
            match search_number(&boxes, i, NUM_ATTEMPTS) {
                SearchResult::NotFound | SearchResult::Looped => {
                    #[cfg(debug_assertions)]
                    println!("{seed}: Invalid attempt.");
                    continue 'main;
                }
                _ => (),
            }
        }

        println!("{seed}: VALID SEED!");
    }
}

/// Looks for seeds that return interesting results.
///
/// "Interesting results" mean that all prisoners take exactly the same amount
/// of attempts to find their corresponding numbers.
fn search_for_interesting_seeds() -> ! {
    'main: loop {
        // Create the RNG
        let seed = unsafe { _rdtsc() };
        let mut rng = Rng::new(seed);

        // Create and randomize the boxes
        let mut boxes = (0..NUM_PRISONERS).collect::<Vec<usize>>();
        rng.shuffle(&mut boxes);

        // The number of attempts we are looking for.
        // If we get a `NotFound` or `Looped` on our first attempt,
        // this result is already uninteresting for us.
        let found_attempt = match search_number(&boxes, 0, NUM_ATTEMPTS) {
            SearchResult::NotFound | SearchResult::Looped => {
                #[cfg(debug_assertions)]
                println!("{seed}: Uninteresting first attempt.");
                continue 'main;
            },
            SearchResult::Found(attempt) => attempt,
        };

        // Now check the rest. An attempt is only interesting if it's the same
        // as the first found attempt. If we find the number but take
        // a different amount of attempts, that is uninteresting..
        for i in 1..NUM_PRISONERS {
            match search_number(&boxes, i, NUM_ATTEMPTS) {
                SearchResult::NotFound | SearchResult::Looped => {
                    #[cfg(debug_assertions)]
                    println!("{seed}: Uninteresting attempt {i}.");
                    continue 'main;
                }
                SearchResult::Found(attempt) =>
                    if attempt != found_attempt {
                        #[cfg(debug_assertions)]
                        println!("{seed}: Uninteresting attempt {i}.");
                        continue 'main;
                    },
            }
        }

        println!("{seed}: INTERESTING SEED! ATTEMPTS: {found_attempt}");
    }
}

/// Searches for a number given the Miltersen looping algorithm.
///
/// `boxes` are the boxes to loop through. `num` is the number we are looking
/// for. `n_attempts` is the number of attempts we make.
fn search_number(boxes: &Vec<usize>, num: usize, n_attempts: usize)
        -> SearchResult {
    // This is our first attempt to find the number
    let mut next_num = boxes[num];
    let first_num = next_num;

    // This is the rest of the attempts
    for attempt in 0..(n_attempts) {
        // We found the number we are looking for
        if next_num == num {
            return SearchResult::Found(attempt+1);
        }

        // Go to the next number
        next_num = boxes[next_num];

        // We got back to our first attempt but didn't find our number
        if next_num == first_num {
            return SearchResult::Looped;
        }
    }

    SearchResult::NotFound
}
