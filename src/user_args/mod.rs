use clap::{Arg, App};

pub struct UserArguments {
	pub search_pattern: String,
	pub file_path: String,
}

fn get_required_args() -> Vec<Arg<'static, 'static>> {
	let mut args: Vec<Arg> = vec![];

	let pattern_arg = Arg::with_name("pattern")
		.help("Pattern is what we will use to find stuff for you in text.")
		.takes_value(true)
		.required(true);
	
	let file_path = Arg::with_name("file_path")
		.help("Path to the file where stuff needs to be found")
		.takes_value(true)
		.required(true);

	args.push(pattern_arg);
	args.push(file_path);
	args
}

pub fn get_user_arguments() -> UserArguments {
	let required_args = get_required_args();
	let args = App::new("VeryLiteGrep")
		.version("1.0")
		.about("Searches for stuff. There are plenty of good stuff, this is just for learning purposes")
		.args(&required_args)
		.get_matches();

	let pattern = args.value_of("pattern").unwrap();
	let file_path = args.value_of("file_path").unwrap();

	UserArguments { 
		search_pattern: pattern.to_string(),
		file_path: file_path.to_string(),
	}
}