const CYCLE: i32 = 100;
const START: i32 = 50;

#[derive(Clone)]
struct Dial {
    position: i32,
    rotations: Vec<i32>,
    op: usize,
}

impl From<&str> for Dial {
    fn from(value: &str) -> Self {
        let rotations = value
            .lines()
            .filter_map(|l| l.replace('L', "-").replace('R', "").parse::<i32>().ok())
            .collect();

        Self {
            position: START,
            rotations,
            op: 0,
        }
    }
}

impl Dial {
    fn rotate(&mut self) -> (bool, usize) {
        if self.op == self.rotations.len() {
            (false, 0)
        } else {
            let cnt = self.zero_count();
            self.position = (self.position + self.rotations[self.op]).rem_euclid(CYCLE);
            self.op += 1;

            (true, cnt)
        }
    }

    fn zero_count(&self) -> usize {
        let rotation = self.rotations[self.op];
        let thres = self.distance_to_zero(rotation);

        (if rotation.abs() >= thres {
            1 + (rotation.abs() - thres) / CYCLE
        } else {
            0
        }) as usize
    }

    const fn distance_to_zero(&self, rotation: i32) -> i32 {
        if self.position == 0 || rotation >= 0 {
            CYCLE - self.position
        } else {
            self.position
        }
    }
}

pub fn day01(input: &str) {
    let mut dial = Dial::from(input);
    let (mut ans1, mut ans2) = (0, 0);

    loop {
        let res = dial.rotate();
        if !res.0 {
            break;
        }
        if dial.position == 0 {
            ans1 += 1;
        }
        ans2 += res.1;
    }

    println!("{ans1} {ans2}"); // 1036 6228
}
