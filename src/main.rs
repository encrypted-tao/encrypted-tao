mod query;

fn main() {
    let qs = query::parser::parse(
        "ASSOC COUNT 123 AUTHORED; ASSOC GET 1 FRIEND [3, 4, 5];",
    );
    for q in qs {
        println!("{:#?}", q);
    }
}
