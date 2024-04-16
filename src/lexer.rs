pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn chop(&mut self, n: usize) -> Option<&'a [char]> {
        let token = Some(&self.content[0..n]);
        self.content = &self.content[n..];

        return token;
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> Option<&'a [char]>
    where
        P: FnMut(&[char], usize) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(self.content, n) {
            n += 1;
        }

        return self.chop(n);
    }

    fn trim_left(&mut self) {
        self.chop_while(|content, idx| content[idx].is_ascii_whitespace());
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        // avoid empty spaces
        self.trim_left();

        if self.content.is_empty() {
            return None;
        }

        // 123
        // 1.23
        if self.content[0].is_numeric() {
            return self.chop_while(|content, idx| {
                let c = content[idx];
                c.is_numeric() || c == '.' && content[idx + 1].is_numeric()
            });
        }

        // hello
        // gl4
        // GL_INVALID_VALUE
        if self.content[0].is_alphabetic() {
            return self.chop_while(|content, idx| {
                let c = content[idx];

                c.is_alphanumeric() || c == '_'
            });
        }

        // else then return a single character such as "√"
        return self.chop(1);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self
            .next_token()
            .map(|ch| ch.iter().collect())
            .map(|ch: String| ch.to_ascii_uppercase().to_string());

        term
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;

    #[test]
    fn should_work() {
        let mut vec = Vec::new();
        for token in Lexer::new(
            &"hello 123 1.23 gl4 GL_INVALID_VALUE, who  are you! ✅"
                .chars()
                .collect::<Vec<_>>(),
        ) {
            vec.push(token);
        }

        assert_eq!(
            vec,
            vec![
                "HELLO",
                "123",
                "1.23",
                "GL4",
                "GL_INVALID_VALUE",
                ",",
                "WHO",
                "ARE",
                "YOU",
                "!",
                "✅"
            ]
        );
    }
}
