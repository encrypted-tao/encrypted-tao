<<<<<<< HEAD
mod ope;
=======
mod tao;
>>>>>>> 5eb0e4cb7fba849e5cca6a3cea1a6059d2f526d2

fn main() {
    let qs = tao::parser::parse(
        "ASSOC ADD 51 LIKES 1001 55 \"hello\"; \
         ASSOC GET 121 FRIEND [1, 2, 3]; \
         ASSOC RGET 123 FRIEND [50, 51, 52] 0 10; \
         ASSOC COUNT 55 COMMENT; \
         ASSOC TRANGE 77 AUTHORED 1000 1200 40; \
         OBJ ADD 2023 USER \"Mark Z\"; \
         OBJ GET 1234; \
         ",
    );
    for q in qs {
        println!("===============");
        println!("Tao Query: {:#?}", q);
        println!("---------------");
        let s = tao::translator::translate(q);
        println!("SQL Query: {:#?}", s);
    }
}
