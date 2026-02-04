#![doc = include_str!("../README.md")]

use std::{env::args, path::PathBuf, process::exit, str::FromStr};

/// The main struct holding the arguments and their configurations
pub struct ArgParse {
    positionals: Vec<Positional>,
    flags: Vec<Flag>,
    about: String,
    help: bool,
    parsed: bool,
}

impl ArgParse {
    /// Create a new instance of [ArgParse].
    pub fn new() -> Self {
        Self {
            positionals: vec![],
            flags: vec![],
            about: String::new(),
            help: true,
            parsed: false,
        }
    }

    /// Set whether to show help with the `--help` and `-h` flags.
    pub fn help(self, set: bool) -> Self {
        Self {
            positionals: self.positionals,
            flags: self.flags,
            about: self.about,
            help: set,
            parsed: false,
        }
    }

    /// Set a description for your command.
    pub fn about(self, about: &str) -> Self {
        Self {
            positionals: self.positionals,
            flags: self.flags,
            about: about.to_string(),
            help: self.help,
            parsed: false,
        }
    }

    /// Add a positional argument to your command.
    ///
    /// - `name`: The name of the positional argument. This will be displayed in the help menu, as
    /// well as acting as the key to retrieve the value in the positional.
    /// - `help`: The help text to display for the positional argument. This will be displayed in
    /// the help menu, if it has been enabled.
    pub fn positional(self, name: &str, help: &str) -> Self {
        let mut positionals = self.positionals;
        positionals.push(Positional {
            name: name.to_string(),
            help: help.to_string(),
            value: String::new(),
        });
        Self {
            positionals,
            flags: self.flags,
            about: self.about,
            help: self.help,
            parsed: false,
        }
    }

    /// Add a flag to your command.
    ///
    /// - `name`: The name of the flag. This will act as the key to retrieve the value in the flag.
    /// It will also be displayed as the name of the argument in the help menu, if the flag is not
    /// a switch.
    /// - `long`: The long name of the flag. This corresponds to the `--long` form of the flag, and
    /// will be displayed as such in the help menu.
    /// - `short`: The optional short name of the flag. If it is set, then it corresponds to the
    /// `-short` form of the flag, and will be displayed as such in the help menu.
    /// - `help`: The help text to display for this flag. This will be displayed in the help menu,
    /// if it has been enabled.
    /// - `switch`: Whether or not this flag is boolean. If this is `true`, then the flag will not
    /// have an argument following it.
    pub fn flag(
        self,
        name: &str,
        long: &str,
        short: Option<&str>,
        help: &str,
        switch: bool,
    ) -> Self {
        let mut flags = self.flags;
        flags.push(Flag {
            name: name.to_string(),
            long: long.to_string(),
            short: short.map(|s| s.to_string()),
            help: help.to_string(),
            value: String::new(),
            switch,
        });
        Self {
            positionals: self.positionals,
            flags,
            about: self.about,
            help: self.help,
            parsed: false,
        }
    }

    /// Get the value of a positional argument using the name set in the configuration. If this
    /// function is called before [ArgParse::parse], then it will return [None].
    pub fn get_positional<T: FromStr>(&self, name: &str) -> Option<T> {
        if !self.parsed {
            return None;
        }

        for pos in &self.positionals {
            if pos.name == name {
                return pos.value.parse().ok();
            }
        }
        None
    }

    /// Get the value of a flag using the name set in the configuration. If this function is called
    /// before [ArgParse::parse], then it will return [None].
    pub fn get_flag<T: FromStr>(&self, name: &str) -> Option<T> {
        if !self.parsed {
            return None;
        }

        for flag in &self.flags {
            if flag.name == name {
                return flag.value.parse().ok();
            }
        }
        None
    }

