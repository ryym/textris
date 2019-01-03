use failure::{Fail, Fallible};
use getopts::Options;
use crate::inputs::KeyConverter;

pub struct Config {
    key: KeyConverter,
}

impl Config {
    pub fn key(&self) -> KeyConverter {
        self.key
    }
}

pub enum CliParsed {
    Help(String),
    Run(Config),
}

pub fn parse_args(args: &[String]) -> Fallible<CliParsed> {
    let _program = &args[0];
    let args = &args[1..];

    let mut opts = Options::new();
    let m = define_opts(&mut opts)
        .parse(args)
        .map_err(|e| e.context("failed to parse arguments"))?;

    if m.opt_present("h") {
        let usage = opts.usage("");
        return Ok(CliParsed::Help(usage));
    }

    let key = if let Some(key) = m.opt_str("key") {
        match key.as_str() {
            "vim" => KeyConverter::Vim,
            "normal" => KeyConverter::Normal,
            key => return Err(format_err!("invalid key type: {}", key)),
        }
    } else {
        KeyConverter::Normal
    };

    Ok(CliParsed::Run(Config { key }))
}

fn define_opts(opts: &mut Options) -> &mut Options {
    opts.optflag("h", "help", "print this help message");
    opts.optopt(
        "k",
        "key",
        "choose key mode (default is normal)",
        "vim | normal",
    );
    opts
}
