use crate::db::{create_schema, load_listed_games, open_database};
use crate::formatting::format_duration;
use crate::ids::game_id_prefix;
use crate::models::ListedGame;

pub fn list_games(wide: bool, filter: Option<&str>) -> Result<(), String> {
    let connection = open_database()?;
    create_schema(&connection)?;
    let games = load_listed_games(&connection, filter)?;

    if games.is_empty() {
        if let Some(filter) = filter {
            println!("Nenhum jogo encontrado para o filtro: {filter}");
        } else {
            println!("Nenhum jogo registrado.");
        }
        return Ok(());
    }

    if wide {
        print_wide_game_list(&games);
    } else {
        print_game_list(&games);
    }

    Ok(())
}

fn print_game_list(games: &[ListedGame]) {
    println!("{:<10}  Nome", "ID");

    for game in games {
        println!(
            "{:<10}  {}",
            game_id_prefix(&game.game_id),
            game.display_name
        );
    }
}

fn print_wide_game_list(games: &[ListedGame]) {
    println!(
        "{:<64}  {:<32}  {:>7}  {:>12}  Última sessão",
        "ID", "Nome", "Sessões", "Tempo total"
    );

    for game in games {
        println!(
            "{:<64}  {:<32}  {:>7}  {:>12}  {}",
            game.game_id,
            game.display_name,
            game.session_count,
            format_duration(game.total_seconds),
            game.last_ended_at
        );
    }
}
