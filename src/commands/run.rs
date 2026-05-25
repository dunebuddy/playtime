use std::process::{Command, ExitStatus};

use chrono::Local;

use crate::db::{NewSession, create_schema, insert_session, next_session_number, open_database};
use crate::ids::{game_id, session_id};
use crate::models::GameConfig;

pub fn run_session(config: &GameConfig) -> Result<ExitStatus, String> {
    let started_at = Local::now();
    let status = run_game_command(&config.command)?;
    let ended_at = Local::now();

    let connection = open_database()?;
    create_schema(&connection)?;

    let game_id = game_id(&config.display_name);
    let session_number = next_session_number(&connection, &game_id)?;
    let command_json = serde_json::to_string(&config.command)
        .map_err(|error| format!("não foi possível serializar o comando: {error}"))?;
    let duration_seconds = (ended_at - started_at).num_seconds().max(0);
    let started_at = started_at.to_rfc3339();
    let ended_at = ended_at.to_rfc3339();
    let created_at = Local::now().to_rfc3339();
    let id = session_id(
        &game_id,
        session_number,
        &started_at,
        &ended_at,
        &command_json,
    );

    let session = NewSession {
        id: &id,
        game_id: &game_id,
        session_number,
        display_name: &config.display_name,
        command_json: &command_json,
        started_at: &started_at,
        ended_at: &ended_at,
        duration_seconds,
        exit_code: status.code(),
        created_at: &created_at,
    };

    insert_session(&connection, &session)?;

    Ok(status)
}

fn run_game_command(command: &[String]) -> Result<ExitStatus, String> {
    Command::new(&command[0])
        .args(&command[1..])
        .status()
        .map_err(|error| format!("não foi possível iniciar o jogo: {error}"))
}
