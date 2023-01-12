use std::str::FromStr;

use crate::anime::{Anime, Genre, Statuses};

const FILE_PATH: &str = "D:\\Yang\\Document\\Personale\\RustProjects\\anime_list\\anime_list.csv";
const DELIMITER: u8 = b'|';

pub fn write_to_csv(anime_list: &Vec<Anime>) {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(DELIMITER)
        .from_path(FILE_PATH)
        .expect("Unable to open file");

    // header
    writer
        .write_record(&["Name", "Genre", "Status", "Season", "Episode", "Score"])
        .expect("Unable to write record");

    for anime in anime_list {
        writer
            .write_record(&[
                anime.name(),
                &format!(
                    "{},{}",
                    anime.genre().first(),
                    if anime.genre().second().is_none() {
                        ""
                    } else {
                        anime.genre().second().as_deref().unwrap()
                    }
                ),
                &anime.status().to_string(),
                &anime.season().to_string(),
                &anime.episode().to_string(),
                &anime.score().to_string(),
            ])
            .expect("Unable to write record");
    }
    writer.flush().expect("Unable to flush");
}

pub fn read_from_csv(anime_list: &mut Vec<Anime>) {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(DELIMITER)
        .from_path(FILE_PATH)
        .expect("Unable to open file");

    for result in reader.records() {
        let record = result.unwrap();

        let name = record.get(0).expect("Unable to get name").to_string();

        let genre: Vec<String> = record
            .get(1)
            .expect("Unable to get genre")
            .split(",")
            .map(|s| s.to_string())
            .collect();
        let genre = if genre.len() == 1 {
            Genre::new(genre[0].clone(), None)
        } else {
            Genre::new(genre[0].clone(), Some(genre[1].clone()))
        };

        let status: Statuses =
            Statuses::from_str(record.get(2).expect("Unable to get status")).unwrap();

        let season = record
            .get(3)
            .expect("Unable to get season")
            .parse::<i32>()
            .unwrap();

        let episode = record
            .get(4)
            .expect("Unable to get episode")
            .parse::<i32>()
            .unwrap();

        let score = record
            .get(5)
            .expect("Unable to get score")
            .parse::<i32>()
            .unwrap();

        anime_list.push(Anime::new(name, genre, status, season, episode, score));
    }
}

///
/// Testing
///
#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::anime;

    use super::*;

    #[test]
    fn test_read_write() {
        let w_al = create_anime_list();
        write_to_csv(&w_al);
        assert!(is_equal(&w_al));

        let mut r_al = vec![];
        read_from_csv(&mut r_al);
        assert!(is_equal(&r_al));
    }

    // helper function
    fn create_anime_list<'a>() -> Vec<Anime> {
        vec![
            Anime::new(
                "anime1".to_string(),
                anime::Genre::new("Action".to_string(), None),
                anime::Statuses::Watching,
                1,
                1,
                1,
            ),
            Anime::new(
                "anime2".to_string(),
                anime::Genre::new("Action".to_string(), Some("Comedy".to_string())),
                anime::Statuses::Watching,
                1,
                1,
                1,
            ),
            Anime::new(
                "anime3".to_string(),
                anime::Genre::new("Action".to_string(), Some("Comedy".to_string())),
                anime::Statuses::Watching,
                1,
                1,
                1,
            ),
        ]
    }

    // helper function
    fn is_equal(al: &Vec<Anime>) -> bool {
        let file = File::open(FILE_PATH).expect("TESTING: Unable to open file");
        let reader = BufReader::new(file);

        // saltare header
        for (i, line) in reader.lines().enumerate().skip(1) {
            let i = i - 1;
            let anime_line = format!(
                "{}|{}|{}|{}|{}|{}",
                al[i].name(),
                format!(
                    "{},{}",
                    al[i].genre().first(),
                    al[i].genre().second().as_deref().unwrap_or("")
                ),
                al[i].status(),
                al[i].season().to_string(),
                al[i].episode().to_string(),
                al[i].score().to_string()
            );
            if line.unwrap() != anime_line {
                return false;
            }
        }
        true
    }
}
