use std::fmt;

/*
	basic fields
*/
trait IsGameField {
	fn get_repr(&self) -> i32 {
		33
	}
	fn rain(&mut self) {
	}
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
}
impl IsGameField for SimpleField {
	fn get_repr(&self) -> i32 {
		self.repr
	}

	fn rain(&mut self) {
		self.repr += 1;
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
}
impl IsGameField for FancyField {
	fn get_repr(&self) -> i32 {
		5
	}

	fn rain(&mut self) {
	}
}
impl fmt::Display for FancyField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F")
    }
}

fn displayBoard (board : &std::vec::Vec<std::vec::Vec<&mut IsGameField>>) {
	for x in 0..9 {
		for y in 0..9 {
			print!(" {}",board[x][y].get_repr());
		}
		print!("\n");
	};
	print!("\n");
	print!("\n");
}
fn addRessources (board : &mut std::vec::Vec<std::vec::Vec<&mut IsGameField>>) {
	for x in 0..9 {
		for y in 0..9 {
			//board[x][y].rain();
		}
	};
}

fn main() {
	let mut simpleFields : std::vec::Vec<SimpleField> = Vec::new();
	let mut fancyFields : std::vec::Vec<FancyField> = Vec::new();

	for x in 0..9 {
		for y in 0..9 {
			if y > 3 {
				simpleFields.push(SimpleField { repr: 2, x:x, y:y });
			} else {
				fancyFields.push(FancyField { x:x, y:y });
			}
		}
	};

	let mut board : std::vec::Vec<std::vec::Vec<Box<&mut IsGameField>>> = Vec::new();
	let mut fancyCounter = 0;
	let mut simpleCounter = 0;
	for x in 0..9 {
		let mut row : std::vec::Vec<&mut IsGameField> = Vec::new();
		for y in 0..9 {
			if simpleFields[simpleCounter].x == x && simpleFields[simpleCounter].y == y {
				row.push(&mut simpleFields[simpleCounter] as &mut IsGameField);
				simpleCounter += 1;
			} else if fancyFields[fancyCounter].x == x && fancyFields[fancyCounter].y == y { 
				row.push(&mut fancyFields[fancyCounter] as &mut IsGameField);
				fancyCounter += 1;
			}
			
		}
		board.push(row);
	};

	{let ref mut test1 = simpleFields[0];};
	{let ref mut test2 = simpleFields[1];};
	//let mut row : std::vec::Vec<&mut IsGameField> = Vec::new();
	//let mut row2 : std::vec::Vec<&mut IsGameField> = Vec::new();
	//row.push(&mut simpleFields[0] as &mut IsGameField);
	//row2.push(&mut simpleFields[1] as &mut IsGameField);

	/*

	let mut simpleFields2 : std::vec::Vec<SimpleField> = Vec::new();
	board[0][0].rain();
	print!("\n");
	print!("\n");

	displayBoard(&board);

	//ten loops for now
	for x in 0..9 {
		{
			//let mut animal = Animal { field:&mut board[3][2], testing:23 };
			//animal.eat();
		}
		//addRessources(&mut board);
		displayBoard(&board);
	}
	displayBoard(&board);
	*/
}

