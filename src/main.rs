use std::path::Path;
use clap::{Command, Arg};
use termion::terminal_size;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("IMAGE")
            .help("Image to display")
            .required(true))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .help("Maximum output width  (defaults to terminal width)")
            .takes_value(true)
            .display_order(10))
        .arg(Arg::new("height")
            .short('h')
            .long("height")
            .help("Maximum output height (defaults to terminal height)")
            .takes_value(true)
            .display_order(20))
        .arg(Arg::new("ratio")
            .short('r')
            .long("ratio")
            .help("Font width/height ratio")
            .default_value("0.5")
            .takes_value(true)
            .display_order(30))
        .arg(Arg::new("block")
            .short('b')
            .long("block")
            .help("Block size")
            .takes_value(true)
            .default_value("4")
            .possible_values(&["1", "2", "4"])
            .display_order(40))
        .get_matches();

    match do_main(matches) {
        Ok(image) => print!("{}", image),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(-1);
        }
    }
}

fn do_main(matches: clap::ArgMatches) -> Result<String, &'static str> {
    let image = matches.value_of("IMAGE").unwrap();
    let image = image::open(&Path::new(image))
        .map_err(|_| "Can't open image")?;

    let (term_w, term_h) = terminal_size().unwrap_or((80, 24));

    let w = match matches.value_of("width") {
        Some(x) => x.parse()
            .map_err(|_| "Can't parse width")?,
        _ => term_w as u32,
    };

    let h = match matches.value_of("height") {
        Some(x) => x.parse()
            .map_err(|_| "Can't parse height")?,
        _ => term_h as u32 - 1,
    };

    let font = matches.value_of("ratio").unwrap().parse()
        .map_err(|_| "Can't parse ratio")?;

    let block = matches.value_of("block").unwrap().parse().unwrap();

    let spec = timv::Spec::new(w, h)
        .font(font)
        .block(block);

    let image = timv::pixelize(&image, spec).render();
    Ok(image)
}

