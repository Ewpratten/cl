use chrono::{DateTime, Utc};
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
            time: encode_date_time(date, time),
            grid: grid.map(Into::into),
            name: name.map(Into::into),
            notes: notes.map(Into::into),
        })
    }
}

fn encode_date_time(date: Option<&str>, time: Option<&str>) -> DateTime<Utc> {
    todo!();
}
