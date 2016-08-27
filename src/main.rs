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
	//fn get_animals(&self) -> &std::vec::Vec<&IsAnimal>;
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


struct SimpleField {
	repr: i32,	
	x: i32,	
	y: i32,	
	energy: i32,	
	//animals: std::vec::Vec<&'a IsAnimal>,
}
impl IsGameField for SimpleField {
	fn get_repr(&self) -> i32 {
		self.repr
	}
	fn get_energy(&self) -> i32 {
		self.energy
	}
	/*fn get_animals(&self) -> &std::vec::Vec<& IsAnimal> {
		&self.animals
	}*/


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
impl fmt::Display for SimpleField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S")
    }
}


struct FancyField {
	x: i32,	
	y: i32,	
	energy: i32,	
	//animals: std::vec::Vec<&'a IsAnimal>,
}
impl IsGameField for FancyField {
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
	/*fn get_animals(&self) -> &std::vec::Vec<& IsAnimal> {
		&self.animals
	}*/
}
impl fmt::Display for FancyField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F")
    }
}


/*
	basic animals
*/
/*
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
*/

struct Board<'a, 'b, 'c> {
	simple: &'a mut std::vec::Vec<SimpleField>,
	fancy:  &'b mut std::vec::Vec<FancyField>,
	byPos:  &'c mut std::vec::Vec<std::vec::Vec<FieldEnum>>,
}
impl<'a, 'b, 'c> Board<'a, 'b, 'c> {
	fn getFieldRepr(&self, x: usize, y: usize) -> i32 {
		self.byPos[x][y].getField(self).get_energy()
	}

	fn rain(&mut self) {
		for x in 0..9 {
			for y in 0..9 {
				self.byPos[x][y].clone().getFieldMut(self).rain();
			}
		}
	}

	fn dry(&mut self) {
		for x in 0..9 {
			for y in 0..9 {
				self.byPos[x][y].clone().getFieldMut(self).dry();
			}
		}
	}
}
impl<'a, 'b, 'c> fmt::Display for Board<'a, 'b, 'c> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in 0..9 {
			for y in 0..9 {
				write!(f," {}", self.byPos[x][y].getField(self).get_energy());
			}
			write!(f,"\n");
		}
		write!(f,"\n");
		write!(f,"\n")
	}
}

enum FieldEnum {
	simple(usize),
	fancy(usize),
}
impl FieldEnum {
	fn getField<'a, 'b, 'c, 'd>(&'a self, board: &'a Board<'b, 'c, 'd>) -> &'a IsGameField {
		match *self {
			FieldEnum::simple(a) => &board.simple[a],
			FieldEnum::fancy(a) => &board.fancy[a],
		}
	}
	fn getFieldMut<'a, 'b, 'c, 'd, 'e>(&'a self, board: &'a mut Board<'b, 'c, 'd>) -> &'a mut IsGameField {
		match *self {
			FieldEnum::simple(a) => &mut board.simple[a],
			FieldEnum::fancy(a) => &mut board.fancy[a],
		}
	}
}
impl Copy for FieldEnum {
}

impl Clone for FieldEnum {
	fn clone(&self) -> Self {
		match *self {
			FieldEnum::simple(a) => FieldEnum::simple(a),
			FieldEnum::fancy(a) => FieldEnum::fancy(a)
		}
	}
}


/*
fn getMutable<'a, T:IsGameField> ( index: usize, vector : &'a mut std::vec::Vec<T>) -> &'a mut IsGameField {
	&mut vector[index]
}

fn get<'a> ( index: usize, board: Board) -> &'a IsGameField {
	&vector[index]
}*/

fn main() {
	let mut simpleFields : std::vec::Vec<SimpleField> = Vec::new();
	let mut fancyFields : std::vec::Vec<FancyField> = Vec::new();
	let mut fieldsByPos : std::vec::Vec<std::vec::Vec<FieldEnum>> = Vec::new();

	for x in 0..9 {
		for y in 0..9 {
			if y > 4 {
				simpleFields.push(SimpleField { repr: 2, x:x, y:y , energy:4 } );
			} else {
				fancyFields.push(FancyField { x:x, y:y, energy:4 } );
			}
		}
	};

	let mut fancyCounter = 0;
	let mut simpleCounter = 0;
	for x in 0..9 {
		let mut row : std::vec::Vec<FieldEnum> = Vec::new();
		for y in 0..9 {
			if simpleCounter < simpleFields.len() && simpleFields[simpleCounter].x == x && simpleFields[simpleCounter].y == y {
				row.push(FieldEnum::simple(simpleCounter));
				simpleCounter += 1;
			} else if fancyCounter < fancyFields.len() && fancyFields[fancyCounter].x == x && fancyFields[fancyCounter].y == y { 
				row.push(FieldEnum::fancy(simpleCounter));
				fancyCounter += 1;
			}
			
		}
		fieldsByPos.push(row);
	};

	let mut board = Board{simple: &mut simpleFields, fancy: &mut fancyFields, byPos: &mut fieldsByPos};

	println!("{}",board);
	board.rain();
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
}
