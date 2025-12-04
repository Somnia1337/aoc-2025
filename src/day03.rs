pub fn max_joltage(bank: &str, turns: usize) -> usize {
    let bank = bank.bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let mut next = 0;

    (0..turns).fold(0, |jot, turn| {
        let mut max_digit = 0;
        let mut p = next;
        let end = bank.len() - turns + (turn + 1);

        bank.iter()
            .enumerate()
            .take(end)
            .skip(next)
            .for_each(|(i, b)| {
                if *b > max_digit {
                    max_digit = *b;
                    p = i;
                }
            });

        next = p + 1;
        jot * 10 + max_digit as usize
    })
}

pub fn day03(input: &str) {
    let ans1 = input
        .lines()
        .map(|bank| max_joltage(bank, 2))
        .sum::<usize>();
    let ans2 = input
        .lines()
        .map(|bank| max_joltage(bank, 12))
        .sum::<usize>();

    println!("{ans1} {ans2}"); // 17085 169408143086082
}
