use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;


#[derive(Debug, Clone, Copy,  PartialEq, Eq, PartialOrd, Ord)]
enum CamelCard {
    JACK=1,
    TWO=2,
    THREE=3,
    FOUR=4,
    FIVE=5,
    SIX=6,
    SEVEN=7,
    EIGHT=8,
    NINE=9,
    TEN=10,
    QUEEN=12,
    KING=13,
    ACE=14
}

#[derive(Debug, Clone, Copy,  PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, Clone, Copy)]
struct Player {
    hand: [CamelCard; 5],
    bid: u64,
    result: HandType

}

fn compare_hands(hand1: &[CamelCard; 5], hand2: &[CamelCard; 5]) -> std::cmp::Ordering {
    for (card1, card2) in hand1.iter().zip(hand2.iter()) {
        match card1.cmp(card2) {
            std::cmp::Ordering::Equal => continue,
            ordering => return ordering.reverse(),
        }
    }

    std::cmp::Ordering::Equal
}

fn main() {
    let mut players: Vec<Player> = Vec::new();
    if let Ok(lines) = read_lines("./input2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                
                match ip.parse::<Player>() {
                    Ok(parsed_input) => {
                        // println!("Parsed Input: {:?}", parsed_input);
                        players.push(parsed_input);
                    }
                    Err(err) => {
                        println!("Error parsing input: {}", err);
                    }
                }
            }
        }
    }

    players.sort_by(|a, b| {
        if a.result != b.result {
            a.result.cmp(&b.result)
        } else {
            compare_hands(&a.hand, &b.hand)
        }
    });

    players.reverse();

    //println!("Sorted Players: {:?}", players);

    let mut total: u64 = 0;

    for (indexer, player ) in players.iter().enumerate() {
        // println!("Player rank: {} bid: {}", indexer + 1, player.bid);
        total += player.bid * (indexer as u64 + 1);
    }

    println!("Total: {}", total);

}

fn parse_card(cardstr: char) -> CamelCard {
    match cardstr {
        '2' => CamelCard::TWO,
        '3' => CamelCard::THREE,
        '4' => CamelCard::FOUR,
        '5' => CamelCard::FIVE,
        '6' => CamelCard::SIX,
        '7' => CamelCard::SEVEN,
        '8' => CamelCard::EIGHT,
        '9' => CamelCard::NINE,
        'T' => CamelCard::TEN,
        'J' => CamelCard::JACK,
        'Q' => CamelCard::QUEEN,
        'K' => CamelCard::KING,
        'A' => CamelCard::ACE,
        _ => panic!("Invalid card")
        
    }
}

fn get_hand_type(cards: [CamelCard; 5]) -> (HandType, [CamelCard; 5]) {
    let mut card_counts: [u8; 15] = [0; 15];
    
    for card in cards.iter() {
        card_counts[*card as usize] += 1;
    }

    let mut pair_count = 0;
    let mut three_count = 0;
    let mut four_count = 0;
    let mut five_count = 0;

    let mut card_no_1 = 0;
    let mut card_no_2 = 0;

    let mut amount_of_jacks = 0;

    cards.iter().filter(|&card| *card == CamelCard::JACK).for_each(|card| {
        amount_of_jacks += 1;
    });

    for (indexer, count) in card_counts.iter().enumerate() {
        match count {
            2 => { 
                pair_count += 1;
                if (card_no_1 == 0) {
                    card_no_1 = indexer;
                } else {
                    card_no_2 = indexer;
                }
            },
            3 => {
                three_count += 1;
                card_no_1 = indexer;
            },
            4 => {
                four_count += 1;
                card_no_1 = indexer;
            },
            5 => {
                five_count += 1;
                card_no_1 = indexer;
            },
            _ => ()
        }
    }

    if five_count == 1 {
        return (HandType::FiveOfAKind, cards);
    }

    if four_count == 1 {
        
        if (amount_of_jacks > 0) {
            return (HandType::FiveOfAKind, cards);
        }

        return (HandType::FourOfAKind, cards);
    }

    if three_count == 1 {

        if (amount_of_jacks == 1) {
            return (HandType::FourOfAKind, cards)
        }

        if (amount_of_jacks == 2) {
            return (HandType::FiveOfAKind, cards)
        }

        if pair_count == 1 {
            return (HandType::FullHouse, cards);
        }

        return (HandType::ThreeOfAKind, cards);
    }

    if pair_count == 2 {

        if (amount_of_jacks == 3) {
            return (HandType::FiveOfAKind, cards)
        }

        if (amount_of_jacks == 2) {
            return (HandType::FourOfAKind, cards)
        }

        if (amount_of_jacks == 1) {
            return (HandType::ThreeOfAKind, cards)
        }

        return (HandType::TwoPair, cards);
    }

    if pair_count == 1 {

        if (amount_of_jacks == 3) {
            return (HandType::FiveOfAKind, cards)
        }

        if (amount_of_jacks == 2) {
            return (HandType::FourOfAKind, cards)
        }

        if (amount_of_jacks == 1) {
            return (HandType::ThreeOfAKind, cards)
        }

        return (HandType::OnePair, cards);
    }

    return (HandType::HighCard, cards);

}



impl FromStr for Player {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let cards_str = parts.next();
        let bet_str = parts.next();
    
        let cards: [CamelCard; 5] = match cards_str {
            Some(s) if s.len() == 5 => {
                let mut chars = s.chars();
                [
                    parse_card(chars.next().unwrap()),
                    parse_card(chars.next().unwrap()),
                    parse_card(chars.next().unwrap()),
                    parse_card(chars.next().unwrap()),
                    parse_card(chars.next().unwrap()),
                ]
            }
            _ => return Err("Invalid cards format"),
        };
    
        let bet: u64 = match bet_str {
            Some(s) => match s.parse() {
                Ok(val) => val,
                Err(_) => return Err("Invalid bet value"),
            },
            None => return Err("Missing bet"),
        };

        let resulttype = get_hand_type(cards);

        Ok(Player { hand: resulttype.1, bid: bet, result: resulttype.0 })
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}