//! "Did you mean ...?" suggestions for misspelled keywords.

/// The candidate closest to `word`, ignoring ASCII case, if it is close enough to
/// be a likely misspelling: at most one edit per three characters, as rustc does.
///
/// Operators take no part: one edit turns almost any short word into `<` or `*`,
/// so neither a word nor a candidate without a letter is matched. An exact match
/// isn't suggested either.
pub(crate) fn suggest(word: &str, candidates: &[&'static str]) -> Option<&'static str> {
    let has_letter = |s: &str| s.bytes().any(|b| b.is_ascii_alphabetic());
    if !has_letter(word) {
        return None;
    }
    let word = word.to_ascii_lowercase();
    let length = word.chars().count();
    let max_distance = length.max(3) / 3;
    candidates
        .iter()
        .filter(|candidate| has_letter(candidate))
        // The distance is at least the difference in length. Skipping candidates
        // that can't be close keeps a long input token from costing
        // `length * candidate length` per candidate.
        .filter(|candidate| candidate.chars().count().abs_diff(length) <= max_distance)
        .map(|&candidate| (distance(&word, &candidate.to_ascii_lowercase()), candidate))
        .filter(|&(d, _)| (1..=max_distance).contains(&d))
        .min_by_key(|&(d, _)| d)
        .map(|(_, candidate)| candidate)
}

/// The edit distance between `a` and `b`: the fewest single-character insertions,
/// deletions, substitutions and swaps of adjacent characters that turn one into the
/// other (optimal string alignment). A swap counts once, as in `strat` for `start`.
fn distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    // d[i][j]: the distance between the first `i` characters of `a` and the first `j` of `b`.
    let mut d = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in d[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(d[i - 2][j - 2] + 1);
            }
            d[i][j] = best;
        }
    }
    d[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    const SECTIONS: &[&str] = &[":predicates", ":functions", ":constants", ":action"];

    #[test]
    fn distance_counts_edits() {
        check!(distance("kitten", "sitting") == 3);
        check!(distance("", "abc") == 3);
        check!(distance("abc", "abc") == 0);
        check!(distance("strat", "start") == 1);
    }

    #[test]
    fn close_words_are_suggested() {
        check!(suggest(":predicats", SECTIONS) == Some(":predicates"));
        check!(suggest(":PREDICATS", SECTIONS) == Some(":predicates"));
        check!(suggest(":actoin", SECTIONS) == Some(":action"));
        check!(suggest("strat", &["start", "end"]) == Some("start"));
    }

    #[test]
    fn distant_words_exact_matches_and_operators_are_not() {
        check!(suggest(":bogus", SECTIONS) == None);
        check!(suggest(":action", SECTIONS) == None);
        check!(suggest("x", &["*", "+"]) == None);
        check!(suggest("<", &["<=", ">="]) == None);
    }

    #[test]
    fn a_long_word_is_not_compared() {
        let word = "x".repeat(1_000_000);
        check!(suggest(&word, SECTIONS) == None);
    }
}
