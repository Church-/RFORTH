use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::env;
use std::io::{self, Read};
use std::error::Error;


pub struct VM {
	stack: Vec<usize>,
	alt_stack: Vec<usize>,
	prog: Vec<&str>,
	dict: Vec<&str>
}

impl VM {
	fn new(&self) -> VM {
		VM {
			stack: Vec::with_capacity(32), 
			alt_stack: Vec::with_capacity(32), 
			prog: Vec::with_cap,
			dict: Vec!["+", "-", "/", "*", ".", "emit", "mod", "dup", "rot", "swap", "tuck", ":", ";", 
			"if", "else", "then", "<", ">", "==", "drop", "finish", "or", "and"],
			};

	}
	
	fn load_prog(&mut self, file: String) {
		self.prog = file.split(" ").collect();
	}

}

fn repl() -> Result<Ok(),> {
	let mut vm = VM::new();
	loop {
		println!("$> "):
		let mut input = String::new();
		stdin().read_to_string(&mut input)?;
		vm.load_prog(input);
		exec_loop(&mut vm)?;
	}

	Ok()	
}

fn exec_file(file_name: &str) -> Result<Ok(),> {
	let mut vm = VM::new();
	let mut fd = File::open(file_name)?;
	let mut buffer = String::new();
	fd.read_to_end(&mut buffer)?;
	vm.load_prog(buffer);
	exec_loop(&mut vm)?
	Ok()
}

fn exec_loop(&mut vm: VM) -> Result<OK(),> {
	while let Some(op) = vm.prog.pop() {
		match op {
			"+" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				let sum = x + y;
				vm.stack.push(sum);
			},
			"-" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				let remainder = x - y;
				vm.stack.push(remainder);
			},
			"/" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				let remainder = y / x;
				vm.stack.push(remainder);
			},
			"*" => {
				let x = vm.stack.pop();
				let x = vm.stack.pop();
				let total = x * y;
				vm.stack.push(total);
			},
			"." => {
				let output = vm.stack.pop();
				println!("{}",output);
			},
			"emit" => {
				let x: char = vm.stack.pop() as char;
			},
			"drop" => {
				vm.stack.pop();
			},
			"dup" => {
				let x = vm.stack.pop();
				vm.stack.push(x);
				vm.stack.push(x);
			},
			"swap" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				vm.stack.push(y);
				vm.stack.push(x);
			},
			"swap" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				vm.stack.push(y);
				vm.stack.push(x);
				vm.stack.push(y);
			},
			".\"" -> {
				let tmp = String::new();
				loop {//Loop and pop items off the prog vec and push to string until a " is found. ???
					
				}
			},
			"<" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				if (y < x) {
					println!("{} is lesser then {}",y,x);
				}
				else {
					println!("{} is greater then {}",x,y);
				}
				vm.stack.push(y);
				vm.stack.push(x);
			},
			">" => {
				let x = vm.stack.pop();
				let y = vm.stack.pop();
				if (y > x) {
					println!("{} is greater then {}",y,x);
				}
				else {
					println!("{} is greater then {}",x,y);
				}
				vm.stack.push(y);
				vm.stack.push(x);
			},
			//Implement boolean operators at some point? Need to ask questions on that.
			"mod" => {
				
			},
			"if" => {},
			"else" => {},
			"then" => {},
			"if" => {},
			"do" => {},
												
		}
	} 
}
	

pub fn run() {
	let args = std::env::args().collect();
	match &args[1] {
		"-i" => repl(),
		"-e" => exec_file(&args[2]),
		"-h" => println!(" The commands are -e followed by a file name to execute the file, 
				 or -i to go into the interactive repl prompt."),
		_ => println!("Invalid command, read the helpfile and try again."),
	}
}