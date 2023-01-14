use std::str::FromStr;

use crate::data::CsvFormat;

use self::{genre::Genre, statuses::Statuses};

pub mod genre;
pub mod statuses;

/// Anime
#[derive(Debug)]
pub struct Anime {
    // nome di anime
    name: String,

    // genere
    genre: Genre,

    // stato
    status: Statuses,

    // stagione
    season: i32,

    // episodio
    episode: i32,

    // voto
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

impl CsvFormat for Anime {
    fn to_csv_line(&self) -> Vec<String> {
        vec![
            self.name().to_string(),
            format!(
                "{},{}",
                self.genre().first(),
                self.genre().second().as_deref().unwrap_or("")
            ),
            self.status().to_string(),
            self.season().to_string(),
            self.episode().to_string(),
            self.score().to_string(),
        ]
    }

    fn from_csv_line(record: Vec<&str>) -> Self {
        let name = record[0].to_string();

        let genre: Vec<String> = record[1].split(",").map(|s| s.to_string()).collect();
        let genre = if genre.len() == 1 {
            Genre::new(genre[0].clone(), None)
        } else {
            Genre::new(genre[0].clone(), Some(genre[1].clone()))
        };

        let status = Statuses::from_str(record[2]).unwrap();
        let season = record[3].parse().unwrap();
        let episode = record[4].parse().unwrap();
        let score = record[5].parse().unwrap();

        Self::new(name, genre, status, season, episode, score)
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
            let anime_line = crate_anime_line_format(a);
            assert_eq!(anime_line, anime_lines[i])
        });
    }

    // helper function
    pub fn create_anime_lines() -> Vec<String> {
        vec![
            "anime1|Action,|Watching|1|1|1".to_string(),
            "anime2|Action,Comedy|Watching|1|1|1".to_string(),
            "anime3|Action,|Watching|1|1|1".to_string(),
        ]
    }

    // helper function
    pub fn crate_anime_line_format(a: &Anime) -> String {
        format!(
            "{}|{},{}|{}|{}|{}|{}",
            a.name(),
            a.genre().first(),
            a.genre().second().as_deref().unwrap_or(""),
            a.status().to_string(),
            a.season().to_string(),
            a.episode().to_string(),
            a.score().to_string()
        )
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
