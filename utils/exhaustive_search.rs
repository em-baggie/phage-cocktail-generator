// phage map refers to mapping of phage names to their indices in the inner vectors of the matrix
// bacteria map refers to mapping of bacteria names to their indices in the outer vectors of the matrix
// cols are phages, rows are bacteria

// TODO
// need to make sure search stops if a combination kills all bacteria

use itertools::Itertools;
use std::collections::HashMap;

type InfectionMatrix = Vec<Vec<bool>>;
type ExhaustiveSearchResults = HashMap<usize, Vec<((Vec<usize>, Vec<usize>))>>;
// output is a hashmap of cocktail size as key to a vector of tuples of (best cocktail, bacteria killed)
// map, filter, filter_map, collect, fold, and for_each.
pub fn exhaustive_search(
    matrix: &InfectionMatrix
    // limit: usize - include later
) -> Option<ExhaustiveSearchResults>
 {
    let mut result: ExhaustiveSearchResults = HashMap::new();
    let max_bacteria = matrix.len();
    let max_phages = matrix[0].len();

    for size in 1..max_phages + 1 { //iterate through all possible cocktail sizes
        let combinations = (0..max_phages).combinations(size); // for each size get all possible combinations of phages
        let mut max_bacteria_killed_by_cocktail = 0;

        for combination in combinations| { // need to find which combination kills the most bacteria and add the combination and bacteria killed to the result
            let mut num_bacteria_killed_by_combination = 0;
            let mut current_best_cocktail = Vec::new();
            let mut indices_bacteria_killed_by_combination = Vec::new();

            for bacteria in 0..max_bacteria {
                combination.iter().for_each(|phage| {
                    if matrix[bacteria][*phage] {
                        num_bacteria_killed_by_combination += 1;
                        indices_bacteria_killed_by_combination.push(bacteria);
                    }
                });
            }

                if num_bacteria_killed_by_combination > max_bacteria_killed_by_cocktail {
                    max_bacteria_killed_by_cocktail = num_bacteria_killed_by_combination;
                    current_best_cocktail.clear();
                    current_best_cocktail.push((combination, indices_bacteria_killed_by_combination));
                    
                } else if num_bacteria_killed_by_combination == max_bacteria_killed_by_cocktail {
                    current_best_cocktail.push((combination, indices_bacteria_killed_by_combination));
                }
            });

            if !current_best_cocktail.is_empty() {
                result.insert(size, current_best_cocktail);
            }

        };
        if max_bacteria_killed_by_cocktail == max_bacteria {
            break
        }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaustive_search() {
        
    }
}
