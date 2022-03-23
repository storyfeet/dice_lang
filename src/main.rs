pub mod context;
pub mod dice;
pub mod expr;
//pub mod instruction; //TODO remove
pub mod parser;
pub mod tokenizer;
//use expr::*;

fn main() -> anyhow::Result<()> {
    for (i, a) in std::env::args().skip(1).enumerate() {
        println!("Roll {} : {}\n", i, a);
        let j = parser::parse_expr(&a)?;

        let mut ct = context::Context::new();
        let dr = j.resolve(&mut ct)?;
        println!("{}\nResult = {}", ct, dr);
    }
    Ok(())
}
