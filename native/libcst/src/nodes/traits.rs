use crate::{
    tokenizer::whitespace_parser::{Config, WhitespaceError},
    Codegen, CodegenState, Comma, EmptyLine, LeftParen, RightParen,
};

pub trait WithComma<'a> {
    fn with_comma(self, comma: Comma<'a>) -> Self;
}

pub trait ParenthesizedNode<'a> {
    fn lpar(&self) -> &Vec<LeftParen<'a>>;
    fn rpar(&self) -> &Vec<RightParen<'a>>;

    fn parenthesize<F>(&self, state: &mut CodegenState<'a>, f: F)
    where
        F: FnOnce(&mut CodegenState<'a>),
    {
        for lpar in self.lpar() {
            lpar.codegen(state);
        }
        f(state);
        for rpar in self.rpar() {
            rpar.codegen(state);
        }
    }

    fn with_parens(self, left: LeftParen<'a>, right: RightParen<'a>) -> Self;
}

pub trait WithLeadingLines<'a> {
    fn leading_lines(&mut self) -> &mut Vec<EmptyLine<'a>>;
}

pub type Result<T> = std::result::Result<T, WhitespaceError>;

pub trait Inflate<'a>
where
    Self: Sized,
{
    fn inflate(self, config: &Config<'a>) -> Result<Self>;
}

impl<'a, T: Inflate<'a>> Inflate<'a> for Option<T> {
    fn inflate(self, config: &Config<'a>) -> Result<Self> {
        self.map(|x| x.inflate(config)).transpose()
    }
}

impl<'a, T: Inflate<'a> + ?Sized> Inflate<'a> for Box<T> {
    fn inflate(self, config: &Config<'a>) -> Result<Self> {
        match (*self).inflate(config) {
            Ok(a) => Ok(Box::new(a)),
            Err(e) => Err(e),
        }
    }
}

impl<'a, T: Inflate<'a>> Inflate<'a> for Vec<T> {
    fn inflate(self, config: &Config<'a>) -> Result<Self> {
        self.into_iter().map(|item| item.inflate(config)).collect()
    }
}