fn main() {
    let mut system_state: u128 = 1; // initialization at 0 sets endianness to big endian
    println!("{}", extract_bits(&system_state, 0, 127));

    // this is not an valid Torn API key (to my knowledge), just a random sequence for testing encoding to system state
    let key = String::from("7mK9xF2b8Nq4zP1w"); 

    system_state = set_bits(system_state, 1, 96, convert_key_to_u128(key));
    println!("Endianness: {}", endianness(&system_state));
    println!("API Key: {}", get_key(&system_state));
}

fn get_key(system: &u128) -> String {
    return convert_u128_to_key(system, extract_bits(system, 1, 96));
}

const KEY_CHARACTERS: [char; 62] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn convert_key_to_u128(key: String) -> u128 {
    let mut return_value: u128 = 0;
    for c in key.chars() {
        for i in 0..62 {
            if KEY_CHARACTERS[i] == c {
                return_value *= 62;
                let j: u128 = i.try_into().unwrap();
                return_value += j;
                break;
            }
        }
    }
    return return_value;
}

fn convert_u128_to_key(system: &u128, data: u128) -> String {
    let mut return_string = String::from("");
    let mut input_data = data;
    while input_data > 0 {
        let remainder: u128 = input_data % 62;
        input_data /= 62;
        let remainder_usize: usize = remainder.try_into().unwrap(); 
        return_string.push(KEY_CHARACTERS[remainder_usize]);
    }
    if endianness(system) == 1 {
        return_string = return_string.chars().rev().collect();
    } 
    return return_string;

}

fn extract_bits(system: &u128, start_bit: u32, end_bit: u32) -> u128{
    let mask_size = end_bit - start_bit + 1;
    // clamping at max value of u128 prevents Rust compiler panicking
    let mask = if mask_size >= 128 { u128::MAX } else { ((1u128 << mask_size) - 1) << start_bit };
    let value = (*system).clone();

    let extracted_bits = (value & mask) >> start_bit;
    return extracted_bits;
}

fn set_bits(system: u128, start_bit: u32, end_bit:u32, data: u128) -> u128 {
    let size = end_bit - start_bit + 1;

    let mut original_state: u128 = system;

    // clamping at max value of u128 prevents Rust compiler panicking
    let size_mask = if size >= 128 { u128::MAX } else { (1u128 << size) - 1 };
    let range_mask = size_mask << start_bit;

    original_state &= !range_mask;

    let masked_data = (data & size_mask) << start_bit;
    let return_value = original_state | masked_data;
    
    return return_value;
}

fn endianness(system: &u128) -> u128 {
    return extract_bits(system, 0, 1);
}