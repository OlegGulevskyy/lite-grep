use regex::Regex;

fn main() {

	let re = Regex::new("seek").unwrap();


	let context_lines_amount = 2;
	let haystack = "\
		Every face, every shop,
		bedroom window, public-house, and
		dark square is a picture
		feverishly turned--in search of what?
		It is the same with books.
		What do we seek
		through millions of pages?";

	let mut results: Vec<(usize, usize, usize)> = vec![];
	let lines_count = haystack.lines().count();

	for (index, line) in haystack.lines().enumerate() {
		let found_re = re.find(line);

		match found_re {
			Some(_) => {
				let line = index + 1;
				let lower_bound = line.saturating_sub(context_lines_amount + 1);
				let upper_bound = if (line + context_lines_amount) > lines_count {
					lines_count
				} else {
					line + context_lines_amount
				};
				
				results.push((line, lower_bound, upper_bound));
			},
			None => (),
		}
	}

	if results.is_empty() {
		return;
	}

	let lines: Vec<(usize, &str)> = haystack.lines().enumerate().map(|(index, item)|(index, item)).collect();

	for (_, (_, lower_bound, upper_bound)) in results.iter().enumerate() {
		let sliced = &lines[*lower_bound..*upper_bound];

		for (line_index, line_text) in sliced {
			println!("Line {}: {}", line_index + 1, line_text.trim());
		}
	}
}