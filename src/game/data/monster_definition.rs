use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct MonsterDefinition {
    pub id: i32,
    pub name: String,

    #[serde(default)]
    pub Sanity_test: i32,

    pub Sanity_dmg: i32,

    #[serde(default)]
    pub Str_test: i32,

    pub Strength_dmg: i32,

    pub Toughness: i32,

    pub Effect_type: String,
    pub Effect: String,
}