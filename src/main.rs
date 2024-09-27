// Copyright (C) Pavel Grebnev 2024
// Distributed under the MIT License (license terms are at http://opensource.org/licenses/MIT).

mod app_arguments;

fn main() {
    let app_arguments = app_arguments::get_app_arguments();
    if let Some(message) = app_arguments.message {
        println!("{}", message);
        std::process::exit(if app_arguments.is_error { 1 } else { 0 });
    }
}
