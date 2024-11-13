// phage map refers to mapping of phage names to their indices in the inner vectors of the matrix
// bacteria map refers to mapping of bacteria names to their indices in the outer vectors of the matrix
// cols are phages, rows are bacteria

use itertools::Itertools;

type InfectionMatrix = Vec<Vec<bool>>;

type ExhaustiveSearchResults = HashMap<usize, Vec<((Vec<usize>, Vec<usize>))>>;
// output is a hashmap of cocktail size to a vector of tuples of (cocktail, bacteria killed)

pub fn exhaustive_search(
    matrix: &InfectionMatrix
    // limit: usize - include later
) -> Option<ExhaustiveSearchResults>
 {
    let mut result: ExhaustiveSearchResults = HashMap::new();

    let max_phages = matrix.len();
    let max_bacteria = matrix[0].len();

    for size in 1..max_phages+1 {
        let combinations = (0..max_phages).combinations(size);

        for combination in combinations {
            let mut max_bacteria_killed = 0;
            let mut best_bacteria_indices = Vec::new();
            let mut best_phage_indices = Vec::new();
            let mut best_cocktails = Vec::new(); // store all best cocktails, contains vector

            for bacteria in max_bacteria {
                let mut num_bacteria_killed = 0;
                let mut killed_bacteria_indices = Vec::new();

                for phages in &combination {
                    if matrix[bacteria][phages] {
                        bacteria_killed += 1;
                        killed_bacteria_indices.push(phages);xscz
                    }
                }

                if bacteria_killed > max_bacteria_killed {
                    max_bacteria_killed = bacteria_killed;
                    best_cocktails.clear(); // Clear previous bests
                    best_phage_indices = combination.clone();
                    best_bacteria_indices = killed_bacteria_indices;
                    best_cocktails.push((best_phage_indices, best_bacteria_indices));

                } else if bacteria_killed == max_bacteria_killed {
                    best_phage_indices = combination.clone();
                    best_bacteria_indices = killed_bacteria_indices;
                    best_cocktails.push((best_phage_indices, best_bacteria_indices));
                }
            }
            result.insert(size, best_cocktails);
        }
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
