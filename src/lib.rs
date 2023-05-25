pub fn expand(input: &str) -> Option<Vec<String>> {
    if input.is_empty() {
        return None;
    }
    let mut expansions = Vec::<String>::new();
    let mut iter = input.chars();
    let mut count = (0, 0); // right, left / open, close
    let mut fixes = (String::new(), String::new()); // prefix, postfix
    let mut inside = String::new();
    while let Some(c) = iter.next() {
        match c {
            '{' => {
                if count.0 != 0 {
                    inside.push(c);
                }
                count.0 += 1;
            }
            '}' => {
                count.1 += 1;
                if count.0 != count.1 {
                    inside.push(c);
                }
            }
            _ if count.0 == 0 => fixes.0.push(c),
            _ if count.0 == count.1 => fixes.1.push(c),
            _ => inside.push(c),
        }
    }
    let parts = split(inside);
    if let Some(pieces) = parts {
        for piece in pieces {
            let (prefix, postfix) = fixes.clone();
            if piece.contains('{') || piece.contains('}') {
                if let Some(recursive_parts) = expand(&piece) {
                    for recursive_part in recursive_parts {
                        let combination = combine(&prefix, &recursive_part, &postfix);
                        expansions.push(combination);
                    }
                }
            } else {
                let combination = combine(&prefix, &piece, &postfix);
                expansions.push(combination);
            }
        }
    } else {
        return None;
    }
    if expansions.is_empty() {
        None
    } else {
        Some(expansions)
    }
}

fn combine(prefix: &str, content: &str, postfix: &str) -> String {
    format!("{}{}{}", prefix, content, postfix)
}

fn split(content: impl ToString) -> Option<Vec<String>> {
    let content = content.to_string();
    if content.is_empty() {
        return None;
    }
    let mut pieces: Vec<String> = Vec::new();
    let mut iter = content.chars();
    let mut count = (0, 0); // right, left / open, close
    let mut piece = String::new();
    while let Some(c) = iter.next() {
        match c {
            '{' | '}' => {
                piece.push(c);
                if c == '{' {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }
            ',' if count.0 == count.1 => {
                pieces.push(piece.clone());
                piece.clear();
            }
            _ => piece.push(c),
        }
    }
    if !piece.is_empty() {
        pieces.push(piece);
    }
    if pieces.is_empty() {
        None
    } else {
        Some(pieces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_simple() {
        let input = "c{d,e}f";
        let expected_output: Vec<String> = vec!["cdf".into(), "cef".into()];
        assert_eq!(expand(input), Some(expected_output));
    }

    #[test]
    fn test_expand_recursive1() {
        let input = "a{b,c{d,e}f,g}h";
        let output: Vec<String> = vec!["abh".into(), "acdfh".into(), "acefh".into(), "agh".into()];
        assert_eq!(expand(input), Some(output));
    }

    #[test]
    fn test_expand_recursive2() {
        let input = "a{b,c{d{1,2},e}f,g}h";
        let output: Vec<String> = vec!["abh".into(), "acd1fh".into(), "acd2fh".into(), "acefh".into(), "agh".into()];
        assert_eq!(expand(input), Some(output));
    }

    #[test]
    fn test_split_complex1() {
        let input = "b,c{d,e}f,g";
        let output: Vec<String> = vec!["b".into(), "c{d,e}f".into(), "g".into()];
        assert_eq!(split(input), Some(output));
    }

    #[test]
    fn test_split_complex2() {
        let input = "a,b,c,d{e,f},g{h,i,j},k";
        let output: Vec<String> = vec![
            "a".into(),
            "b".into(),
            "c".into(),
            "d{e,f}".into(),
            "g{h,i,j}".into(),
            "k".into(),
        ];
        assert_eq!(split(input), Some(output));
    }

    #[test]
    fn test_basic_brace_expansion() {
        let input = "{apple,banana,cherry}";
        let expected_output: Vec<String> = vec!["apple".into(), "banana".into(), "cherry".into()];
        assert_eq!(expand(&input), Some(expected_output))
    }
}
