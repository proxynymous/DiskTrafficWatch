
use std::io::Result;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::time::Duration;

extern crate dtlib;

fn main() {
    let log_result = parse_log(dtlib::constants::LOG_FILE);

    if !log_result.is_ok() {
        println!("Could not load log.");
    }

    else {
        let log = log_result.unwrap();
        let stats_opt = calc_stats(log);

        if !stats_opt.is_some() {
            println!("Could not calculate stats.");
        }

        else {
            let stats = stats_opt.unwrap();

            print!(
                " == DT Stats == \n\
                 \n\
                 Sectors written: {}\n\
                 GB written: {}\n\
                 \n\
                 Uptime (h): {}\n\
                 Uptime (d): {}\n\
                 Uptime (y): {}\n\
                 \n\
                 GB/h: {}\n\
                 GB/d: {}\n\
                 GB/y: {}\n\
                 \n\
                 Days until 100GB written: {}\n\
                 Days until 1TB written: {}\n\
                 Days until 100TB written: {} = {} years\n",
                stats.sectors_written,
                stats.gbytes_written,
                stats.uptime_hours,
                stats.uptime_days,
                stats.uptime_years,
                stats.gbytes_per_hour,
                stats.gbytes_per_day,
                stats.gbytes_per_year,
                stats.days_until_100_gb,
                stats.days_until_1_tb,
                stats.days_until_100_tb, stats.years_until_100_tb
            );
        }
    }
}

struct DtStats {
    sectors_written : u64,
    gbytes_written : u64,

    uptime_hours : f64,
    uptime_days : f64,
    uptime_years : f32,

    gbytes_per_hour : f64,
    gbytes_per_day : f64,
    gbytes_per_year : f64,

    days_until_100_gb : u64,
    days_until_1_tb : u64,
    days_until_100_tb : u64,
    years_until_100_tb : u64
}

struct LogEntry {
    sectors_written : usize,
    uptime : Duration
}

fn calc_stats(entries : LogIterator) -> Option<DtStats> {
    let (sectors, uptime_opt) = entries
        .map(|entry| (entry.sectors_written as u64, entry.uptime))
        .fold(
            (0 as u64, Some(Duration::new(0, 0))),
            |(sectors, uptime), (new_sectors, new_uptime)|
                (
                    sectors + new_sectors,
                    uptime.and_then(
                        |uptime| uptime.checked_add(new_uptime)
                    )
                )
        );

    if uptime_opt.is_some() {
        let uptime = uptime_opt.unwrap();
        let gbytes_written : u64 = (sectors * dtlib::constants::BLOCK_SIZE as u64) / (1024 * 1024 * 1024);

        let uptime_hours = uptime.as_secs() as f64 / (60 * 60) as f64;
        let uptime_days =  uptime_hours / 24 as f64;
        let uptime_years = uptime_days / 365 as f64;

        let gbytes_per_hour = gbytes_written as f64 / uptime_hours;
        let gbytes_per_day = gbytes_written as f64 / uptime_days;
        let gbytes_per_year =  gbytes_written as f64 / uptime_years;

        return Some(
            DtStats {
                sectors_written: sectors,
                gbytes_written: gbytes_written,

                uptime_hours: uptime_hours,
                uptime_days: uptime_days,
                uptime_years: uptime_years as f32,

                gbytes_per_hour: gbytes_per_hour,
                gbytes_per_day: gbytes_per_day,
                gbytes_per_year: gbytes_per_year,

                days_until_100_gb: (100.0 as f64 / gbytes_per_day) as u64,
                days_until_1_tb: (1024.0 as f64 / gbytes_per_day) as u64,
                days_until_100_tb: (102400.0 as f64 / gbytes_per_day) as u64,
                years_until_100_tb: (102400.0 as f64 / gbytes_per_day) as u64 / 365
            }
        );
    }

    else {
        return None;
    }
}

fn parse_log_line(line : &String) -> Option<LogEntry> {
    let mut split = line.split(':');
    let sectors_opt = split.next()
                           .and_then(|s| s.parse::<usize>().ok());
    let utime_opt = split.next()
                         .and_then(|s| s.parse::<u64>().ok())
                         .map(|t| Duration::from_secs(t));

    return match (sectors_opt, utime_opt) {
        (Some(sectors), Some(uptime)) =>
            Some(
                LogEntry{
                    sectors_written: sectors,
                    uptime: uptime
                }
            ),
        _ => None
    }
}

type LogIterator = Box<Iterator<Item=LogEntry>>;

fn parse_log(path : &str) -> Result<LogIterator> {
    let file = File::open(path);

    return file.map(
        |f| {
            let buf_file = BufReader::new(f);

            let it = buf_file
                .lines()
                .filter(|line| line.is_ok())
                .map(|line| line.unwrap())
                .map(|line| parse_log_line(&line))
                .filter(|opt| opt.is_some())
                .map(|opt| opt.unwrap());

            return Box::new(it) as LogIterator;
        }
    );
}
