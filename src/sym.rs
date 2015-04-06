use std::collections::HashMap;


pub struct Info {
	pub val: i64,
}

pub type Syms = HashMap<String, Info>;

pub struct Table {
	children: Vec<Table>, //Represents nested scopes.
	pub syms: Syms,
	selected: bool,
	selected_index: usize, //Index into children for currently selected branch
}


impl Table {

	pub fn new () -> Table {
		Table {
			children: Vec::new(),
			syms: Syms::new(),
			selected: true,
			selected_index: 0,
		}
	}

	pub fn get (&self, id: &String) -> &Info {
		self.syms.get(id).unwrap()
	}
	
	
	fn current (&mut self) -> &mut Table { 
		if self.selected { 
			self 
		}
		else { 
			self.traverse().current() 
		}
	}

	
	fn traverse (&mut self) -> &mut Table {
		assert!(!self.selected);
		&mut self.children[self.selected_index]
	}


	fn add_scope (&mut self) {
		let curr = self.current();
		curr.children.push(Table::new());
		curr.selected_index += 1;
		curr.selected = false;
	}


	fn ptr_eq (a: *const Table, b: *const Table) -> bool { 
		a == b 
	}


	fn prev_aux (&mut self, curr: *const Table) -> &mut Table {
		if Table::ptr_eq(curr, self.traverse()) {
			self 
		}
		else {
			self.traverse().prev_aux(curr) 
		}
	}


	fn prev (&mut self) -> &mut Table{
		let curr = self.current() as *const Table;
		self.prev_aux(curr)
	}
	

	fn exit_scope (&mut self) {
		self.selected = false;
		let new = self.prev();
		new.selected = true;
	}
}
