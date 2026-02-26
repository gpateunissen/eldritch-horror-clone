pub struct InvestigatorDefinition{
    pub name: String,
    pub base_health: i32,
    pub base_sanity: i32,
    pub initial_location: String,
    pub initial_items: Vec<String>,
    pub special_ability: String,
}

impl InvestigatorDefinition{
    pub fn new() -> Self{
        Self {
            name: "Pepito Grillo".to_string(),
            base_health: 5,
            base_sanity: 5,
            initial_location: "London".to_string(),
            initial_items: Vec::new(),
            special_ability: String::new(),
        }
    }
}
