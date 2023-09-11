#[derive(Debug)]
pub enum Command {
    FileRead(String),
    RunInlineCode(String),
    Noop,
}

pub fn read_command() -> Command {
    let matches = clap_app!(haskell =>
        (version: "0.1.0")
        (author: "Brian Teague <brian.teague@gmail.com>")
        (about: "The Haskell programming language")
        (@setting ArgRequiredElseHelp)
        (@arg src: -s --src +takes_value "Path of the source file")
        (@arg run: -r --run +takes_value "Code you want to run inline")
    )
    .get_matches();

    let src_path = matches.value_of("src").map(|s| s.to_string());
    let run_string = matches.value_of("run").map(|s| s.to_string());
    match (src_path, run_string) {
        (Some(s), _) => Command::FileRead(s),
        (_, Some(s)) => Command::RunInlineCode(s),
        _ => Command::Noop,
    }
}
