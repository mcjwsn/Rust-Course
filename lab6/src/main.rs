use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;

const DATA_FILE: &str = "data.json";
const API_QUERY: &str = "https://api.openf1.org/v1/car_data?driver_number=55&session_key=9159";

#[derive(Serialize, Deserialize)]
struct CarData {
    brake: u32,
    date: DateTime<Utc>,
    driver_number: u32,
    drs: u8,
    meeting_key: u32,
    n_gear: u8,
    rpm: u32,
    session_key: u32,
    speed: u32,
    throttle: u8,
}

fn fetch_car_data(url: &str, data_file: &str) -> Result<()> {
    let data = reqwest::blocking::get(url)?.text()?;
    let car_data: Vec<CarData> = serde_json::from_str(&data)?;
    let writer = BufWriter::new(File::create(data_file)?);
    serde_json::to_writer(writer, &car_data)?;
    Ok(())
}

fn load_car_data_from_file(data_file: &str) -> Result<Vec<CarData>> {
    let car_data: Vec<CarData> =
        serde_json::from_reader(BufReader::new(File::open(data_file)?))?;
    Ok(car_data)
}

fn check_if_data_exists(data_file: &str) -> bool {
    Path::new(data_file).exists()
}
fn average_speed(data: &[CarData], session: u32) -> Option<f64> { // avr dla sesji
    let (sum, count) = data
        .iter()
        .filter(|d| d.session_key == session)
        .map(|d| d.speed as f64)
        .fold((0.0_f64, 0_usize), |(acc_sum, acc_count), spd| {
            (acc_sum + spd, acc_count + 1)
        });

    if count > 0 {
        Some(sum / count as f64)
    } else {
        None
    }
}

fn high_speed_times( // czasy w ktorych przekroczyl prog
    data: &[CarData],
    session: u32,
    threshold: u32,
) -> Vec<DateTime<Utc>> {
    data.iter()
        .filter(|d| d.session_key == session && d.speed > threshold)
        .map(|d| d.date)
        .collect()
}

fn max_rpm(data: &[CarData], session: u32) -> Option<(u32, u8)> { // max rpm i bieg
    data.iter()
        .filter(|d| d.session_key == session)
        .max_by_key(|d| d.rpm)
        .map(|d| (d.rpm, d.n_gear))
}

fn main() -> Result<()> {
    if !check_if_data_exists(DATA_FILE) {
        fetch_car_data(API_QUERY, DATA_FILE)?;
    }

    let car_data = load_car_data_from_file(DATA_FILE)?;

    let session_id = 9159;

    match average_speed(&car_data, session_id) {
        Some(avg) => println!("Średnia prędkość w sesji {}: {:.2} km/h", session_id, avg),
        None => println!("Brak pomiarów prędkości dla sesji {}.", session_id),
    }

    let threshold = 200;
    let times = high_speed_times(&car_data, session_id, threshold);
    if !times.is_empty() {
        println!(
            "Czasy, gdy prędkość przekroczyła {} km/h w sesji {}:",
            threshold, session_id
        );
        for t in times {
            println!("  {}", t);
        }
    } else {
        println!(
            "Brak momentów, gdy prędkość przekroczyła {} km/h w sesji {}.",
            threshold, session_id
        );
    }

    match max_rpm(&car_data, session_id) {
        Some((rpm, gear)) => println!(
            "Maksymalne obroty w sesji {}: {} rpm na biegu {}",
            session_id, rpm, gear
        ),
        None => println!("Brak pomiarów rpm dla sesji {}.", session_id),
    }

    Ok(())
}
