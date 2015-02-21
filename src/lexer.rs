pub use self::LexingState::*;
use token::Token;

#[derive(Copy, PartialEq)]
pub enum LexingState {
	None,
	Num,
	Add,
	Sub,
}

pub fn lex(input: &str) -> Vec<Token> {
	let mut curr_state = None;
	let mut tokens = Vec::new();
	let mut curr_str = String::new();
	
	for c in input.chars() {
		let result = lex_char(c, curr_state);
		if let (Some(new_state), done_tok) = result {
			if done_tok && new_state != curr_state {
				if curr_state != None {
					tokens.push(Token::new(curr_str, curr_state));
				}
				curr_str = String::new();
			}
			curr_state = new_state;
		}
		curr_str.push(c);
	}
	tokens.push(Token::new(curr_str, curr_state));
	return tokens
}

//Finished current token flag will be necessary later on when there are more complex tokens which use multiple states to process.
fn lex_char(c: char, current_state: LexingState) -> (Option<LexingState>, bool) {
	match (c, current_state) { //                   State Transition:    State Transition:
		(x,   _) if x.is_numeric()               => (Some(Num),         true),
		(x,   _) if x.is_whitespace()            => (Some(None),          true),
		('+', _)                                 => (Some(Add),           true),
		('-', _)                                 => (Some(Sub),           true),
		_                                        =>  panic!(),
	}
}
