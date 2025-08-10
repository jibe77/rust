#[cfg(target_os = "macos")]
fn name_me() {
    println!("ma fonction sous macos.")
}

#[cfg(target_os = "windows")]
fn name_me() {
    println!("ma fonction sous windows.")
}

fn main() {
    println!("Hello, world!");
    name_me();
}
