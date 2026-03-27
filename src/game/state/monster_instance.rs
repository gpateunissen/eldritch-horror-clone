#[derive(Debug)]
pub struct MonsterInstance {
    pub name: String,
    pub current_toughness: i32,
    pub sanity_dmg: i32,
    pub strength_dmg: i32,
    pub sanity_test: i32,
    pub strength_test: i32,
}
