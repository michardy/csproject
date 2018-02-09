extern crate chrono;

use std::env;

use std::io;
use std::io::Write;

use std::fs;
use std::fs::File;

use chrono::{Local, DateTime};

use std::process::Command;

const DIREC: &'static str = "/home/michael/Code/cs/";
const PROJ: &'static str = "/home/michael/projects/";

fn input(prompt: &'static str)->String {
	print!("{}", prompt);
	std::io::stdout().flush().ok().expect("Could not flush STDOUT!");
	let mut uin = String::new();
	io::stdin().read_line(&mut uin).ok().expect("Error reading line");
	uin
}

#[derive(Clone)]
struct TemplatedFile {
	name: String,
	desc: String,
	date: DateTime<Local>,
	letter: String,
	dir: String
}

impl TemplatedFile {
	fn write_java(self) {
		let mut template = include_str!("java.java").to_string();
		let date = self.date.format("%Y-%m-%d").to_string();
		template = template.replace("%l", &self.letter);
		template = template.replace("%d", &date);
		template = template.replace("%n", &self.name);
		template = template.replace("%D", &self.desc);
		let mut file = File::create(self.dir + "/" + &self.name + ".java")
			.expect("Error creating java file");
		file.write_fmt(
			format_args!("{}", template)
		).expect("Error writing to java file");
	}
	fn write_c(self) {
		let mut template = include_str!("c.c").to_string();
		let date = self.date.format("%Y-%m-%d").to_string();
		template = template.replace("%l", &self.letter);
		template = template.replace("%d", &date);
		template = template.replace("%n", &self.name);
		template = template.replace("%D", &self.desc);
		let mut file = File::create(self.dir + "/" + &self.name + ".c")
			.expect("Error creating c file");
		file.write_fmt(
			format_args!("{}", template)
		).expect("Error writing to c file");
	}
}

struct MakeFile {
	letter: String,
	date: DateTime<Local>,
	main: String,
	files: String,
	classes: String,
	assn: String,
	dir: String
}

impl MakeFile {
	fn from_files(
		names: Vec<TemplatedFile>,
		main: String,
		letter: String,
		date: DateTime<Local>,
		assn: String,
		dir: String
	) -> MakeFile {
		let mut files: String = String::new();
		let mut classes: String = String::new();
		for n in &names {
			if n.name != main {
				files += &(String::from(" ") + &n.name + ".java");
			}
		}
		for n in names {
			if n.name != main {
				classes += &(String::from(" ") + &n.name + ".class");
			}
		}
		MakeFile {
			letter: letter,
			date: date,
			main: main,
			files: files,
			classes: classes,
			assn: assn,
			dir: dir
		}
	}
	fn write_java(self) {
		let mut template = include_str!("JMakefile").to_string();
		let date = self.date.format("%Y-%m-%d").to_string();
		template = template.replace("%l", &self.letter);
		template = template.replace("%a", &self.assn);
		template = template.replace("%d", &date);
		template = template.replace("%n", "Makefile");
		template = template.replace("%C", &self.main);
		template = template.replace("%j", &self.files);
		template = template.replace("%c", &self.classes);
		let mut file = File::create(self.dir + "/Makefile").expect("Error creating Makefile");
		file.write_fmt(
			format_args!("{}", template)
		).expect("Error writing Makefile");
	}
	fn write_c(self) {
		let mut template = include_str!("CMakefile").to_string();
		let date = self.date.format("%Y-%m-%d").to_string();
		template = template.replace("%l", &self.letter);
		template = template.replace("%a", &self.assn);
		template = template.replace("%d", &date);
		template = template.replace("%n", "Makefile");
		template = template.replace("%m", &self.main);
		template = template.replace("%s", &self.files);
		template = template.replace("%o", &self.classes);
		let mut file = File::create(self.dir + "/Makefile").expect("Error creating Makefile");
		file.write_fmt(
			format_args!("{}", template)
		).expect("Error writing Makefile");
	}
}

struct ReadMe {
	letter: String,
	date: DateTime<Local>,
	info: String,
	assn: String,
	dir: String
}

impl ReadMe {
	fn from_java_files(
		names: Vec<TemplatedFile>,
		letter: String,
		date: DateTime<Local>,
		assn: String,
		dir: String
	) -> ReadMe {
		let mut info: String = String::new();
		for n in &names {
			info += &(String::from("\n") + &n.name + ".java\t" + &n.desc);
		}
		ReadMe {
			letter: letter,
			date: date,
			info: info,
			assn: assn,
			dir: dir
		}
	}
	fn from_c_files(
		names: Vec<TemplatedFile>,
		letter: String,
		date: DateTime<Local>,
		assn: String,
		dir: String
	) -> ReadMe {
		let mut info: String = String::new();
		for n in &names {
			info += &(String::from("\n") + &n.name + ".c\t" + &n.desc);
		}
		ReadMe {
			letter: letter,
			date: date,
			info: info,
			assn: assn,
			dir: dir
		}
	}
	fn write(self) {
		let mut template = include_str!("README").to_string();
		let date = self.date.format("%Y-%m-%d").to_string();
		template = template.replace("%l", &self.letter);
		template = template.replace("%a", &self.assn);
		template = template.replace("%d", &date);
		template = template.replace("%f", &self.info);
		let mut file = File::create(self.dir + "/README").expect("Error creating README");
		file.write_fmt(
			format_args!("{}", template)
		).expect("Error writing README");
	}
}

