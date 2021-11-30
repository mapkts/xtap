use std::cmp;

/// Finds the Levenshtein distance between two strings.
pub fn lev_distance(a: &str, b: &str) -> usize {
    // cases which don't require further computation
    if a.is_empty() {
        return b.chars().count();
    } else if b.is_empty() {
        return a.chars().count();
    }

    let mut dcol: Vec<_> = (0..=b.len()).collect();
    let mut t_last = 0;

    for (i, sc) in a.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in b.chars().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = cmp::min(current, next);
                dcol[j + 1] = cmp::min(dcol[j + 1], dcol[j]) + 1;
            }
            current = next;
            t_last = j;
        }
    }
    dcol[t_last + 1]
}

/// Finds the closest element from `iter` matching `choice`.
pub fn closest<'a, T>(
    choice: &str,
    iter: impl Iterator<Item = T>,
    key: impl Fn(&T) -> &'a str,
) -> Option<T> {
    // only consider candidates with a lev_distance of 3 or less.
    iter.map(|e| (lev_distance(choice, key(&e)), e))
        .filter(|&(d, _)| d < 4)
        .min_by_key(|t| t.0)
        .map(|t| t.1)
}

pub fn closest_msg<'a, T>(
    choice: &str,
    iter: impl Iterator<Item = T>,
    key: impl Fn(&T) -> &'a str,
) -> String {
    match closest(choice, iter, &key) {
        Some(e) => format!("\n\n\tDid you mean `{}`?", key(&e)),
        None => String::new(),
    }
}
