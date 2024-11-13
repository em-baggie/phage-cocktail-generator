// phage map refers to mapping of phage names to their indices in the inner vectors of the matrix
// bacteria map refers to mapping of bacteria names to their indices in the outer vectors of the matrix
// cols are phages, rows are bacteria

use itertools::Itertools;

type InfectionMatrix = Vec<Vec<bool>>;

type ExhaustiveSearchResults = HashMap<usize, Vec<(String, String, usize)>>;

// output is a hashmap of cocktail size to a vector of tuples of (phage name, bacteria name, number of bacteria killed)

pub fn exhaustive_search(
    matrix: &InfectionMatrix,
    phage_map: &HashMap<&str, usize>,
    bacteria_map: &HashMap<&str, usize>,
    // limit: usize - include later
) -> Option<ExhaustiveSearchResults>
 {
    let mut result: ExhaustiveSearchResults = HashMap::new();

    let max_phages = matrix.len();
    // let max_bacteria = matrix[0].len();

    // steps
    // loop for all phage cocktail sizes from 1 to max
    // check all possible combinations in that cocktail size, and output the one that kills the most bacteria (may have multiple with same number of bacteria killed)
    // add result to hashmap
    // return hashmap
    // handle if no combination kills any bacteria

    for size in 1..max_phages+1 {
        let combinations = (0..max_phages).combinations(size);

        for combination in combinations {
            let mut max_bacteria_killed = 0;
            let mut best_cocktail = Vec::new();

            for bacteria in 0..matrix[0].len() {
                let mut bacteria_killed = 0;
                for phages in combination {
                    if matrix[bacteria][phages] {
                        bacteria_killed += 1;
                    }
                }
                if bacteria_killed > max_bacteria_killed {
                    max_bacteria_killed = bacteria_killed;
                    best_cocktail = combination;
                }
            }
        // need to find names of phages in best cocktail from using indices 
        // need to find names of bacteria in best cocktail from using indices
        // add to result hashmap - add cocktail size as key, vector of tuples (phage name, bacteria name, bacteria killed) as value
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaustive_search() {
        
    }
}
