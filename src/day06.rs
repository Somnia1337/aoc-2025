pub fn day06(input: &str) {
    let nums_lines = input.lines().count() - 1;
    let nums = input
        .lines()
        .take(nums_lines)
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let ops_line = input.lines().last().unwrap();
    let ops = ops_line.split_ascii_whitespace().collect::<Vec<&str>>();

    assert!(nums.iter().all(|v| v.len() == ops.len()));

    let n = nums[0].len();
    let ans1 = (0..n)
        .map(|i| {
            nums.iter()
                .map(|v| v[i])
                .fold(usize::from(ops[i] != "+"), |ans, num| {
                    if ops[i] == "+" { ans + num } else { ans * num }
                })
        })
        .sum::<usize>();

    let mut i = input.lines().map(str::len).max().unwrap();
    let mut buffer = vec![];
    let mut ans2 = 0;
    loop {
        let num = input
            .lines()
            .take(nums_lines)
            .map(|l| l.chars().nth(i).unwrap_or(' '))
            .collect::<String>();
        let num = num.trim();
        if !num.is_empty() {
            buffer.push(num.parse::<usize>().unwrap());
        }

        let op = ops_line.chars().nth(i).unwrap_or(' ');
        if op == '+' {
            ans2 += buffer.iter().fold(0, |ans, num| ans + *num);
            buffer.clear();
        } else if op == '*' {
            ans2 += buffer.iter().product::<usize>();
            buffer.clear();
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    println!("{ans1} {ans2}"); // 4693419406682 9029931401920
}
