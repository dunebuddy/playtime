#[derive(Debug)]
pub enum AppCommand {
    Version,
    Run(GameConfig),
    List {
        wide: bool,
        filter: Option<String>,
    },
    Info {
        game_id_prefix: String,
    },
    Sessions {
        game_id_prefix: String,
        wide: bool,
        descending: bool,
    },
    Session {
        session_id_prefix: String,
    },
}

#[derive(Debug)]
pub struct GameConfig {
    pub display_name: String,
    pub command: Vec<String>,
}

#[derive(Debug)]
pub struct ListedGame {
    pub game_id: String,
    pub display_name: String,
    pub session_count: i64,
    pub total_seconds: i64,
    pub last_ended_at: String,
}

#[derive(Debug)]
pub struct GameInfo {
    pub game_id: String,
    pub display_name: String,
    pub session_count: i64,
    pub total_seconds: i64,
    pub last_ended_at: String,
    pub last_command: Vec<String>,
}

#[derive(Debug)]
pub struct ListedSession {
    pub id: String,
    pub session_number: i64,
    pub display_name: String,
    pub started_at: String,
    pub ended_at: String,
    pub duration_seconds: i64,
    pub exit_code: Option<i32>,
    pub command: Vec<String>,
    pub created_at: String,
}
