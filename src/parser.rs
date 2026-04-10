use std::fs::OpenOptions;
use std::io::{self, Write};

use reqsh::get_request;

pub struct ShellCommand {
    pub name: String,
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub append: bool,
}

enum RedirectType {
    Stdout,
    Stderr,
}

pub enum ShellSignal {
    Continue,
    Exit,
}

impl ShellCommand {
    pub fn build(command_line: &str) -> Result<ShellCommand, &str> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut chars = command_line.chars().peekable();
        let mut pending_redirect: Option<RedirectType> = None;
        let mut stdout: Option<String> = None;
        let mut stderr: Option<String> = None;
        let mut append = false;

        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    while let Some(c) = chars.next() {
                        if c == '\'' {
                            break;
                        } else {
                            current.push(c);
                        }
                    }
                }

                '"' => {
                    while let Some(c) = chars.next() {
                        if c == '"' {
                            break;
                        }
                        if c == '\\' {
                            if let Some(next) = chars.next() {
                                match next {
                                    '"' | '\\' | '$' | '\n' => current.push(next),
                                    _ => {
                                        current.push('\\');
                                        current.push(next);
                                    }
                                }
                            }
                        } else {
                            current.push(c);
                        }
                    }
                }

                '\\' => {
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }

                ' ' | '\t' => {
                    if !current.is_empty() {
                        if let Some(rtype) = pending_redirect.take() {
                            match rtype {
                                RedirectType::Stdout => stdout = Some(current.clone()),
                                RedirectType::Stderr => stderr = Some(current.clone()),
                            }
                        } else {
                            tokens.push(current.clone());
                        }
                        current.clear();
                    }
                }

                '1' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pending_redirect = Some(RedirectType::Stdout);
                        if let Some('>') = chars.peek() {
                            chars.next();
                            append = true;
                        }
                    } else {
                        current.push('1');
                    }
                }

                '2' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pending_redirect = Some(RedirectType::Stderr);
                        if let Some('>') = chars.peek() {
                            chars.next();
                            append = true;
                        }
                    } else {
                        current.push('2');
                    }
                }

                '>' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pending_redirect = Some(RedirectType::Stdout);
                        append = true;
                    } else {
                        pending_redirect = Some(RedirectType::Stdout);
                    }
                }

                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            if let Some(rtype) = pending_redirect.take() {
                match rtype {
                    RedirectType::Stdout => stdout = Some(current.clone()),
                    RedirectType::Stderr => stderr = Some(current.clone()),
                }
            } else {
                tokens.push(current.clone());
            }
            current.clear();
        }

        let (name, args) = tokens.split_first().unwrap();

        Ok(ShellCommand {
            name: name.clone(),
            args: args.to_vec(),
            stdout,
            stderr,
            append,
        })
    }

    pub fn execute(&self) -> ShellSignal {
        let mut stdout: Box<dyn Write> = match &self.stdout {
            Some(file) => Box::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(self.append)
                    .open(file)
                    .unwrap(),
            ),
            None => Box::new(io::stdout()),
        };

        let mut stderr: Box<dyn Write> = match &self.stderr {
            Some(file) => Box::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(self.append)
                    .open(file)
                    .unwrap(),
            ),
            None => Box::new(io::stderr()),
        };

        match self.name.as_str() {
            "exit" => return ShellSignal::Exit,
            "help" => {
                writeln!(stdout, "Available commands:").unwrap();
                writeln!(stdout, "  help - Show this help message").unwrap();
                writeln!(stdout, "  exit - Exit the shell").unwrap();
            }
            "GET" => {
                let response = get_request(&self.args[0]);
                writeln!(stdout, "{response}").unwrap();
            }
            _ => {
                writeln!(stdout, "Executing command: {}", self.name).unwrap();
                if !self.args.is_empty() {
                    writeln!(stderr, "With arguments: {:?}", self.args).unwrap();
                }
            }
        }

        ShellSignal::Continue
    }
}
