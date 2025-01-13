// ufo_states.rs
#[derive(Debug, Clone)]
pub enum UFOState {
    Flying,
    Landed,
    Hovering,
}

impl UFOState {
    pub fn transition_to_flying(self) -> Result<Self, &'static str> {
        match self {
            UFOState::Landed | UFOState::Hovering => Ok(UFOState::Flying),
            UFOState::Flying => Err("Already in Flying state"),
        }
    }

    pub fn transition_to_landed(self) -> Result<Self, &'static str> {
        match self {
            UFOState::Flying | UFOState::Hovering => Ok(UFOState::Landed),
            UFOState::Landed => Err("Already in Landed state"),
        }
    }

    pub fn transition_to_hovering(self) -> Result<Self, &'static str> {
        match self {
            UFOState::Flying | UFOState::Landed => Ok(UFOState::Hovering),
            UFOState::Hovering => Err("Already in Hovering state"),
        }
    }
}
