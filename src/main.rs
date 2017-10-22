fn move_tower(i: u8, size: u8, source: &str, aux: &str, dest: &str) {
    if size == 1 {
        println!("[{}] move disk {} from {} to {}", i, size, source, dest);
    } else {
        move_tower(i + 1, size - 1, source, dest, aux);
        println!("[{}] move disk {} from {} to {}", i, size, source, dest);
        move_tower(i + 1, size - 1, aux, source, dest);
    }
}

fn main() {
    println!("--- solution for size: 1 ---");
    move_tower(0, 1, "Source", "Aux", "Dest");
    println!();

    println!("--- solution for size: 2 ---");
    move_tower(0, 2, "Source", "Aux", "Dest");
    println!();

    println!("--- solution for size: 3 ---");
    move_tower(0, 3, "Source", "Aux", "Dest");
    println!();
}
