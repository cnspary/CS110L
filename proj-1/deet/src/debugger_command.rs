pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Continue,
    Break(String),
    Backtrace,
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            },
            "c" | "continue" => {
                Some(DebuggerCommand::Continue)
            },
            "b" | "break" => {
                Some(DebuggerCommand::Break(tokens[1].to_string()))
            },
            "bt" | "backtrace" => {
                Some(DebuggerCommand::Backtrace)
            },
            // Default case:
            _ => None,
        }
    }
}
