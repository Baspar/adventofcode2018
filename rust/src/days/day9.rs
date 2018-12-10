use regex::Regex;
use std::collections::HashMap;

pub fn part1 (input: &str) -> String {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let (nb_players, last_marbles) = re.captures_iter(input)
        .map(|cap| {
            let nb_players: i64 = (cap[1]).parse().unwrap();
            let last_marbles: i64 = (cap[2]).parse().unwrap();
            (nb_players, last_marbles)
        }).collect::<Vec<(i64, i64)>>()[0];

    let mut circle: Vec<i64> = vec![0];
    let mut current_index: i64 = 0;
    let mut scores: HashMap<i64, i64> = HashMap::new();

    for current_marble in 1..last_marbles {
        // println!("{:?}", circle.iter()
        //          .enumerate()
        //          .map(|(i, x)| {
        //              if i == current_index as usize {
        //                  format!("({})", x)
        //              } else {
        //                  format!("{}", x)
        //              }
        //          })
        //          .collect::<Vec<String>>());
        let current_player = ((current_marble - 1) % nb_players) + 1;
        if current_marble % 23 == 0 {
            let player_score = scores.entry(current_player).or_insert(0);
            *player_score += current_marble;
            current_index = (circle.len() as i64 + current_index - 7) % circle.len() as i64;
            *player_score += circle.remove(current_index as usize);
        } else {
            current_index = (current_index + 1) % circle.len() as i64;
            circle.insert((current_index + 1) as usize, current_marble);
            current_index += 1;
        }
    }

    let mut max_score = 0;
    let mut max_player = -1;
    for (&player, &score) in &scores {
        if score > max_score {
            max_score = score;
            max_player = player;
        }
    }
    println!("{:?}", scores);
    println!("{:?} {:?}", max_player, max_score);

    format!("{}", max_score)
}

pub fn part2 (input: &str) -> String {
    return String::from(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day9_part1 () {
        assert_eq!(super::part1("10 players; last marble is worth 1618 points"), "8317");
        assert_eq!(super::part1("13 players; last marble is worth 7999 points"), "146373");
        assert_eq!(super::part1("21 players; last marble is worth 6111 points"), "54718");
        assert_eq!(super::part1("30 players; last marble is worth 5807 points"), "37305");
    }

    #[test]
    fn day9_part2 () {
        assert_eq!(0, 0);
    }
}
