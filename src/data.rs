use std::str::FromStr;

use crate::anime::{self, Anime, Genre, Statuses};

const FILE_PATH: &str = "D:\\Yang\\Document\\Personale\\RustProjects\\anime_list\\anime_list.csv";
const DELIMITER: u8 = b'|';

pub fn write_to_csv(anime_list: &Vec<Anime>) {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(DELIMITER)
        .double_quote(false)
        .flexible(false)
        .from_path(FILE_PATH)
        .expect("Unable to open file");

    writer
        .write_record(&["name", "genre", "status", "season", "episode", "score"])
        .expect("Unable to write record");
    anime_list.into_iter().for_each(|anime| {
        writer
            .write_record([
                anime.name(),
                &anime.genre().to_string(),
                &anime.status().to_string(),
                &anime.progress().season().to_string(),
                &anime.progress().episode().to_string(),
                &anime.score().to_string(),
            ])
            .expect("Unable to write record");
    });
}

pub fn read_from_csv(anime_list: &mut Vec<Anime>) {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(DELIMITER)
        .double_quote(false)
        .flexible(false)
        .from_path(FILE_PATH)
        .expect("Unable to open file");

    for result in reader.records() {
        let record = result.unwrap();
        let name = record.get(0).unwrap().to_string();

        let genre: Vec<String> = record
            .get(1)
            .unwrap()
            .to_string()
            .split(",")
            .map(|s| s.to_string())
            .collect();
        let genre = if genre.len() == 1 {
            Genre::new(genre[0].clone(), None)
        } else {
            Genre::new(genre[0].clone(), Some(genre[1].clone()))
        };

        let status: Statuses = Statuses::from_str(record.get(2).unwrap()).unwrap();

        let progress = anime::Progress::new(
            record.get(3).unwrap().parse::<i32>().unwrap(),
            record.get(4).unwrap().parse::<i32>().unwrap(),
        );

        let score = record.get(5).unwrap().parse::<u8>().unwrap();
        anime_list.push(Anime::new(name, genre, status, progress, score));
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
    fn create_anime_list() -> Vec<Anime> {
        vec![
            Anime::new(
                "anime1".to_string(),
                anime::Genre::new("Action".to_string(), None),
                anime::Statuses::Watching,
                anime::Progress::new(1, 1),
                1,
            ),
            Anime::new(
                "anime2".to_string(),
                anime::Genre::new("Action".to_string(), Some("Adventure".to_string())),
                anime::Statuses::Watching,
                anime::Progress::new(1, 1),
                1,
            ),
            Anime::new(
                "anime3".to_string(),
                anime::Genre::new("Action".to_string(), Some("Adventure".to_string())),
                anime::Statuses::Watching,
                anime::Progress::new(1, 1),
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
                al[i].genre().to_string(),
                al[i].status().to_string(),
                al[i].progress().season().to_string(),
                al[i].progress().episode().to_string(),
                al[i].score().to_string()
            );
            if line.unwrap() != anime_line {
                return false;
            }
        }
        true
    }
}
