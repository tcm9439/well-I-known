mod wik_commands;
mod wik_commands_handlers;

use wik_commands::*;
use clap::{Command, CommandFactory, Parser};
use std::{io::{self, Write}, process::exit};

fn main_command_dispatcher(command: &Option<WikMainCommands>) {
    match command {
        Some(WikMainCommands::Init { file }) => {
            wik_commands_handlers::init_server(file);
        }
        Some(WikMainCommands::Login { user }) => {
            wik_commands_handlers::login(user);
            start_sub_command_shell();
        }
        _ => {
            println!("Unknown command.");
            let _ = WikMainArgs::command().print_help();
        }
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

/// run a interactive "shell"
fn start_sub_command_shell() {
    let sub_command: Command = WikLoggedInCommandArgs::command();
    let sub_command_name: &str = sub_command.get_name();
    loop {
        print_prompt();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let split_input = std::iter::once(sub_command_name).chain(input.split_whitespace());
        let sub_cli = WikLoggedInCommandArgs::try_parse_from(split_input);
        if let Ok(sub_cli) = sub_cli {
            if let Some(command) = sub_cli.command {
                sub_command_dispatcher(&command);
            }
            continue;
        }
        println!("Unknown or incomplete command.");
        let _ = WikLoggedInCommandArgs::command().print_help();
    }
}

fn sub_command_dispatcher(command: &WikLoggedInCommands) {
    match command {
        WikLoggedInCommands::Exit => {
            println!("Exit.");
            exit(0);
        }
        _ => {
            println!("Command skipped.");
        }
    }
}

fn main() {
    let cli = WikMainArgs::parse();
    main_command_dispatcher(&cli.command);
}