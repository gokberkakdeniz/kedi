extern crate clap;
extern crate regex;

use clap::{App, Arg};
use std::{fs, io};
use io::Read;
use regex::Regex;

fn main() {
    let matches = App::new("kedi")
        .version("1.0")
        .about("cat clone written in rust")
        .author("tncga")
        .after_help("With no FILE, or when FILE is -, read standard input.")
        .help_message("display this help and exit")
        .version_message("output version information and exit")
        .bin_name("kedi")
        .usage("kedi [FLAG]... [FILE]...")
        .arg(Arg::with_name("FILE")
            .help("input file")
            .multiple(true)
            .takes_value(true)
            .default_value("-")
            .index(1))
        .arg(Arg::with_name("show-all")
            .short("A")
            .long("show-all")
            .help("equivalent to -vET"))
        .arg(Arg::with_name("number-nonblank")
            .short("b")
            .long("number-nonblank")
            .help("number nonempty output lines, overrides -n"))            
        .arg(Arg::with_name("show-nonprinting-and-end")
            .short("e")
            .help("equivalent to -vE"))            
        .arg(Arg::with_name("show-ends")
            .short("E")
            .long("show-ends")
            .help("display $ at end of each line"))
        .arg(Arg::with_name("line-number")
            .short("n")
            .long("number")
            .help("number all output lines"))
        .arg(Arg::with_name("squeeze-blank")
            .short("s")
            .long("squeeze-blank")
            .help("suppress repeated empty output lines"))
        .arg(Arg::with_name("show-nonprinting-and-tab")
            .short("t")
            .help("equivalent to -vT"))                                
        .arg(Arg::with_name("show-tabs")
            .short("T")
            .long("show-tabs")
            .help("display TAB characters as ^I"))
        .arg(Arg::with_name("u")
            .short("u")
            .help("(ignored)"))
        .arg(Arg::with_name("show-nonprinting")
            .short("v")
            .long("show-nonprinting")
            .help("use ^ and M- notation, except for LFD and TAB"))
        .get_matches();
    
    let show_all = matches.is_present("show-all");
    let show_nonprinting_and_end = matches.is_present("show-nonprinting-and-end");
    let show_nonprinting_and_tab = matches.is_present("show-nonprinting-and-tab");

    let files = matches.values_of("FILE").unwrap();

    let number_nonblank = matches.is_present("number-nonblank");
    let show_line_numbers = matches.is_present("line-number") || number_nonblank;
    let show_nonprinting = matches.is_present("show-nonprinting") || show_all || show_nonprinting_and_end;
    let show_ends = matches.is_present("show-ends") || show_all || show_nonprinting_and_end;
    let show_tabs = matches.is_present("show-tabs") || show_all || show_nonprinting_and_tab;
    let squeeze_blank = matches.is_present("squeeze-blank");

    for file in files {
        let mut content: String = 
            match fs::metadata(file) {
                Ok(_) => fs::read_to_string(file).unwrap(),
                Err(_) if file == "-" => {
                    let mut buffer = String::new();
                    match io::stdin().read_to_string(&mut buffer) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("{} [file_name = stdin (standart input)]", e);
                            continue;
                        }
                    };
                    buffer
                },
                Err(msg) => {
                    eprintln!("{} [file_name = {}]", msg, file);
                    continue;
                }
            }
            .bytes()
            .map(|b| {
                match b {
                    9 => {
                        if show_tabs {
                            "^I".to_string()
                        } else {
                            (b as char).to_string()
                        }                        
                    }
                    10 => {
                        if show_ends {
                            "$\n".to_string()
                        } else {
                            (b as char).to_string()
                        }
                    },
                    13 => "".to_string(),
                    0..=31 => {
                        if show_nonprinting {
                            let mut ctrl = String::from("^");
                            ctrl.push((b+64) as char);
                            ctrl
                        } else {
                            (b as char).to_string()
                        }

                    },
                    32..=126 => {
                        (b as char).to_string()
                    },
                    _ => {
                        if show_nonprinting {                        
                            let mut meta = String::from("M-");
                            if b >= 160 {
                                if b < 255 {
                                    meta.push((b - 128) as char);
                                } else {
                                    meta.push_str(&"^?");
                                }
                            } else {
                                meta.push('^');
                                meta.push((b - 64) as char);
                            }
                            meta
                        } else {
                            (b as char).to_string()
                        }
                    }
                }
            })
            .collect();
        
        if squeeze_blank {
            let re = Regex::new(r"\n{3,}").unwrap();
            content = re.replace_all(&content, "\n\n").to_string();
        }

        if show_line_numbers {
            let mut line_number = 0;
            content = content.lines().map(|line| {
                if !line.is_empty() || !number_nonblank {
                    line_number += 1;
                    format!("{:6} {}\n", &line_number, line)
                } else {
                    "\n".to_string()
                }
            })
            .collect();
            content.pop();
        }

        print!("{}", content);
    }
}
