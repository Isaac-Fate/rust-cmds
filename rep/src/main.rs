use clap::{command, value_parser, Arg, ArgAction};

fn main() { 
    let mut command = command!()
        .about("Echo in Rust");

    let text_arg = Arg::new("text")
        .required(true)
        .value_name("TEXT")
        .action(ArgAction::Append)
        .help("Input text");

    let is_newline_omitted_flag = Arg::new("is_newline_omitted")
        .short('n')
        .action(ArgAction::SetTrue)
        .help("Do not print the trailing newline character");

    let repeating_times_arg = Arg::new("repeating_times")
        .short('k')
        .required(false)
        .value_name("REPEATING_TIMES")
        .value_parser(value_parser!(usize))
        .default_value("1")
        .help("Number of times to repeat the input text");

    let separator_arg = Arg::new("separator")
        .short('s')
        .long("sep")
        .value_name("SEPARATOR")
        .default_value("")
        .help("Separator between repeated texts");

    command = command.args([
        text_arg,
        is_newline_omitted_flag,
        repeating_times_arg,
        separator_arg
    ]);

    let matches = command.get_matches();

    let texts: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .cloned()
        .collect();
    let text = texts.join(" ");

    let is_newline_omitted = matches.get_flag("is_newline_omitted");

    let repeating_times = matches
        .get_one::<usize>("repeating_times")
        .unwrap()
        .to_owned();

    let separator = matches
        .get_one::<String>("separator")
        .unwrap()
        .to_owned();

    let text = vec![text.as_str()]
        .repeat(repeating_times)
        .join(&separator);

    if is_newline_omitted {
        print!("{}", text);
    } else {
        println!("{}", text);
    }
}
