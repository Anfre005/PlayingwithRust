const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // This is commented as it was a part of the 2nd excercise in the readme
    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;

    // The random variable which was not going to be used.
    let randomunused: i32;

    // Trying to modify a constant variable.
    // READY_AMOUNT = 1;
    
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles... ", ready, missiles);

    // Keeping this fromt he second excercise. the goal was to do the calculations directly in the print statement.
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}