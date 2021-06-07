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

use clap::clap_app;
mod vars;

fn max_precision(v: &str) -> Result<(), String> {
    if v.parse::<u8>().unwrap() > vars::MAX_PRECISION {
        Err(String::from("maximum precision supported is 9"))
    } else {
        return Ok(())
    }
}

fn main() {
   let matches = clap_app!(rcgb =>
        (version: "0.02")
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
        (@arg enum: -e --enum +takes_value default_value("false") "enumerate results")
        (@arg suppress: -s --suppress "suppress SI postfix")
        (@arg nowarn: -W --("no-warnings") "suppress warnings when invalid numbers are submitted; the processing will continue")
        (@arg prec: -p --precision +takes_value default_value("2") validator(max_precision) "show results with a precision on N decimal places (max: 9)")
        (@arg INPUT: +takes_value +required +multiple "Numbers to convert")
    ).get_matches();

    if matches.is_present("tib") {
        println!("tib found!")
    }
    println!("Value from input: {:?}", matches.values_of("INPUT").map(|values| values.collect::<Vec<_>>()));
}
