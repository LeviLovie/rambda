use anyhow::Result;
use rambda::vm::Vm;
use serde::{Deserialize, Serialize};

const CONFIG_FOLDER: &str = "rambda";
const CONFIG_FILE: &str = "config.yaml";
const DEFAULT_CONFIG: &[u8] = include_bytes!("../default_config.yaml");

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub use_utf8: bool,
    pub use_color: bool,
    pub merge_args: bool,
    pub magic: Option<char>,
    pub print_effect: bool,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_folder = dirs::config_dir()
            .ok_or(anyhow::anyhow!("Failed to get config directory"))?
            .join(CONFIG_FOLDER);
        if !config_folder.exists() {
            std::fs::create_dir_all(&config_folder)?;
        }
        let config_file = config_folder.join(CONFIG_FILE);
        if !config_file.exists() {
            std::fs::write(config_file.clone(), DEFAULT_CONFIG)?;
        }
        let config: Config = serde_yaml::from_reader(std::fs::File::open(config_file)?)?;
        Ok(config)
    }
}

pub struct State {
    pub displayed_history: Vec<String>,
    pub history: Vec<String>,
    pub exit: bool,
    pub vm: Vm,
    pub config: Config,
}

impl State {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;

        Ok(Self {
            displayed_history: Vec::new(),
            history: Vec::new(),
            exit: false,
            vm: Vm::new(),
            config,
        })
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
                            println!("Red: {}", red_type.fmt_with_config(true, true));
                            self.history.push(format!(
                                "  {} {}",
                                red_type
                                    .fmt_with_config(self.config.use_color, self.config.use_utf8),
                                expr.fmt_with_config(
                                    self.config.use_color,
                                    self.config.use_utf8,
                                    self.config.merge_args
                                ),
                            ));
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
