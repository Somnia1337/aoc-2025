struct Range {
    start: i64,
    end: i64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('-').unwrap_or_default();
        let (start, end) = (
            start.parse().unwrap_or_default(),
            end.parse().unwrap_or_default(),
        );

        Self { start, end }
    }
}

impl Range {
    fn is_invalid_1(id: i64) -> bool {
        let id = id.to_string();
        let k = id.len() >> 1;

        k << 1 == id.len() && id.eq(&id[0..k].repeat(2))
    }

    fn is_invalid_2(id: i64) -> bool {
        let id = id.to_string();
        let l = id.len();

        (1..=l >> 1).any(|k| l.is_multiple_of(k) && id.eq(&id[0..k].repeat(l / k)))
    }

    fn invalid_ids(&self, invalidator: &dyn Fn(i64) -> bool) -> Vec<i64> {
        (self.start..=self.end)
            .filter(|id| invalidator(*id))
            .collect()
    }
}

pub fn day02(input: &str) {
    let ranges = input.split(',').map(Range::from).collect::<Vec<Range>>();

    let ans1 = ranges
        .iter()
        .flat_map(|r| r.invalid_ids(&Range::is_invalid_1))
        .sum::<i64>();
    let ans2 = ranges
        .iter()
        .flat_map(|r| r.invalid_ids(&Range::is_invalid_2))
        .sum::<i64>();

    println!("{ans1} {ans2}"); // 38158151648 45283684555
}
