use clap::{AppSettings, Arg};
use iq::context::BasicContext;
use regex::Regex;
use std::fs;

fn main() {
    let matches = clap::command!("iq")
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("blank")
                .short('b')
                .long("blank")
                .takes_value(true)
                .value_name("dimensions")
                .help("Use a blank canvas of provided size 'HxW' (ex. '100x300')"),
        )
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Pass a file containing expressions to run"),
        )
        .arg(
            Arg::with_name("expressions")
                .short('e')
                .long("expr")
                .takes_value(true)
                .help("The expressions to evaluate"),
        )
        .arg(Arg::with_name("input_path").help("The path to the input image"))
        .arg(Arg::with_name("output_path").help("Where to write the output image"))
        .get_matches();

    let input_context = match matches.value_of("blank") {
        Some(blank_dimensions_string) => {
            if matches.value_of("input_path").is_none() {
                panic!("Either 'blank' OR an input path should be provided. Not both.")
            }

            let re = Regex::new(r"(\d+)x(\d+)").unwrap();
            let captures = re
                .captures(blank_dimensions_string)
                .expect("blank input should be HxW");
            let height = captures.get(1).unwrap().as_str();
            let width = captures.get(2).unwrap().as_str();

            BasicContext::blank(height.parse().unwrap(), width.parse().unwrap())
        }
        None => BasicContext::from_path(
            matches
                .value_of("input_path")
                .expect("Either 'blank' should be specified or an input path"),
        ),
    };

    let script_content =
        match matches.value_of("file") {
            Some(file_path) => {
                fs::read_to_string(file_path).expect("Provided file path cannot be read")
            }
            None => String::from(matches.value_of("expressions").expect(
                "Expressions must be passed as string via --expr or via the --file parameter",
            )),
        };

    let context = iq::execute(input_context, script_content);
    if let Some(output_path) = matches.value_of("output_path") {
        context.write(output_path);
    }
}
