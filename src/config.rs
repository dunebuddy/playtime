use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::models::GameConfig;

#[derive(Debug, Deserialize)]
struct TomlGameConfig {
    display_name: String,
    command: Vec<String>,
}

pub fn read_toml_config(path: &Path) -> Result<GameConfig, String> {
    let contents = fs::read_to_string(path)
        .map_err(|_| format!("arquivo de configuração não encontrado: {}", path.display()))?;

    let config: TomlGameConfig = toml::from_str(&contents).map_err(|_| {
        "não foi possível ler a configuração. Verifique se o arquivo contém display_name e command."
            .to_string()
    })?;

    let display_name = normalize_display_name(config.display_name)?;

    if config.command.is_empty() {
        return Err("nenhum comando foi informado para executar o jogo.".to_string());
    }

    Ok(GameConfig {
        display_name,
        command: config.command,
    })
}

pub fn normalize_display_name(display_name: String) -> Result<String, String> {
    let display_name = display_name.trim().to_string();

    if display_name.is_empty() {
        return Err("display_name não pode ser vazio.".to_string());
    }

    Ok(display_name)
}

pub fn normalize_optional_filter(filter: String) -> Option<String> {
    let filter = filter.trim().to_string();

    if filter.is_empty() {
        None
    } else {
        Some(filter)
    }
}
