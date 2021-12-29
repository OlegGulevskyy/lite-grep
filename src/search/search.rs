use regex::Regex;

pub fn get_search_results(regexp: Regex, text: &Vec<String>) -> Vec<(usize, usize, usize, &String)> {
	let mut results: Vec<(usize, usize, usize, &String)> = vec![];
	let context_lines_amount = 2;
	let lines_count = text.len();

	for (index, line) in text.iter().enumerate() {
		let found_re = regexp.find(line);

		match found_re {
			Some(_) => {
				let hit_text = line;
				let line = index + 1;
				let lower_bound = line.saturating_sub(context_lines_amount + 1);
				let upper_bound = if (line + context_lines_amount) > lines_count {
					lines_count
				} else {
					line + context_lines_amount
				};

				results.push((line, lower_bound, upper_bound, hit_text));
			},
			None => (),
		}
	}

	results
}