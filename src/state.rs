use std::collections::VecDeque;

use discord_presence::{
    models::{Activity, ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps},
    Event,
};

#[derive(Debug, Default, Clone)]
pub struct Events(VecDeque<Event>);

impl Events {
    /// Return and remove the earliest event in the queue
    ///
    /// If it returns `None`, then the queue is empty
    pub fn respond(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    /// Return and remove the most recent event in the queue
    ///
    /// If it returns `None`, then the queue is empty
    pub fn respond_latest(&mut self) -> Option<Event> {
        self.0.pop_back()
    }

    /// Check if the given event has fired, removing it from the queue if so
    pub fn respond_specific(&mut self, event: Event) -> bool {
        if let Some(index) = self.0.iter().position(|e| *e == event) {
            self.0.remove(index);
            true
        } else {
            false
        }
    }

    /// Ignore all events, removing them
    pub fn clear(&mut self) {
        self.0.clear()
    }

    fn add(&mut self, event: discord_presence::Event) {
        self.0.push_back(event);
    }

    fn remove(&mut self, event: discord_presence::Event) {
        self.0.retain(|e| e != &event);
    }
}

/// The state that holds the Discord activity
#[derive(Debug, Default, Clone)]
pub struct ActivityState {
    /// The player's current party status
    pub state: Option<String>,
    /// What the player is currently doing
    pub details: Option<String>,
    /// Whether this activity is an instanced context, like a match
    pub instance: Option<bool>,
    /// Helps create elapsed/remaining timestamps on a player's profile
    pub timestamps: Option<ActivityTimestamps>,
    /// Assets to display on the player's profile
    pub assets: Option<ActivityAssets>,
    /// Information about the player's party. NOTE: Joining a party is not currently supported
    pub party: Option<ActivityParty>,
    /// Secret passwords for joining and spectating the player's game. NOTE: Joining a party is not currently supported
    pub secrets: Option<ActivitySecrets>,
    /// The events that have fired for this activity
    pub events: Events,
}

impl From<ActivityState> for Activity {
    /// Converts the ActivityState into a Discord Presence
    fn from(state: ActivityState) -> Self {
        Activity {
            state: state.state,
            assets: state.assets,
            details: state.details,
            party: state.party,
            secrets: state.secrets,
            timestamps: state.timestamps,
            instance: state.instance,
        }
    }
}
