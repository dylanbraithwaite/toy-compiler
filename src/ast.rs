use sym;

pub struct AddNode(pub Box<Node>, pub Box<Node>, pub bool);
	
pub struct AssignNode(pub String, pub Box<Node>);

pub struct IdNode(pub String);

pub struct NumNode(pub i64);

pub struct ScopeNode(pub Vec<Box<Node>>);//Maybe this should own symbol table?


pub trait Node {
	fn codegen(&self, syms: &mut sym::Table) -> i64;
}


impl Node for AssignNode {
	fn codegen(&self, syms: &mut sym::Table) -> i64 {
		let &AssignNode(ref id, ref rhs) = self;
		if syms.syms.contains_key(id) {
			panic!("Multiple assignments to same identifier: {}", id); //TODO: allow mutation
		}
		else {
			let rhs_val: i64 = rhs.codegen(syms).clone();
			syms.syms.insert(id.clone(), sym::Info{ val: rhs_val });
		}
		0i64
	}
}


impl Node for IdNode {
	fn codegen(&self, syms: &mut sym::Table) -> i64 {
		let &IdNode(ref id) = self;
		syms.syms[id].val.clone()
	}
}


impl Node for AddNode {
	fn codegen(&self, syms: &mut sym::Table) -> i64 {
		let &AddNode(ref lhs, ref rhs, ref neg) = self;
		lhs.codegen(syms) + rhs.codegen(syms) * if *neg {-1} else {1}
	}
}


impl Node for NumNode {
	fn codegen(&self, syms: &mut sym::Table) -> i64 {
		let &NumNode(ref val) = self;
		val.clone()
	}
}


impl Node for ScopeNode {
	fn codegen(&self, syms: &mut sym::Table) -> i64 {
		let mut return_val = 0;
		let &ScopeNode(ref lines) = self;
		for line in lines {
			//Allow for side effects from codegen without using the exprs' vals.
			return_val = line.codegen(syms); 
		}
		return_val
	}
}
