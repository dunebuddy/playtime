use std::ffi::OsString;
use std::path::PathBuf;

use crate::config::{normalize_display_name, normalize_optional_filter, read_toml_config};
use crate::ids::{normalize_game_id_prefix, normalize_session_id_prefix};
use crate::models::{AppCommand, GameConfig};

pub fn resolve_app_command(args: Vec<OsString>) -> Result<AppCommand, String> {
    if args.is_empty() {
        return Err(usage());
    }

    if args.len() == 1 && (args[0] == "--version" || args[0] == "-V") {
        return Ok(AppCommand::Version);
    }

    if args[0] == "list" {
        return resolve_list_command(&args[1..]);
    }

    if args[0] == "info" {
        return resolve_info_command(&args[1..]);
    }

    if args[0] == "sessions" {
        return resolve_sessions_command(&args[1..]);
    }

    if args[0] == "session" {
        return resolve_session_command(&args[1..]);
    }

    if let Some(separator_index) = args.iter().position(|arg| arg == "--") {
        return resolve_inline_config(&args, separator_index).map(AppCommand::Run);
    }

    if args.len() == 1 {
        let config_path = PathBuf::from(&args[0]);
        return read_toml_config(&config_path).map(AppCommand::Run);
    }

    Err(usage())
}

fn resolve_list_command(args: &[OsString]) -> Result<AppCommand, String> {
    let mut wide = false;
    let mut filter_parts = Vec::new();

    for arg in args {
        if arg == "-w" || arg == "--wide" {
            wide = true;
        } else {
            filter_parts.push(os_string_to_string(arg)?);
        }
    }

    let filter = normalize_optional_filter(filter_parts.join(" "));

    Ok(AppCommand::List { wide, filter })
}

fn resolve_info_command(args: &[OsString]) -> Result<AppCommand, String> {
    if args.len() != 1 {
        return Err(usage());
    }

    let game_id_prefix = os_string_to_string(&args[0])?;
    let game_id_prefix = normalize_game_id_prefix(&game_id_prefix)?;

    Ok(AppCommand::Info { game_id_prefix })
}

fn resolve_sessions_command(args: &[OsString]) -> Result<AppCommand, String> {
    let mut game_id_prefix = None;
    let mut wide = false;
    let mut descending = false;

    for arg in args {
        if arg == "-w" || arg == "--wide" {
            wide = true;
        } else if arg == "-d" || arg == "--desc" {
            descending = true;
        } else if game_id_prefix.is_none() {
            let value = os_string_to_string(arg)?;
            game_id_prefix = Some(normalize_game_id_prefix(&value)?);
        } else {
            return Err(usage());
        }
    }

    let game_id_prefix = game_id_prefix.ok_or_else(usage)?;

    Ok(AppCommand::Sessions {
        game_id_prefix,
        wide,
        descending,
    })
}

fn resolve_session_command(args: &[OsString]) -> Result<AppCommand, String> {
    if args.len() != 1 {
        return Err(usage());
    }

    let session_id_prefix = os_string_to_string(&args[0])?;
    let session_id_prefix = normalize_session_id_prefix(&session_id_prefix)?;

    Ok(AppCommand::Session { session_id_prefix })
}

fn resolve_inline_config(args: &[OsString], separator_index: usize) -> Result<GameConfig, String> {
    let display_name_parts = &args[..separator_index];
    let command_parts = &args[separator_index + 1..];

    if display_name_parts.is_empty() {
        return Err("nenhum nome bonito foi informado para o jogo.".to_string());
    }

    if command_parts.is_empty() {
        return Err("nenhum comando foi informado para executar o jogo.".to_string());
    }

    let display_name = display_name_parts
        .iter()
        .map(os_string_to_string)
        .collect::<Result<Vec<_>, _>>()?
        .join(" ");
    let display_name = normalize_display_name(display_name)?;

    let command = command_parts
        .iter()
        .map(os_string_to_string)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GameConfig {
        display_name,
        command,
    })
}

fn os_string_to_string(value: &OsString) -> Result<String, String> {
    value
        .clone()
        .into_string()
        .map_err(|_| "argumentos devem estar em UTF-8 válido.".to_string())
}

pub fn usage() -> String {
    [
        "uso:",
        "  playtime <config-file>",
        "  playtime \"<display-name>\" -- <command> [args...]",
        "  playtime list [-w|--wide] [filter...]",
        "  playtime info <game-id-prefix>",
        "  playtime sessions <game-id-prefix> [-d|--desc] [-w|--wide]",
        "  playtime session <session-id-prefix>",
        "  playtime --version",
    ]
    .join("\n")
}
