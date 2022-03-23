pub mod context;
pub mod dice;
pub mod expr;
//pub mod instruction; //TODO remove
pub mod parser;
pub mod tokenizer;
use expr::*;

fn main() -> anyhow::Result<()> {
    let j = Expr {
        v: Box::new(ExValue::Num(3)),
        ops: vec![
            Operation::D(ExValue::Num(6)),
            Operation::Label(ExValue::Word("Fish".to_string())),
            Operation::Sum,
        ],
    };
    let mut ct = context::Context::new();
    let dr = j.resolve(&mut ct)?;
    println!("{}\nResult = {}", ct, dr);
    Ok(())
}
