use crate::AppState;
extern crate signal_hook;
use actix::prelude::*;
use actix::utils::IntervalFunc;
use actix_web::web;
use signal_hook::flag as signal_flag;
use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Ping {
    state: web::Data<AppState>,
}

impl Ping {
    pub fn new(state: web::Data<AppState>) -> Ping {
        Ping { state: state }
    }

    fn tick_loop(&mut self, context: &mut Context<Self>) {
        let term = Arc::new(AtomicUsize::new(0));
        const SIGTERM: usize = signal_hook::SIGTERM as usize;
        const SIGINT: usize = signal_hook::SIGINT as usize;
        const SIGQUIT: usize = signal_hook::SIGQUIT as usize;
        signal_flag::register_usize(signal_hook::SIGTERM, Arc::clone(&term), SIGTERM).unwrap();
        signal_flag::register_usize(signal_hook::SIGINT, Arc::clone(&term), SIGINT).unwrap();
        signal_flag::register_usize(signal_hook::SIGQUIT, Arc::clone(&term), SIGQUIT).unwrap();

        loop {
            match term.load(Ordering::Relaxed) {
                0 => {
                    // Do some useful stuff here
                    let now = Instant::now();
                    println!("tick start {:?}", now);
                    println!("sleep");
                    sleep(Duration::from_millis(20000));
                    println!("tick end {:?}", Instant::now());
                    println!("   ");
                }
                SIGTERM => {
                    eprintln!("Terminating on the TERM signal");
                    break;
                }
                SIGINT => {
                    eprintln!("Terminating on the INT signal");
                    break;
                }
                SIGQUIT => {
                    eprintln!("Terminating on the QUIT signal");
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    fn tick_interval(&mut self, context: &mut Context<Self>) {
        println!("tick interval");
        //        dbg!(&self.state);
    }

    fn tick_interval_many(&mut self, context: &mut Context<Self>, num: &u32) {
        println!("tick {}", num);
        //        dbg!(&self.state);
    }
}

impl Actor for Ping {
    type Context = Context<Self>;

    fn started(&mut self, context: &mut Context<Self>) {
        //#1  not bad one problem. If long time for all domains ping
        // will fall behind
        // spawn an interval stream into our context
        IntervalFunc::new(
            Duration::from_secs(self.state.config.ping_interval),
            Self::tick_interval,
        )
        .finish()
        .spawn(context);

        // Problem with hot adding domains. need workaround
        for i in 0..10 {
            IntervalFunc::new(
                Duration::from_secs(self.state.config.ping_interval),
                move |s, c| Self::tick_interval_many(s, c, &i),
            )
            .finish()
            .spawn(context);
        }

        // Problem with sync code. everything doesn't work.
        TimerFunc::new(Duration::from_millis(100), Self::tick_loop).spawn(context);

        // need spawn new thread or move to another process
    }
}
