use crate::{MarshallingYard, Train};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

use colored::Colorize;
use rand::Rng;

pub struct Conductor {
    pub id: usize,
    pub yard: Arc<Mutex<MarshallingYard>>,
    pub job: Job,
}

pub enum Job {
    Enter(Train, usize),
    Exit(usize),
    None,
}

pub fn start_conductor(con: Conductor) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            match con.job {
                Job::Enter(t, x) => {
                    if con.yard.lock().unwrap().entering(t, x) {
                        println!(
                            "Conductor {} just entered track {} with train {}!",
                            con.id.to_string().red().bold(),
                            x.to_string().blue(),
                            t.val.to_string().green()
                        );

                        return;
                    }
                }
                Job::Exit(x) => {
                    let train_id = con.yard.lock().unwrap().exiting(x);
                    if train_id != -1 {
                        println!(
                            "Conductor {} just exited track {} with train {}!",
                            con.id.to_string().red().bold(),
                            x.to_string().blue(),
                            train_id.to_string().green()
                        );

                        return;
                    }
                }
                Job::None => {
                    return;
                }
            }
            thread::sleep(time::Duration::from_millis(rng.gen_range(220..270)));
        }
    });

    handle
}
