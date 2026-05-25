use crate::db::{create_schema, find_sessions_by_id_prefix, open_database};
use crate::formatting::{
    format_ambiguous_session_id, format_command_for_display, format_duration, format_exit_code,
};
use crate::models::ListedSession;

pub fn show_session_info(session_id_prefix: &str) -> Result<(), String> {
    let connection = open_database()?;
    create_schema(&connection)?;

    let matches = find_sessions_by_id_prefix(&connection, session_id_prefix)?;

    match matches.len() {
        0 => Err(format!(
            "nenhuma sessão encontrada com id começando por '{session_id_prefix}'."
        )),
        1 => {
            print_session_info(&matches[0]);
            Ok(())
        }
        _ => Err(format_ambiguous_session_id(session_id_prefix, &matches)),
    }
}

fn print_session_info(session: &ListedSession) {
    println!("ID:              {}", session.id);
    println!("Jogo:            {}", session.display_name);
    println!("Sessão:          {}", session.session_number);
    println!("Início:          {}", session.started_at);
    println!("Fim:             {}", session.ended_at);
    println!(
        "Duração:         {}",
        format_duration(session.duration_seconds)
    );
    println!("Exit code:       {}", format_exit_code(session.exit_code));
    println!("Criada em:       {}", session.created_at);
    println!(
        "Comando:         {}",
        format_command_for_display(&session.command)
    );
}
