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
        current_toughness: monster.toughness,
        sanity_dmg: monster.sanity_dmg,
        strength_dmg: monster.strength_dmg,
        sanity_test: monster.sanity_test,
        strength_test: monster.str_test,
    };

    while
        investigator.current_health > 0 &&
        investigator.current_sanity > 0 &&
        monster_instance.current_toughness > 0
    {
        // 🧠 FASE 1: Willpower.
        let willpower = def.stats.willpower + investigator.stat_modifiers.willpower;
        let dice = (willpower + monster_instance.sanity_test).max(1);
        println!("\nPrueba de voluntad: {} dados", dice);

        let rolls = roll_dice(dice);
        println!("Tiradas: {:?}", rolls);

        let successes = count_successes(&rolls);

        if successes == 0 {
            println!("Fallaste la prueba de voluntad 😱");
            investigator.current_sanity -= monster_instance.sanity_dmg;
            println!("Pierdes {} de cordura", monster_instance.sanity_dmg);
            if investigator.current_sanity <= 0 {
                println!("Has perdido toda la cordura! \n === FIN DEL COMBATE ===");
                return;
            }
            println!("{} puntos de cordura restantes", investigator.current_sanity);
        } else {
            println!("Superaste la prueba de voluntad 👍");
        }

        // 💪 FASE 1: Combat.
        let strength = def.stats.strength + investigator.stat_modifiers.strength;
        let dice = (strength + monster_instance.strength_test).max(1);

        println!("\nPrueba de fuerza: {} dados", dice);

        let rolls = roll_dice(dice);
        println!("Tiradas: {:?}", rolls);

        let successes = count_successes(&rolls);

        if successes > 0 {
            println!("Haces daño al monstruo 🩸");
            monster_instance.current_toughness -= successes;

            if monster_instance.current_toughness <= 0 {
                println!("🎉 Has derrotado al monstruo! \n === FIN DEL COMBATE ===");
            } else {
                println!(
                    "El monstruo resiste... le queda {} de vida",
                    monster_instance.current_toughness
                );
            }
        } else {
            println!("Fallaste el ataque 💥");
            investigator.current_health -= monster_instance.strength_dmg;
            println!("Recibes {} de daño", monster_instance.strength_dmg);
            if investigator.current_health <= 0 {
                println!("💀 Has muerto \n === FIN DEL COMBATE ===");
                return;
            }
            println!("{} puntos de salud restantes", investigator.current_health);
        }

        println!("\n=== FIN DE RONDA DE COMBATE ===");
    }
}
