use sha2::{Digest, Sha256};

pub fn game_id(display_name: &str) -> String {
    let normalized_name = display_name.trim().to_lowercase();
    let hash = Sha256::digest(normalized_name.as_bytes());

    hash.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn session_id(
    game_id: &str,
    session_number: i64,
    started_at: &str,
    ended_at: &str,
    command_json: &str,
) -> String {
    let input = format!("{game_id}:{session_number}:{started_at}:{ended_at}:{command_json}");
    let hash = Sha256::digest(input.as_bytes());

    hash.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn game_id_prefix(game_id: &str) -> &str {
    id_prefix(game_id)
}

pub fn id_prefix(id: &str) -> &str {
    let end = id.len().min(10);

    &id[..end]
}

pub fn normalize_game_id_prefix(game_id_prefix: &str) -> Result<String, String> {
    let game_id_prefix = game_id_prefix.trim();

    if game_id_prefix.is_empty() {
        return Err("id do jogo não pode ser vazio.".to_string());
    }

    if !game_id_prefix.chars().all(|char| char.is_ascii_hexdigit()) {
        return Err("id do jogo deve conter apenas caracteres hexadecimais.".to_string());
    }

    Ok(game_id_prefix.to_lowercase())
}

pub fn normalize_session_id_prefix(session_id_prefix: &str) -> Result<String, String> {
    let session_id_prefix = session_id_prefix.trim();

    if session_id_prefix.is_empty() {
        return Err("id da sessão não pode ser vazio.".to_string());
    }

    if !session_id_prefix
        .chars()
        .all(|char| char.is_ascii_hexdigit())
    {
        return Err("id da sessão deve conter apenas caracteres hexadecimais.".to_string());
    }

    Ok(session_id_prefix.to_lowercase())
}