    /// Parse the arguments to the program into the [ArgParse] struct. This function should be
    /// called before calling [ArgParse::get_positional] or [ArgParse::get_flag].
    pub fn parse(&mut self) {
        let mut remaining_args = vec![];
        let argv: Vec<_> = args().skip(1).collect();

        if self.help && (argv.contains(&"--help".to_string()) || argv.contains(&"-h".to_string())) {
            self.display_help();
            exit(0)
        }

        let mut skip = 0;
        for (i, arg) in argv.iter().enumerate() {
            while skip > 0 {
                skip -= 1;
                continue;
            }

            if arg == "--" {
                remaining_args.append(&mut argv[i + 1..].to_vec());
                break;
            }

            if arg.starts_with("--") {
                match self.flags.iter_mut().find(|flag| flag.long == &arg[2..]) {
                    Some(f) => {
                        if f.switch {
                            f.value = String::from("true")
                        } else {
                            f.value = argv[i + 1].clone();
                            skip = 1;
                        }
                        continue;
                    }
                    None => {
                        eprintln!("Unknown flag: {arg}");
                        exit(1)
                    }
                };
            }

            if arg == "-" {
                remaining_args.push(arg.clone());
                continue;
            }

            if arg.starts_with("-") {
                match self
                    .flags
                    .iter_mut()
                    .find(|flag| flag.short == Some(arg[1..].to_string()))
                {
                    Some(f) => {
                        if f.switch {
                            f.value = String::from("true")
                        } else {
                            f.value = argv[i + 1].clone();
                            skip = 1;
                        }
                        continue;
                    }
                    None => {
                        eprintln!("Unknown flag: {arg}");
                        exit(1)
                    }
                }
            }

            remaining_args.push(arg.clone());
        }

        if remaining_args.len() < self.positionals.len() {
            eprintln!("Not enough arguments provided");
            exit(1)
        }

        for (i, arg) in remaining_args.iter().enumerate() {
            self.positionals[i].value = arg.clone()
        }

        self.parsed = true
    }

    fn display_help(&self) {
        let max_pos_len = &self
            .positionals
            .iter()
            .max_by(|a, b| a.name.len().cmp(&b.name.len()))
            .map(|p| p.help_len())
            .unwrap_or_default();
        let max_flag_len = &self
            .flags
            .iter()
            .max_by(|a, b| a.help_len().cmp(&b.help_len()))
            .map(|f| f.help_len())
            .unwrap_or_default();
        let max_len = 10.max(*max_pos_len).max(*max_flag_len); // "-h, --help"

        let prog_path = args().next().unwrap().parse::<PathBuf>().unwrap();
        let prog_name = prog_path.file_name().unwrap().to_string_lossy();

        print!("USAGE: {prog_name}");
        for pos in &self.positionals {
            print!(" <{}>", pos.name);
        }
        for flag in &self.flags {
            if flag.switch {
                print!(" [--{}]", flag.long)
            } else {
                print!(" [--{} <{}>]", flag.long, flag.name)
            }
        }

        println!();
        if &self.about != "" {
            println!("{}", &self.about);
        }

        if &self.positionals.len() > &0 {
            println!();
            println!("POSITIONALS:");
            for pos in &self.positionals {
                println!(
                    "  <{}>{}    {}",
                    pos.name,
                    " ".repeat(max_len - pos.help_len()),
                    pos.help,
                )
            }
        }

        if &self.flags.len() > &0 {
            println!();
            println!("FLAGS:");
            for flag in &self.flags {
                println!(
                    "  {}--{}{}{}    {}",
                    match &flag.short {
                        Some(s) => format!("-{}, ", s),
                        None => String::new(),
                    },
                    flag.long,
                    if flag.switch {
                        String::new()
                    } else {
                        format!(" <{}>", flag.name)
                    },
                    " ".repeat(max_len - flag.help_len()),
                    flag.help
                )
            }

            if self.help {
                println!(
                    "  -h, --help{}    Display this help and exit.",
                    " ".repeat(max_len - 10)
                )
            }
        }
    }
}

struct Positional {
    name: String,
    help: String,
    value: String,
}

impl Positional {
    fn help_len(&self) -> usize {
        self.name.len() + 2 // "<{pos.name}>"
    }
}

struct Flag {
    name: String,
    long: String,
    short: Option<String>,
    help: String,
    value: String,
    switch: bool,
}

impl Flag {
    fn help_len(&self) -> usize {
        self.long.len()
            + 2 // "--{flag.long}"
            + self.short.as_ref().map(|s| s.len() + 3).unwrap_or_default() // "-{flag.short}, "
            + if self.switch { 0 } else { self.name.len() + 3 } // " <{flag.name}>"
    }
}
