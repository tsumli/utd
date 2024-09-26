use chrono::{TimeZone, Utc};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]

struct Args {
    unix_time: String,
    #[clap(default_value = "UTC")]
    zone: String,
}

fn main() {
    let args = Args::parse();
    let unix_time = args.unix_time;
    let zone = args.zone;

    let dt = Utc.timestamp_nanos(unix_time.parse::<i64>().unwrap_or_default());
    let tz = zone.parse::<chrono_tz::Tz>().unwrap_or_default();
    let dt_tz = dt.with_timezone(&tz);
    println!("{}", dt_tz);
}
