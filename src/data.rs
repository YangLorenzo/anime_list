use crate::anime::Anime;

/// CsvFormat
pub trait CsvFormat {
    fn to_csv_line(&self) -> Vec<String>;
    fn from_csv_line(csv: Vec<&str>) -> Self;
}

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
            .write_record(anime.to_csv_line())
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
        let record = record.iter().map(|s| s).collect::<Vec<&str>>();

        anime_list.push(Anime::from_csv_line(record));
    }
}

/// Testing
#[cfg(test)]
mod tests {
    use super::*;
    use crate::anime;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    #[test]
    fn test_write() {
        let anime_list = anime::tests::create_anime_list();
        write_to_csv(&anime_list);
        let anime_lines = anime::tests::create_anime_lines();

        let file = File::open(FILE_PATH).expect("Unable to open file");
        let reader = BufReader::new(file);

        // skip header
        reader.lines().skip(1).enumerate().for_each(|item| {
            let (i, anime) = item;
            assert_eq!(anime_lines[i], anime.unwrap());
        });
    }

    #[test]
    fn test_read() {
        let anime_lines = anime::tests::create_anime_lines();

        let mut anime_list: Vec<Anime> = Vec::new();
        read_from_csv(&mut anime_list);

        anime_list.iter().enumerate().for_each(|item| {
            let (i, a) = item;
            let anime = anime::tests::crate_anime_line_format(a);
            assert_eq!(anime, anime_lines[i]);
        })
    }
}
