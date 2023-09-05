use std::{time::{SystemTime, UNIX_EPOCH}, fmt::{Debug, Display}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Ok,
    Err(String),
    Unknown
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StatusMap {
    pub controller :Status,
    pub lock :Status,
    pub rfid :Status,
    pub led :Status,
    pub speaker :Status,
}

const STATUS_MAP_OK :StatusMap = StatusMap {
    controller: Status::Ok,
    lock: Status::Ok,
    rfid: Status::Ok,
    led: Status::Ok,
    speaker: Status::Ok,
};

const STATUS_MAP_UNKNOWN :StatusMap = StatusMap {
    controller: Status::Unknown,
    lock: Status::Unknown,
    rfid: Status::Unknown,
    led: Status::Unknown,
    speaker: Status::Unknown,
};

#[derive(Debug)]
pub struct HeartbeatParseError;

impl Display for HeartbeatParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid heartbeat string format.")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Heartbeat {
    pub status :StatusMap,
    pub code :Option<String>,

    pub timestamp :u64
}

impl Heartbeat {
    pub fn new() -> Self {
        Self {
            status: StatusMap {
                controller: Status::Ok,
                lock: Status::Ok,
                rfid: Status::Ok,
                led: Status::Ok,
                speaker: Status::Ok,
            },
            code: None,

            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }

    pub fn from_heartbeat(h :String) -> Result<Self, HeartbeatParseError> {
        let mut frags = h.split(';');

        let Some(card_id) = frags.next() else {
            return Err(HeartbeatParseError);
        };
        let Some(_health) = frags.next() else {
            return Err(HeartbeatParseError);
        };

        Ok(Self {
            status: StatusMap {
                ..STATUS_MAP_OK
            },
            code: match card_id {
                "0" => None,
                other => {
                    if other.len() != 8 {
                        return Err(HeartbeatParseError);
                    } else { Some(String::from(other)) }
                }
            },

            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        })
    }

    pub fn set_connection_broken(self) -> Self {
        Self {
            status: StatusMap {
                controller: Status::Err("Serial connection broken".to_string()),
                ..STATUS_MAP_UNKNOWN
            },
            ..self
        }
    }

    pub fn set_connection_timeout(self) -> Self {
        Self {
            status: StatusMap {
                controller: Status::Err("Serial connection timeout".to_string()),
                ..STATUS_MAP_UNKNOWN
            },
            ..self
        }
    }

    pub fn set_invalid_heartbeat(self) -> Self {
        Self {
            status: StatusMap {
                controller: Status::Err("Invalid heartbeat received".to_string()),
                ..STATUS_MAP_UNKNOWN
            },
            ..self
        }
    }

    pub fn all_ok(&self) -> bool {
        self.status.controller == Status::Ok &&
        self.status.lock == Status::Ok &&
        self.status.rfid == Status::Ok &&
        self.status.led == Status::Ok &&
        self.status.speaker == Status::Ok
    }
}