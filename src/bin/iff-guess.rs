//! Interchange File Format structure guesser.

use iff_parser::prelude::*;

fn main() {
    // parse args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    // parse file using guesser
    let mut parser = IFFParser::file(&args[1]).unwrap();
    parser.parse(IFFParser::heuristic).unwrap();

    println!("{:#08} EOF{}", parser.position().unwrap(), " ".repeat(40));
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn aiff() {
        let mut cmd = Command::cargo_bin("iff-guess").unwrap();
        cmd.arg("data/noise.aiff");
        cmd.assert().success();
    }

    #[test]
    fn ftxt() {
        let mut cmd = Command::cargo_bin("iff-guess").unwrap();
        cmd.arg("data/amiga.ftxt");
        cmd.assert().success();
    }

    #[test]
    fn ilbm() {
        let mut cmd = Command::cargo_bin("iff-guess").unwrap();
        cmd.arg("data/scott.ilbm");
        cmd.assert().success();
    }
}
