
use std::io::prelude::*;
use std::time::Duration;
use std::fs::OpenOptions;
mod dulib;

const LOG_FILE : &str = "/var/log/dulog";

fn main() {
  let written = dulib::du::get_bytes_written();
  let utime = dulib::uptime::get_uptime();

  match (
  	  written.ok(),
  	  utime.ok()
  	) {
    (Some(gbs), Some(utime)) => {
      let log_str = format_log_string(gbs, utime);

      if append_to_log(LOG_FILE, log_str.as_str()) {
        println!("Successfully saved du stats to {}", LOG_FILE);
      }

      else {
        println!("Failed saving du stats to {}", LOG_FILE);
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



