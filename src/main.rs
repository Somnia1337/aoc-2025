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
    "01" => day01
}
