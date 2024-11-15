mod utils;

fn main() {
    let excel_data = utils::excel_parser::read_excel("./all_host_range_small.xlsx", "test", (0, 0), (7, 10)).unwrap();
    let (phage_map, bacteria_map, max_phages, max_bacteria, matrix) = excel_data.get_data();
    let result = utils::search::exhaustive_search(&matrix, max_phages, max_bacteria);
    println!("{:?}", result);
}
