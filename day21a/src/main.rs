use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let data: Vec<_> = input.lines().map(|line| {
        let mut iter = line.split(" (contains ");
        let ingredients: HashSet<_> = iter.next().unwrap().split_whitespace().collect();
        let allergens: Vec<_> = iter.next().unwrap().split(')').next().unwrap().split(", ").collect();
        (ingredients, allergens)
    }).collect();

    let mut ingredients_by_allergen = HashMap::new();

    for (ingredients, allergens) in &data {
        for &allergen in allergens {
            ingredients_by_allergen.insert(allergen, match ingredients_by_allergen.get(allergen) {
                Some(ingredient_list) => ingredients.intersection(ingredient_list).cloned().collect(),
                None => ingredients.iter().cloned().collect(),
            });
        }
    }

    let possibly_allergenic: HashSet<_> = ingredients_by_allergen.values().flat_map(|ingredients| ingredients).cloned().collect();
    
    println!("{}", data.iter().map(|(ingredients, _)| {
        ingredients.iter().filter(|&&ingredient| !possibly_allergenic.contains(ingredient)).count()
    }).sum::<usize>());
}
