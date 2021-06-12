/*The MIT License (MIT)

Copyright (c) 2021 David Ryack

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use clap::{clap_app};
use crate::tty::istty;
use crate::read_from_stdio::read_from_stdin;
use crate::vars::LICENSE_TEXT;
use std::process;
//use term::stderr;

mod tty;
mod read_from_stdio;
mod vars;

fn main() {
    let mut app = clap_app!(rcgb =>
        (version: "0.10")
        (author: "dryack <git.lamashtu@gmail.com>")
        //(license: "MIT")
        (about: "Compute GigaBytes (in Rust): A kluge that accepts numerical input and spits out the value in Gigabytes, Megabytes, or Kilobytes.")
        (@arg help: -h --help "Display help message" )
        (@arg license: --license "Display license")
        (@arg version: -V --version "Display version")
        (@arg tib: -t --tib display_order(1) +takes_value "display in TiB")
        (@arg gib: -g --gib display_order(2) +takes_value "display in GiB")
        (@arg mib: -m --mib display_order(3) +takes_value "display in MiB")
        (@arg kib: -k --kib display_order(4) +takes_value "display in KiB")
        (@arg enumerate: -e --enum +takes_value default_value("false") "enumerate results")
        (@arg suppress: -s --suppress "suppress SI postfix")
        (@arg nowarn: -W --("no-warnings") "suppress warnings when invalid numbers are submitted; the processing will continue")
        (@arg prec: -p --precision +takes_value default_value("2") possible_values(&["0","1","2","3","4","5","6","7","8","9"]) "show results with a precision of N decimal places (max: 9)")
        (@arg INPUT: +takes_value +multiple "Numbers to convert")
    );
    let matches = app.clone().get_matches();

    if matches.is_present("license") {
        println!("{}", LICENSE_TEXT);
        std::process::exit(0);
    }

    if istty() {
        if !matches.is_present("INPUT") {
            eprintln!("error: <INPUT> empty\n");
            match app.print_help() {
                Ok(_n) => {}
                Err(error) => println!("error: {}", error)
            }
        } else {
            println!("Values from input: {:?}", matches.values_of("INPUT").map(|values| values.collect::<Vec<_>>()));
        }
    } else {
        println!("Values from input: {:?}", matches.values_of("INPUT").map(|values| values.collect::<Vec<_>>()));
        println!("Values from STDIN: {:?}", read_from_stdin());
    }
    if matches.is_present("tib") {
        println!("tib found!")
    }

}
