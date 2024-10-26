use crate::term::Term;

pub fn minimize(terms: &[Term]) -> Vec<Term> {
    let (mut iteration, mut merges) = step_quine(terms);
    while merges > 0 {
        (iteration, merges) = step_quine(&iteration);
    }
    iteration
}

pub fn trim_redundant_terms(terms: &[Term]) -> Vec<Term> {
    let mut is_redundant: Vec<bool> = vec![false; terms.len()];
    for (ind1, term1) in terms.iter().enumerate() {
        if is_redundant[ind1] {
            continue;
        }
        for (ind2, term2) in terms.iter().enumerate().skip(ind1 + 1) {
            if is_redundant[ind2] {
                continue;
            }

            if let Some(redundant) = Term::is_one_redundant(term1, term2) {
                println!(
                    "Between '{}' and '{}' the #{} is redundant",
                    term1, term2, redundant
                );
                if redundant == 1 {
                    is_redundant[ind1] = true;
                } else {
                    is_redundant[ind2] = true;
                }
            }
        }
    }

    terms
        .iter()
        .enumerate()
        .filter(|(i, _)| !is_redundant[*i]) // Filter where is_redundant[i] is false
        .map(|(_, term)| term.clone())
        .collect()
}

fn step_quine(terms: &[Term]) -> (Vec<Term>, usize) {
    let mut merges = 0;
    let mut merged_pairs: Vec<Term> = vec![];
    let mut has_been_used: Vec<bool> = vec![false; terms.len()];

    for (ind1, term1) in terms.iter().enumerate() {
        for (ind2, term2) in terms.iter().enumerate().skip(ind1) {
            match Term::new_from_neighbors(term1, term2) {
                None => {}
                Some(merged) => {
                    has_been_used[ind1] = true;
                    has_been_used[ind2] = true;
                    println!("Merged '{}' + '{}' => '{}'", term1, term2, merged);
                    merged_pairs.push(merged);
                    merges += 1;
                }
            }
        }
        if !has_been_used[ind1] {
            merged_pairs.push(term1.clone());
        }
    }

    (trim_redundant_terms(&merged_pairs), merges)
}
