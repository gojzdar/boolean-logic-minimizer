use std::env::args;
mod algorithm;
mod types;
use algorithm::mintermmini;
use term::Term;
use types::*;

fn print_terms(terms: &[Term]) {
    for term in terms.iter() {
        print!("{}, ", term);
    }
    println!();
}

fn main() {
    let cmd_args: Vec<String> = args().collect();
    println!(r#"argument structure: <number of variables> <minterm [minterm [...]]>"#);
    let n = cmd_args[1]
        .parse::<i32>()
        .expect("First argument must be <number of variables>");
    let max_minterm = (1 << n) - 1;
    let mut terms: Vec<Term> = vec![];

    for arg in cmd_args.iter().skip(2) {
        let minterm: u64 = arg.parse::<u64>().expect("Minterm is not a number!");
        if minterm > max_minterm {
            eprintln!(
                "Minterm {} cannot be represented with {} variables (0 <= m <= {})",
                minterm, n, max_minterm
            );
            return;
        }

        let term = Term::new_from_minterm(n, minterm);
        println!("Minterm {} -> {}", minterm, term);
        terms.push(term);
    }
    print_terms(&terms);
    print_terms(&mintermmini::minimize(&terms));
}
