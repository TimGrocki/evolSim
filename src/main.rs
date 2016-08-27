use std::fmt;

/*
	basic fields
*/
trait IsGameField {
	fn get_repr(&self, &Board) -> i32;
	fn get_energy(&self) -> i32;
	fn rain(&mut self) {}
	fn dry(&mut self) {}
	fn tick(&self) {}
	fn gather_ressources(&mut self) -> i32 {0}
	fn get_animals(&self) -> &std::vec::Vec<AnimalEnum>;
	fn add_animal(& mut self, AnimalEnum);
}
impl fmt::Display for IsGameField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S")
    }
}

struct SimpleField {
	repr: i32,	
	x: i32,	
	y: i32,	
	energy: i32,	
	animals: std::vec::Vec<AnimalEnum>,
}
impl IsGameField for SimpleField {
	fn get_repr(&self, board: &Board) -> i32 {
		if self.animals.len() > 0 {
			self.animals[0].get_animal(board).get_repr()
		} else {
			self.repr
		}
	}
	fn get_energy(&self) -> i32 {
		self.energy
	}
	fn get_animals(&self) -> &std::vec::Vec<AnimalEnum> {
		&self.animals
	}


	fn rain(&mut self) {
		self.energy += 1;
	}
	fn dry(&mut self) {
		self.energy -= 1;
	}
	fn tick(&self) {
		for animal in &self.animals {
			//self.animals[0].get_animal(board).tick()
		}
	}
	fn gather_ressources(&mut self) -> i32 {
		if self.energy > 0 {
			self.energy -= 1;
			1
		} else {
			0
		}
	}
	fn add_animal(& mut self, animal: AnimalEnum) {
		self.animals.push(animal);
	}
}
impl fmt::Display for SimpleField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S")
    }
}


struct FancyField {
	x: i32,	
	y: i32,	
	energy: i32,	
	animals: std::vec::Vec<AnimalEnum>,
}
impl IsGameField for FancyField {
	fn get_repr(&self, board: &Board) -> i32 {
		if self.animals.len() > 0 {
			self.animals[0].get_animal(board).get_repr()
		} else {
			1
		}
	}
	fn get_energy(&self) -> i32 {
		self.energy
	}
	fn gather_ressources(&mut self) -> i32 {
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
	fn get_animals(&self) -> &std::vec::Vec<AnimalEnum> {
		&self.animals
	}
	fn add_animal(& mut self, animal: AnimalEnum) {
		self.animals.push(animal);
	}
	fn tick(&self) {
		for animal in &self.animals {
			println!("animal tick")
		}
	}
}
impl fmt::Display for FancyField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F")
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
		8
	}
	fn tick(&self) {
	}
}

struct SomeAnimal {
	energy: i32,
}

impl IsAnimal for SomeAnimal {
}
struct SomeOtherAnimal {
	energy: i32,
}

impl IsAnimal for SomeOtherAnimal {
	fn get_repr(&self) -> i32 {
		self.energy + 100;
		9
	}
}
/*
	fn eat(&mut self, simple: &mut std::vec::Vec<SimpleField>, fancy: &mut std::vec::Vec<FancyField>) {
		self.energy += self.field.get_field_mut(simple,fancy).gatherRessources();
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
}
*/

struct Board<'a, 'b, 'c> {
	simple: &'a mut std::vec::Vec<SimpleField>,
	fancy:  &'b mut std::vec::Vec<FancyField>,
	animals: &'a mut std::vec::Vec<SomeAnimal>,
	other_animals:  &'b mut std::vec::Vec<SomeOtherAnimal>,
	by_pos:  &'c mut std::vec::Vec<std::vec::Vec<FieldEnum>>,
}
impl<'a, 'b, 'c> Board<'a, 'b, 'c> {
	fn get_field_repr(&self, x: usize, y: usize) -> i32 {
		self.by_pos[x][y].get_field(self).get_repr(self)
	}

	fn rain(&mut self) {
		for x in 0..9 {
			for y in 0..9 {
				self.by_pos[x][y].clone().get_field_mut(self).rain();
			}
		}
	}

	fn dry(&mut self) {
		for x in 0..9 {
			for y in 0..9 {
				self.by_pos[x][y].clone().get_field_mut(self).dry();
			}
		}
	}

	fn tick(&mut self) {
		for x in 0..9 {
			for y in 0..9 {
				self.by_pos[x][y].clone().get_field(self).tick();
			}
		}
	}

	fn add_animal(&mut self, animal: SomeAnimal, x:usize , y:usize) {
		self.animals.push(animal);
		let animal_pointer = AnimalEnum::Simple(self.animals.len()-1);
		self.by_pos[x][y].clone().get_field_mut(self).add_animal(animal_pointer);
	}

	fn add_animal2(&mut self, animal: SomeOtherAnimal, x:usize , y:usize) {
		self.other_animals.push(animal);
		let animal_pointer = AnimalEnum::Other(self.other_animals.len()-1);
		self.by_pos[x][y].clone().get_field_mut(self).add_animal(animal_pointer);
	}
}
impl<'a, 'b, 'c> fmt::Display for Board<'a, 'b, 'c> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in 0..9 {
			for y in 0..9 {
				write!(f," {}", self.by_pos[x][y].get_field(self).get_repr(self));
			}
			write!(f,"\n");
		}
		write!(f,"\n");
		write!(f,"\n")
	}
}

