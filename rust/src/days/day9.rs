use regex::Regex;
use std::collections::VecDeque;

fn parse_input(input: &str) -> (usize, usize) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let nb_players: usize = (cap[1]).parse().unwrap();
            let last_marbles: usize = (cap[2]).parse().unwrap();
            (nb_players, last_marbles)
        }).collect::<Vec<(usize, usize)>>()[0]

}

fn rotate_right(circle: &mut VecDeque<usize>, i: usize) {
    for _ in 0..i {
        let i = circle.pop_back().unwrap();
        circle.push_front(i);
    }
}
fn rotate_left(circle: &mut VecDeque<usize>, i: usize) {
    for _ in 0..i {
        let i = circle.pop_front().unwrap();
        circle.push_back(i);
    }
}

fn main(nb_players: usize, last_marbles: usize) -> usize {
    let mut circle: VecDeque<usize> = VecDeque::new();
    circle.push_back(0);
    let mut scores = vec![0; nb_players];

    for current_marble in 1..last_marbles+1 {
        let current_player = (current_marble - 1) % nb_players;
        if current_marble % 23 == 0 {
            scores[current_player] += current_marble;
            rotate_right(&mut circle, 7);
            scores[current_player] += circle.pop_front().unwrap();
        } else {
            rotate_left(&mut circle, 2);
            circle.push_front(current_marble);
        }
    }

    let mut max_score = 0;
    for score in scores {
        if score > max_score {
            max_score = score;
        }
    };

    max_score
}

pub fn part1 (input: &str) -> String {
    let (nb_players, last_marbles) = parse_input(input);
    let res = main(nb_players, last_marbles);
    format!("{}", res)
}

pub fn part2 (input: &str) -> String {
    let (nb_players, last_marbles) = parse_input(input);
    let res = main(nb_players, 100 * last_marbles);
    format!("{}", res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn day9_part1 () {
        assert_eq!(super::part1("10 players; last marble is worth 1618 points"), "8317");
        // assert_eq!(super::part1("13 players; last marble is worth 7999 points"), "146373");
        // assert_eq!(super::part1("17 players; last marble is worth 1104 points"), "2764");
        // assert_eq!(super::part1("21 players; last marble is worth 6111 points"), "54718");
        // assert_eq!(super::part1("30 players; last marble is worth 5807 points"), "37305");
    }

    #[test]
    fn day9_part2 () {
        assert_eq!(0, 0);
    }
}
