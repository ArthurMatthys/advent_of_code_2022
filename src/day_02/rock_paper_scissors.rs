fn get_points_from_round(opp: &str, played: &str) -> u32 {
    let tot = match played {
        "X" => 1, // rock
        "Y" => 2, // paper
        "Z" => 3, // scissors
        _ => unreachable!(),
    };

    tot + match opp {
        "A" => match played {
            // rock
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => unreachable!(),
        },
        "B" => match played {
            // paper
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unreachable!(),
        },
        "C" => match played {
            // scissors
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn get_points_from_result(opp: &str, target: &str) -> u32 {
    let tot = match target {
        "X" => 0, // loose
        "Y" => 3, // draw
        "Z" => 6, // win
        _ => unreachable!(),
    };

    tot + match opp {
        "A" => match target {
            // rock
            "X" => 3, // paper
            "Y" => 1, //
            "Z" => 2,
            _ => unreachable!(),
        },
        "B" => match target {
            // paper
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        },
        "C" => match target {
            // scissors
            "X" => 2,
            "Y" => 3,
            "Z" => 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn count_points(input: &str) {
    println!(
        "res {}",
        input.split("\n").fold(0, |acc, x| {
            let mut parts = x.split_whitespace();
            let opp = parts.next().unwrap();
            let played = parts.next().unwrap();
            acc + get_points_from_round(opp, played)
        })
    );
}

pub fn count_points_with_result(input: &str) {
    println!(
        "res {}",
        input.split("\n").fold(0, |acc, x| {
            let mut parts = x.split_whitespace();
            let opp = parts.next().unwrap();
            let played = parts.next().unwrap();
            acc + get_points_from_result(opp, played)
        })
    );
}
