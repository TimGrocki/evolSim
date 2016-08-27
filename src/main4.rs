use std::fmt;

/*
 basisc
*/
struct Pos {
	x: u32,
	y: u32
}

/*
 fields
*/
enum AbstractField<N: test> {
	Field(N),
	test1(Pos,u32),
	test2(Pos,u32),
	test3(Pos,u32),
	AnimalField(Pos,u32),
	PlantField(Pos,u32)
}	

impl AbstractField {
	fn test (&self) {
		match *self {
			//AbstractField::Field(a,b) => println!("testing....Field"),
			//AbstractField::AnimalField(a,b) => println!("testing....AnimalField"),
			//AbstractField::PlantLevel(a,b) => println!("testing....PlantLevel")
			AbstractField::Field(&f) => println!("pos: "),
			_ => println!("testing...mäh")
		}
	}

	fn getRessources(&self) -> u32 {
		match *self {
			//AbstractField::Field(Pos { x:x, y:y },b) => b,
			//AbstractField::AnimalField(Pos { x:x, y:y },b) => b,
			//AbstractField::PlantField(Pos { x:x, y:y },b) => b,
			//_ => 0
			AbstractField::Field(N) => 0,
			_ => 0
		}
	}

	fn incrementRessources(&mut self) {
		match *self {
			AbstractField::Field(&f) => println!("pos: "),
			_ => ()
		};
	}
}

impl fmt::Display for AbstractField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,".")
    }
}

struct Field {
	absPos: Pos,
	ressources: u32,
}

impl Field {
	fn test (&self) {
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
		println!("testing2....");
        }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.ressources)
    }
}

struct AnimalField {
	ressources: u32,
}

impl fmt::Display for AnimalField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"X")
    }
}

struct PlantField {
	plantLevel: u32,
	ressources: u32,
}

impl fmt::Display for PlantField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"X")
    }
}

/*
 animals
*/
struct Animal<'board> {
	field: &'board mut Field,
	testing: i16,
}

impl<'board> Animal<'board> {
	fn eat(&mut self) {
		//self.testing = 20;
		//self.field.test();
		print!("{}",self.testing);
		print!("nomnom");
		print!("\n");
		print!("\n");
	}
}

fn displayRessources(board:&Vec<Vec<AbstractField>>) {
	for x in 0..9 {
		for y in 0..9 {
			print!(" {}", board[x][y].getRessources());
		}
		print!("\n");
	};
	print!("\n");
	print!("\n");
}

fn addRessources(board:&mut Vec<Vec<AbstractField>>) {
	for x in 0..9 {
		for y in 0..9 {
			board[x][y].incrementRessources();
		}
	};
}

fn main() {
	let mut board = vec![];
	for x in 0..9 {
		let mut row = vec![];
		for y in 0..9 {
			row.push(AbstractField::Field( Field {absPos:Pos { x:x, y:y }, ressources:0 }));
		}
		board.push(row);
	};
	print!("\n");
	print!("\n");

	for x in 0..9 {
		for y in 0..9 {
			print!(" {}",board[x][y])
		}
		print!("\n");
	};
	print!("\n");
	print!("\n");

	displayRessources(&board);
	
	//ten loops for now
	for x in 0..9 {
		{
			//let mut animal = Animal { field:&mut board[3][2], testing:23 };
			//animal.eat();
		}
		addRessources(&mut board);
		displayRessources(&board);
	}
	displayRessources(&board);

}

