#![allow(dead_code)]

// looked this up, used in generate_random_api_key()
use rand::RngExt;

const API_KEY_CHARSET: [char; 62] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// Each Attacker has their own API key, retrieved from the state variable; consequently, Attacker lifetime is tied to that of its state variable

// https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-functionality-with-derived-traits
// #[derive(Debug)] allows for print!({some_attacker:?}) to output "Attacker { state }"
#[derive(Debug)] 
struct Attacker<'state_lifetime> {
    state: &'state_lifetime mut u128,
}

/// Each Faction has a leader which is an Attacker; Faction lifetime is tied to those of its Attackers and consequently, their state variables
#[derive(Debug)]
struct Faction<'state_lifetime> {
    leader: &'state_lifetime mut Attacker<'state_lifetime>,
}

/// gets the state bits from index start_bit to index end_bit INCLUSIVE
fn get_state_bits(state: &u128, start_bit: u32, end_bit: u32) -> u128{
    let mask_size = end_bit - start_bit + 1;
    // clamping at max value of u128 prevents Rust compiler panicking
    let mask = if mask_size >= 128 { u128::MAX } else { ((1u128 << mask_size) - 1) << start_bit };
    let value = (*state).clone();

    let extracted_bits = (value & mask) >> start_bit;
    return extracted_bits;
}

/// returns the endianness of the state variable, which is stored in the bit at the 0th index
fn get_state_endianness(system: &u128) -> u128 {
    return get_state_bits(system, 0, 0);
}

/// gets the bits corresponding to the API key in state
fn get_state_api_key(state: &u128) -> String {
    return convert_u128_to_api_key(state, get_state_bits(state, 1, 96));
}

/// gets number of attacks from state variable
fn get_state_attacks(state: &u128) -> u128 {
    return get_state_bits(state, 97, 108);
}

/// replaces bits from index start_bit to index end_bit with given data 
fn set_state_bits(state: &mut u128, start_bit: u32, end_bit:u32, data: u128) -> () {
    let size = end_bit - start_bit + 1;

    let mut original_state: u128 = *state;

    // clamping at max value of u128 prevents Rust compiler panicking
    let size_mask = if size >= 128 { u128::MAX } else { (1u128 << size) - 1 };
    let range_mask = size_mask << start_bit;

    original_state &= !range_mask;

    let masked_data = (data & size_mask) << start_bit;
    let return_state = original_state | masked_data;

    *state = return_state;
}

/// sets API key to given API key
fn set_state_api_key(state: &mut u128, api_key: u128) -> () {
    set_state_bits(state, 1, 97, api_key);
}

/// sets number of attacks to given attacks
fn set_state_attacks(state: &mut u128, num_attacks: u128) -> () {
    set_state_bits(state, 97, 109, num_attacks);
}

/// increases state attacks by given number
fn increase_state_attacks_by(state: &mut u128, delta_attacks: u128) -> () {
    set_state_bits(state, 97, 109, get_state_attacks(state) + delta_attacks);
}

/// decreases state attacks by given number
fn decrease_state_attacks_by(state: &mut u128, delta_attacks: u128) -> () {
    set_state_bits(state, 97, 109, get_state_attacks(state) - delta_attacks);
}


/// generates a random string in the format of a Torn API key for testing purposes
fn generate_random_api_key() -> String {
    let mut api_key = String::from("");
    let mut rng = rand::rng();
    for _api_key_index in 0..16 {
        let random_index = rng.random_range(0..62);
        api_key.push_str(&String::from(API_KEY_CHARSET[random_index]));
    }
    return String::from(api_key);
}

/// generates a random initial state composed of a Torn API key for testing purposes
fn generate_random_initial_state() -> u128 {
    let mut random_initial_state: u128 = 1;
    let mut rng = rand::rng();
    set_state_api_key(&mut random_initial_state, convert_api_key_to_u128(&generate_random_api_key()));
    set_state_attacks(&mut random_initial_state, rng.random_range(0..64));
    return random_initial_state;
}

