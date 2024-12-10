use crate::train::Train;
use std::collections::{HashSet, VecDeque};

pub struct MarshallingYard {
    pub track_count: usize,
    pub tracks: [Option<Train>; 5],
    pub job_board_enter: VecDeque<usize>,
    pub job_board_exit: VecDeque<usize>
}

impl MarshallingYard {
    pub fn entering(&mut self, train: Train, track: usize) -> bool {
        let mut success = false;
        match self.tracks[track].as_mut() {
            Some(_) => {println!("Track {} already occupied for train {}!", track,*train)} 
            None => {
                self.tracks[track] = Some(train);
                println!("Train {} entering on track {}!", *train, track);
                success = true;
                self.job_board_exit.push_back(track);

            }
        }
        success
    }

    pub fn exiting(&mut self, track: usize) -> i32 {
        println!("EXIT!");
        let mut train_id: i32 = -1;

        match self.tracks[track] {
            Some(t) => {
                train_id = *t;
                println!("Train {} exiting on track {}!", *t, track);
                self.tracks[track] = None;
                self.job_board_enter.push_back(track);
            }
            None => {println!("No train to exit for track {}!", track);}
        }
        train_id
    }


    pub fn get_occupied_tracks(&self) -> HashSet<usize> {
        let mut occ: HashSet<usize> = Default::default();
        // let cur_tracks = self.tracks.lock().unwrap();

        for i in 0..self.track_count - 1 {
            match self.tracks[i as usize] {
                Some(_) => occ.insert(i),
                None => false,
            };
        }
        occ
    }
}
