mod reader;

pub use reader::*;

/// Converts an alphabetic column label to an equivalent numeric index.
///
/// # Examples
///
/// ```
/// use xtap_core::xlsx::label_to_index;
///
/// assert_eq!(label_to_index("AFK"), Some(843));
/// assert_eq!(label_to_index("afk"), Some(843));
/// assert_eq!(label_to_index("A1"), None);
/// ```
pub fn label_to_index(label: &str) -> Option<u32> {
    // case we don't need further computation.
    if label.is_empty() {
        return None;
    }

    let mut idx = 0;
    let mut pow = 1;
    for ch in label.chars().rev() {
        if let Some(n) = char_to_number(ch) {
            idx += n * pow;
            pow *= 26;
        } else {
            return None;
        }
    }

    Some(idx)
}

#[inline]
fn char_to_number(ch: char) -> Option<u32> {
    match ch {
        'A' | 'a' => Some(1),
        'B' | 'b' => Some(2),
        'C' | 'c' => Some(3),
        'D' | 'd' => Some(4),
        'E' | 'e' => Some(5),
        'F' | 'f' => Some(6),
        'G' | 'g' => Some(7),
        'H' | 'h' => Some(8),
        'I' | 'i' => Some(9),
        'J' | 'j' => Some(10),
        'K' | 'k' => Some(11),
        'L' | 'l' => Some(12),
        'M' | 'm' => Some(13),
        'N' | 'n' => Some(14),
        'O' | 'o' => Some(15),
        'P' | 'p' => Some(16),
        'Q' | 'q' => Some(17),
        'R' | 'r' => Some(18),
        'S' | 's' => Some(19),
        'T' | 't' => Some(20),
        'U' | 'u' => Some(21),
        'V' | 'v' => Some(22),
        'W' | 'w' => Some(23),
        'X' | 'x' => Some(24),
        'Y' | 'y' => Some(25),
        'Z' | 'z' => Some(26),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_to_index() {
        assert_eq!(label_to_index("C"), Some(3));
        assert_eq!(label_to_index("CD"), Some(82));
        assert_eq!(label_to_index("AFK"), Some(843));

        assert_eq!(label_to_index("c"), Some(3));
        assert_eq!(label_to_index("cd"), Some(82));
        assert_eq!(label_to_index("afk"), Some(843));

        assert_eq!(label_to_index(""), None);
        assert_eq!(label_to_index("42"), None);
        assert_eq!(label_to_index("A1"), None);
    }
}
