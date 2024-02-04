use poker_eval::keys;

fn main() {
    banner("poker keys", 10);

    let keys = keys::build();
    println!("{}", keys);
}

fn banner(txt: &str, n: u8) {
    let s = "-".repeat(n as usize);
    println!("\n{} {} {}", s, txt, s);
}