fn create_project(name: &String) -> [String; 2] {
	let mut template = include_str!("project.geany").to_string();
	let out: String = String::from(DIREC) + &name;
	let project: String = String::from(PROJ) + &name + &".geany";
	template = template.replace("%p", &out);
	template = template.replace("%n", &name);
	let mut file = File::create(&project)
		.expect("Error creating project");
	file.write_fmt(
		format_args!("{}", template)
	).expect("Error writing project");
	fs::create_dir_all(&out).expect(
		"Output Generation: error creating root path"
	);
	Command::new("git")
		.args(&["init", &out])
		.output()
		.expect("Error initializing git repository");
	[project, out]
}

fn init_java() {
	let mut files: Vec<TemplatedFile> = Vec::new();
	let mut scl = input("Sub class: ");
	let mut len = scl.chars().count(); // .len() gets # of bytes
	scl.truncate(len - 1);
	let mut assign = input("Assignment name: ");
	len = assign.chars().count(); // .len() gets # of bytes
	assign.truncate(len - 1);
	let mut main_c = input("Main class name: ");
	len = main_c.chars().count();
	main_c.truncate(len - 1);
	let mut main_d = input("Main class description: ");
	len = main_d.chars().count();
	main_d.truncate(len - 1);
	let mut classes: Vec<String> = Vec::new();
	let mut classes_desc: Vec<String> = Vec::new();
	let mut uin: String = String::from("empty");
	let date: DateTime<Local> = Local::now();
	while uin != "" {
		uin = input("Additional class (leave blank for none): ");
		len = uin.chars().count();
		uin.truncate(len - 1);
		classes.push(uin.clone());
		if uin != "" {
			uin = input("Description: ");
			len = uin.chars().count();
			uin.truncate(len - 1);
			classes_desc.push(uin.clone());
		}
	}
	let dirs: [String; 2] = create_project(&assign);
	files.push(TemplatedFile {
		name: main_c.clone(),
		desc: main_d,
		date: date,
		letter: scl.clone(),
		dir: dirs[1].clone()
	});
	for c in 0..(classes.len()-1) {
		files.push(TemplatedFile {
			name: classes[c].clone(),
			desc: classes_desc[c].clone(),
			date: date,
			letter: scl.clone(),
			dir: dirs[1].clone()
		});
	}
	MakeFile::from_files(
		files.clone(),
		main_c.clone(),
		scl.clone(),
		date,
		assign.clone(),
		dirs[1].clone()
	).write_java();
	ReadMe::from_java_files(
		files.clone(),
		scl.clone(),
		date,
		assign.clone(),
		dirs[1].clone()
	).write();
	for f in files {
		f.write_java();
	}
	Command::new("git")
		.args(&["add", &dirs[1]])
		.output()
		.expect("Error adding files to git");
	Command::new("geany")
		.args(&[dirs[0].clone()])
		.output()
		.expect("Error opening project in geany");
}

fn init_c() {
	let mut files: Vec<TemplatedFile> = Vec::new();
	let mut scl = input("Sub class: ");
	let mut len = scl.chars().count(); // .len() gets # of bytes
	scl.truncate(len - 1);
	let mut assign = input("Assignment name: ");
	len = assign.chars().count(); // .len() gets # of bytes
	assign.truncate(len - 1);
	let mut main_c = input("Program name: ");
	len = main_c.chars().count();
	main_c.truncate(len - 1);
	let mut main_d = input("Program description: ");
	len = main_d.chars().count();
	main_d.truncate(len - 1);
	let mut libs: Vec<String> = Vec::new();
	let mut libs_desc: Vec<String> = Vec::new();
	let mut uin: String = String::from("empty");
	let date: DateTime<Local> = Local::now();
	while uin != "" {
		uin = input("Additional file (leave blank for none): ");
		len = uin.chars().count();
		uin.truncate(len - 1);
		libs.push(uin.clone());
		if uin != "" {
			uin = input("Description: ");
			len = uin.chars().count();
			uin.truncate(len - 1);
			libs_desc.push(uin.clone());
		}
	}
	let dirs: [String; 2] = create_project(&assign);
	files.push(TemplatedFile {
		name: main_c.clone(),
		desc: main_d,
		date: date,
		letter: scl.clone(),
		dir: dirs[1].clone()
	});
	for c in 0..(libs.len()-1) {
		files.push(TemplatedFile {
			name: libs[c].clone(),
			desc: libs_desc[c].clone(),
			date: date,
			letter: scl.clone(),
			dir: dirs[1].clone()
		});
	}
	MakeFile::from_files(
		files.clone(),
		main_c.clone(),
		scl.clone(),
		date,
		assign.clone(),
		dirs[1].clone()
	).write_c();
	ReadMe::from_c_files(
		files.clone(),
		scl.clone(),
		date,
		assign.clone(),
		dirs[1].clone()
	).write();
	for f in files {
		f.write_c();
	}
	Command::new("git")
		.args(&["add", &dirs[1]])
		.output()
		.expect("Error adding files to git");
	Command::new("geany")
		.args(&[dirs[0].clone()])
		.output()
		.expect("Error opening project in geany");
}

fn main() {
	if env::args().count() == 2 {
		if env::args().nth(1).unwrap() == "java" {
			init_java();
		} else if env::args().nth(1).unwrap() == "c" {
			init_c();
		}
	} else {
		println!(include_str!("help.txt"));
	}
}
