#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

macro_rules! aoc_days {
    ($($day:literal => $mod:ident),+ $(,)?) => {
        $(
            mod $mod;
        )+

        fn main() {
            let day = std::env::args()
                .nth(1)
                .expect("Usage: cargo run -- <day>");

            let input_path = match day.as_str() {
                $(
                    $day => concat!("input/day", $day),
                )+
                _ => panic!("Unknown day: {}", day),
            };

            let input = std::fs::read_to_string(input_path)
                .expect("Failed to read input");

            match day.as_str() {
                $(
                    $day => $mod::$mod(&input),
                )+
                _ => unreachable!(),
            };
        }
    };
}

aoc_days! {
    "01" => day01,
    "02" => day02,
    "03" => day03,
    "04" => day04,
    "05" => day05,
    "06" => day06,
    "07" => day07,
    "08" => day08,
    "09" => day09,
    "10" => day10,
}
