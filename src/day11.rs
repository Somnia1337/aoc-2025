use std::collections::HashMap;

const YOU: &str = "you";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";
const OUT: &str = "out";

struct Node {
    name: String,
    out: Vec<String>,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let (name, out) = value.split_once(':').unwrap_or_default();
        let name = name.to_string();
        let out = out
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect();

        Self { name, out }
    }
}

fn dfs_1(map: &[Vec<usize>], i: usize, out: usize) -> usize {
    if i == out {
        1
    } else {
        map[i].iter().map(|n| dfs_1(map, *n, out)).sum()
    }
}

fn dfs_2(
    map: &[Vec<usize>],
    i: usize,
    out: usize,
    dac: usize,
    fft: usize,
    v_dac: bool,
    v_fft: bool,
    mem: &mut HashMap<(usize, bool, bool), usize>,
) -> usize {
    if i == out {
        return usize::from(v_dac && v_fft);
    }

    let key = (i, v_dac, v_fft);
    if let Some(&v) = mem.get(&key) {
        return v;
    }

    let v_dac = v_dac || i == dac;
    let v_fft = v_fft || i == fft;

    let result = map[i]
        .iter()
        .map(|&n| dfs_2(map, n, out, dac, fft, v_dac, v_fft, mem))
        .sum();

    mem.insert(key, result);
    result
}

pub fn day11(input: &str) {
    let map = input.lines().map(Node::from).collect::<Vec<Node>>();

    let name_id = map
        .iter()
        .enumerate()
        .map(|(i, n)| (n.name.as_str(), i))
        .collect::<HashMap<&str, usize>>();

    assert!(name_id.contains_key(YOU));
    assert!(name_id.contains_key(SVR));
    assert!(name_id.contains_key(DAC));
    assert!(name_id.contains_key(FFT));
    assert!(!name_id.contains_key(OUT));

    let you = *name_id.get(YOU).unwrap();
    let svr = *name_id.get(SVR).unwrap();
    let dac = *name_id.get(DAC).unwrap();
    let fft = *name_id.get(FFT).unwrap();
    let out = name_id.len();

    let map = map
        .iter()
        .map(|n| {
            n.out
                .iter()
                .filter_map(|o| {
                    if o.eq(OUT) {
                        Some(&out)
                    } else {
                        name_id.get(o.as_str())
                    }
                })
                .copied()
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let ans1 = dfs_1(&map, you, out);
    let ans2 = dfs_2(&map, svr, out, dac, fft, false, false, &mut HashMap::new());

    println!("{ans1} {ans2}"); // 670 332052564714990
}
