const PAPER: u8 = b'@';
const EMPTY: u8 = b'.';

fn is_accessible(wall: &[Vec<u8>], x: usize, y: usize) -> bool {
    wall[x][y] == PAPER
        && (-1..=1)
            .map(|dx| {
                (-1..=1)
                    .map(|dy| {
                        if dx == 0 && dy == 0 {
                            0
                        } else {
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            if (0..wall.len() as i32).contains(&nx)
                                && (0..wall[0].len() as i32).contains(&ny)
                            {
                                usize::from(wall[nx as usize][ny as usize] == PAPER)
                            } else {
                                0
                            }
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
            < 4
}

fn accessible_pos(wall: &[Vec<u8>]) -> Vec<(usize, usize)> {
    wall.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            (0..row.len())
                .filter(|c| is_accessible(wall, r, *c))
                .map(|c| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn remove(wall: &mut [Vec<u8>], pos: &[(usize, usize)]) {
    for &(r, c) in pos {
        assert_eq!(wall[r][c], PAPER);
        wall[r][c] = EMPTY;
    }
}

pub fn day04(input: &str) {
    let mut wall = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let ans1 = accessible_pos(&wall).len();
    let ans2 = (0..)
        .map_while(|_| {
            let acc = accessible_pos(&wall);
            if acc.is_empty() {
                None
            } else {
                remove(&mut wall, &acc);
                Some(acc.len())
            }
        })
        .sum::<usize>();

    println!("{ans1} {ans2}"); // 1445 8317
}
