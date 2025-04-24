use rambda::vm::Vm;

pub struct State {
    pub displayed_history: Vec<String>,
    pub history: Vec<String>,
    pub exit: bool,
    pub vm: Vm,
}

impl State {
    pub fn new() -> Self {
        Self {
            displayed_history: Vec::new(),
            history: Vec::new(),
            exit: false,
            vm: Vm::new(),
        }
    }

    pub fn exec(&mut self, input: String) {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let command = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();
        if args.len() >= 2 && args[0] == ":=" {
            let body = args[1..].join(" ");
            self.history.push(format!("{} := {}", command, body));
            return;
        }
        match command {
            "clear" => {
                self.history.clear();
            }
            "exit" => {
                self.exit = true;
                self.history.push("Exiting...".to_string());
            }
            "eval" => {
                let expr = args.join(" ");
                if let Err(e) = self.vm.parse_expr(&expr) {
                    self.history.push(format!("Error: {}", e));
                }
                self.history.push(expr);
                match self.vm.eval() {
                    Ok(steps) => {
                        for (red_type, expr) in steps {
                            self.history.push(format!("  ->{} {}", red_type, expr));
                        }
                    }
                    Err(err) => {
                        self.history.push(format!("Error: {}", err));
                    }
                }
                self.history.push(String::new());
            }
            _ => {
                self.history.push(format!("Unknown command: {}", command));
            }
        }
    }
}
