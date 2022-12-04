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
pub fn get_badges(input: &str) {
    let mut tot = 0;
    let mut tmp = vec![];
    for line in input.split('\n') {
        tmp.push(line);
        if tmp.len() == 3 {
            let first = tmp.pop().unwrap();
            let second = tmp.pop().unwrap();
            let third = tmp.pop().unwrap();

            let mut common: Vec<_> = first
                .chars()
                .filter(|c| second.contains(*c))
                .filter(|c| third.contains(*c))
                .collect();
            tot += char_to_prio(common.pop().unwrap());
        }
    }
    println!("res : {tot}");
}
