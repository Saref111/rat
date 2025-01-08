pub fn enumerate_lines(lines: Vec<String>, number_nonblank: bool) -> Vec<String> {
    let mut new_vec = Vec::new();
    let mut count = 0;
    for line in  lines {
        if line.is_empty() {
            new_vec.push(if number_nonblank { line } else {
                count.to_string() + ": " + &line
            });
        } else {
            new_vec.push(count.to_string() + ": " + &line);
        }

        count += 1;
    }

    new_vec
}

pub fn suppress_empty_lines(lines: Vec<String>) -> Vec<String> {
    lines.iter().enumerate().fold(vec![], |mut acc, (i, l)| {
        if i == 0 {
            acc.push(l.to_owned());
            return acc;
        }

        match &lines.get(i - 1) {
            Some(prev_line) => if l.is_empty() && prev_line.is_empty() {} else {
                acc.push(l.to_owned());
            },
            None => {
                acc.push(l.to_owned());
            }
        }

        acc
    })
}