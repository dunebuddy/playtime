use crate::db::{
    create_schema, find_games_by_id_prefix, load_game_info, load_latest_sessions, open_database,
};
use crate::formatting::{
    format_ambiguous_game_id, format_command_for_display, format_duration, format_exit_code,
};
use crate::models::{GameInfo, ListedSession};

pub fn show_game_info(game_id_prefix_value: &str) -> Result<(), String> {
    let connection = open_database()?;
    create_schema(&connection)?;

    let matches = find_games_by_id_prefix(&connection, game_id_prefix_value)?;

    match matches.len() {
        0 => Err(format!(
            "nenhum jogo encontrado com id começando por '{game_id_prefix_value}'."
        )),
        1 => {
            let game_id = &matches[0].game_id;
            let info = load_game_info(&connection, game_id)?;
            let sessions = load_latest_sessions(&connection, game_id, 10, true)?;

            print_game_info(&info, &sessions);
            Ok(())
        }
        _ => Err(format_ambiguous_game_id(game_id_prefix_value, &matches)),
    }
}

fn print_game_info(info: &GameInfo, sessions: &[ListedSession]) {
    println!("ID:              {}", info.game_id);
    println!("Nome:            {}", info.display_name);
    println!("Sessões:         {}", info.session_count);
    println!("Tempo total:     {}", format_duration(info.total_seconds));
    println!("Última sessão:   {}", info.last_ended_at);
    println!(
        "Último comando:  {}",
        format_command_for_display(&info.last_command)
    );
    println!();
    println!("Últimas sessões");
    println!(
        "{:>7}  {:<35}  {:<35}  {:>12}  {:>9}",
        "Sessão", "Início", "Fim", "Duração", "Exit code"
    );

    for session in sessions {
        println!(
            "{:>7}  {:<35}  {:<35}  {:>12}  {:>9}",
            session.session_number,
            session.started_at,
            session.ended_at,
            format_duration(session.duration_seconds),
            format_exit_code(session.exit_code)
        );
    }

    println!();
    println!("Mostrando no máximo as últimas 10 sessões, da mais recente para a mais antiga.");
}
