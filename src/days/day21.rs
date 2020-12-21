use std::collections::HashMap;

use crate::puzzle::{io, File, Puzzle};

pub struct Day21;

fn parse_input(input: &Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut foods = vec![];
    for line in input {
        let split: Vec<&str> = line.split("(contains").collect();
        let food_ingredients: Vec<String> =
            split[0].trim().split(" ").map(|x| x.to_string()).collect();
        let allergens: Vec<String> = split[1]
            .trim()
            .split(")")
            .nth(0)
            .unwrap()
            .split(", ")
            .map(|x| x.to_string())
            .collect();
        foods.push((food_ingredients, allergens));
    }
    foods
}

impl Day21 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let mut ingredients: HashMap<String, Vec<String>> = HashMap::new();
        let foods = parse_input(&input);

        // Populate a hash table with allergen candidates for each ingredient.
        for (food_ingredients, allergens) in &foods {
            for ingredient in food_ingredients {
                if let Some(ingredient_allergens) = ingredients.get_mut(ingredient) {
                    for allergen in allergens {
                        if !ingredient_allergens.contains(&allergen) {
                            ingredient_allergens.push(allergen.clone());
                        }
                    }
                } else {
                    ingredients.insert(ingredient.clone(), allergens.clone());
                }
            }
        }

        // When an allergen is found in one food, remove that allergen candidate from all ingredients not in that food.
        for (food_ingredients, allergens) in &foods {
            for allergen in allergens {
                for (ingredient, ingredient_allergens) in &mut ingredients {
                    if !food_ingredients.contains(&ingredient) {
                        ingredient_allergens.retain(|allergen2| allergen2 != allergen);
                    }
                }
            }
        }

        // The safe ingredients are now the ingredients with an empty allergen candidate list.
        let safe_ingredients: Vec<String> = ingredients
            .iter()
            .filter(|(_, allergens)| allergens.is_empty())
            .map(|(ingredient, _)| ingredient.clone())
            .collect();

        // Count how often the safe ingredients appear in the food recipes.
        foods.iter().fold(0, |count, (ingredients, _)| {
            count
                + ingredients.iter().fold(0, |inner_count, ingredient| {
                    if safe_ingredients.contains(ingredient) {
                        inner_count + 1
                    } else {
                        inner_count
                    }
                })
        })
    }

    fn solve_part2(&self, input: Vec<String>) -> String {
        let mut ingredients: HashMap<String, Vec<String>> = HashMap::new();
        let foods = parse_input(&input);

        // Populate a hash table with allergen candidates for each ingredient.
        for (food_ingredients, allergens) in &foods {
            for ingredient in food_ingredients {
                if let Some(ingredient_allergens) = ingredients.get_mut(ingredient) {
                    for allergen in allergens {
                        if !ingredient_allergens.contains(&allergen) {
                            ingredient_allergens.push(allergen.clone());
                        }
                    }
                } else {
                    ingredients.insert(ingredient.clone(), allergens.clone());
                }
            }
        }

        // When an allergen is found in one food, remove that allergen candidate from all ingredients not in that food.
        for (food_ingredients, allergens) in &foods {
            for allergen in allergens {
                for (ingredient, ingredient_allergens) in &mut ingredients {
                    if !food_ingredients.contains(&ingredient) {
                        ingredient_allergens.retain(|x| x != allergen);
                    }
                }
            }
        }

        // When an ingredient only has one allergen candidate left it can be removed from all
        // other ingredients candidate lists. Keep cleaning up the candidate lists like this until
        // all allergens are found.
        ingredients.retain(|_, allergens| !allergens.is_empty());
        let mut go_again = true;
        while go_again {
            go_again = false;
            for (ingredient, allergens) in ingredients.clone() {
                if allergens.len() == 1 {
                    for (ingredient2, allergens2) in &mut ingredients {
                        if *ingredient2 != ingredient {
                            let len_before = allergens2.len();
                            allergens2.retain(|allergens3| *allergens3 != allergens[0]);
                            if allergens2.len() != len_before {
                                go_again = true;
                            }
                        }
                    }
                }
            }
        }

        // Create a sorted ingredient vector and return a String version of it.
        let mut ingredients_vector: Vec<_> = ingredients.iter().collect();
        ingredients_vector.sort_by(|x, y| x.1[0].cmp(&y.1[0]));
        let mut result = String::new();
        for (i, (ingredient, _)) in ingredients_vector.iter().enumerate() {
            if i > 0 {
                result += ",";
            }
            result += ingredient;
        }
        result
    }
}

impl Puzzle for Day21 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let input: Vec<String> = lines.expect("No input file").map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(input.clone()).to_string(),
            self.solve_part2(input.clone()).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! string_vec {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day21 {}.solve_part1(string_vec!(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
                "trh fvjkl sbzzf mxmxvkd (contains dairy)",
                "sqjhc fvjkl (contains soy)",
                "sqjhc mxmxvkd sbzzf (contains fish)"
            )),
            5
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day21 {}.solve_part2(string_vec!(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
                "trh fvjkl sbzzf mxmxvkd (contains dairy)",
                "sqjhc fvjkl (contains soy)",
                "sqjhc mxmxvkd sbzzf (contains fish)"
            )),
            "mxmxvkd,sqjhc,fvjkl"
        );
    }
}
