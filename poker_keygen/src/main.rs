mod key_gen_face_five_parallel;
mod key_gen_face_seven_parallel;
mod key_gen_flush_five;
mod key_gen_flush_seven;
mod key_gen_suit;

fn banner(txt: &str, n: u8) {
    let s = "-".repeat(n as usize);
    println!("\n{} {} {}", s, txt, s);
}

fn main() {
    banner("key_gen_suit", 10);
    key_gen_suit::build();

    banner("key_gen_flush_five", 10);
    key_gen_flush_five::build();

    banner("key_gen_flush_seven", 10);
    key_gen_flush_seven::build();

    banner("key_gen_face_five_parallel", 10);
    key_gen_face_five_parallel::build();

    banner("key_gen_face_seven_parallel", 10);
    key_gen_face_seven_parallel::build();
}
