use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut decks: Vec<std::collections::VecDeque<u8>> = input.split("\n\n").map(|deck| {
        deck.lines().skip(1).map(|card| card.parse().unwrap()).collect()
    }).collect();

    while decks.iter().all(|deck| !deck.is_empty()) {
        let top_cards: Vec<_> = decks.iter_mut().map(|deck| deck.pop_front().unwrap()).collect();
        let index = top_cards.iter().enumerate().max_by_key(|&(_, card)| card).unwrap().0;
        decks[index].push_back(top_cards[index]);
        decks[index].push_back(top_cards[1 - index]);
    }

    println!("{}", decks.iter().map(
        |deck| deck.iter().rev().zip(1..).map(|(&card, n)| n * card as usize).sum::<usize>()
    ).max().unwrap());
}
