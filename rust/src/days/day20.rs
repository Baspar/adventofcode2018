use std::collections::{HashMap,VecDeque};

struct ExplorationNode {
    x: i64,
    y: i64,
    distance: usize
}

fn make_map (input: &str) -> HashMap<i64, HashMap<i64, usize>> {
    let mut map = HashMap::new();
    let mut positions = VecDeque::new();
    let mut x = 0;
    let mut y = 0;
    let mut distance = 0;
    for c in input.chars() {
        match c {
            'N' => { y = y - 1; distance = distance + 1 },
            'S' => { y = y + 1; distance = distance + 1 },
            'E' => { x = x + 1; distance = distance + 1 },
            'W' => { x = x - 1; distance = distance + 1 },
            '|' => {
                let old_pos = (positions.back().unwrap() as &ExplorationNode).clone();
                x = old_pos.x;
                y = old_pos.y;
                distance = old_pos.distance;
            },
            '(' => { positions.push_back(ExplorationNode{x, y, distance}) },
            ')' => {
                let old_pos = positions.pop_back().unwrap();
                x = old_pos.x;
                y = old_pos.y;
                distance = old_pos.distance;
            },
            _ => {}
        }

        let is_best_path = match map.entry(x).or_insert(HashMap::new()).get(&y) {
            Some(prev_distance) => distance < *prev_distance,
            _ => true
        };
        if is_best_path {
            map.get_mut(&x).unwrap().insert(y, distance);
        }
    }

    return map;
}

pub fn part1 (input: &str) -> String {
    let map = make_map(input);

    let mut max_distance = 0;
    for (_, m) in map {
        for (_, distance) in m {
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    return format!("{}", max_distance);
}

pub fn part2 (input: &str) -> String {
    let map = make_map(input);

    let mut nb_above_1000 = 0;
    for (_, m) in map {
        for (_, distance) in m {
            if distance >= 1000 {
                nb_above_1000 += 1;
            }
        }
    }
    return format!("{}", nb_above_1000);
}


#[cfg(test)]
mod tests {
    #[test]
    fn day20_part1 () {
        assert_eq!(super::part1("^WNE$"), "3");
        assert_eq!(super::part1("^ENWWW(NEEE|SSE(EE|N))$"), "10");
        assert_eq!(super::part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), "18");
        assert_eq!(super::part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), "23");
        assert_eq!(super::part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), "31");
    }

    #[test]
    fn day20_part2 () {
        assert_eq!(0, 0);
    }
}