enum FieldEnum {
	Simple(usize),
	Fancy(usize),
}
impl FieldEnum {
	fn get_field<'a, 'b, 'c, 'd>(&'a self, board: &'a Board<'b, 'c, 'd>) -> &'a IsGameField {
		match *self {
			FieldEnum::Simple(a) => &board.simple[a],
			FieldEnum::Fancy(a) => &board.fancy[a],
		}
	}
	fn get_field_mut<'a, 'b, 'c, 'd, 'e>(&'a self, board: &'a mut Board<'b, 'c, 'd>) -> &'a mut IsGameField {
		match *self {
			FieldEnum::Simple(a) => &mut board.simple[a],
			FieldEnum::Fancy(a) => &mut board.fancy[a],
		}
	}
}
impl Copy for FieldEnum {
}

impl Clone for FieldEnum {
	fn clone(&self) -> Self {
		match *self {
			FieldEnum::Simple(a) => FieldEnum::Simple(a),
			FieldEnum::Fancy(a) => FieldEnum::Fancy(a)
		}
	}
}

enum AnimalEnum {
	Simple(usize),
	Other(usize),
}
impl AnimalEnum {
	fn get_animal<'a, 'b, 'c, 'd>(&'a self, board: &'a Board<'b, 'c, 'd>) -> &'a IsAnimal {
		match *self {
			AnimalEnum::Simple(a) => &board.animals[a],
			AnimalEnum::Other(a) => &board.other_animals[a],
		}
	}
	fn get_animal_mut<'a, 'b, 'c, 'd>(&'a self, board: &'a mut Board<'b, 'c, 'd>) -> &'a mut IsAnimal {
		match *self {
			AnimalEnum::Simple(a) => &mut board.animals[a],
			AnimalEnum::Other(a) => &mut board.other_animals[a],
		}
	}
}
impl Copy for AnimalEnum {
}

impl Clone for AnimalEnum {
	fn clone(&self) -> Self {
		match *self {
			AnimalEnum::Simple(a) => AnimalEnum::Simple(a),
			AnimalEnum::Other(a) => AnimalEnum::Other(a)
		}
	}
}

fn main() {
	let mut simple_fields : std::vec::Vec<SimpleField> = Vec::new();
	let mut fancy_fields : std::vec::Vec<FancyField> = Vec::new();
	let mut fields_by_pos : std::vec::Vec<std::vec::Vec<FieldEnum>> = Vec::new();
	let mut animals : std::vec::Vec<SomeAnimal> = Vec::new();
	let mut other_animals : std::vec::Vec<SomeOtherAnimal> = Vec::new();

	for x in 0..9 {
		for y in 0..9 {
			if y > 4 {
				simple_fields.push(SimpleField { repr: 2, x:x, y:y , energy:4, animals:Vec::new() } );
			} else {
				fancy_fields.push(FancyField { x:x, y:y, energy:4, animals:Vec::new() } );
			}
		}
	};

	let mut fancy_counter = 0;
	let mut simple_counter = 0;
	for x in 0..9 {
		let mut row : std::vec::Vec<FieldEnum> = Vec::new();
		for y in 0..9 {
			if simple_counter < simple_fields.len() && simple_fields[simple_counter].x == x && simple_fields[simple_counter].y == y {
				row.push(FieldEnum::Simple(simple_counter));
				simple_counter += 1;
			} else if fancy_counter < fancy_fields.len() && fancy_fields[fancy_counter].x == x && fancy_fields[fancy_counter].y == y { 
				row.push(FieldEnum::Fancy(fancy_counter));
				fancy_counter += 1;
			}
			
		}
		fields_by_pos.push(row);
	};

	animals.push(SomeAnimal { energy: 20 } );
	animals.push(SomeAnimal { energy: 20 } );
	animals.push(SomeAnimal { energy: 20 } );
	other_animals.push(SomeOtherAnimal { energy: 20 } );
	other_animals.push(SomeOtherAnimal { energy: 20 } );

	let mut board = Board{simple: &mut simple_fields, fancy: &mut fancy_fields, by_pos: &mut fields_by_pos, animals: &mut animals, other_animals: &mut other_animals};

	println!("{}",board);
	board.rain();
	board.add_animal(SomeAnimal { energy: 20 }, 1, 0);
	board.add_animal(SomeAnimal { energy: 20 }, 1, 7);
	board.add_animal2(SomeOtherAnimal { energy: 20 }, 5, 7);
	board.add_animal(SomeAnimal { energy: 20 }, 2, 3);
	board.add_animal2(SomeOtherAnimal { energy: 20 }, 7, 4);
	println!("{}",board);
	board.dry();
	board.dry();
	board.dry();
	println!("{}",board);
	board.rain();
	board.rain();
	board.rain();
	board.rain();
	println!("{}",board);
	board.tick();
	println!("{}",board);
	board.tick();
	println!("{}",board);
	board.tick();
	println!("{}",board);
	board.tick();
	println!("{}",board);
}
