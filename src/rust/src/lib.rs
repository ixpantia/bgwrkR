#![allow(non_snake_case)]
use extendr_api::prelude::*;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;

const ONE_SECOND: Duration = Duration::from_secs(1);

struct BackgroundWorker {
    _thread: Option<std::thread::JoinHandle<()>>,
}

/// Represents a background running R script.
/// @export
#[extendr]
impl BackgroundWorker {
    /// @title Start a new `BackgroundWorker`.
    /// @param script The path to the Rscript to run in the background
    /// @param schedule A cron-like schedule. See example for more detail.
    /// @examples
    /// #           sec  min   hour   day of month   month   day of week   year
    /// schedule <- "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2"
    /// BackgroundWorker$new("script.R", schedule)
    /// @export
    fn new(script: String, schedule: &str) -> Result<BackgroundWorker> {
        if let Ok(worker_id) = std::env::var("FAUCET_WORKER_ID") {
            if worker_id != "1" {
                return Ok(BackgroundWorker { _thread: None });
            }
        }
        let schedule = cron::Schedule::from_str(schedule)
            .map_err(|e| e.to_string())?
            .upcoming_owned(chrono::Local);
        let thread = std::thread::spawn(move || {
            // We loop over all the possible next times
            for next in schedule {
                // We wait until time is greater than the current
                // next schedule
                'waiter: loop {
                    // Out cron can run with up to a One Second
                    // specificity
                    std::thread::sleep(ONE_SECOND);
                    let current_time = chrono::Local::now();
                    if current_time >= next {
                        break 'waiter;
                    }
                }
                let _status = Command::new("Rscript")
                    .arg(&script)
                    .stdin(std::process::Stdio::null())
                    .status();
            }
        });
        Ok(BackgroundWorker {
            _thread: Some(thread),
        })
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod bgwrkR;
    impl BackgroundWorker;
}
