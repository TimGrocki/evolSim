use std::fmt;

struct Pos {
	x: u32,
	y: u32
}

struct Field {
	absPos: Pos,
	ressources: u16,
}

impl Field {
	fn test(&mut self) {
		self.ressources -= 1;
		println!("{}",self.ressources);
		println!("testing....");
	}
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.ressources)
    }
}

struct Animal<'board> {
	field: &'board mut Field,
	testing: i16,
}

impl<'board> Animal<'board> {
	fn eat(&mut self) {
		self.testing = 20;
		self.field.test();
		print!("{}",self.testing);
		print!("nomnom");
		print!("\n");
		print!("\n");
	}
}

fn displayRessources(board:&Vec<Vec<Field>>) {
	for x in 0..9 {
		for y in 0..9 {
			print!(" {}", board[x][y].ressources);
		}
		print!("\n");
	};
	print!("\n");
	print!("\n");
}

fn addRessources(board:&mut Vec<Vec<Field>>) {
	for x in 0..9 {
		for y in 0..9 {
			board[x][y].ressources += 1;
		}
	};
}

fn main() {
	let mut board = vec![];
	for x in 0..9 {
		let mut row = vec![];
		for y in 0..9 {
			row.push(Field { absPos:Pos { x:x, y:y }, ressources:0 } );
		}
		board.push(row);
	};
	print!("\n");
	print!("\n");

	for x in 0..9 {
		for y in 0..9 {
			print!(" {}/{}", board[x][y].absPos.x, board[x][y].absPos.y);
		}
		print!("\n");
	};
	print!("\n");
	print!("\n");

	displayRessources(&board);
	
	//ten loops for now
	for 0..9 {
		{
			let mut animal = Animal { field:&mut board[3][2], testing:23 };
			animal.eat();
		}
		addRessources(&mut board);
	}
	displayRessources(&board);

}

