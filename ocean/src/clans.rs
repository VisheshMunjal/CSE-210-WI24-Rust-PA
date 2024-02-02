use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clans: HashMap<String, HashSet<String>>, 
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {
            clans: HashMap::new()
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        //unimplemented!();
        if let Some(members) = self.clans.get(clan_id) {
            members.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        //unimplemented!();
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        //unimplemented!();
        self.clans.get(clan_id).map_or(0, |members| members.len())
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let largest_clan = self.clans.iter().max_by_key(|(_, members)| members.len());
        largest_clan.map(|(clan_id, _)| clan_id.clone())
    }
    // This add a member to the clan given an id of clan and name of crab
    pub fn add_member_to_clan(&mut self, clan_id: String, crab_name: String) {
        let clan = self.clans.entry(clan_id).or_insert_with(HashSet::new);
        clan.insert(crab_name);
    }
}