use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use failure::Error;
use serde::{Deserialize, Serialize};

/// Defines a single log entry
#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// Other callsign
    pub callsign: String,

    /// QSO frequency
    pub frequency_khz: f32,

    /// QSO band
    pub band: String,

    /// QSO mode
    pub mode: String,

    /// RST sent
    pub sent_rst: Option<String>,

    /// RST received
    pub recv_rst: Option<String>,

    /// Time of QSO
    pub time: DateTime<Utc>,

    /// Other station's grid
    pub grid: Option<String>,

    /// Other Ops name
    pub name: Option<String>,

    /// Notes
    pub notes: Option<String>,

    /// Transmission power used
    pub tx_pwr: Option<f32>
}

impl LogEntry {
    pub fn new(
        callsign: &str,
        frequency: f32,
        mode: &str,
        rst_sent: Option<&str>,
        rst_recv: Option<&str>,
        date: Option<&str>,
        time: Option<&str>,
        time_is_utc: bool,
        grid: Option<&str>,
        name: Option<&str>,
        notes: Option<&str>,
        tx_pwr: Option<f32>
    ) -> Result<Self, Error> {
        Ok(Self {
            callsign: callsign.to_string(),
            frequency_khz: frequency,
            band: hambands::search::get_band_for_frequency((frequency * 1000.0) as u64)?
                .name
                .to_string(),
            mode: mode.to_string(),
            sent_rst: rst_sent.map(Into::into),
            recv_rst: rst_recv.map(Into::into),
            time: encode_date_time(date.map(Into::into), time.map(Into::into), time_is_utc),
            grid: grid.map(Into::into),
            name: name.map(Into::into),
            notes: notes.map(Into::into),
            tx_pwr
        })
    }

    pub fn new_strings(
        callsign: String,
        frequency: f32,
        mode: String,
        rst_sent: Option<String>,
        rst_recv: Option<String>,
        date: Option<String>,
        time: Option<String>,
        time_is_utc: bool,
        grid: Option<String>,
        name: Option<String>,
        notes: Option<String>,
        tx_pwr: Option<f32>
    ) -> Result<Self, Error> {
        Ok(Self {
            callsign,
            frequency_khz: frequency,
            band: hambands::search::get_band_for_frequency((frequency * 1000.0) as u64)?
                .name
                .to_string(),
            mode,
            sent_rst: rst_sent,
            recv_rst: rst_recv,
            time: encode_date_time(date, time, time_is_utc),
            grid,
            name,
            notes,
            tx_pwr
        })
    }
}

pub fn encode_date_time(
    date: Option<String>,
    time: Option<String>,
    already_utc: bool,
) -> DateTime<Utc> {
    let date = match date {
        Some(date) => {
            NaiveDate::parse_from_str(date.replace("UTC", "").as_str(), "%Y-%m-%d").unwrap()
        }
        None => Utc::today().naive_local(),
    };
    DateTime::from(match time {
        Some(time) => {
            let time = NaiveTime::parse_from_str(time.as_str(), "%H:%M").unwrap();

            match already_utc {
                true => Local.from_utc_datetime(&date.and_time(time)),
                false => Local.from_local_datetime(&date.and_time(time)).unwrap(),
            }
        }
        None => Local
            .from_local_datetime(&date.and_time(Local::now().time()))
            .unwrap(),
    })
}
