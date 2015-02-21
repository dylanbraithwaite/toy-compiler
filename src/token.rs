use lexer;
use self::Token::*;
use self::OperatorType::*;

#[derive(Debug)]
pub enum Token {
	Operator(OperatorType),
	IntLitteral(u64),
}

#[derive(Debug)]
pub enum OperatorType {
	Add,
	Sub,
}

impl Token {
	pub fn new(data: String, state: lexer::LexingState) -> Token {
		match state {
			lexer::Num  => IntLitteral(data.parse().unwrap()),
			lexer::Add  => Operator(Add),
			lexer::Sub  => Operator(Sub),
			_    => panic!(),
		}
	}
}
