use std::fmt::{self};

/// Anime
pub struct Anime {
    name: String,

    genre: Genre,

    status: Statuses,

    season: i32,

    episode: i32,

    score: i32,
}

impl Anime {
    pub fn new(
        name: String,
        genre: Genre,
        status: Statuses,
        season: i32,
        episode: i32,
        score: i32,
    ) -> Self {
        Self {
            name,
            genre,
            status,
            season,
            episode,
            score,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn genre(&self) -> &Genre {
        &self.genre
    }

    pub fn status(&self) -> &Statuses {
        &self.status
    }

    pub fn season(&self) -> i32 {
        self.season
    }

    pub fn episode(&self) -> i32 {
        self.episode
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

/// Genre
pub struct Genre {
    // primo genere
    first: String,
    // secondo genere
    second: Option<String>,
}

impl Genre {
    pub fn new(first: String, second: Option<String>) -> Self {
        Self { first, second }
    }

    pub fn first(&self) -> &String {
        &self.first
    }

    pub fn second(&self) -> &Option<String> {
        &self.second
    }
}

/// Statuses
#[derive(Debug)]
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

impl fmt::Display for Statuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Statuses {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Watching" => Ok(Statuses::Watching),
            "Completed" => Ok(Statuses::Completed),
            "Dropped" => Ok(Statuses::Dropped),
            "OnHold" => Ok(Statuses::OnHold),
            "PlanToWatch" => Ok(Statuses::PlanToWatch),
            _ => Err(format!("{} is not a valid status", s)),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Anime, Genre, Statuses};

    #[test]
    fn test_anime() {
        let anime_list = create_anime_list();
        let anime_lines = create_anime_lines();

        anime_list.iter().enumerate().for_each(|item| {
            let (i, a) = item;
            let line = format!(
                "{}|{}|{}|{}|{}|{}|{}",
                a.name(),
                a.genre().first(),
                a.genre().second().as_deref().unwrap_or(""),
                a.status(),
                a.season(),
                a.episode(),
                a.score()
            );
            assert_eq!(line, anime_lines[i])
        });
    }

    // helper function
    pub fn create_anime_lines() -> Vec<String> {
        vec![
            "anime1|Action,|1|1|1".to_string(),
            "anime2|Action,Comedy|1|1|1".to_string(),
            "anime3|Action,|1|1|1".to_string(),
        ]
    }

    // helper function
    pub fn create_anime_list() -> Vec<Anime> {
        vec![
            Anime::new(
                "anime1".to_string(),
                Genre::new("Action".to_string(), None),
                Statuses::Watching,
                1,
                1,
                1,
            ),
            Anime::new(
                "anime2".to_string(),
                Genre::new("Action".to_string(), Some("Comedy".to_string())),
                Statuses::Watching,
                1,
                1,
                1,
            ),
            Anime::new(
                "anime3".to_string(),
                Genre::new("Action".to_string(), None),
                Statuses::Watching,
                1,
                1,
                1,
            ),
        ]
    }
}
