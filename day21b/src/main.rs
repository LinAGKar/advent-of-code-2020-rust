use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ingredients_by_allergen = HashMap::<_, HashSet<&str>>::new();

    for line in input.lines() {
        let mut iter = line.split(" (contains ");
        let ingredients: Vec<_> = iter.next().unwrap().split_whitespace().collect();

        for allergen in iter.next().unwrap().split(')').next().unwrap().split(", ") {
            ingredients_by_allergen.insert(allergen, match ingredients_by_allergen.get(allergen) {
                Some(ingredient_list) => ingredients.iter().cloned().filter(|&ingredient| {
                    ingredient_list.contains(ingredient)
                }).collect(),
                None => ingredients.iter().cloned().collect(),
            });
        }
    }

    let mut ingredients_by_allergen_done = BTreeMap::new();

    while ingredients_by_allergen.len() > 0 {
        let mut consumed_ingredients = Vec::new();

        ingredients_by_allergen = ingredients_by_allergen.into_iter().filter(|(allergen, ingredients)| {
            if ingredients.len() == 1 {
                let ingredient = *ingredients.iter().next().unwrap();
                consumed_ingredients.push(ingredient);
                ingredients_by_allergen_done.insert(*allergen, ingredient);
                false
            } else if ingredients.len() == 0 {
                panic!("");
            } else {
                true
            }
        }).collect();

        for ingredient in &consumed_ingredients {
            for (_, ingredient_list) in &mut ingredients_by_allergen {
                ingredient_list.remove(ingredient);
            } 
        }
    }

    let allergenic: Vec<_> = ingredients_by_allergen_done.values().cloned().collect();
    println!("{}", (&allergenic).join(","));
}
