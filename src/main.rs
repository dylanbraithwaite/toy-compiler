use LexingState::*;

enum Token {
	Operator(OperatorType),
	IntLitteral(u64),
}

enum OperatorType {
	Add,
	Sub,
}

#[derive(Copy, PartialEq)]
enum LexingState {
	None,
	Num,
	Add,
	Sub,
}

fn main() {
	let input = "5+72  -3+ 9 ";
	
	let mut current_state = None;
	let mut tokens = Vec::new();
	let mut working_string = String::new();
	
	for c in input.chars() 
	{
		if let (Some(new_state), finished_token) = lex_char(c, current_state)
		{
			if finished_token && new_state != current_state
			{ 
				if current_state != None { tokens.push(create_token(working_string, current_state)) };
				working_string = String::new();
			}
			current_state = new_state;
		}
		working_string.push(c);
	}

}

//Finished current token flag will be necessary later on when there are more complex tokens which use multiple states to process.
fn lex_char(c: char, current_state: LexingState) -> (Option<LexingState>, bool) {
	match (c, current_state) { //                   State Transition:    State Transition:
		(x,   _) if x.is_numeric()               => (Some(Num),           true),
		(x,   _) if x.is_whitespace()            => (Some(None),          true),
		('+', _)                                 => (Some(Add),          true),
		('-', _)                                 => (Some(Sub),         true),
		_                                        =>  panic!(),
	}
}

fn create_token(data: String, state: LexingState) -> Token {
	match state {
		Num   => Token::IntLitteral(data.parse().unwrap()),
		Add  => Token::Operator(OperatorType::Add),
		Sub => Token::Operator(OperatorType::Sub),
		_     => panic!(),
	}
}
