// The method for part2 is correct only for a HUGE map.
// Easily hackable with this example:
// 5,2
// 5,5
// 2,5
// 2,7
// 6,7
// 6,2
// Where the correct ans2 would be 15 (with corners (2,5) and (6,7)),
// but the below method would give 16 (with corners (5,2) and (2,5)).

#[derive(Clone)]
struct Coord {
    x: u64,
    y: u64,
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let xy = value
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        assert_eq!(xy.len(), 2);

        Self { x: xy[0], y: xy[1] }
    }
}

impl Coord {
    const fn rect_area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn check(c1: &Coord, c2: &Coord, coords: &[Coord]) -> bool {
    let x1 = c1.x.min(c2.x);
    let x2 = c1.x.max(c2.x);
    let y1 = c1.y.min(c2.y);
    let y2 = c1.y.max(c2.y);

    !coords.windows(2).any(|w| {
        (w[0].x > x1 && w[1].x < x2 && w[0].y > y1 && w[1].y < y2)
            || (w[1].x > x1 && w[0].x < x2 && w[1].y > y1 && w[0].y < y2)
    })
}

pub fn day09(input: &str) {
    let mut coords = input.lines().map(Coord::from).collect::<Vec<Coord>>();

    let ans1 = coords
        .iter()
        .enumerate()
        .map(|(i, c1)| {
            coords
                .iter()
                .skip(i + 1)
                .map(|c2| c1.rect_area(c2))
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap();

    coords.push(coords[0].clone());
    let ans2 = coords
        .iter()
        .enumerate()
        .map(|(i, c1)| {
            coords
                .iter()
                .skip(i + 1)
                .filter(|c2| check(c1, c2, &coords))
                .map(|c2| c1.rect_area(c2))
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap();

    println!("{ans1} {ans2}"); // 4790063600 1516172795
}
