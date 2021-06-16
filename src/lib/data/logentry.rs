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
        grid: Option<&str>,
        name: Option<&str>,
        notes: Option<&str>,
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
            time: encode_date_time(date.map(Into::into), time.map(Into::into)),
            grid: grid.map(Into::into),
            name: name.map(Into::into),
            notes: notes.map(Into::into),
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
        grid: Option<String>,
        name: Option<String>,
        notes: Option<String>,
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
            time: encode_date_time(date, time),
            grid,
            name,
            notes,
        })
    }
}

pub fn encode_date_time(date: Option<String>, time: Option<String>) -> DateTime<Utc> {
    let date = match date {
        Some(date) => {
            NaiveDate::parse_from_str(date.replace("UTC", "").as_str(), "%Y-%m-%d").unwrap()
        }
        None => Utc::today().naive_local(),
    };
    DateTime::from(
        Local
            .from_local_datetime(&match time {
                Some(time) => {
                    let time = NaiveTime::parse_from_str(time.as_str(), "%H:%M").unwrap();
                    date.and_time(time)
                }
                None => date.and_time(Local::now().time()),
            })
            .unwrap(),
    )
}
