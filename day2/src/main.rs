use std::fs::read_to_string;

fn main() {
    // let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    let input = read_to_string("input.txt").unwrap();
    let result = analyze(&input);
    println!("{}", result);
}

fn analyze(input: &str) -> usize {
    input
        .split("\n")
        .filter(|r| !r.is_empty())
        .filter(|r| {
            let mut f = check_report(r);
            if !f {
                let c: Vec<String> = r.clone().split_whitespace().map(|s| s.to_owned()).collect();
                for i in 0..c.len() {
                    let mut b = c.clone();
                    b.remove(i);
                    let a = check_report(&b.join(" "));
                    if a {
                        f = a;
                        break;
                    }
                }
            }
            println!("{}", r);
            f
        })
        .count()
}

fn check_report(report: &str) -> bool {
    println!("{}", report);
    let levels: Vec<i32> = report
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut check_forward = levels.clone();
    check_forward.sort();
    let mut check_reverse = check_forward.clone();
    check_reverse.reverse();
    if levels == check_forward || levels == check_reverse {
        println!("SIZE: {}", levels.len());
        for i in 0..(levels.len() - 1) {
            let gap = (levels[i] - levels[i + 1]).abs();
            println!("{} -> {} == {}", levels[i], levels[i + 1], gap);
            if gap < 1 || gap > 3 {
                return false;
            }
        }
        return true;
    } else {
        return false;
    }
}
