use std::fmt;

/*
	basic fields
*/
trait IsGameField {
	fn get_repr(&self) -> i32;
	fn get_energy(&self) -> i32;
	fn rain(&mut self) {}
	fn dry(&mut self) {}
	fn gatherRessources(&mut self) -> i32 {0}
	fn get_animals(&self) -> &std::vec::Vec<&IsAnimal>;
}
impl fmt::Display for IsGameField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S")
    }
}

struct Field<N: IsGameField> {
	inst: N,
}

impl<N: IsGameField> Field<N> {
}

impl<N: IsGameField> fmt::Display for Field<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}", self.inst.get_repr())
    }
}


struct SimpleField<'a> {
	repr: i32,	
	x: i32,	
	y: i32,	
	energy: i32,	
	animals: std::vec::Vec<&'a IsAnimal>,
}
impl<'a> IsGameField for SimpleField<'a> {
	fn get_repr(&self) -> i32 {
		self.repr
	}
	fn get_energy(&self) -> i32 {
		self.energy
	}
	fn get_animals(&self) -> &std::vec::Vec<& IsAnimal> {
		&self.animals
	}


	fn rain(&mut self) {
		self.energy += 1;
	}
	fn dry(&mut self) {
		self.energy -= 1;
	}
	fn gatherRessources(&mut self) -> i32 {
		if self.energy > 0 {
			self.energy -= 1;
			1
		} else {
			0
		}
	}
}
impl<'a> fmt::Display for SimpleField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S")
    }
}


struct FancyField<'a> {
	x: i32,	
	y: i32,	
	energy: i32,	
	animals: std::vec::Vec<&'a IsAnimal>,
}
impl<'a> IsGameField for FancyField<'a> {
	fn get_repr(&self) -> i32 {
		2
	}
	fn get_energy(&self) -> i32 {
		self.energy
	}
	fn gatherRessources(&mut self) -> i32 {
		let amount;
		if self.energy > 7 {
			amount = 3
		} else if self.energy > 3 {
			amount = 2
		} else if self.energy > 0 {
			amount = 1
		} else {
			amount = 0
		};
		self.energy -= amount;
		println!("{}", amount);
		amount		
	}
	fn get_animals(&self) -> &std::vec::Vec<& IsAnimal> {
		&self.animals
	}
}
impl<'a> fmt::Display for FancyField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F")
    }
}

enum FieldEnum {
	SimpleField(i32,i32,usize),
	FancyField(i32,i32,usize),
}

impl Copy for FieldEnum {
}

impl Clone for FieldEnum {
	fn clone(&self) -> Self {
		match *self {
			FieldEnum::SimpleField(a,b,c) => FieldEnum::SimpleField(a,b,c),
			FieldEnum::FancyField(a,b,c) => FieldEnum::FancyField(a,b,c)
		}
	}
}

impl FieldEnum {
	fn getType(&self) -> i32 {
		match *self {
			FieldEnum::SimpleField(a,b,c) => 0,
			FieldEnum::FancyField(a,b,c) => 1
		}
	}
	fn getFieldMut<'a>(&'a self, simple: &'a mut std::vec::Vec<SimpleField>, fancy: &'a mut std::vec::Vec<FancyField>) -> &mut IsGameField {
		match *self {
			FieldEnum::SimpleField(a,b,c) => &mut simple[c] as & mut IsGameField,
			FieldEnum::FancyField(a,b,c) => &mut fancy[c] as & mut IsGameField
		}
	}
	fn getPos<'a>(&'a self) -> (i32,i32) {
		match *self {
			FieldEnum::SimpleField(a,b,c) => (a,b),
			FieldEnum::FancyField(a,b,c) => (a,b)
		}
	}
	fn getField<'a>(&'a self, simple: &'a std::vec::Vec<SimpleField>, fancy: &'a std::vec::Vec<FancyField>) -> & IsGameField {
		match *self {
			FieldEnum::SimpleField(a,b,c) => &simple[c] as & IsGameField,
			FieldEnum::FancyField(a,b,c) => &fancy[c] as & IsGameField
		}
	}
}


