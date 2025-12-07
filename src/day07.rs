pub fn day07(input: &str) {
    let n = input.lines().next().unwrap().len();
    let mut beams = vec![false; n];
    let mut weight = vec![0; n];
    let mut ans1 = 0;

    let start = input
        .lines()
        .next()
        .unwrap()
        .bytes()
        .position(|b| b == b'S')
        .unwrap();
    beams[start] = true;
    weight[start] = 1;
    input.lines().skip(1).for_each(|l| {
        let splits = l
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'^')
            .collect::<Vec<(usize, u8)>>();

        for &(i, _) in &splits {
            if beams[i] {
                beams[i] = false;
                beams[i.saturating_sub(1)] = true;
                beams[i.saturating_add(1)] = true;
                ans1 += 1;
            }
        }

        let mut new_weight = vec![0; n];
        let splits = splits.iter().map(|&(i, _)| i).collect::<Vec<usize>>();
        beams
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .for_each(|(i, _)| {
                if weight[i] != 0 {
                    new_weight[i] = weight[i];
                }

                let j = i.saturating_sub(1);
                let k = i.saturating_add(1);

                if splits.contains(&j) {
                    new_weight[i] += weight[j];
                }
                if splits.contains(&k) {
                    new_weight[i] += weight[k];
                }
            });
        weight = new_weight;
    });
    let ans2 = weight.iter().sum::<usize>();

    println!("{ans1} {ans2}"); // 1546 13883459503480
}
