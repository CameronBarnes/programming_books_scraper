use crate::parsing::parse_github_programming_books;

mod types;
mod parsing;

fn main() {
    
    let results = parse_github_programming_books();

    println!("{}", serde_json::to_string(&results).unwrap());

}
