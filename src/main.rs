mod game;

use rand::Rng;
use serde_json;
use std::fs;

use crate::game::data::monster_definition::MonsterDefinition;
use crate::game::logic::combat::fight;
use crate::game::data::investigator_definition::InvestigatorDefinition;
use crate::game::state::investigator_instance::InvestigatorInstance;

use std::io::{ self, Write };
fn main() {
    println!("Bienvenido a Eldritch Horror en CLI!");

    let data = fs
        ::read_to_string("data/investigators.json")
        .expect("No se pudo leer investigators.json");

    let definitions: Vec<InvestigatorDefinition> = serde_json
        ::from_str(&data)
        .expect("Error parseando JSON");

    println!("Elige tu investigador: ");
    for def in &definitions {
        println!("{}: {}", def.id, def.name);
    }

    print!("Introduce el ID del investigador: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error leyendo la entrada");
    let input_id: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada no valida");
            return;
        }
    };

    let chosen_def = match definitions.iter().find(|d| d.id == input_id) {
        Some(def) => def,
        None => {
            println!("No se encontró un investigador con ese ID");
            return;
        }
    };

    println!("Introduce tu nombre de jugador: ");
    io::stdout().flush().unwrap();
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("Error leyendo el nombre");
    let player_name = player_name.trim().to_string();

    let instance = InvestigatorInstance {
        player_name,
        definition_id: chosen_def.id,
        current_health: chosen_def.max_health,
        current_sanity: chosen_def.max_sanity,
        location_id: chosen_def.starting_location_id,
        items: chosen_def.starting_item_ids.clone(),
        stat_modifiers: crate::game::state::investigator_instance::StatModifiers {
            knowledge: 0,
            influence: 0,
            observation: 0,
            strength: 0,
            willpower: 0,
        },
    };

    println!("\n=== Investigador seleccionado ===");
    println!("Nombre: {}", chosen_def.name);
    println!("Profesión: {}", chosen_def.fluff.profession);
    println!("Historia: {}", chosen_def.fluff.backstory);
    println!("Frase célebre: {}", chosen_def.fluff.catchphrase);
    println!("Traits IDs: {:?}", chosen_def.traits);
    println!("Salud máxima: {}", chosen_def.max_health);
    println!("Cordura máxima: {}", chosen_def.max_sanity);
    println!("Ubicación inicial ID: {}", chosen_def.starting_location_id);
    println!("Items iniciales IDs: {:?}", chosen_def.starting_item_ids);
    println!("Evento muerte ID: {}", chosen_def.death_event_id);
    println!("Evento locura ID: {}", chosen_def.insanity_event_id);

    let s = &chosen_def.stats;
    println!(
        "Stats:  sabiduría={} 
        influencia={} 
        observación={} 
        fuerza={} 
        voluntad={}",
        s.knowledge,
        s.influence,
        s.observation,
        s.strength,
        s.willpower
    );

    let monster_data = fs
        ::read_to_string("data/monsters.json")
        .expect("No se pudo leer monsters.json");

    let monsters: Vec<MonsterDefinition> = serde_json
        ::from_str(&monster_data)
        .expect("Error parseando monsters");

    let mut rng = rand::thread_rng();
    let monster = &monsters[rng.gen_range(0..monsters.len())];

    let mut instance = instance;

    fight(&mut instance, chosen_def, monster);
}
