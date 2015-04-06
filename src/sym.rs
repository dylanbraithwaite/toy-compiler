use std::collections::HashMap;


pub struct Info {
	pub val: i64,
}

pub type Syms = HashMap<String, Info>;

pub struct Table {
	pub syms: Syms,
}


impl Table {

	pub fn new () -> Table {
		Table {
			syms: Syms::new(),
		}
	}

	pub fn get (&self, id: &String) -> &Info {
		self.syms.get(id).unwrap()
	}
}
