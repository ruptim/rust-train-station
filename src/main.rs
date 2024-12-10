use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::min;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time;

use raylib::prelude::*;

use crate::conductor::{start_conductor, Conductor, Job};
use crate::marshalling_yard::MarshallingYard;
use crate::train::Train;

mod conductor;
mod marshalling_yard;
mod train;

const TRACK_BASE_POSI_Y: i32 = 106;
const TRACK_BASE_POSI_X: i32 = 100;
const TRACK_BASE_RADIUS: f32 = 20.0;

const TRACK_COUNT: usize = 5;
const MAX_PENDING_JOBS_PER: usize = 5;

fn main() {
    let mut start_time = time::SystemTime::now();
    let mut timeout = time::Duration::from_millis(500);
    let mut train_id = 0;
    let mut conductor_id = 0;
    let mut rng = rand::thread_rng();

    let mut vec: VecDeque<usize> = (0..min(MAX_PENDING_JOBS_PER, TRACK_COUNT)).collect();
    vec.make_contiguous().shuffle(&mut rng);

    let yard = Arc::new(Mutex::new(MarshallingYard {
        track_count: TRACK_COUNT,
        tracks: [None; TRACK_COUNT],
        job_board_enter: vec,
        job_board_exit: VecDeque::new(),
    }));

    let (mut rl, thread) = raylib::init().size(640, 480).title("Train Station").build();

    println!("===============================================");
    while !rl.window_should_close() {
        let cur_time = time::SystemTime::now();
        let elapsed = cur_time.duration_since(start_time);

        if elapsed.unwrap() > timeout {
            let mut yard_unlocked = yard.lock().unwrap();

            let mut job = Job::None;
            let mut job_type: bool = rng.gen_bool(0.4);

            if job_type {
                let track = yard_unlocked.job_board_exit.pop_front();
                match track {
                    Some(y) => {
                        job = Job::Exit(y);
                    }
                    None => job_type = false,
                }
            }
            if !job_type {
                match yard_unlocked.job_board_enter.pop_front() {
                    Some(j) => {
                        job = Job::Enter(Train { val: train_id }, j);
                    }
                    None => {}
                }
            }

            let con = Conductor {
                id: conductor_id,
                yard: yard.clone(),
                job,
            };

            start_conductor(con);

            train_id += 1;
            conductor_id += 1;
            timeout = time::Duration::from_millis(rng.gen_range(200..1000));
            start_time = time::SystemTime::now();
        }

        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);

            let occ_tracks = yard.lock().unwrap().get_occupied_tracks();
            for track in 0..TRACK_COUNT {
                let posi_y: i32 = 20 + (track as i32 * TRACK_BASE_POSI_Y);
                if occ_tracks.contains(&track) {
                    d.draw_circle(
                        TRACK_BASE_POSI_X,
                        posi_y,
                        TRACK_BASE_RADIUS,
                        Color::DARKBLUE,
                    );
                } else {
                    d.draw_circle_lines(
                        TRACK_BASE_POSI_X,
                        posi_y,
                        TRACK_BASE_RADIUS,
                        Color::DARKBLUE,
                    );
                }
                let text_x: i32 = TRACK_BASE_POSI_X + (TRACK_BASE_RADIUS * 2.0) as i32;
                d.draw_text(
                    format!("Track {}", track).as_str(),
                    text_x,
                    posi_y - 10,
                    20,
                    Color::BLACK,
                );
            }
        }
    }
}
