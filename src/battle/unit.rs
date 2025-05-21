pub struct Unit {
    name: String,
    max_hp: u32,
    current_hp: u32,
    atk: u32,
    spd: u32,
    abilities: Vec<Ability>,
}

pub enum Ability {
    BasicAttack,
}

impl Unit {

    // Constructor
    pub fn new(name: String, max_hp: u32, atk: u32, spd: u32) -> Self {
        Unit {
            name,
            max_hp,
            current_hp: max_hp,
            atk,
            spd,
            abilities: vec![Ability::BasicAttack],
        }
    }

    ///////////////////////////////////////////////
    
    // Getters and Setters

    pub fn is_alive(&self) -> bool {
        return self.current_hp > 0;
    }

    fn take_damage(&mut self, amount: u32) {
        if amount > self.current_hp as u32 {
            self.current_hp = 0;
        } else {
            self.current_hp -= amount;
        }
    }

    pub fn get_name(&self) -> String {
        return String::from(self.name.clone());
    }

    ////////////////////////////////////////////////////////
    
    // Battle functions

    fn attack(&self, target: &mut Unit) {
        println!("{} attacks {} for {} damage!", self.name, target.name, self.atk);
        target.take_damage(self.atk);
    }

    pub fn take_turn(&mut self, target: &mut Unit) {
        // Logic for taking a turn, e.g., using an ability or attacking
        // For now, just wrapper around attack
        if !self.is_alive() {
            println!("{} is KO'd and cannot take a turn!", self.name);
            return;
        }
        self.attack(target);
    }
    
}