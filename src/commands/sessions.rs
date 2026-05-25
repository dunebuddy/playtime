use crate::db::{create_schema, find_games_by_id_prefix, load_latest_sessions, open_database};
use crate::formatting::{
    format_ambiguous_game_id, format_command_for_display, format_duration, format_exit_code,
    truncate_text,
};
use crate::ids::{game_id_prefix, id_prefix};
use crate::models::ListedSession;

pub fn list_sessions(
    game_id_prefix_value: &str,
    wide: bool,
    descending: bool,
) -> Result<(), String> {
    let connection = open_database()?;
    create_schema(&connection)?;

    let matches = find_games_by_id_prefix(&connection, game_id_prefix_value)?;

    match matches.len() {
        0 => Err(format!(
            "nenhum jogo encontrado com id começando por '{game_id_prefix_value}'."
        )),
        1 => {
            let game = &matches[0];
            let sessions = load_latest_sessions(&connection, &game.game_id, i64::MAX, descending)?;

            if sessions.is_empty() {
                println!("Nenhuma sessão registrada para {}.", game.display_name);
                return Ok(());
            }

            println!(
                "Jogo: {} ({})",
                game.display_name,
                game_id_prefix(&game.game_id)
            );
            println!();

            if wide {
                print_wide_session_list(&sessions);
            } else {
                print_session_list(&sessions);
            }

            Ok(())
        }
        _ => Err(format_ambiguous_game_id(game_id_prefix_value, &matches)),
    }
}

fn print_session_list(sessions: &[ListedSession]) {
    println!(
        "{:<10}  {:>7}  {:<25}  {:<25}  {:>12}  {:>9}  Comando",
        "ID", "Sessão", "Início", "Fim", "Duração", "Exit code"
    );

    for session in sessions {
        println!(
            "{:<10}  {:>7}  {:<25}  {:<25}  {:>12}  {:>9}  {}",
            id_prefix(&session.id),
            session.session_number,
            truncate_text(&session.started_at, 25),
            truncate_text(&session.ended_at, 25),
            format_duration(session.duration_seconds),
            format_exit_code(session.exit_code),
            truncate_text(&format_command_for_display(&session.command), 48)
        );
    }
}

fn print_wide_session_list(sessions: &[ListedSession]) {
    println!(
        "{:<64}  {:>7}  {:<35}  {:<35}  {:>12}  {:>9}  Comando",
        "ID", "Sessão", "Início", "Fim", "Duração", "Exit code"
    );

    for session in sessions {
        println!(
            "{:<64}  {:>7}  {:<35}  {:<35}  {:>12}  {:>9}  {}",
            session.id,
            session.session_number,
            session.started_at,
            session.ended_at,
            format_duration(session.duration_seconds),
            format_exit_code(session.exit_code),
            format_command_for_display(&session.command)
        );
    }
}
