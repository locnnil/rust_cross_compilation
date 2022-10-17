
 #[cfg(target_arch = "x86_64")]
fn main() {
    println!("\nHello from x86_64 arch!\n");
}

#[cfg(target_arch = "arm")]
fn main() {
    println!("\nHello from ARM arch!\n");
}

