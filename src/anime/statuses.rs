use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Statuses {
    Watching,

    Completed,

    Dropped,

    OnHold,

    PlanToWatch,
}

impl fmt::Display for Statuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Statuses {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Watching" => Ok(Statuses::Watching),
            "Completed" => Ok(Statuses::Completed),
            "Dropped" => Ok(Statuses::Dropped),
            "OnHold" => Ok(Statuses::OnHold),
            "PlanToWatch" => Ok(Statuses::PlanToWatch),
            _ => Err(format!("{} is not a valid status", s)),
        }
    }
}
