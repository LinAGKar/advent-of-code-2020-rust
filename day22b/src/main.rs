use std::collections::VecDeque;
use std::io::Read;

fn play_combat(mut decks: Vec<VecDeque<u8>>, outermost: bool) -> (usize, Option<VecDeque<u8>>) {
    if !outermost && decks[0].iter().max() >= decks[1].iter().max() {
        return (0, None);
    }

    let mut seen = std::collections::HashSet::new();

    while decks.iter().all(|deck| !deck.is_empty()) {
        seen.insert(decks.clone());

        let top_cards: Vec<_> = decks.iter_mut().map(|deck| deck.pop_front().unwrap()).collect();

        let index = if top_cards.iter().zip(decks.iter()).all(|(&card, deck)| {
            deck.len() >= card as usize
        }) {
            play_combat(
                top_cards.iter().zip(decks.iter()).map(
                    |(&card, deck)| deck.iter().cloned().take(card as usize).collect()
                ).collect(),
                false,
            ).0
        } else {
            top_cards.iter().enumerate().max_by_key(|&(_, card)| card).unwrap().0
        };

        decks[index].push_back(top_cards[index]);
        decks[index].push_back(top_cards[1 - index]);

        if seen.contains(&decks) {
            break;
        }
    }

    let (winner, result) = decks.into_iter().enumerate().filter(|(_, deck)| !deck.is_empty()).next().unwrap();
    if outermost {
        (winner, Some(result))
    } else {
        (winner, None)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", play_combat(
        input.split("\n\n").map(|deck| {
            deck.lines().skip(1).map(|card| card.parse().unwrap()).collect()
        }).collect(),
        true,
    ).1.unwrap().iter().rev().zip(1..).map(|(&card, n)| n * card as usize).sum::<usize>());
}
