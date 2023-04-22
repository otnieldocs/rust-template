mod utils;
use utils::add_numbers;

fn main() {
    let project_name = env!("CARGO_PKG_NAME");
    println!("================= {} =================", project_name);
    println!("5 + 2 = {}", add_numbers(5, 2));
}