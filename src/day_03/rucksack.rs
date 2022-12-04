use std::collections::HashSet;

fn char_to_prio(c: char) -> u32 {
    if c.is_uppercase() {
        ((c as u8) - b'A' + 27) as u32
    } else {
        ((c as u8) - b'a' + 1) as u32
    }
}

pub fn get_priorities(input: &str) {
    let mut tot = 0;
    for line in input.split('\n') {
        let len = line.len();
        let (right, left) = line.split_at(len / 2);
        for c in left.chars() {
            if right.contains(c) {
                tot += char_to_prio(c);
                break;
            }
        }
    }
    println!("res : {tot}");
}
