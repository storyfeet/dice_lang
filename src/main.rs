pub mod dice;
pub mod instruction;
pub mod parser;
pub mod tokenizer;
pub mod tracker;
use dice::*;
use instruction::*;

fn main() -> anyhow::Result<()> {
    let j = Job::dice(Dice::D(6).n(3))
        .push_ins(Instruction::Sum)
        .push_ins(Instruction::Push)
        .push_ins(Instruction::Append(Job::pop()));

    let mut tr = tracker::Tracker::new();
    let dr = j.run(&mut tr)?;
    println!("{}\nResult = {}", tr, dr);
    Ok(())
}
