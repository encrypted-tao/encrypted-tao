mod query;

fn main() {
    query::parser::parse("ASSOC COUNT 123 AUTHORED;");
    println!("Hello, world!");
}
