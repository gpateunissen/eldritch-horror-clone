use crate::game::data::investigator_definition::InvestigatorDefinition;

#[derive(Debug)]
pub struct InvestigatorInstance {
    pub player_name: String,

    pub definition_id: i32,

    pub current_health: i32,
    pub current_sanity: i32,

    pub location_id: i32,

    pub items: Vec<i32>,

    pub stat_modifiers: StatModifiers,
}

#[derive(Debug)]
pub struct StatModifiers {
    pub knowledge: i32,
    pub influence: i32,
    pub observation: i32,
    pub strength: i32,
    pub willpower: i32,
}