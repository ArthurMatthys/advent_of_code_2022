pub fn max_calories(input: &str) {
    let mut max = 0;
    let mut sub_tot = 0;
    for line in input.split('\n') {
        let line = line.trim();
        if line == "" {
            max = max.max(sub_tot);
            sub_tot = 0;
            continue;
        }
        sub_tot += line.parse::<u32>().expect("cannot convert to u32");
    }
    max = max.max(sub_tot);
    print!("max calories : {max}");
}

pub fn top_three(input: &str) {
    let mut calories = vec![];
    let mut sub_tot = 0;
    for line in input.split('\n') {
        let line = line.trim();
        if line == "" {
            calories.push(sub_tot);
            sub_tot = 0;
            continue;
        }
        sub_tot += line.parse::<u32>().expect("cannot convert to u32");
    }
    calories.push(sub_tot);
    calories.sort();

    print!(
        "max calories : {}",
        calories.pop().expect("Need at least 1 value")
            + calories.pop().expect("Need at least 2 value")
            + calories.pop().expect("Need at least 3 value")
    );
}
