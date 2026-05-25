use crate::models::{ListedGame, ListedSession};

pub fn format_duration(total_seconds: i64) -> String {
    let total_seconds = total_seconds.max(0);
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours}h {minutes:02}m {seconds:02}s")
}

pub fn format_exit_code(exit_code: Option<i32>) -> String {
    exit_code
        .map(|code| code.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub fn format_command_for_display(command: &[String]) -> String {
    command
        .iter()
        .map(|part| {
            if part.contains(char::is_whitespace) {
                format!("{part:?}")
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn truncate_text(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    if max_chars <= 1 {
        return "…".to_string();
    }

    let mut result = value.chars().take(max_chars - 1).collect::<String>();
    result.push('…');
    result
}

pub fn escape_like_pattern(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

pub fn format_ambiguous_game_id(game_id_prefix: &str, matches: &[ListedGame]) -> String {
    let mut message = format!(
        "id de jogo ambíguo: '{game_id_prefix}' corresponde a {} jogos.\nUse mais caracteres do id.\n\nJogos encontrados:",
        matches.len()
    );

    for game in matches {
        message.push_str(&format!("\n{}  {}", game.game_id, game.display_name));
    }

    message
}

pub fn format_ambiguous_session_id(session_id_prefix: &str, matches: &[ListedSession]) -> String {
    let mut message = format!(
        "id de sessão ambíguo: '{session_id_prefix}' corresponde a {} sessões.\nUse mais caracteres do id.\n\nSessões encontradas:",
        matches.len()
    );

    for session in matches {
        message.push_str(&format!(
            "\n{}  {}  sessão {}  {}",
            session.id, session.display_name, session.session_number, session.ended_at
        ));
    }

    message
}
