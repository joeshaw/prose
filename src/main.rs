use std::io::{self, BufRead, StdinLock};

mod reformat;
use reformat::{FormatOpts, reformat};
use clap::{Arg, App};

fn print_reformatted(opts: &FormatOpts, buf: &[String]) {
    if !buf.is_empty() {
        println!("{}", reformat(&opts, &buf.join("\n")));
    }
}

fn process_paragraphs(io: &mut StdinLock, opts: FormatOpts) -> io::Result<()> {
    let mut buf = vec![];
    for line in io.lines() {
        let l = line?;
        if l == "" {
            print_reformatted(&opts, &buf);
            println!();
            buf = vec![];
        } else {
            buf.push(l);
        }
    }
    print_reformatted(&opts, &buf);
    Ok(())
}

fn matches_to_format_opts(matches: clap::ArgMatches) -> FormatOpts {
    let width: usize = matches.value_of("width").unwrap().parse().expect("Choose a valid number");
    let last_line = matches.is_present("last line");
    let reduce_jaggedness = matches.is_present("better fit");

    FormatOpts { max_length: width, last_line, reduce_jaggedness }
}

fn main() {
    let matches = App::new("prose")
        .version("0.1")
        .about("Reformats prose to specified width")
        .arg(Arg::with_name("width")
             .short("w")
             .long("width")
             .value_name("WIDTH")
             .default_value("72")
             .help("Sets the maximum width for a line")
             .takes_value(true))
        .arg(Arg::with_name("last line")
             .short("l")
             .long("last-line")
             .help("Treat last line of a paragraph like the rest")
             .takes_value(false))
        .arg(Arg::with_name("better fit")
             .short("f")
             .long("use-better-fit")
             .help("Be more aggressive in reducing jagged line endings, even if it means a narrower width")
             .takes_value(false))
        .get_matches();

    let opts = matches_to_format_opts(matches);
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if let Err(err) = process_paragraphs(&mut handle, opts) {
        eprintln!("{}", err)
    }
}
