use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

static STRENGTH: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        hands.push(Hand::new(line));
    }

    hands.sort_by_key(|hand| hand.score);

    let mut score = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        let rank = i + 1;
        score += rank * hand.bid;
    }

    println!("score: {}", score);
}

#[derive(Debug)]
enum Rank {
    FIVE_OF_A_KIND,
    FOUR_OF_A_KIND,
    FULL_HOUSE,
    THREE_OF_A_KIND,
    TWO_PAIR,
    ONE_PAIR,
    HIGH_CARD,
}

impl Rank {
    fn new(cards: String) -> Rank {
        let freqs = get_frequencies(cards);
        if freqs.contains(&5) {
            return Rank::FIVE_OF_A_KIND;
        } else if freqs.contains(&4) {
            return Rank::FOUR_OF_A_KIND;
        } else if freqs.contains(&3) && freqs.contains(&2) {
            return Rank::FULL_HOUSE;
        } else if freqs.contains(&3) {
            return Rank::THREE_OF_A_KIND;
        } else if freqs.contains(&2) {
            let count = freqs.into_iter().filter(|freq| freq == &2).count();
            if count > 1 {
                return Rank::TWO_PAIR;
            } else {
                return Rank::ONE_PAIR;
            }
        } else {
            return Rank::HIGH_CARD;
        }
    }

    fn score(&self) -> usize {
        match self {
            Rank::FIVE_OF_A_KIND => 7,
            Rank::FOUR_OF_A_KIND => 6,
            Rank::FULL_HOUSE => 5,
            Rank::THREE_OF_A_KIND => 4,
            Rank::TWO_PAIR => 3,
            Rank::ONE_PAIR => 2,
            Rank::HIGH_CARD => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    rank: Rank,
    bid: usize,
    score: usize,
}

impl Hand {
    fn new(line: String) -> Hand {
        let (cards, bid) = Hand::parse_line(line);
        let rank: Rank = Rank::new(cards.clone());
        let cards_array: [char; 5] = cards
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .expect("Card array was not correct size?");
        let mut hand = Hand {
            cards: cards_array,
            rank,
            bid,
            score: 0,
        };
        hand.set_score();
        return hand;
    }

    fn parse_line(line: String) -> (String, usize) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let cards = split[0].to_owned();
        let bid = split[1].parse().unwrap();
        return (cards, bid);
    }

    fn set_score(&mut self) -> () {
        let base: usize = 13;
        let mut score: usize = 0;
        score += base.pow(6) * self.rank.score();
        for i in 0..5 {
            let power = 5 - i as u32;
            score += base.pow(power) * card_score(self.cards[i]);
        }
        self.score = score;
    }
}

fn freq_map_setup() -> HashMap<char, u8> {
    let mut freq_map: HashMap<char, u8> = HashMap::new();
    for card in STRENGTH {
        freq_map.insert(card, 0);
    }
    return freq_map;
}

fn count_cards(cards: String) -> HashMap<char, u8> {
    let mut freq_map = freq_map_setup();
    for card in cards.chars() {
        *freq_map.entry(card).or_insert(0) += 1;
    }
    return freq_map;
}

fn get_frequencies(cards: String) -> Vec<u8> {
    let freq_map = count_cards(cards);
    let mut freq_counter: Vec<u8> = Vec::new();
    for freq in freq_map.into_values() {
        if freq > 0 {
            freq_counter.push(freq);
        }
    }
    return freq_counter;
}

fn card_score(card: char) -> usize {
    let mut score = 0;
    for (i, strength) in STRENGTH.into_iter().enumerate() {
        if card == strength {
            score = 13 - i;
        }
    }
    return score;
}
