use std::str::Chars;

pub struct Scanner<'a> {
    pub chars: Chars<'a>,
    pub offset: usize,
    pub line: usize,
}

#[derive(Clone)]
pub struct ScannerBranch<'a> {
    pub chars: Chars<'a>,
    pub offset: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Self {
            chars: source.chars(),
            offset: 0,
            line: 1,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.peek_at(0)
    }

    pub fn peek_at(&self, at: usize) -> Option<char> {
        self.chars.clone().nth(at)
    }

    pub fn advance(&mut self) -> Option<char> {
        let char = self.chars.next()?;

        self.offset += 1;
        if char == '\n' {
            self.line += 1;
        }

        Some(char)
    }

    pub fn rest(&self) -> &'a str {
        self.chars.as_str()
    }

    pub fn branch(&self) -> ScannerBranch<'a> {
        ScannerBranch {
            chars: self.chars.clone(),
            offset: self.offset,
            line: self.line,
        }
    }

    pub fn restore(&mut self, branch: ScannerBranch<'a>) {
        self.chars = branch.chars;
        self.offset = branch.offset;
        self.line = branch.line;
    }
}
