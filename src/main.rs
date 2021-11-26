use std::env;

use rand::{prelude::IteratorRandom, thread_rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let rng = 1..from;
        let numbers = rng.choose_multiple(&mut thread_rng(), take);

        Self {
            take,
            from,
            numbers,
        }
    }

    #[allow(dead_code)]
    fn get_numbers(self) -> Vec<usize> {
        self.numbers
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    format!(
        "{} of {}: [{}]",
        lotto.take,
        lotto.from,
        lotto
            .numbers
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn parse_arg(args: &Vec<String>, index: usize) -> usize {
    args.get(index)
        .expect("Invalid number of arguments. Pass two numbers!")
        .parse()
        .expect("Invalid argument format. Pass two numbers!")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lotto = Lotto::new(parse_arg(&args, 1), parse_arg(&args, 2));
    println!("{}", format_lotto_results(&lotto));
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
