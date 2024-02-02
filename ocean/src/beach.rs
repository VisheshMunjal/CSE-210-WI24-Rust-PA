use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    pub size: usize,
    pub collection: Vec<Crab>,
    pub clan_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        //unimplemented!();
        Beach{size: 0, collection: Vec::new(), clan_system: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        //unimplemented!();
        self.size
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        //unimplemented!();
        self.collection.push(crab);
        self.size += 1;
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        //unimplemented!();
        &self.collection[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        //unimplemented!();
        self.collection.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        //unimplemented!();
        if self.size == 0 {
            None
        } else {
            Some(self.collection.iter().max_by_key(|&crab| crab.speed).unwrap())
        }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        //unimplemented!();
        self.collection.iter().filter(|&crab| crab.name == name).collect()
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if i >= self.size || j >= self.size {
            panic!("Indices are out of bounds");
        }
        let parent1 = &self.collection[i];
        let parent2 = &self.collection[j];
        let new_diet = Diet::random_diet();
        let new_color = Color::cross(&parent1.color, &parent2.color);
        let new_crab = Crab {
            name,
            speed: 1,
            color: new_color,
            diet: new_diet,
            reefs: Vec::new()
        };
        self.collection.push(new_crab);
        self.size += 1; 
    }
    

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clan_system.add_member_to_clan(clan_id.to_string(), crab_name.to_string());
    
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1_members = self.clan_system.get_clan_member_names(id1);
        let clan2_members = self.clan_system.get_clan_member_names(id2);

        if clan1_members.is_empty() || clan2_members.is_empty() {
            return Err("Invalid clan IDs".to_string());
        }

        let clan1_avg_speed: f64 = clan1_members.iter().map(|name| self.calculate_speed_for_crab_by_name(name)).sum::<u32>() as f64 / clan1_members.len() as f64;
        let clan2_avg_speed: f64 = clan2_members.iter().map(|name| self.calculate_speed_for_crab_by_name(name)).sum::<u32>() as f64 / clan2_members.len() as f64;

        if clan1_avg_speed > clan2_avg_speed {
            Ok(Some(id1.to_string()))
        } else if clan2_avg_speed > clan1_avg_speed {
            Ok(Some(id2.to_string()))
        } else {
            Ok(None)
        }
    }
    fn calculate_speed_for_crab_by_name(&self, crab_name: &str) -> u32 {
        for crab in &self.collection {
            if crab.name == crab_name {
                return crab.speed;
            }
        }
        return 0 // Crab with the given name not found
    }
    
}
