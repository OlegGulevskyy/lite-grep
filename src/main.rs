
fn main() {
	let context_lines_amount = 2;
	let needle = "oo";
	let haystack = "\
		Every face, every shop,
		bedroom window, public-house, and
		dark square is a picture
		feverishly turned--in search of what?
		It is the same with books.
		What do we seek
		through millions of pages?";

	let mut results: Vec<(usize, usize, usize)> = vec![];

	for (index, line) in haystack.lines().enumerate() {
		if line.contains(needle) {
			let line = index + 1;
			let lower_bound = line.saturating_sub(context_lines_amount + 1);
			let upper_bound = line + context_lines_amount;
			results.push((line, lower_bound, upper_bound));
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