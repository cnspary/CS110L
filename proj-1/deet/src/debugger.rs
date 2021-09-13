use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::{Inferior, Status};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Breakpoint {
    pub addr: usize,
    pub orig_byte: u8,
}

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: HashMap<usize, Breakpoint>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }

            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    match &mut self.inferior {
                        Some(inf) => match inf.kill() {
                            Ok(_) => {
                                println!("Killing running inferior (pid {})", inf.pid());
                            }
                            Err(_) => (),
                        },
                        None => {}
                    }

                    if let Some(inferior) =
                        Inferior::new(&self.target, &args, &mut self.breakpoints)
                    {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // TODO (milestone 1): make the inferior run
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        let inferior = self.inferior.as_mut().unwrap();
                        let res = inferior.con(&self.breakpoints);
                        match res {
                            Ok(status) => match status {
                                Status::Stopped(sign, _rip) => {
                                    println!("Child stopped (signal {})", sign);
                                    match self.debug_data.get_line_from_addr(_rip) {
                                        Some(line_info) => {
                                            println!(
                                                "Stopped at {}: {}",
                                                line_info.file, line_info.number
                                            );
                                        }
                                        None => {
                                            println!("Stopped at %rip register: {:#x}", _rip);
                                        }
                                    };
                                }
                                Status::Exited(code) => {
                                    println!("The program exited with code {}", code);
                                }
                                Status::Signaled(sign) => {
                                    println!("The program stop by sign {}", sign);
                                }
                            },
                            Err(err) => {
                                println!("{}", err);
                            }
                        }
                    } else {
                        println!("Error starting subprocess");
                    }
                }

                DebuggerCommand::Continue => {
                    match self.inferior {
                        Some(_) => {
                            let inferior = self.inferior.as_mut().unwrap();
                            let res = inferior.con(&self.breakpoints);
                            match res {
                                Ok(status) => match status {
                                    Status::Stopped(sign, _rip) => {
                                        println!("Child stopped (signal {})", sign);
                                        match self.debug_data.get_line_from_addr(_rip) {
                                            Some(line_info) => {
                                                println!(
                                                    "Stopped at {}: {}",
                                                    line_info.file, line_info.number
                                                );
                                            }
                                            None => {
                                                println!("Stopped at %rip register: {:#x}", _rip);
                                            }
                                        };
                                    }
                                    Status::Exited(code) => {
                                        println!("The program exited with code {}", code);
                                    }
                                    Status::Signaled(sign) => {
                                        println!("The program stop by sign {}", sign);
                                    }
                                },
                                Err(err) => {
                                    println!("{}", err);
                                }
                            }
                        }
                        None => {
                            println!("No processes are running!");
                        }
                    };
                }

                DebuggerCommand::Quit => {
                    match &mut self.inferior {
                        Some(inf) => {
                            // println!("Killing running inferior (pid {})", inf.pid());

                            match inf.kill() {
                                Ok(_) => {
                                    println!("Killing running inferior (pid {})", inf.pid());
                                }
                                Err(_) => (),
                            }
                        }
                        None => {}
                    }
                    return;
                }

                DebuggerCommand::Break(arg) => {
                    let breakpoint_addr: Option<usize>;
                    if arg.starts_with("*") {
                        if let Some(addr) = self.parse_address(&arg[..]).ok() {
                            breakpoint_addr = Some(addr);
                        } else {
                            breakpoint_addr = None;
                            println!("Invalid address");
                        }
                    } else if let Some(line) = usize::from_str_radix(&arg, 10).ok() {
                        if let Some(addr) = self.debug_data.get_addr_for_line(None, line) {
                            breakpoint_addr = Some(addr);
                        } else {
                            breakpoint_addr = None;
                            println!("Invalid line number");
                        }
                    } else if let Some(addr) = self.debug_data.get_addr_for_function(None, &arg) {
                        breakpoint_addr = Some(addr);
                    } else {
                        breakpoint_addr = None;
                        println!("Usage: b|break|breakpoint *address|line|func");
                    }

                    match breakpoint_addr {
                        Some(addr) => {
                            if self.breakpoints.contains_key(&addr) {
                                println!("Already set breakpoint at {:#x}", addr);
                            } else {
                                if self.inferior.is_some() {
                                    if let Some(orig_instr) =
                                        self.inferior.as_mut().unwrap().write_byte(addr, 0xcc).ok()
                                    {
                                        println!(
                                            "Set breakpoint {} at {:#x}",
                                            self.breakpoints.len(),
                                            addr
                                        );
                                        self.breakpoints.insert(
                                            addr,
                                            Breakpoint {
                                                addr: addr,
                                                orig_byte: orig_instr,
                                            },
                                        );
                                    } else {
                                        println!("Invalid breakpoint address at {:#x}", addr);
                                    }
                                } else {
                                    println!(
                                        "Set breakpoint {} at {:#x}",
                                        self.breakpoints.len(),
                                        addr
                                    );
                                    self.breakpoints.insert(
                                        addr,
                                        Breakpoint {
                                            addr: addr,
                                            orig_byte: 0,
                                        },
                                    );
                                }
                            }
                        }
                        None => (),
                    }
                }

                DebuggerCommand::Backtrace => match &self.inferior {
                    Some(inf) => {
                        inf.print_backtrace(&self.debug_data).ok();
                    }
                    None => {
                        println!("No processes are running!");
                    }
                },
            }
        }
    }

    fn parse_address(&self, addr: &str) -> Result<usize, String> {
        let addr_without_0x = if addr.to_lowercase().starts_with("*0x") {
            &addr[3..]
        } else {
            return Err("address illegal".to_string());
        };

        match usize::from_str_radix(addr_without_0x, 16) {
            Ok(v) => Ok(v),
            Err(_) => Err(format!("Can not parse address 0x{}", addr_without_0x)),
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
