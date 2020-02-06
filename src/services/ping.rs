use crate::AppState;
extern crate signal_hook;
use crate::db::models::domains::{Domain, InsertDomainStatusRequest};
use actix::prelude::*;
use actix_web::web;
use chrono::Utc;
use futures::executor::block_on;
use reqwest::blocking::Client;
use std::fs;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

//impl Ping {
//    pub fn new(state: web::Data<AppState>) -> Ping {
//        Ping { state: state }
//    }
//
//    fn tick_loop(&mut self, context: &mut Context<Self>) {
//        let term = Arc::new(AtomicUsize::new(0));
//        const SIGTERM: usize = signal_hook::SIGTERM as usize;
//        const SIGINT: usize = signal_hook::SIGINT as usize;
//        const SIGQUIT: usize = signal_hook::SIGQUIT as usize;
//        signal_flag::register_usize(signal_hook::SIGTERM, Arc::clone(&term), SIGTERM).unwrap();
//        signal_flag::register_usize(signal_hook::SIGINT, Arc::clone(&term), SIGINT).unwrap();
//        signal_flag::register_usize(signal_hook::SIGQUIT, Arc::clone(&term), SIGQUIT).unwrap();
//
//        loop {
//            //            let _ = Delay(Duration::from_millis(1)).await;
//            match term.load(Ordering::Relaxed) {
//                0 => {
//                    // Do some useful stuff here
//                    let now = Instant::now();
//                    println!("tick start {:?}", now);
//                    println!("sleep");
//                    sleep(Duration::from_millis(20000));
//                    println!("tick end {:?}", Instant::now());
//                    println!("   ");
//                }
//                SIGTERM => {
//                    eprintln!("Terminating on the TERM signal");
//                    break;
//                }
//                SIGINT => {
//                    eprintln!("Terminating on the INT signal");
//                    break;
//                }
//                SIGQUIT => {
//                    eprintln!("Terminating on the QUIT signal");
//                    break;
//                }
//                _ => unreachable!(),
//            }
//        }
//    }
//
//    fn tick_interval(&mut self, context: &mut Context<Self>) {
//        println!("tick interval");
//        //        dbg!(&self.state);
//    }
//
//    fn tick_interval_many(&mut self, context: &mut Context<Self>, num: &u32) {
//        println!("tick {}", num);
//        //        dbg!(&self.state);
//    }
//}
//
//impl Actor for Ping {
//    type Context = SyncContext<Self>;
//
//    fn started(&mut self, context: &mut SyncContext<Self>) {
//#1  not bad one problem. If long time for all domains ping
// will fall behind
// spawn an interval stream into our context
//        IntervalFunc::new(
//            Duration::from_secs(self.state.config.ping_interval),
//            Self::tick_interval,
//        )
//        .finish()
//        .spawn(context);

// Problem with hot adding domains. need workaround
//        for i in 0..10 {
//            IntervalFunc::new(
//                Duration::from_secs(self.state.config.ping_interval),
//                move |s, c| Self::tick_interval_many(s, c, &i),
//            )
//            .finish()
//            .spawn(context);
//        }

// Problem with sync code. everything doesn't work.
//        TimerFunc::new(Duration::from_millis(100), Self::tick_loop).spawn(context);

// need spawn new thread or move to another process
//    }
//}
pub struct PingRequest {
    pub domain: Domain,
    pub state: Arc<web::Data<AppState>>,
}

impl Message for PingRequest {
    type Result = Result<bool, IoError>;
}

pub struct Ping {
    client: Client,
}

impl Ping {
    pub fn new() -> Ping {
        Ping {
            client: Client::builder()
                .timeout(Duration::from_secs(3))
                .build()
                .unwrap(),
        }
    }
}

impl Actor for Ping {
    type Context = SyncContext<Self>;
}

impl Handler<PingRequest> for Ping {
    type Result = Result<bool, IoError>;

    fn handle(&mut self, msg: PingRequest, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let media_root = msg.state.config.media_root();

        let folders = format!("{}{}", &media_root, &msg.domain.name);
        fs::create_dir_all(&folders).unwrap();

        let now = Utc::now();
        let timestamp = format!("{}", now.timestamp());

        let filename = format!(
            "{}/{}.{}",
            &msg.domain.name,
            &timestamp,
            &msg.state.config.image_format()
        );

        let result = self.client.head(&msg.domain.url).send().unwrap();
        let duration_since = Utc::now().signed_duration_since(now).num_milliseconds();

        let headers_str = format!("{:#?}", result.headers());
        let status = result.status();

        let full_path = format!("{}{}", &media_root, &filename);

        let command_result = Command::new("wkhtmltoimage".to_string())
            .arg("--height")
            .arg("600")
            .arg("--width")
            .arg("400")
            .arg("--quality")
            .arg("30")
            .arg("--javascript-delay")
            .arg("5000")
            .arg("--no-stop-slow-scripts")
            .arg("--enable-javascript")
            .arg("--debug-javascript")
            .arg("--log-level")
            .arg("error")
            .arg(&msg.domain.url)
            .arg(&full_path)
            .status();

        if let Ok(command_result) = command_result {
            if command_result.success() {
                block_on(msg.state.db.send(InsertDomainStatusRequest {
                    date: now.naive_utc(),
                    loading_time: duration_since as i32,
                    headers: headers_str,
                    status_code: u16::from(status) as i32,
                    filename: filename,
                    domain_id: msg.domain.id,
                }))
                .map_err(|err| IoError::new(IoErrorKind::Interrupted, err))
                .and_then(|result| match result {
                    Ok(filenames_for_remove) => {
                        filenames_for_remove.iter().for_each(|name| {
                            fs::remove_file(format!("{}{}", &media_root, name)).unwrap();
                            ()
                        });
                        Ok(())
                    }
                    Err(err) => {
                        error!("{}", err);
                        error!("need remove {}", &full_path);
                        fs::remove_file(&full_path).unwrap();
                        Ok(())
                    }
                })
                .ok();
            }
        };

        Ok(true)
    }
}
