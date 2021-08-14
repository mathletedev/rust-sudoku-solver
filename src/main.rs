use std::io::{self, Write};

fn find_empty(grid: [[u8; 9]; 9]) -> (usize, usize) {
	for (i, row) in grid.iter().enumerate() {
		for (j, val) in row.iter().enumerate() {
			if *val == 0 {
				return (i, j);
			}
		}
	}

	return (9, 9);
}

fn is_valid(grid: [[u8; 9]; 9], pos: (usize, usize), val: u8) -> bool {
	for i in 0..9 {
		if grid[pos.0][i] == val || grid[i][pos.1] == val {
			return false;
		}
	}

	let box_pos = (3 * (pos.0 / 3), 3 * (pos.1 / 3));
	for i in box_pos.0..box_pos.0 + 3 {
		for j in box_pos.1..box_pos.1 + 3 {
			if grid[i][j] == val {
				return false;
			};
		}
	}

	return true;
}

fn print_grid(grid: [[u8; 9]; 9]) {
	println!("\nFinished!\n┌───────┬───────┬───────┐");

	for (i, row) in grid.iter().enumerate() {
		for (j, val) in row.iter().enumerate() {
			if j % 3 == 0 {
				print!("│ ")
			}

			print!("{} ", val);
		}

		print!("│\n");

		if i == 2 || i == 5 {
			println!("├───────┼───────┼───────┤");
		}
	}

	println!("└───────┴───────┴───────┘");
}

fn solve(mut grid: [[u8; 9]; 9]) -> bool {
	let empty = find_empty(grid);

	if empty.0 == 9 {
		print_grid(grid);
		return true;
	}

	for n in 1..=9 {
		if is_valid(grid, empty, n) {
			grid[empty.0][empty.1] = n;

			if solve(grid) {
				return true;
			}
		}
	}

	grid[empty.0][empty.1] = 0;

	return false;
}

fn main() {
	println!("┌{}┐\n│ Rust Sudoku Solver │\n└{0}┘", "─".repeat(20));

	let mut grid = [[0; 9]; 9];
	let mut i = 0;

	'outer: while i < 9 {
		print!("Enter row {}: ", i + 1);
		io::stdout().flush().expect("Failed to write line");

		let mut row = String::new();

		io::stdin().read_line(&mut row).unwrap();

		let mut count = 0;
		for (j, num) in row.split_whitespace().enumerate() {
			count += 1;

			grid[i][j] = match num.parse::<u8>() {
				Ok(int) => int,
				Err(_) => {
					println!("Invalid input values, try again\n");
					continue 'outer;
				}
			}
		}

		if count != 9 {
			println!("Invalid input length, try again\n");
			continue;
		}

		i += 1;
	}

	println!("\nSolving...");

	solve(grid);
}
