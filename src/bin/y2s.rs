use clap::{App, Arg};
use log::info;
use env_logger::Env;

fn process(source: &str, dest: &str) {
    info!("Reading ... {}", source);
    info!("writing ... {}", dest);
}

fn main() {
    let matches = App::new("YAML2Scriptor")
        .author("Melodypapa <melodypapa@outlook.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input YAML file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output EB scriptor XML file")
                .required(true)
                .index(2),
        )
        .about("convert yaml to EB scriptor xml.")
        .get_matches();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    //println!("{:?}", matches);

    let source = matches.value_of("INPUT").unwrap();
    let dest = matches.value_of("OUTPUT").unwrap();

    process(source, dest);
}
