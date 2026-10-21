use rand::RngExt;

const API_KEY_CHARACTERS: [char; 62] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// Each Attacker has their own API key, retrieved from the state variable
struct Attacker {
    state: u128,
    key: String,
    attacks: i32,
}

/// Factions can contain up to 100 Attackers, so are parametrized as such
struct Faction<Attacker> {
    users: [Attacker; 100],
    best_attacker: Attacker,
    total_attacks: i32,
}



/// gets the state bits from index start_bit to index end_bit
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
    return get_state_bits(system, 0, 1);
}

/// gets the bits corresponding to the API key in state
fn get_state_api_key(state: &u128) -> String {
    return convert_u128_to_api_key(state, get_state_bits(state, 1, 97));
}

/// gets number of attacks from state variable
fn get_state_attacks(state: &u128) -> u128 {
    return get_state_bits(state, 97, 109);
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

/// sets number of attacks to given attacks
fn set_state_attacks(state: &mut u128, num_attacks: u128) -> () {
    set_state_bits(state, 97, 109, num_attacks);
}

/// generates a random string in the format of a Torn API key for testing purposes
fn generate_random_api_key() -> String {
    let mut api_key = String::from("");
    let mut rng = rand::rng();
    for _api_key_index in 0..16 {
        let random_index = rng.random_range(0..62);
        api_key.push_str(&String::from(API_KEY_CHARACTERS[random_index]));
    }
    return String::from(api_key);
}

/// converts an input Torn API key into base-2, reading the API key as a base-62 integer
fn convert_api_key_to_u128(key: &String) -> u128 {
    let mut return_value: u128 = 0;
    for key_char in key.chars() {
        for api_key_character_index in 0..62 {
            if API_KEY_CHARACTERS[api_key_character_index] == key_char {
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
        let remainder_usize: usize = remainder.try_into().unwrap(); 
        return_string.push(API_KEY_CHARACTERS[remainder_usize]);
    }
    if get_state_endianness(state) == 1 {
        return_string = return_string.chars().rev().collect();
    } 
    return return_string;

}

fn main() {
    let api_key = generate_random_api_key();

    let mut attacker_state: u128 = 1; // initialization at 1 sets endianness to big endian
    set_state_bits(&mut attacker_state, 1, 97, convert_api_key_to_u128(&api_key));
    println!("{}", get_state_bits(&attacker_state, 1, 97));

    println!("{}", &api_key);
    println!("{}", convert_api_key_to_u128(&api_key));
}