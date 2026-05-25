use std::env;
use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use rusqlite::{Connection, params};

use crate::formatting::escape_like_pattern;
use crate::models::{GameInfo, ListedGame, ListedSession};

pub struct NewSession<'a> {
    pub id: &'a str,
    pub game_id: &'a str,
    pub session_number: i64,
    pub display_name: &'a str,
    pub command_json: &'a str,
    pub started_at: &'a str,
    pub ended_at: &'a str,
    pub duration_seconds: i64,
    pub exit_code: Option<i32>,
    pub created_at: &'a str,
}

pub fn open_database() -> Result<Connection, String> {
    let db_path = database_path()?;

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("não foi possível criar diretório de dados: {error}"))?;
    }

    Connection::open(&db_path)
        .map_err(|error| format!("não foi possível abrir o banco SQLite: {error}"))
}

pub fn create_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                game_id TEXT NOT NULL,
                session_number INTEGER NOT NULL,
                display_name TEXT NOT NULL,
                command_json TEXT NOT NULL,
                started_at TEXT NOT NULL,
                ended_at TEXT NOT NULL,
                duration_seconds INTEGER NOT NULL,
                exit_code INTEGER,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_game_id ON sessions (game_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_display_name ON sessions (display_name);
            ",
        )
        .map_err(|error| format!("não foi possível preparar o banco SQLite: {error}"))
}

