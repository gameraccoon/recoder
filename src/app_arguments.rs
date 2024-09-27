// Copyright (C) Pavel Grebnev 2024
// Distributed under the MIT License (license terms are at http://opensource.org/licenses/MIT).

#[derive(Default, Clone)]
pub struct AppArguments {
    pub templates_path: Option<String>,
    pub definitions_path: Option<String>,
    pub results_root_path: Option<String>,
}

pub struct ArgumentsParsingResult {
    pub app_arguments: AppArguments,
    pub message: Option<String>,
    pub is_error: bool,
}

struct ArgumentDefinition {
    name: &'static str,
    syntax: &'static str,
    shorthand: Option<&'static str>,
    description: &'static str,
    number_of_args: usize,
    is_required: bool,
}

const SUPPORTED_ARGS: &[ArgumentDefinition] = &[
    ArgumentDefinition {
        name: "--help",
        syntax: "--help",
        shorthand: Some("-h"),
        description: "Show this help",
        number_of_args: 0,
        is_required: false,
    },
    ArgumentDefinition {
        name: "--version",
        syntax: "--version",
        shorthand: Some("-v"),
        description: "Show the application version",
        number_of_args: 0,
        is_required: false,
    },
    ArgumentDefinition {
        name: "--templates-path",
        syntax: "--templates-path <path>",
        shorthand: Some("-t"),
        description: "Set path to the templates directory",
        number_of_args: 1,
        is_required: true,
    },
    ArgumentDefinition {
        name: "--definitions-path",
        syntax: "--definitions-path <path>",
        shorthand: Some("-d"),
        description: "Set path to the directory with definitions",
        number_of_args: 1,
        is_required: true,
    },
    ArgumentDefinition {
        name: "--results-root-path",
        syntax: "--results-root-path <path>",
        shorthand: Some("-r"),
        description: "Set root directory for results, default is the current directory",
        number_of_args: 1,
        is_required: false,
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
    let args: Vec<String> = std::env::args().collect();

    let mut templates_path = None;
    let mut definitions_path = None;
    let mut results_root_path = None;

    let mut i: usize = 1;
    while i < args.len() {
        let arg = &args[i];

        let found_arg = if arg.starts_with("--") {
            SUPPORTED_ARGS
                .iter()
                .find(|supported_arg| supported_arg.name == arg)
        } else if arg.starts_with("-") {
            SUPPORTED_ARGS
                .iter()
                .find(|supported_arg| supported_arg.shorthand == Some(arg))
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

        if arg == "--help" || arg == "-h" {
            return ArgumentsParsingResult::message(get_help_text());
        }
        if arg == "--version" || arg == "-v" {
            return ArgumentsParsingResult::message(env!("CARGO_PKG_VERSION").to_string());
        } else if arg == "--templates-path" || arg == "-t" {
            templates_path = Some(args[i + 1].clone());
        } else if arg == "--definitions-path" || arg == "-d" {
            definitions_path = Some(args[i + 1].clone());
        } else if arg == "--results-root-path" || arg == "-r" {
            results_root_path = Some(args[i + 1].clone());
        }

        i += 1 + found_arg.number_of_args;
    }

    let mut missing_args = Vec::new();
    for supported_arg in SUPPORTED_ARGS {
        if supported_arg.is_required {
            let mut found = false;
            for arg in &args {
                if arg == supported_arg.name || arg == supported_arg.shorthand.unwrap_or("") {
                    found = true;
                    break;
                }
            }
            if !found {
                missing_args.push(supported_arg.name);
            }
        }
    }

    if missing_args.len() > 0 {
        return ArgumentsParsingResult::error(format!(
            "Missing required arguments: {}\nUse --help to see the list of supported arguments",
            missing_args.join(", ")
        ));
    }

    ArgumentsParsingResult::parsed(AppArguments {
        templates_path,
        definitions_path,
        results_root_path,
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
