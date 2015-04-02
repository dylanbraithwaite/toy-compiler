pub trait Node {
	fn codegen(&self) -> i64;
}

pub struct AddNode(pub Box<Node>, pub Box<Node>, pub bool);

pub struct NumNode(pub i64);

impl Node for AddNode {
	fn codegen(&self) -> i64 {
		let &AddNode(ref lhs, ref rhs, ref neg) = self;
		lhs.codegen() + rhs.codegen() * if *neg {-1} else {1}
	}
}

impl Node for NumNode {
	fn codegen(&self) -> i64{
		let &NumNode(ref val) = self;
		val.clone()
	}
}
