use regex;
use self::TokenType::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
	LineEnd,
	Assign,
	Id,
	Sub,
	WhiteSpace,
	Add,
	Num,
}

pub struct Token {
	pub ttype: TokenType,
	pub data: String,
}

pub type Tokens = Vec<Token>;
pub type TokenSlice<'a> = &'a [Token];

static regexes : &'static [(regex::Regex, TokenType)] = &[
	(regex!(r";|\n"),                LineEnd),
	(regex!(r"\+"),                  Add),
	(regex!(r"-"),                   Sub),
	(regex!(r":="),                  Assign),
	(regex!(r"[:alpha:][:alnum:]*"), Id),
	(regex!(r"[:digit:]+"),          Num),
	(regex!(r"[:space:]+"),          WhiteSpace),
];

pub fn lex(mut input: &str) -> Tokens {

	let mut output = Tokens::new();

	while input.len() != 0 {
		for &(ref pattern, token) in regexes {
			if let Some((0, end)) = pattern.find(input) {

				let head = &input[..end];
				let tail = &input[end..];

				output.push(Token{ttype: token, data: head.to_string()});
				input = tail;
			
				break;
			}
		}
	}

	output
} 