/*
	basic animals
*/
trait IsAnimal {
	fn eat(&mut self, simple: &mut std::vec::Vec<SimpleField>, fancy: &mut std::vec::Vec<FancyField>) {
	}
	fn roam(&mut self, board: &std::vec::Vec<std::vec::Vec<FieldEnum>>) {
	}
	fn get_repr(&self) -> i32 {
		100
	}
}

struct SomeAnimal {
	field: FieldEnum,
	energy: i32,
}

impl IsAnimal for SomeAnimal {
	fn eat(&mut self, simple: &mut std::vec::Vec<SimpleField>, fancy: &mut std::vec::Vec<FancyField>) {
		self.energy += self.field.getFieldMut(simple,fancy).gatherRessources();
	}
	fn roam(&mut self, board: &std::vec::Vec<std::vec::Vec<FieldEnum>>) {
		let (x,y) = self.field.getPos();
		println!("{}/{}",x,y);
		println!("go left!!");

		if y > 0 {
			self.field = board[x as usize][(y-1) as usize];
		}

		let (x,y) = self.field.getPos();
		println!("{}/{}",x,y);
	}
	fn get_repr(&self) -> i32 {
		self.energy + 100
	}
}


/*
	usefull stuff
*/
fn displayBoard (board : &std::vec::Vec<std::vec::Vec<FieldEnum>>, simple: &std::vec::Vec<SimpleField>, fancy: &std::vec::Vec<FancyField>) {
	for x in 0..9 {
		for y in 0..9 {
			print!(" {}",board[x][y].getField(simple,fancy).get_repr());
		}
		print!("\t");
		for y in 0..9 {
			print!(" {}",board[x][y].getField(simple,fancy).get_energy());
		}
		print!("\n");
	};
	print!("\n");
	print!("\n");
}
fn addRessources (board : &std::vec::Vec<std::vec::Vec<FieldEnum>>, simple : &mut std::vec::Vec<SimpleField>, fancy: &mut std::vec::Vec<FancyField>) {
	for x in 0..9 {
		for y in 0..9 {
			board[x][y].getFieldMut(simple,fancy).rain();
		}
	};
}
fn drainRessources (board : &std::vec::Vec<std::vec::Vec<FieldEnum>>, simple : &mut std::vec::Vec<SimpleField>, fancy: &mut std::vec::Vec<FancyField>) {
	for x in 0..9 {
		for y in 0..9 {
			board[x][y].getFieldMut(simple,fancy).dry();
		}
	};
}

fn main() {
	let mut simpleFields : std::vec::Vec<SimpleField> = Vec::new();
	let mut fancyFields : std::vec::Vec<FancyField> = Vec::new();

	for x in 0..9 {
		for y in 0..9 {
			if y > 6 {
				simpleFields.push(SimpleField { repr: 2, x:x, y:y , energy:4 , animals:Vec::new() });
			} else {
				fancyFields.push(FancyField { x:x, y:y, energy:4, animals:Vec::new() });
			}
		}
	};

	let mut board : std::vec::Vec<std::vec::Vec<FieldEnum>> = Vec::new();
	let mut fancyCounter = 0;
	let mut simpleCounter = 0;
	for x in 0..9 {
		let mut row : std::vec::Vec<FieldEnum> = Vec::new();
		for y in 0..9 {
			if simpleCounter < simpleFields.len() && simpleFields[simpleCounter].x == x && simpleFields[simpleCounter].y == y {
				row.push(FieldEnum::SimpleField(x,y,simpleCounter));
				simpleCounter += 1;
			} else if fancyCounter < fancyFields.len() && fancyFields[fancyCounter].x == x && fancyFields[fancyCounter].y == y { 
				row.push(FieldEnum::FancyField(x,y,fancyCounter));
				fancyCounter += 1;
			}
			
		}
		board.push(row);
	};

	displayBoard(&board, &simpleFields, &fancyFields);

	let mut animal = SomeAnimal { field: board[4][7], energy: 5 };

	//ten loops for now
	for x in 0..9 {
		addRessources(&board, &mut simpleFields, &mut fancyFields);
		addRessources(&board, &mut simpleFields, &mut fancyFields);
		drainRessources(&board, &mut simpleFields, &mut fancyFields);
		animal.eat(&mut simpleFields, &mut fancyFields);
		animal.roam(&board);
		println!("{}",animal.get_repr());
		displayBoard(&board, &simpleFields, &fancyFields);
	}
}

