pub mod dice;
pub mod instruction;
pub mod tracker;
use dice::*;
use instruction::*;

fn main() -> anyhow::Result<()> {
    let j = Job::dice(Dice::D(6).n(3)).push_ins(Instruction::Sum);
    let mut st = tracker::Tracker::new();
    let dr = j.run(&mut st)?;
    println!("Rolls = {:?}\nResult = {:?}", st, dr);
    Ok(())
}
