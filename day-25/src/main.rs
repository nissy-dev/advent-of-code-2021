use std::fs;

fn main() {
    let src = fs::read_to_string("input-part1.txt").unwrap();
    let mut map = parse(&src);

    let mut cnt = 0;
    loop {
        let (new_map, update_cnt) = map_update(&mut map);
        cnt += 1;
        if update_cnt == 0 {
            break;
        }
        map = new_map;
    }

    println!("part 1 ans: {}", cnt);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapValue {
    Empty,
    EastSeaCucumbers,
    SouthSeaCucumbers,
}

fn parse(src: &str) -> Vec<Vec<MapValue>> {
    src.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => MapValue::Empty,
                    '>' => MapValue::EastSeaCucumbers,
                    'v' => MapValue::SouthSeaCucumbers,
                    _ => panic!("Unknown character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn map_update(map: &mut Vec<Vec<MapValue>>) -> (Vec<Vec<MapValue>>, usize) {
    let mut update_cnt = 0;
    let map_south_length = map.len();
    let map_east_length = map[0].len();

    // east
    let mut update_east = vec![];
    for s in 0..map_south_length {
        for e in 0..map_east_length {
            if &map[s][e] == &MapValue::EastSeaCucumbers {
                let next_e = if e == map_east_length - 1 { 0 } else { e + 1 };
                if &map[s][next_e] == &MapValue::Empty {
                    update_east.push((s, e, next_e));
                }
            }
        }
    }

    for (s, e, next_e) in update_east {
        map[s][e] = MapValue::Empty;
        map[s][next_e] = MapValue::EastSeaCucumbers;
        update_cnt += 1;
    }

    // south
    let mut update_south = vec![];
    for e in 0..map_east_length {
        for s in 0..map_south_length {
            if &map[s][e] == &MapValue::SouthSeaCucumbers {
                let next_s = if s == map_south_length - 1 { 0 } else { s + 1 };
                if &map[next_s][e] == &MapValue::Empty {
                    update_south.push((s, e, next_s));
                }
            }
        }
    }

    for (s, e, next_s) in update_south {
        map[s][e] = MapValue::Empty;
        map[next_s][e] = MapValue::SouthSeaCucumbers;
        update_cnt += 1;
    }

    (map.clone(), update_cnt)
}
