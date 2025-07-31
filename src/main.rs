use std::io;

fn main() {
    println!("Enter the nth fibonacci number you want to generate");
    let mut seq_num = String::new();
    io::stdin()
        .read_line(&mut seq_num)
        .expect("Could not read line");

    let seq_num: u32 = seq_num.trim().parse().expect("Not a u32 value");
    let fib = gen_fib_num(seq_num);
    println!("Result: {fib}");
}

// 1 1 2 3 5 8 13
fn gen_fib_num(seq_num: u32) -> u64 {
    if seq_num <= 2 {
        return 1;
    }
    return gen_fib_num(seq_num - 2) + gen_fib_num(seq_num - 1);
}
