use erl_tokenize::{Token, Tokenizer};

use {Result, TokenReader, Parse};
use cst::{Expr, Pattern, Type, Form, ModuleDecl};

pub struct Parser<'text> {
    tokens: Vec<Token<'text>>,
}
impl<'text> Parser<'text> {
    pub fn new(text: &'text str) -> Result<Self> {
        let result = Tokenizer::new(text).collect::<::std::result::Result<Vec<_>, _>>();
        let tokens = track_try!(result);
        Ok(Parser { tokens })
    }
    pub fn tokens(&self) -> &[Token<'text>] {
        &self.tokens
    }
    pub fn parse_expr<'token>(&'token self) -> Result<Expr<'token, 'text>> {
        let mut reader = TokenReader::new(&self.tokens);
        let expr = track_try!(Expr::parse(&mut reader), "line_num={}", reader.line_num());
        Ok(expr)
    }
    pub fn parse_pattern<'token>(&'token self) -> Result<Pattern<'token, 'text>> {
        let mut reader = TokenReader::new(&self.tokens);
        let pattern = track_try!(Pattern::parse(&mut reader),
                                 "line_num={}",
                                 reader.line_num());
        Ok(pattern)
    }
    pub fn parse_type<'token>(&'token self) -> Result<Type<'token, 'text>> {
        let mut reader = TokenReader::new(&self.tokens);
        let ty = track_try!(Type::parse(&mut reader), "line_num={}", reader.line_num());
        Ok(ty)
    }
    pub fn parse_form<'token>(&'token self) -> Result<Form<'token, 'text>> {
        let mut reader = TokenReader::new(&self.tokens);
        let form = track_try!(Form::parse(&mut reader), "line_num={}", reader.line_num());
        Ok(form)
    }
    pub fn parse_module<'token>(&'token self) -> Result<ModuleDecl> {
        let mut reader = TokenReader::new(&self.tokens);
        let form = track_try!(ModuleDecl::parse(&mut reader),
                              "line_num={}",
                              reader.line_num());
        Ok(form)
    }
}
