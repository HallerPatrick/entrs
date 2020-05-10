use clap::Clap;
use std::env;
use std::io::{self, Read};
use std::process::Command;

use failure::{Error, Fail, ResultExt};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

#[derive(Debug, Fail)]
enum EntrError {
    #[fail(display = "No files or dirs to watch")]
    NoFilesToWatch,
}

#[derive(Clap, Debug)]
#[clap(
    version = "0.1.0",
    about = "Run arbitrary commands when files change powered by Rust!",
    author = "Patrick Haller <patrickhaller40@googlemail.com>"
)]
#[clap(setting = clap::AppSettings::ArgRequiredElseHelp)]
struct Entr {
    #[clap(short, about = "Clear screen before executing utility")]
    clear: bool,

    #[clap(short, about = "Execute utility first after files have changed")]
    postpone: bool,

    #[clap(short, about = "Watch for file changes recursively")]
    recursive: bool,

    #[clap(
        short,
        about = "Evaluate the first argument using the interpreter specified by the SHELL environment variable"
    )]
    use_shell: bool,

    utility: Vec<String>,
}

impl Entr {
    pub fn run(mut self) -> Result<(), Error> {
        self.utility = if !self.use_shell {
            self.utility
        } else {
            let mut shell = Entr::get_shell_cmd();
            shell.append(&mut self.utility);
            shell
        };

        if self.utility.is_empty() {}

        // Read stdin
        //
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .with_context(|_| "Failed to read files to watch".to_string())?;

        // Collect all files passed from stdin
        let files: Vec<&str> = buf.trim().split('\n').filter(|s| !s.is_empty()).collect();

        if files.is_empty() {
            return Err(EntrError::NoFilesToWatch.into());
        }

        let recursive_mode = if self.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher: RecommendedWatcher =
            Watcher::new_immediate(move |res| tx.send(res).unwrap())?;

        for &f in &files {
            watcher
                .watch(f, recursive_mode)
                .with_context(|_| format!("Failed to watchs {}", f))?;
        }

        // Running first iteration manually
        if !self.postpone {
            self.run_utility()?;
        }

        loop {
            match rx.recv() {
                Ok(Ok(e)) => {
                    if let notify::EventKind::Modify(_) = e.kind {
                        self.run_utility()?
                    }
                }
                Ok(Err(e)) => Err(e).with_context(|_| "Could not determine event".to_string())?,
                Err(e) => Err(e).with_context(|_| "Error watching files".to_string())?,
            }
        }
    }

    /// Get the sytem's shell command string
    fn get_shell_cmd() -> Vec<String> {
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        vec![shell, "-c".to_string()]
    }

    /// Clear the terminal screen
    fn clear_term_screen(&self) -> Result<(), Error> {
        Command::new("clear").status()?;
        Ok(())
    }

    ///
    /// Run the provided utility and clear screen before hand
    /// if provided through argument flag
    ///
    fn run_utility(&self) -> Result<(), Error> {
        if self.clear {
            self.clear_term_screen()
                .with_context(|_| "Failed to clear terminal screen".to_string())?;
        }

        Command::new(&self.utility[0])
            .args(&self.utility[1..])
            .spawn()
            .with_context(|_| format!("{} Failed to run the provided utility", &self.utility[0]))?;

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let entr = Entr::parse();
    entr.run()?;
    Ok(())
}
