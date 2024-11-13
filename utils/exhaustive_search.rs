type InfectionMatrix = Vec<Vec<i32>>;

// phage map refers to mapping of phage names to their indices in the inner vectors of the matrix
// bacteria map refers to mapping of bacteria names to their indices in the outer vectors of the matrix

pub fn exhaustive_search(matrix: Vec<Vec<i32>>, phage_map: HashMap<&str, usize>, bacteria_map: HashMap<&str, usize>, limit: usize) -> Vec<(String, String)>
 {
    let mut result: (Vec<String>, Vec<String>) = (vec![], vec![]);

    let max_phages = matrix.len();
    let max_bacteria = matrix[0].len();

    // steps
    // 1. generate all possible combinations of phages
    //


    (vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaustive_search() {
        let mut example_matrix: InfectionMatrix = vec![
        // columns are phages
        vec![0, 1, 1], // rows are bacteria
        vec![0, 0, 0],
        vec![1, 1, 0],
    ];

    let phage_map: HashMap<&str, usize> = HashMap::new();
    let bacteria_map: HashMap<&str, usize> = HashMap::new();

    phage_map.insert("Phage 1", 0);
    phage_map.insert("Phage 2", 1);
    phage_map.insert("Phage 3", 2);

    bacteria_map.insert("Bacteria 1", 0);
    bacteria_map.insert("Bacteria 2", 1);
    bacteria_map.insert("Bacteria 3", 2);

    let result = exhaustive_search(example_matrix, phage_map, bacteria_map);
    assert_eq!(result, vec!["Bacteria 1", "Bacteria 3"]);
        
    }
}
