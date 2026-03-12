use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InvestigatorDefinition {
    pub id: i32,
    pub name: String,
    pub traits: Vec<i32>,
    pub max_health: i32,
    pub max_sanity: i32,
    pub starting_location_id: i32,
    pub death_event_id: i32,
    pub insanity_event_id: i32,
    pub starting_item_ids: Vec<i32>,
    pub stats: Stats,
    pub fluff: Fluff,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub knowledge: i32,
    pub influence: i32,
    pub observation: i32,
    pub strength: i32,
    pub willpower: i32,
}

#[derive(Debug, Deserialize)]
pub struct Fluff {
    pub profession: String,
    pub backstory: String,
    pub catchphrase: String,
}
