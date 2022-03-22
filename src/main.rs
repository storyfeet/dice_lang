pub mod context;
pub mod dice;
pub mod instruction;
pub mod parser;
pub mod tokenizer;
use dice::*;
use instruction::*;

fn main() -> anyhow::Result<()> {
    let j = Job::dice(Dice::D(6).n(3))
        .push_ins(Instruction::Sum)
        .push_ins(Instruction::Push)
        .push_ins(Instruction::Append(Job::pop()));

    let mut ct = context::Context::new();
    let dr = j.run(&mut tr)?;
    println!("{}\nResult = {}", tr, dr);
    Ok(())
}
