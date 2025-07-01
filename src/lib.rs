/// Scores how well `query` matches the `candi` string.
/// 
/// The score is based on character positions with higher weight for characters
/// matched earlier in the `query`. Exact matches yield the highest score (100).
///
/// # Arguments
///
/// * `query` - The search query string slice.
/// * `candi` - The candidate string slice to be matched against.
///
/// # Returns
///
/// A usize score between 0 and 100, higher means better match.
///
/// # Examples
///
/// ```
/// let score = matchr::score("fefe", "fefe");
/// assert_eq!(score, 100);
/// ```
pub fn score(query: &str, candi: &str) -> usize {
    let mut glob_score: usize = 0;

    let mut quer_chars = query.chars();

    for i in 0..query.len() {
        let query_char = quer_chars.next();
        let mut candi_chars = candi.chars();

        for _char_index in 0..candi.len() {
            let letter = candi_chars.next();

            if query_char == letter {
                let score_for_pos = match i {
                    0 => 20,
                    1 => 15,
                    2 => 10,
                    _ => 5,
                };
                glob_score += score_for_pos;
                break;
            } else {
                glob_score = glob_score.saturating_sub(5);
            }
        }

        if glob_score >= 100 {
            break;
        }
    }

    if query == candi {
        return 100;
    }

    glob_score
}


/// Matches multiple `items` against the `query` and returns
/// a sorted vector of tuples containing the item and its match score.
///
/// # Arguments
///
/// * `query` - The search query string slice.
/// * `items` - Slice of string slices to be matched.
///
/// # Returns
///
/// A vector of tuples `(item, score)`, sorted by descending score.
///
/// # Examples
///
/// ```
/// let items = ["fefe", "feature", "banana"];
/// let results = matchr::match_items("fefe", &items);
/// assert_eq!(results[0].0, "fefe");
/// ```
pub fn match_items<'a>(query: &str, items: &[&'a str]) -> Vec<(&'a str, usize)> {
    let mut scored: Vec<_> = items
        .iter()
        .map(|item| (*item, score(query, item)))
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        let query = "fefe";
        let candidates = [
            "fefe",
            "fefete",
	    "fefafef",
            "eeeeeeeeeee",
            "feature",
            "banana",
            "effort",
        ];

        let results = match_items(query, &candidates);

        for (item, score) in &results {
            println!("{} => score: {}", item, score);
        }
    }
}
