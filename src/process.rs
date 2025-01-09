#[derive(Clone, Copy)]
pub struct ProcessingArgs {
    pub show_nonprinting: bool,
    pub squeeze_blank: bool,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub show_tabs: bool,
}

pub fn process(file_string: String, args: ProcessingArgs) -> Vec<String> {
    let mut lines: Vec<String> = file_string.lines().map(|s| s.to_owned()).collect();

    if args.show_nonprinting {
        lines = show_nonprinting(lines);
    }

    if args.squeeze_blank {
        lines = suppress_empty_lines(lines);
    }

    if args.number || args.number_nonblank {
        lines = enumerate_lines(lines, args.number_nonblank);
    }

    if args.show_ends {
        lines = lines.into_iter().map(|l| l + "$").collect();
    }

    if args.show_tabs {
        lines = lines.into_iter().map(|l| l.replace("\t", "^I")).collect();
    }

    lines
}

fn show_nonprinting(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().map(|line| line.chars()
        .map(|c| match c {
            // Printable ASCII (32-126) or whitespace characters (like '\n', '\t') remain unchanged
            ' '..='~' | '\n' | '\t' => c.to_string(),
            // Control characters (0-31, excluding '\t' and '\n') are shown as ^ notation
            '\x00'..='\x1F' => format!("^{}", (c as u8 + 64) as char),
            // Delete character (ASCII 127)
            '\x7F' => "^?".to_string(),
            // Extended ASCII and Unicode are shown as M- notation
            c if c as u32 > 127 => format!("M-{}", ((c as u32 - 128) as u8 as char)),
            // Fallback (unexpected)
            _ => format!("?"),
        })
        .collect::<String>()
    ).collect()
}

fn enumerate_lines(lines: Vec<String>, number_nonblank: bool) -> Vec<String> {
    let mut count = 0; 

    lines
        .into_iter()
        .map(|line| {
            if !line.is_empty() || !number_nonblank {
                count += 1;
                format!("{count}: {line}")
            } else {
                line
            }
        })
        .collect()
}

fn suppress_empty_lines(lines: Vec<String>) -> Vec<String> {
    lines
    .into_iter()
    .fold(Vec::new(), |mut acc, line| {
        if !(line.is_empty() && acc.last().map(|l| l.is_empty()).unwrap_or(false)) {
            acc.push(line);
        }
        acc
    })
}