/// converts an input Torn API key into base-2, reading the API key as a base-62 integer
fn convert_api_key_to_u128(key: &String) -> u128 {
    let mut return_value: u128 = 0;
    for key_char in key.chars() {
        for api_key_character_index in 0..62 {
            if API_KEY_CHARSET[api_key_character_index] == key_char {
                return_value *= 62;
                let added_value: u128 = api_key_character_index.try_into().unwrap();
                return_value += added_value;
                break;
            }
        }
    }
    return return_value;
}

/// converts a binary/base-2 integer into its base-62 equivalent as a string, outputting the API key it represents
fn convert_u128_to_api_key(state: &u128, data: u128) -> String {
    let mut return_string = String::from("");
    let mut input_data = data;
    while input_data > 0 {
        let remainder: u128 = input_data % 62;
        input_data /= 62;
        // "remainder as usize" also works (probably), .try_into().unwrap() was what Rust compiler gave me first
        let remainder_usize: usize = remainder.try_into().unwrap(); 
        return_string.push(API_KEY_CHARSET[remainder_usize]);
    }
    if get_state_endianness(state) == 1 {
        return_string = return_string.chars().rev().collect();
    } 
    return return_string;
}

// impl<'state> declares that 'state lifetime will be used in this scope
// Attacker<'state> applies 'state lifetime to the Attacker struct
impl<'state_lifetime> Attacker<'state_lifetime> {
    /// constructor for new Attacker given a state u128
    pub fn create_attacker_with_state(attacker_state: &'state_lifetime mut u128) -> Self {
        Self {
            state: attacker_state,
        }
    }
    /// gets API key from its own state
    fn get_api_key(&self) -> String {
        return get_state_api_key(self.state);
    }
    /// gets number of attacks from its own state
    fn get_attacks(&self) -> u128 {
        return get_state_attacks(self.state);
    }
    /// gets endianness from its own state
    fn get_endianness(&self) -> u128 {
        return get_state_endianness(self.state);
    }
    /// gets its own state
    fn get_state(&self) -> u128 {
        return *self.state;
    }
    /// sets own API key to given value
    fn set_api_key(&mut self, api_key: &String) {
        set_state_api_key(self.state, convert_api_key_to_u128(&api_key));
    }
    /// increases attacks in its own state variable
    fn increase_state_attacks_by(&mut self, delta_attacks: u128) -> () {
        increase_state_attacks_by(self.state, delta_attacks);
    }
}

impl<'state_lifetime> Faction<'state_lifetime> {
    /// constructor for a Faction given an Attacker
    pub fn create_faction_shell_with_attacker(first_attacker: &'state_lifetime mut Attacker<'state_lifetime>) -> Self {
        Self {
            leader: first_attacker,
        }
    }
    /// returns true if its leader has more attacks than the other faction's leader
    pub fn has_more_attacks_than(&self, other_faction: &Faction<'state_lifetime>) -> bool {
        return self.leader.get_attacks() > other_faction.leader.get_attacks();
    }
}

fn main() {
    let mut attacker_1_state = generate_random_initial_state();
    let mut attacker_2_state = generate_random_initial_state();
    
    
    {
        let mut attacker_1 = Attacker::create_attacker_with_state(&mut attacker_1_state);
        println!("Attacker 1 Stored Key: {}", attacker_1.get_api_key());
        let mut attacker_2 = Attacker::create_attacker_with_state(&mut attacker_2_state);
        println!("Attacker 1 Stored Key: {}", attacker_2.get_api_key());
        
        println!("Attacker 1 status as binary: {:b}", attacker_1.get_state());
        println!("Attacker 2 status as binary: {:b}", attacker_2.get_state());        

        println!("Attacker 1 status as u128: {attacker_1:?}");
        println!("Attacker 2 status as u128: {attacker_2:?}");
        
        println!("Attacker 1 Attacks: {}", attacker_1.get_attacks());
        println!("Attacker 2 Attacks: {}", attacker_2.get_attacks());
        let faction_1: Faction;
        let faction_2: Faction;
        {
            faction_1 = Faction::create_faction_shell_with_attacker(&mut attacker_1);
            faction_2 = Faction::create_faction_shell_with_attacker(&mut attacker_2);
        }
        println!("Does Faction_1 have more attacks than Faction_2? {}", faction_1.has_more_attacks_than(&faction_2));
    }
    
}
