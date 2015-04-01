use token;
use self::LexState::*;

#[derive(Copy, PartialEq)]
pub enum LexState {
	None,
	Num,
	Add,
	Sub,
}

pub fn lex(input: &str) -> Vec<token::Tok> {
	let mut curr_state = None;
	let mut tokens = Vec::new();
	let mut curr_str = String::new();
	
	for c in input.chars() {
		let new_state = lex_char(c, curr_state);
		if curr_state != new_state {
			if curr_state != None {
				tokens.push(token::Tok::new(curr_str, curr_state));
			}
			curr_str = String::new();
			curr_state = new_state;
		}
		curr_str.push(c);
	}

	if curr_state != None {
		tokens.push(token::Tok::new(curr_str, curr_state));
	}

	return tokens
}

fn lex_char(c: char, current_state: LexState) -> LexState {
	match (c, current_state) { 
		(x,   _) if x.is_numeric()    => Num,
		(x,   _) if x.is_whitespace() => None,
		('+', _)                      => Add,
		('-', _)                      => Sub,
		_                             => panic!(),
	}
}
