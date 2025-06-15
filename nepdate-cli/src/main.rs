use bikram::bikram::Bikram;
use chrono::{Datelike, Local};
use clap::Parser;
use clap::Subcommand;
use std::env;
use std::process;

mod nepali_calendar;

/// Convert to Nepali Date
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Conv {
    /// Convert date to Bikram Sambat, commonly referred to as BS (year month day)
    #[arg(short, long)]
    ad_to_bs: Option<String>,

    /// Convert Gregorian Date, commonly referred to as AD (year month day)
    #[arg(short, long)]
    bs_to_ad: Option<String>,

    /// Show the current Nepali date (BS)
    #[arg(short = 'n', long)]
    today_nepali: bool,

    /// Show the current English date (AD)
    #[arg(short = 'e', long)]
    today_english: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    English {
        /// Displays the current Gregorian date
        #[arg(short, long)]
        today: bool,

        /// Date will be displayed in this format. It must be compatible with
        /// chrono format for Rust. If no format is specified then default
        /// format will be used.
        /// Default format: m/d/Y -> 5/10/2025
        #[arg(long)]
        format: Option<String>,
    },
    Nepali {
        #[arg(long)]
        format: String,
    },
}

fn _prev_main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 || args[1] != "--conv" || (args[2] != "--tobs" && args[2] != "--toad") {
        eprintln!(
            "Usage: {}\n\x1b[31m Convert to Nepali Date: --conv --tobs year month day\n Convert to Gregorian Date: --conv --toad year month day\x1b[0m",
            args[0]
        );

        // Get current date
        let now_date = Local::now();
        let year = now_date.year();
        let month = now_date.month();
        let day = now_date.day();

        let mut bsdate = Bikram::new();
        bsdate.from_gregorian(year, month as i32, day as i32);

        let bs_weekday_name = bsdate.get_weekday_name(year, month as i32, day as i32);

        println!("\x1b[36m   Today's Date:\x1b[0m");
        println!(
            " \x1b[33m Gregorian: \x1b[0m \x1b[35m{} {} {} {} \x1b[0m",
            year, month, day, bs_weekday_name
        );
        println!(
            " \x1b[33m Bikram Sambat: \x1b[0m \x1b[35m{} {} {} {} \x1b[33m days in bikram month: \x1b[0m{} \x1b[0m",
            bsdate.get_year(),
            bsdate.get_month(),
            bsdate.get_day(),
            bs_weekday_name,
            bsdate.days_in_month(bsdate.get_year(), bsdate.get_month())
        );
        process::exit(1);
    }

    let conv_type = &args[2];
    let year = args[3].parse::<i32>().expect("Year must be an integer.");
    let month = args[4].parse::<i32>().expect("Month must be an integer.");
    let day = args[5].parse::<i32>().expect("Day must be an integer.");

    let mut bsdate = Bikram::new();

    if conv_type == "--tobs" {
        bsdate.from_gregorian(year, month, day);
    } else if conv_type == "--toad" {
        bsdate.from_nepali(year, month, day);
    }

    let converted_year = bsdate.get_year();
    let converted_month = bsdate.get_month();
    let converted_day = bsdate.get_day();
    let bs_weekday_name = bsdate.get_weekday_name(converted_year, converted_month, converted_day);

    if conv_type == "--tobs" {
        let _gregorian_weekday_name =
            bsdate.get_weekday_name(converted_year, converted_month, converted_day);

        println!(
            " \x1b[33m Bikram Sambat Date: \x1b[0m \x1b[35m{} {} {} {} \x1b[0m \x1b[33m days in bikram month: \x1b[0m{} \x1b[0m",
            converted_year,
            converted_month,
            converted_day,
            bs_weekday_name,
            bsdate.days_in_month(bsdate.get_year(), bsdate.get_month())
        );
    } else if conv_type == "--toad" {
        let gregorian_weekday_name =
            bsdate.get_weekday_name(converted_year, converted_month, converted_day);

        println!(
            "\x1b[33m Gregorian Date: \x1b[0m \x1b[35m{} {} {} {} \x1b[0m",
            converted_year, converted_month, converted_day, gregorian_weekday_name
        );
    }
}

fn main() {
    let conv = Conv::parse();

    println!("{:#?}", conv.command.unwrap());

    let now_date = Local::now();
    let year = now_date.year();
    let month = now_date.month();
    let day = now_date.day();

    if conv.today_nepali {
        let mut bsdate = Bikram::new();
        bsdate.from_gregorian(year, month as i32, day as i32);

        println!(
            "{} {}, {}",
            bsdate.get_day(),
            nepali_calendar::human_month(bsdate.get_month()),
            bsdate.get_year(),
        );
    }

    if conv.today_english {
        println!("{}/{}/{} (m/d/Y)", month, day, year);
    }
}
