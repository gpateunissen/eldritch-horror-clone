use crate::game::data::investigator_definition::InvestigatorDefinition;
use crate::game::data::monster_definition::MonsterDefinition;
use crate::game::state::investigator_instance::InvestigatorInstance;
use crate::game::utils::dice::{ roll_dice, count_successes };
use crate::game::state::monster_instance::MonsterInstance;

pub fn fight(
    investigator: &mut InvestigatorInstance,
    def: &InvestigatorDefinition,
    monster: &MonsterDefinition
) {
    println!("\n=== ENCUENTRO CON {} ===", monster.name);

    //Instancia monstruo
    let mut monster_instance = MonsterInstance {
        name: monster.name.clone(),
        current_toughness: monster.Toughness,
        sanity_dmg: monster.Sanity_dmg,
        strength_dmg: monster.Strength_dmg,
        sanity_test: monster.Sanity_test,
        strength_test: monster.Str_test,
    };

    // 🧠 FASE 1: Willpower.
    let willpower = def.stats.willpower + investigator.stat_modifiers.willpower;
    let dice = willpower + monster.Sanity_test;

    println!("\nPrueba de voluntad: {} dados", dice);

    let rolls = roll_dice(dice.max(1));
    println!("Tiradas: {:?}", rolls);

    let successes = count_successes(&rolls);

    if successes == 0 {
        println!("Fallaste la prueba de voluntad 😱");
        investigator.current_sanity -= monster.Sanity_dmg;
        println!("Pierdes {} de cordura", monster.Sanity_dmg);
    } else {
        println!("Superaste la prueba de voluntad 👍");
    }

    if investigator.current_sanity <= 0 {
        println!("Has perdido toda la cordura!");
        return;
    }

    // 🧠 FASE 1: Combat.
    let strength = def.stats.strength + investigator.stat_modifiers.strength;

    println!("\nPrueba de fuerza: {} dados", dice);

    let rolls = roll_dice(dice.max(1));
    println!("Tiradas: {:?}", rolls);

    let successes = count_successes(&rolls);

    if successes > 0 {
        println!("Haces daño al monstruo 🩸");
        if successes >= monster.Toughness {
            println!("🎉 Has derrotado al monstruo!");
        } else {
            println!("El monstruo resiste...");
        }
    } else {
        println!("Fallaste el ataque 💥");
        investigator.current_health -= monster.Strength_dmg;
        println!("Recibes {} de daño", monster.Strength_dmg);
    }

    if investigator.current_health <= 0 {
        println!("💀 Has muerto");
        return;
    }

    println!("\n=== FIN DEL COMBATE ===");
}