pub fn insert_session(connection: &Connection, session: &NewSession<'_>) -> Result<(), String> {
    connection
        .execute(
            "
            INSERT INTO sessions (
                id,
                game_id,
                session_number,
                display_name,
                command_json,
                started_at,
                ended_at,
                duration_seconds,
                exit_code,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ",
            params![
                session.id,
                session.game_id,
                session.session_number,
                session.display_name,
                session.command_json,
                session.started_at,
                session.ended_at,
                session.duration_seconds,
                session.exit_code,
                session.created_at,
            ],
        )
        .map_err(|error| format!("não foi possível salvar a sessão: {error}"))?;

    Ok(())
}

pub fn next_session_number(connection: &Connection, game_id: &str) -> Result<i64, String> {
    connection
        .query_row(
            "SELECT COALESCE(MAX(session_number), 0) + 1 FROM sessions WHERE game_id = ?1",
            [game_id],
            |row| row.get(0),
        )
        .map_err(|error| format!("não foi possível calcular o número da sessão: {error}"))
}

pub fn load_listed_games(
    connection: &Connection,
    filter: Option<&str>,
) -> Result<Vec<ListedGame>, String> {
    if let Some(filter) = filter {
        let pattern = format!("%{}%", escape_like_pattern(&filter.to_lowercase()));
        let mut statement = connection
            .prepare(
                "
                SELECT
                    game_id,
                    (
                        SELECT display_name
                        FROM sessions latest
                        WHERE latest.game_id = sessions.game_id
                        ORDER BY ended_at DESC, session_number DESC
                        LIMIT 1
                    ) AS display_name,
                    COUNT(*) AS session_count,
                    COALESCE(SUM(duration_seconds), 0) AS total_seconds,
                    MAX(ended_at) AS last_ended_at
                FROM sessions
                WHERE game_id IN (
                    SELECT DISTINCT game_id
                    FROM sessions
                    WHERE LOWER(display_name) LIKE ?1 ESCAPE '\\'
                       OR LOWER(command_json) LIKE ?1 ESCAPE '\\'
                )
                GROUP BY game_id
                ORDER BY LOWER(display_name)
                ",
            )
            .map_err(|error| format!("não foi possível consultar jogos: {error}"))?;

        return query_listed_games(&mut statement, [pattern]);
    }

    let mut statement = connection
        .prepare(
            "
            SELECT
                game_id,
                (
                    SELECT display_name
                    FROM sessions latest
                    WHERE latest.game_id = sessions.game_id
                    ORDER BY ended_at DESC, session_number DESC
                    LIMIT 1
                ) AS display_name,
                COUNT(*) AS session_count,
                COALESCE(SUM(duration_seconds), 0) AS total_seconds,
                MAX(ended_at) AS last_ended_at
            FROM sessions
            GROUP BY game_id
            ORDER BY LOWER(display_name)
            ",
        )
        .map_err(|error| format!("não foi possível consultar jogos: {error}"))?;

    query_listed_games(&mut statement, [])
}

pub fn find_games_by_id_prefix(
    connection: &Connection,
    game_id_prefix: &str,
) -> Result<Vec<ListedGame>, String> {
    let pattern = format!("{}%", game_id_prefix.to_lowercase());
    let mut statement = connection
        .prepare(
            "
            SELECT
                game_id,
                (
                    SELECT display_name
                    FROM sessions latest
                    WHERE latest.game_id = sessions.game_id
                    ORDER BY ended_at DESC, session_number DESC
                    LIMIT 1
                ) AS display_name,
                COUNT(*) AS session_count,
                COALESCE(SUM(duration_seconds), 0) AS total_seconds,
                MAX(ended_at) AS last_ended_at
            FROM sessions
            WHERE LOWER(game_id) LIKE ?1
            GROUP BY game_id
            ORDER BY LOWER(display_name)
            ",
        )
        .map_err(|error| format!("não foi possível consultar jogos: {error}"))?;

    query_listed_games(&mut statement, [pattern])
}

pub fn load_game_info(connection: &Connection, game_id: &str) -> Result<GameInfo, String> {
    let (game_id, display_name, session_count, total_seconds, last_ended_at, command_json): (
        String,
        String,
        i64,
        i64,
        String,
        String,
    ) = connection
        .query_row(
            "
            SELECT
                game_id,
                (
                    SELECT display_name
                    FROM sessions latest
                    WHERE latest.game_id = sessions.game_id
                    ORDER BY ended_at DESC, session_number DESC
                    LIMIT 1
                ) AS display_name,
                COUNT(*) AS session_count,
                COALESCE(SUM(duration_seconds), 0) AS total_seconds,
                MAX(ended_at) AS last_ended_at,
                (
                    SELECT command_json
                    FROM sessions latest
                    WHERE latest.game_id = sessions.game_id
                    ORDER BY ended_at DESC, session_number DESC
                    LIMIT 1
                ) AS command_json
            FROM sessions
            WHERE game_id = ?1
            GROUP BY game_id
            ",
            [game_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )
        .map_err(|error| format!("não foi possível carregar informações do jogo: {error}"))?;

    let last_command = serde_json::from_str(&command_json)
        .map_err(|error| format!("não foi possível ler o último comando usado: {error}"))?;

    Ok(GameInfo {
        game_id,
        display_name,
        session_count,
        total_seconds,
        last_ended_at,
        last_command,
    })
}

pub fn load_latest_sessions(
    connection: &Connection,
    game_id: &str,
    limit: i64,
    descending: bool,
) -> Result<Vec<ListedSession>, String> {
    let query = if descending {
        "
        SELECT
            id,
            session_number,
            display_name,
            started_at,
            ended_at,
            duration_seconds,
            exit_code,
            command_json,
            created_at
        FROM sessions
        WHERE game_id = ?1
        ORDER BY ended_at DESC, session_number DESC
        LIMIT ?2
        "
    } else {
        "
        SELECT
            id,
            session_number,
            display_name,
            started_at,
            ended_at,
            duration_seconds,
            exit_code,
            command_json,
            created_at
        FROM sessions
        WHERE game_id = ?1
        ORDER BY ended_at ASC, session_number ASC
        LIMIT ?2
        "
    };

    let mut statement = connection
        .prepare(query)
        .map_err(|error| format!("não foi possível consultar sessões: {error}"))?;

    let rows = statement
        .query_map(params![game_id, limit], map_session_row)
        .map_err(|error| format!("não foi possível buscar sessões: {error}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("não foi possível ler sessões: {error}"))
}

pub fn find_sessions_by_id_prefix(
    connection: &Connection,
    session_id_prefix: &str,
) -> Result<Vec<ListedSession>, String> {
    let pattern = format!("{}%", session_id_prefix.to_lowercase());
    let mut statement = connection
        .prepare(
            "
            SELECT
                id,
                session_number,
                display_name,
                started_at,
                ended_at,
                duration_seconds,
                exit_code,
                command_json,
                created_at
            FROM sessions
            WHERE LOWER(id) LIKE ?1
            ORDER BY ended_at DESC, session_number DESC
            ",
        )
        .map_err(|error| format!("não foi possível consultar sessões: {error}"))?;

    let rows = statement
        .query_map([pattern], map_session_row)
        .map_err(|error| format!("não foi possível buscar sessões: {error}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("não foi possível ler sessões: {error}"))
}

fn query_listed_games<P>(
    statement: &mut rusqlite::Statement<'_>,
    params: P,
) -> Result<Vec<ListedGame>, String>
where
    P: rusqlite::Params,
{
    let rows = statement
        .query_map(params, |row| {
            Ok(ListedGame {
                game_id: row.get(0)?,
                display_name: row.get(1)?,
                session_count: row.get(2)?,
                total_seconds: row.get(3)?,
                last_ended_at: row.get(4)?,
            })
        })
        .map_err(|error| format!("não foi possível listar jogos: {error}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("não foi possível ler jogos: {error}"))
}

fn map_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ListedSession> {
    let command_json: String = row.get(7)?;
    let command = serde_json::from_str(&command_json).unwrap_or_default();

    Ok(ListedSession {
        id: row.get(0)?,
        session_number: row.get(1)?,
        display_name: row.get(2)?,
        started_at: row.get(3)?,
        ended_at: row.get(4)?,
        duration_seconds: row.get(5)?,
        exit_code: row.get(6)?,
        command,
        created_at: row.get(8)?,
    })
}

fn database_path() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("PLAYTIME_DB_PATH") {
        return Ok(PathBuf::from(path));
    }

    let dirs = ProjectDirs::from("", "", "playtime")
        .ok_or_else(|| "não foi possível localizar o diretório de dados do usuário.".to_string())?;

    Ok(dirs.data_dir().join("playtime.db"))
}
