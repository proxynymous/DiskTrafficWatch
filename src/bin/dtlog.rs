
use std::io::prelude::*;
use std::time::Duration;
use std::fs::OpenOptions;

extern crate dtlib;

fn main() {
  let written = dtlib::dt::get_bytes_written();
  let utime = dtlib::uptime::get_uptime();

  match (
  	  written.ok(),
  	  utime.ok()
  	) {
    (Some(gbs), Some(utime)) => {
      let log_str = format_log_string(gbs, utime);

      if append_to_log(dtlib::constants::LOG_FILE, log_str.as_str()) {
        println!("Successfully saved dt stats to {}", dtlib::constants::LOG_FILE);
      }

      else {
        println!("Failed saving dt stats to {}", dtlib::constants::LOG_FILE);
      }
    },
    _ => println!("Failed to retrieve information.")
  }
}

fn format_log_string(sectors : usize, uptime : Duration) -> String {
  return format!("{}:{}", sectors, uptime.as_secs());
}

//http://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file
fn append_to_log(log_path : &str, out : &str) -> bool {
  let file_option =
        OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path);

  if file_option.is_ok() {
    let mut file = file_option.unwrap();

    if let Err(_) = writeln!(file, "{}", out) {
      return false;
    }

    else {
      return true;
    }
  }

  else {
    return false;
  }
}
