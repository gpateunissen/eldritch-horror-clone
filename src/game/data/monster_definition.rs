use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MonsterDefinition {
    pub id: i32,
    pub name: String,

    #[serde(default)]
    pub sanity_test: i32,
    pub sanity_dmg: i32,

    #[serde(default)]
    pub str_test: i32,
    pub strength_dmg: i32,
    pub toughness: i32,
    pub effect_type: String,
    pub effect: String,
}
