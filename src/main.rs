use std::env;
use std::process;

use cli::resolve_app_command;
use commands::{list_games, list_sessions, run_session, show_game_info, show_session_info};
use models::AppCommand;

mod cli;
mod commands;
mod config;
mod db;
mod formatting;
mod ids;
mod models;

fn main() {
    let result = run();

    let exit_code = match result {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("Erro: {error}");
            1
        }
    };

    process::exit(exit_code);
}

fn run() -> Result<i32, String> {
    match resolve_app_command(env::args_os().skip(1).collect())? {
        AppCommand::Version => {
            println!("playtime {}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
        AppCommand::Run(config) => {
            if config.command.is_empty() {
                return Err("nenhum comando foi informado para executar o jogo.".to_string());
            }

            let status = run_session(&config)?;
            Ok(status.code().unwrap_or(1))
        }
        AppCommand::List { wide, filter } => {
            list_games(wide, filter.as_deref())?;
            Ok(0)
        }
        AppCommand::Info { game_id_prefix } => {
            show_game_info(&game_id_prefix)?;
            Ok(0)
        }
        AppCommand::Sessions {
            game_id_prefix,
            wide,
            descending,
        } => {
            list_sessions(&game_id_prefix, wide, descending)?;
            Ok(0)
        }
        AppCommand::Session { session_id_prefix } => {
            show_session_info(&session_id_prefix)?;
            Ok(0)
        }
    }
}
