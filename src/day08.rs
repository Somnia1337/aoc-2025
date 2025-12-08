const PAIRS: usize = 1000;
const MUL: usize = 3;

struct JBox {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for JBox {
    fn from(value: &str) -> Self {
        let p = value
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        Self {
            x: p[0],
            y: p[1],
            z: p[2],
        }
    }
}

impl JBox {
    const fn dist_sq(&self, other: &Self) -> u64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn find(root: &mut [usize], i: usize) -> usize {
    if root[i] == i {
        i
    } else {
        root[i] = find(root, root[i]);
        root[i]
    }
}

fn union(root: &mut [usize], x: usize, y: usize) {
    root[find(root, x)] = find(root, y);
}

fn sizes(root: &mut [usize]) -> Vec<usize> {
    let mut count = vec![0; root.len()];
    (0..root.len()).for_each(|i| count[find(root, i)] += 1);

    count
}

fn all_connected(root: &mut [usize]) -> bool {
    let r = find(root, 0);

    (1..root.len()).all(|i| find(root, i) == r)
}

pub fn day08(input: &str) {
    let jboxes = input.lines().map(JBox::from).collect::<Vec<JBox>>();

    let mut dists = jboxes
        .iter()
        .enumerate()
        .flat_map(|(i, b1)| {
            jboxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(|(j, b2)| (i, j, b1.dist_sq(b2)))
                .collect::<Vec<(usize, usize, u64)>>()
        })
        .collect::<Vec<(usize, usize, u64)>>();
    dists.sort_unstable_by_key(|d| d.2);

    let ans1 = part1(&jboxes, &dists);
    let ans2 = part2(&jboxes, &dists);

    println!("{ans1} {ans2}"); // 122430 8135565324
}

fn part1(jboxes: &[JBox], dists: &[(usize, usize, u64)]) -> usize {
    let mut r = (0..jboxes.len()).collect::<Vec<usize>>();
    dists
        .iter()
        .take(PAIRS)
        .for_each(|d| union(&mut r, d.0, d.1));

    let mut sizes = sizes(&mut r);
    sizes.sort_unstable_by(|s1, s2| s2.cmp(s1));

    sizes.iter().take(MUL).fold(1, |ans, s| s * ans)
}

fn part2(jboxes: &[JBox], dists: &[(usize, usize, u64)]) -> u64 {
    let mut r = (0..jboxes.len()).collect::<Vec<usize>>();
    for d in dists {
        union(&mut r, d.0, d.1);
        if all_connected(&mut r) {
            return jboxes[d.0].x * jboxes[d.1].x;
        }
    }

    unreachable!()
}
