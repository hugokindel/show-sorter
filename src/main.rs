use clap::{Arg, App};
use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let matches = App::new("show_sorter")
        .version("1.0")
        .author("KINDEL Hugo <hugokindel.pro@protonmail.com>")
        .about("Sort TV shows folder")
        .arg(Arg::with_name("input")
            .help("Sets the input folder to sort")
            .required(true))
        .get_matches();

    let folder_path = matches.value_of("input").unwrap();

    let re = Regex::new(r"(?x)s(?P<season>\d{2})e(?P<episode>\d{2})").unwrap();

    for path in fs::read_dir(folder_path).unwrap() {
        let filepath_from = path.unwrap().path();
        let filepath_from_str = &*filepath_from.to_str().unwrap().to_lowercase();

        let captured = re.captures(filepath_from_str);

        if captured.is_some() {
            let caps = captured.unwrap();
            let season = &caps["season"];
            let episode = &caps["episode"];

            let mut filepath_to = filepath_from.clone();
            filepath_to.pop();
            filepath_to.push(format!("S{}E{}", season, episode));

            if filepath_from.extension().is_some() {
                filepath_to.set_extension(filepath_from.extension().unwrap());
            }

            fs::rename(filepath_from, filepath_to)?;
        }
    }

    Ok(())
}
