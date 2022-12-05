fn contains((lmin, lmax): (u32, u32), (rmin, rmax): (u32, u32)) -> bool {
    (rmin <= lmin && lmin <= rmax && rmin <= lmax && lmax <= rmax)
        || (lmin <= rmin && rmin <= lmax && lmin <= rmax && rmax <= lmax)
}

fn overlap((lmin, lmax): (u32, u32), (rmin, rmax): (u32, u32)) -> bool {
    (rmin <= lmin && lmin <= rmax)
        || (rmin <= lmax && lmax <= rmax)
        || (lmin <= rmin && rmin <= lmax)
        || (lmin <= rmax && rmax <= lmax)
}

pub fn get_pairs_number(input: &str) {
    let mut count = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut tmp = vec![];
        for side in line.split(',') {
            for nbr in side.split('-') {
                tmp.push(nbr.parse::<u32>().unwrap());
            }
        }
        let lmax = tmp.pop().unwrap();
        let lmin = tmp.pop().unwrap();
        let rmax = tmp.pop().unwrap();
        let rmin = tmp.pop().unwrap();
        if contains((lmin, lmax), (rmin, rmax)) {
            count += 1
        }
    }
    println!("res : {count}");
}
pub fn get_overlaps(input: &str) {
    let mut count = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut tmp = vec![];
        for side in line.split(',') {
            for nbr in side.split('-') {
                tmp.push(nbr.parse::<u32>().unwrap());
            }
        }
        let lmax = tmp.pop().unwrap();
        let lmin = tmp.pop().unwrap();
        let rmax = tmp.pop().unwrap();
        let rmin = tmp.pop().unwrap();
        if overlap((lmin, lmax), (rmin, rmax)) {
            count += 1
        }
    }
    println!("res : {count}");
}
