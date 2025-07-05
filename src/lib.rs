/// Scores how well `query` matches the `candi` string.
///
/// The score is based on whether `query` is a subsequence of `candi`, with additional weighting:
/// - Characters matched earlier in `candi` get higher weight.
/// - Consecutive matched characters earn a small bonus.
/// - Exact matches yield the highest score (100).
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
    if query.is_empty() {
        return 0;
    }

    if !is_subsequence(query, candi) {
        return 0;
    }

    let mut score = 0usize;
    let mut candi_chars = candi.char_indices();
    let mut last_pos = None;

    for qc in query.chars() {
        let mut found = false;
        while let Some((pos, cc)) = candi_chars.next() {
            if cc == qc {
                let pos_score = 10usize.saturating_sub(pos);
                score += pos_score;

                if let Some(lp) = last_pos {
                    if pos == lp + 1 {
                        score += score / 10;
                    }
                }

                last_pos = Some(pos);
                found = true;
                break;
            }
        }
        if !found {
            return 0;
        }
    }

    if query == candi {
        return 100;
    }
    let max_possible = query.len() * 15;

    ((score * 100) / max_possible).min(100)
}

fn is_subsequence(query: &str, candi: &str) -> bool {
    let mut candi_chars = candi.chars();
    for qc in query.chars() {
        if candi_chars.find(|cc| *cc == qc).is_none() {
            return false;
        }
    }
    true
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
	.filter(|&(_, score)| score > 0)
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        let query = "xb";
        let candidates = [
            "eeeeeeeeeeeeeeeeeeeeeeeee",
            "cat",
            "cp",
            "mv",
            "rm",
            "touch",
            "mkdir",
            "rmdir",
            "grep",
            "find",
            "xargs",
            "cut",
            "head",
            "tail",
            "less",
            "more",
            "man",
            "chmod",
            "chown",
            "ping",
            "curl",
            "wget",
            "ssh",
            "scp",
            "ps",
            "kill",
            "top",
            "htop",
            "nano",
            "vim",
            "xbps-install",
            "xbps-remove",
            "xbps-query",
            "sudo",
            "doas",
            "su",
            "env",
            "export",
            "uname",
            "whoami",
            "uptime",
            "date",
            "cal",
            "clear",
            "tput",
            "printf",
            "echo",
        ];

        let results = match_items(query, &candidates);

        for (item, score) in &results {
            println!("{} => score: {}", item, score);
        }
    }
}
