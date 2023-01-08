use std::{fmt::Display, str::FromStr};

/// Anime
pub struct Anime {
    // nome di anime
    name: String,

    // genere, il secondo valore è opzionale
    genre: (String, Option<String>),

    // stato dell'anime
    status: Statuses,

    // progresso dell'anime
    progress: Progress,

    // punteggio dell'anime
    score: u8,
}

impl Anime {
    pub fn new(
        name: String,
        genre: (String, Option<String>),
        status: Statuses,
        progress: Progress,
        score: u8,
    ) -> Self {
        Self {
            name,
            genre,
            status,
            progress,
            score,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn genre(&self) -> &(String, Option<String>) {
        &self.genre
    }

    pub fn status(&self) -> &Statuses {
        &self.status
    }

    pub fn progress(&self) -> &Progress {
        &self.progress
    }

    pub fn score(&self) -> &u8 {
        &self.score
    }
}

// --------------------------------------------
/// Progress
pub struct Progress {
    // numero di episodi visti
    episode: i32,
    // stagione
    season: i32,
}

impl Progress {
    pub fn new(episode: i32, season: i32) -> Self {
        if episode < 0 || season < 1 {
            panic!("Invalid episode or season number");
        } else {
            Self { episode, season }
        }
    }

    pub fn episode(&self) -> &i32 {
        &self.episode
    }

    pub fn season(&self) -> &i32 {
        &self.season
    }
}

// --------------------------------------------
/// Statuses
pub enum Statuses {
    // in corso
    Watching,

    // completato
    Completed,

    // abbandonato
    Dropped,

    // in pausa
    OnHold,

    // da vedere
    PlanToWatch,
}

/// Display for Statuses
/// enum to string
impl Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statuses::Watching => write!(f, "Watching"),
            Statuses::Completed => write!(f, "Completed"),
            Statuses::Dropped => write!(f, "Dropped"),
            Statuses::OnHold => write!(f, "OnHold"),
            Statuses::PlanToWatch => write!(f, "PlanToWatch"),
        }
    }
}

/// FromStr for Statuses
/// string to enum
impl FromStr for Statuses {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Watching" => Ok(Statuses::Watching),
            "Completed" => Ok(Statuses::Completed),
            "Dropped" => Ok(Statuses::Dropped),
            "OnHold" => Ok(Statuses::OnHold),
            "PlanToWatch" => Ok(Statuses::PlanToWatch),
            _ => Err(()),
        }
    }
}
