// cols are phages, rows are bacteria
// ways to increase speed:
// cache results of killed bacteria for each combination
// prune combinations that cannot possibly be the best
// multi threading
// use a more efficient data structure for the matrix/result
// assumes non empty matrix

use itertools::Itertools;
use std::collections:: {HashMap, HashSet};

type InfectionMatrix = HashMap<(usize, usize), bool>; // (bacteria, phage) -> is infected

type ExhaustiveSearchResults = HashMap<usize, (usize, Vec<Vec<usize>>)>;
// output is a hashmap of cocktail size as key to a vector of best cocktail)
pub fn exhaustive_search(
    matrix: &InfectionMatrix, 
    max_phages: usize,
    max_bacteria: usize,
    limit: usize,
    // limit: usize - include later
) -> Option<ExhaustiveSearchResults>
 {
    // early exit if no combinations can be made
    // add this check to parser
    if matrix.is_empty() {
        return None; 
    }

    let mut result: ExhaustiveSearchResults = HashMap::new();

    // loop through all possible sizes of phage cocktail
    for size in 1..=limit { 

        // for each size get all possible combinations of phages
        let combinations = (0..max_phages).combinations(size);

        // 1. find (combination, killed_bacteria)
        let all_results: Vec<_> = combinations
            .map(|combination| {
                // find the number of unique bacteria killed by each combination
                let killed_bacteria: HashSet<_> = (0..max_bacteria)
                    .filter(|&bacteria| {
                        combination.iter().any(|&phage| {
                            matrix.get(&(bacteria, phage)) == Some(&true)
                        })
                    })
                    .collect();
                (combination, killed_bacteria.len())
            })
            .collect();
        
        // 2. find the max killed bacteria count
        let max_killed_bacteria = all_results.iter().map(|(_, count)| *count).max().unwrap_or(0);

        // 3. find all combinations that have this max killed bacteria count
        let best_cocktails: Vec<_> = all_results
        .into_iter()
        .filter(|&(_, count)| count == max_killed_bacteria)
        .map(|(cocktail, _)| cocktail)
        .collect();

        // 4. add the best cocktails to the result
        result.insert(size, (max_killed_bacteria, best_cocktails));

        // 5. if the max killed bacteria count is the same as the number of bacteria, return the result (early exit)
        if max_killed_bacteria == max_bacteria {
            return Some(result);
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_matrix() {
        let matrix = HashMap::from([
            ((0,0), true),
            ((0,1), false),
            ((0,2), true),
            ((1,0), true),
            ((1,1), true),
            ((1,2), true),
            ((2,0), false),
            ((2,1), false),
            ((2,2), true),
        ]);

        let result = exhaustive_search(&matrix, 3, 3, 3);

        println!("{:?}", result);
        assert_eq!(
            result,
            Some(HashMap::from([(1, (3, vec![vec![2]]))]))
        );
    }
    
    #[test]
    fn test_empty_matrix() {
        let matrix = HashMap::from([]);
        let result = exhaustive_search(&matrix, 0, 0, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bigger_matrix() {
        let matrix = HashMap::from([
            ((0, 0), true),  // Bacteria 0, Phage 0
            ((0, 1), false), // Bacteria 0, Phage 1
            ((0, 2), true),  // Bacteria 0, Phage 2
            ((0, 3), true),  // Bacteria 0, Phage 3
            ((0, 4), false), // Bacteria 0, Phage 4
            ((0, 5), false), // Bacteria 0, Phage 5
            ((0, 6), false), // Bacteria 0, Phage 6
            ((0, 7), true),  // Bacteria 0, Phage 7
            ((0, 8), true),  // Bacteria 0, Phage 8
            ((1, 0), true),  // Bacteria 1, Phage 0
            ((1, 1), true),  // Bacteria 1, Phage 1
            ((1, 2), true),  // Bacteria 1, Phage 2
            ((1, 3), false), // Bacteria 1, Phage 3
            ((1, 4), false), // Bacteria 1, Phage 4
            ((1, 5), false), // Bacteria 1, Phage 5
            ((1, 6), false), // Bacteria 1, Phage 6
            ((1, 7), true),  // Bacteria 1, Phage 7
            ((1, 8), true),  // Bacteria 1, Phage 8
            ((2, 0), false), // Bacteria 2, Phage 0
            ((2, 1), true),  // Bacteria 2, Phage 1
            ((2, 2), true),  // Bacteria 2, Phage 2
            ((2, 3), true),  // Bacteria 2, Phage 3
            ((2, 4), false), // Bacteria 2, Phage 4
            ((2, 5), false), // Bacteria 2, Phage 5
            ((2, 6), false), // Bacteria 2, Phage 6
            ((2, 7), false), // Bacteria 2, Phage 7
            ((2, 8), true),  // Bacteria 2, Phage 8
            ((3, 0), true),  // Bacteria 3, Phage 0
            ((3, 1), true),  // Bacteria 3, Phage 1
            ((3, 2), true),  // Bacteria 3, Phage 2
            ((3, 3), false), // Bacteria 3, Phage 3
            ((3, 4), false), // Bacteria 3, Phage 4
            ((3, 5), false), // Bacteria 3, Phage 5
            ((3, 6), true),  // Bacteria 3, Phage 6
            ((3, 7), false), // Bacteria 3, Phage 7
            ((3, 8), false), // Bacteria 3, Phage 8
            ((4, 0), false), // Bacteria 4, Phage 0
            ((4, 1), false), // Bacteria 4, Phage 1
            ((4, 2), false),  // Bacteria 4, Phage 2
            ((4, 3), true),  // Bacteria 4, Phage 3
            ((4, 4), true),  // Bacteria 4, Phage 4
            ((4, 5), false), // Bacteria 4, Phage 5
            ((4, 6), true),  // Bacteria 4, Phage 6
            ((4, 7), true),  // Bacteria 4, Phage 7
            ((4, 8), true),  // Bacteria 4, Phage 8
            ]);


        let result = exhaustive_search(&matrix, 9, 5, 9);
        println!("{:?}", result);
    }
}
