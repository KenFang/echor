use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echo")
        .author("Ken Fang, kenfang@deva9.com")
        .version("1.0.2")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .allow_invalid_utf8(true)
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print the newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
