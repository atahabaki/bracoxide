///
///
#[derive(Debug, PartialEq)]
enum Token {
    //{
    OBra,
    //}
    CBra,
    //,
    Comma,
    //Any Non-number text
    Text(String),
    //Number
    Number(String),
    //..
    Range,
}

#[derive(Debug, PartialEq)]
enum TokenizeError {
    EmptyContent,
    Unpredicted,
}

fn tokenize(content: &str) -> Result<Vec<Token>, TokenizeError> {
    if content.is_empty() {
        return Err(TokenizeError::EmptyContent);
    }
    let mut tokens = Vec::<Token>::new();
    let mut is_escape = false;
    // text_buffer, number_buffer
    let mut buffers = (String::new(), String::new());
    let mut iter = content.chars();
    // Push buffers into tokens.
    let tokenize_buffers = |tokens: &mut Vec<Token>, buffers: &mut (String, String)| {
        if !buffers.0.is_empty() {
            tokens.push(Token::Text(buffers.0.clone()));
            buffers.0.clear();
        }
        if !buffers.1.is_empty() {
            tokens.push(Token::Number(buffers.1.clone()));
            buffers.1.clear();
        }
    };
    while let Some(c) = iter.next() {
        println!("{:?}, {}",buffers, c);
        match (c, is_escape) {
            (_, true) => {
                buffers.0.push(c);
                buffers.1.clear();
                is_escape = false;
            }
            ('\\', false) => is_escape = true,
            // @1: COMMENT
            // Look it is '{' OR '}' OR ','
            // No other c value can pass this match ARM
            // And now look to @2
            ('{' | '}' | ',', _) => {
                tokenize_buffers(&mut tokens, &mut buffers);
                match c {
                    '{' => tokens.push(Token::OBra),
                    '}' => tokens.push(Token::CBra),
                    ',' => tokens.push(Token::Comma),
                    // @2: COMMENT
                    // Look @1 the above catch, you see
                    // c can be just '{' OR '}' OR ','.
                    // AND Why the god damn rust wants me to handle all cases,
                    // Where I got covered all cases above.
                    _ => return Err(TokenizeError::Unpredicted),
                }
            }
            ('.', _) => {
                let mut r_iter = iter.clone();
                if let Some(cx) = r_iter.next() {
                    match cx {
                        '.' => {
                            tokenize_buffers(&mut tokens, &mut buffers);
                            tokens.push(Token::Range);
                            iter = r_iter;
                            continue;
                        }
                        _ => buffers.0.push(c),
                    }
                } else {
                    buffers.0.push(c);
                }
            }
            ('0'..='9', _) => {
                if !buffers.0.is_empty() {
                    tokens.push(Token::Text(buffers.0.clone()));
                    buffers.0.clear();
                }
                buffers.1.push(c);
            },
            _ => {
                if !buffers.1.is_empty() {
                    tokens.push(Token::Number(buffers.1.clone()));
                    buffers.1.clear();
                }
                buffers.0.push(c);
            },
        }
    }
    if !buffers.0.is_empty() {
        tokens.push(Token::Text(buffers.0.clone()));
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        // Test case 1: {1..3}
        assert_eq!(
            tokenize("{1..3}"),
            Ok(vec![
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra
            ])
        );

        // Test case 2: {a,b,c}
        assert_eq!(
            tokenize("{a,b,c}"),
            Ok(vec![
                Token::OBra,
                Token::Text("a".to_owned()),
                Token::Comma,
                Token::Text("b".to_owned()),
                Token::Comma,
                Token::Text("c".to_owned()),
                Token::CBra
            ])
        );

        // Test case 3: {A{1,2},B{1,2}}
        assert_eq!(
            tokenize("{A{1,2},B{1,2}}"),
            Ok(vec![
                Token::OBra,
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra,
                Token::Comma,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra,
                Token::CBra
            ])
        );

        // Test case 4: A{1,2}B{1,2}
        assert_eq!(
            tokenize("A{1,2}B{1,2}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra
            ])
        );

        // Test case 5: a{b{c,d}e{f,g}h}i
        assert_eq!(
            tokenize("a{b{c,d}e{f,g}h}i"),
            Ok(vec![
                Token::Text("a".to_owned()),
                Token::OBra,
                Token::Text("b".to_owned()),
                Token::OBra,
                Token::Text("c".to_owned()),
                Token::Comma,
                Token::Text("d".to_owned()),
                Token::CBra,
                Token::Text("e".to_owned()),
                Token::OBra,
                Token::Text("f".to_owned()),
                Token::Comma,
                Token::Text("g".to_owned()),
                Token::CBra,
                Token::Text("h".to_owned()),
                Token::CBra,
                Token::Text("i".to_owned())
            ])
        );

        // Test case 6: a{b{c,d},e{f,g}}h
        assert_eq!(
            tokenize("a{b{c,d},e{f,g}}h"),
            Ok(vec![
                Token::Text("a".to_owned()),
                Token::OBra,
                Token::Text("b".to_owned()),
                Token::OBra,
                Token::Text("c".to_owned()),
                Token::Comma,
                Token::Text("d".to_owned()),
                Token::CBra,
                Token::Comma,
                Token::Text("e".to_owned()),
                Token::OBra,
                Token::Text("f".to_owned()),
                Token::Comma,
                Token::Text("g".to_owned()),
                Token::CBra,
                Token::CBra,
                Token::Text("h".to_owned())
            ])
        );

        // Test case 7: A{1..3}B{2,5}
        assert_eq!(
            tokenize("A{1..3}B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 8: A{1..3},B{2,5}
        assert_eq!(
            tokenize("A{1..3},B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Comma,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 9: A{1..3}\,B{2,5}
        assert_eq!(
            tokenize("A{1..3}\\,B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Text(",B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );
        
        // Test case 10: A{1..3}.B{2,5}
        assert_eq!(
            tokenize("A{1..3}.B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Text(".B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );
        // Test case 11: A1..3.B{2,5}
        assert_eq!(
            tokenize("A1..3.B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::Text(".B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 12: A{1..3}..B{2,5}
        assert_eq!(
            tokenize("A{1..3}..B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Range,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );
    }
}
