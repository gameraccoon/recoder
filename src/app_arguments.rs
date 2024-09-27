// Copyright (C) Pavel Grebnev 2024
// Distributed under the MIT License (license terms are at http://opensource.org/licenses/MIT).

#[derive(Default, Clone)]
pub struct AppArguments {
    pub path_to_config: Option<String>,
}

pub struct ArgumentsParsingResult {
    pub app_arguments: AppArguments,
    pub message: Option<String>,
    pub is_error: bool,
}

struct ArgumentDefinition {
    name: &'static str,
    syntax: &'static str,
    description: &'static str,
    number_of_args: usize,
}

const SUPPORTED_ARGS: &[ArgumentDefinition] = &[
    ArgumentDefinition {
        name: "--help",
        syntax: "--help",
        description: "Show this help",
        number_of_args: 0,
    },
    ArgumentDefinition {
        name: "--version",
        syntax: "--version",
        description: "Show the application version",
        number_of_args: 0,
    },
    ArgumentDefinition {
        name: "--config",
        syntax: "--config <path>",
        description: "Set custom path to the config file",
        number_of_args: 1,
    },
];

impl ArgumentsParsingResult {
    fn parsed(app_arguments: AppArguments) -> Self {
        Self {
            app_arguments,
            message: None,
            is_error: false,
        }
    }

    fn error(message: String) -> Self {
        Self {
            app_arguments: AppArguments::default(),
            message: Some(message),
            is_error: true,
        }
    }

    fn message(message: String) -> Self {
        Self {
            app_arguments: AppArguments::default(),
            message: Some(message),
            is_error: false,
        }
    }
}

pub fn get_app_arguments() -> ArgumentsParsingResult {
    let mut custom_config_path = None;

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return ArgumentsParsingResult::error(std::format!(
            "No arguments provided\n{}",
            get_help_text(),
        ));
    }

    let mut i: usize = 1;
    while i < args.len() {
        let arg = &args[i];

        let found_arg = if arg.starts_with("--") {
            SUPPORTED_ARGS
                .iter()
                .find(|supported_arg| supported_arg.name == arg)
        } else {
            None
        };

        let Some(found_arg) = found_arg else {
            return ArgumentsParsingResult::error(format!(
                "Unsupported argument: {}\nUse --help to see the list of supported arguments",
                arg
            ));
        };

        if found_arg.number_of_args > 0 && i + found_arg.number_of_args >= args.len() {
            return ArgumentsParsingResult::error(format!(
                "Not enough arguments for {}\nUse --help to see the list of supported arguments",
                arg
            ));
        };

        if arg == "--help" {
            return ArgumentsParsingResult::message(get_help_text());
        }
        if arg == "--version" {
            return ArgumentsParsingResult::message(env!("CARGO_PKG_VERSION").to_string());
        }

        if arg == "--config-path" {
            if i + 1 < args.len() {
                custom_config_path = Some(args[i + 1].clone());
            }
        }

        i += 1 + found_arg.number_of_args;
    }

    ArgumentsParsingResult::parsed(AppArguments {
        path_to_config: custom_config_path,
    })
}

fn get_help_text() -> String {
    let mut help_text = "Supported arguments:\n".to_string();
    let mut max_syntax_len = 0;
    for arg in SUPPORTED_ARGS {
        max_syntax_len = max_syntax_len.max(arg.syntax.len());
    }
    for arg in SUPPORTED_ARGS {
        help_text.push_str(&arg.syntax);
        for _ in 0..max_syntax_len - arg.syntax.len() + 1 {
            help_text.push(' ');
        }
        help_text.push_str(arg.description);
        help_text.push_str("\n");
    }
    help_text.push_str("\n");
    help_text.push_str("Example: recoder --config-path C:\\config.json --logs-path C:\\logs --work-path C:\\work --env VAR1 value1 --env VAR2 value2");

    help_text
}
