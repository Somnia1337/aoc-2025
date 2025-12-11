use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, default_solver, variable,
};

#[derive(Debug)]
struct Machine {
    indicator: Vec<bool>,
    toggles: Vec<Vec<bool>>,
    joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let bytes = value.bytes().collect::<Vec<u8>>();
        let i1 = bytes.iter().position(|b| *b == b'[').unwrap_or_default();
        let i2 = bytes.iter().rposition(|b| *b == b']').unwrap_or_default();
        let t1 = bytes.iter().position(|b| *b == b'(').unwrap_or_default();
        let t2 = bytes.iter().rposition(|b| *b == b')').unwrap_or_default();
        let j1 = bytes.iter().position(|b| *b == b'{').unwrap_or_default();
        let j2 = bytes.iter().rposition(|b| *b == b'}').unwrap_or_default();

        let i = &value[i1 + 1..i2];
        let t = &value[t1..t2];
        let j = &value[j1 + 1..j2];
        let n = i.len();

        let indicator = i.bytes().map(|b| b == b'#').collect();
        let toggles = t
            .split(')')
            .map(|t| {
                let mut toggle = vec![false; n];
                t.trim()
                    .strip_prefix('(')
                    .unwrap()
                    .split(',')
                    .map(|p| p.parse::<usize>().unwrap())
                    .for_each(|p| toggle[p] = true);
                toggle
            })
            .collect();
        let joltage = j.split(',').map(|j| j.parse::<usize>().unwrap()).collect();

        Self {
            indicator,
            toggles,
            joltage,
        }
    }
}

impl Machine {
    fn indicator_min_toggle_seq(&self) -> Vec<bool> {
        let n = self.toggles.len();
        (0..2usize.pow(n as u32))
            .filter_map(|m| {
                let mut ops = vec![false; n];
                let mut m = m;
                let mut i = 0;
                while m > 0 {
                    if m & 1 == 1 {
                        ops[i] = true;
                    }
                    m >>= 1;
                    i += 1;
                }

                let mut ind = self.indicator.clone();
                ops.iter()
                    .enumerate()
                    .filter(|(_, op)| **op)
                    .for_each(|(i, _)| {
                        self.toggles[i]
                            .iter()
                            .enumerate()
                            .filter(|(_, t)| **t)
                            .for_each(|(i, _)| ind[i] ^= true);
                    });

                if ind.iter().all(|on| !on) {
                    Some(ops.clone())
                } else {
                    None
                }
            })
            .min_by_key(|v| v.iter().filter(|t| **t).count())
            .unwrap_or_default()
    }

    fn joltage_min_toggle_seq(&self) -> Vec<usize> {
        let mut vars = ProblemVariables::new();
        let toggles = (0..self.toggles.len())
            .map(|_| vars.add(variable().integer().min(0).max(1000)))
            .collect::<Vec<Variable>>();
        let mut problem = vars
            .minimise(toggles.iter().sum::<Expression>())
            .using(default_solver);

        for (i, &tar_jolt) in self.joltage.iter().enumerate() {
            let constraint = self
                .toggles
                .iter()
                .enumerate()
                .filter(|(_, tog)| tog[i])
                .map(|(j, _)| toggles[j])
                .sum::<Expression>();

            problem = problem.with(constraint.eq(tar_jolt as f64));
        }

        let solution = problem.solve().unwrap();

        toggles
            .iter()
            .map(|&var| solution.value(var).round() as usize)
            .collect()
    }
}

pub fn day10(input: &str) {
    let machines = input.lines().map(Machine::from).collect::<Vec<Machine>>();

    let ans1 = machines
        .iter()
        .map(|m| m.indicator_min_toggle_seq().iter().filter(|t| **t).count())
        .sum::<usize>();
    let ans2 = machines
        .iter()
        .map(|m| m.joltage_min_toggle_seq().iter().sum::<usize>())
        .sum::<usize>();

    println!("{ans1} {ans2}"); // 509 20083
}
