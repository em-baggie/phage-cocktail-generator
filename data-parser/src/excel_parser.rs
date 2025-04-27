use calamine::{ Reader, open_workbook, Xlsx };
use std::collections::HashMap;

// make sure parsers discard empty matrices, and do not include empty values in the hashmaps

#[derive(Debug)]
pub struct ExcelData {
    phage_map: HashMap<usize, String>,
    bacteria_map: HashMap<usize, String>,
    max_phages: usize,
    max_bacteria: usize,
    matrix: HashMap<(usize, usize), bool>,
}

impl ExcelData {
    // New method to access the data
    pub fn get_data(&self) -> (&HashMap<usize, String>, &HashMap<usize, String>, usize, usize, &HashMap<(usize, usize), bool>) {
        (&self.phage_map, &self.bacteria_map, self.max_phages, self.max_bacteria, &self.matrix)
    }
}

pub fn read_excel(file_path: &str, sheet_name: &str, start: (u32, u32), end: (u32, u32)) -> Result<ExcelData, String> {
    println!("Attempting to read file: {}", file_path);
    println!("Looking for sheet: {}", sheet_name);
    println!("Range: {:?} to {:?}", start, end);
    
    let mut matrix = HashMap::new();
    let mut phage_map = HashMap::new();
    let mut bacteria_map = HashMap::new();

    let mut workbook: Xlsx<_> = open_workbook(file_path).map_err(|e| format!("Cannot open file: {}", e))?;
    println!("Opened workbook");
    let sheet = workbook.worksheet_range(sheet_name).map_err(|e| format!("Cannot find sheet: {}", e))?;
    println!("Opened sheet");
    let range = sheet.range(start, end);
    println!("Opened range");
    if range.is_empty() {
        return Err("The sheet is empty.".to_string());
    }

    let max_bacteria = range.height() - 1;
    let max_phages = range.width() - 1;

    println!("Max phages: {}", max_phages);
    println!("Max bacteria: {}", max_bacteria);
    // make hashmap of (usize, usize), bool
    for row in 0..max_bacteria {
        for col in 0..max_phages {
            let cell_value = range.get_value((row as u32 + 1, col as u32 + 1));
            let bool_value = match cell_value {
                Some(value) => *value == 1.0, // true if value is 1
                None => false, // false if there's no value
            }; 
            matrix.insert((row as usize, col as usize), bool_value);
        }
    }

    // cols are phages, rows are bacteria
    (0..max_phages).into_iter().for_each(|phage| {
        let phage_name = range.get_value((0, phage as u32 + 1)).unwrap().to_string();
        phage_map.insert(phage, phage_name);
    });

    // make bacteria map
    (0..max_bacteria).into_iter().for_each(|bacteria| {
        let bacteria_name = range.get_value((bacteria as u32 + 1, 0)).unwrap().to_string();
        bacteria_map.insert(bacteria, bacteria_name);
    });

    let data = ExcelData {
        max_phages,
        max_bacteria,
        matrix,
        phage_map,
        bacteria_map,
    };
    println!("--------------------------------");
    println!("Max phages: {}", data.max_phages);
    println!("--------------------------------");
    println!("Max bacteria: {}", data.max_bacteria);
    println!("--------------------------------");
    println!("Matrix: {:?}", data.matrix);
    println!("--------------------------------");
    println!("Phage map: {:?}", data.phage_map);
    println!("--------------------------------");
    println!("Bacteria map: {:?}", data.bacteria_map);
    println!("--------------------------------");

    Ok(data)

}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_excel() {
//         let result = read_excel("./all_host_range_small.xlsx", "test", (0, 0), (7, 10));
//         assert_eq!(result.is_ok(), true);
//     }
// }