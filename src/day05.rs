use std::ops::RangeInclusive;

pub fn day05(input: &str) {
    let mut ranges = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            let (start, end) = (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            );
            RangeInclusive::new(start, end)
        })
        .collect::<Vec<RangeInclusive<usize>>>();
    let ids = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let ans1 = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    let ans2 = {
        ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()).then(r1.end().cmp(r2.end())));

        let mut i = 1;
        loop {
            if i == ranges.len() {
                break;
            }

            if ranges[i].start() <= ranges[i - 1].end() {
                ranges[i - 1] = RangeInclusive::new(
                    *ranges[i - 1].start(),
                    *ranges[i - 1].end().max(ranges[i].end()),
                );
                ranges.remove(i);
                i -= 1;
            }

            i += 1;
        }

        ranges
            .into_iter()
            .map(std::iter::Iterator::count)
            .sum::<usize>()
    };

    println!("{ans1} {ans2}"); // 674 352509891817881
}
