use chrono::{Duration, Local};
use rand::seq::SliceRandom;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let today = Local::now();

    let two_months_ago = today - Duration::days(60);

    let mut all_dates = Vec::new();
    let mut date = two_months_ago;
    while date <= today {
        all_dates.push(date);
        date = date + Duration::days(1);
    }

    let mut rng = rand::thread_rng();
    let random_days: Vec<_> = all_dates.choose_multiple(&mut rng, 20).cloned().collect();

    let formatted_dates: Vec<String> = random_days
        .into_iter()
        .map(|day| day.format("%Y-%m-%d").to_string())
        .collect();

    let date_pattern = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    let content_dir = Path::new("content/post");
    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path();
        let mut content = String::new();

        File::open(file_path)?.read_to_string(&mut content)?;

        let mut updated_content = content.clone();
        let matches: Vec<_> = date_pattern.find_iter(&content).collect();

        if !matches.is_empty() {
            for mat in matches {
                if let Some(new_date) = formatted_dates.choose(&mut rng) {
                    println!(
                        "Replacing date {} with {} in {:?}",
                        &mat.as_str(),
                        new_date,
                        file_path
                    );
                    updated_content = updated_content.replacen(mat.as_str(), new_date, 1);
                }
            }

            let mut file = File::create(file_path)?;
            file.write_all(updated_content.as_bytes())?;
        }
    }

    Ok(())
}
