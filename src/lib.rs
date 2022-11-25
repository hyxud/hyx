use std::fs::{self};
use json::JsonValue;


pub struct Manager {
    json: JsonValue,
}

impl Manager {
    pub fn init(path: &str) -> Self {
        let json_file= fs::read_to_string(path).expect("Error while reading the file");
        let contents = json::parse(&json_file).expect("coudn't parse file");


        Self { json: contents }
    }
    pub fn create(&self, id: u32) -> u32 {
        let mut out = 0;
        set_data(&mut out, 0, 8, id);
        out
    }
    pub fn get(&self, target: &str, data: &u32) -> u32 {
        let id = Manager::get_id(data);
        let mut bit_offset: u32 = 0;
        for (key, val) in self.json[id.to_string()]["structure"].entries() {
            if key == target {
                return get_data(data, bit_offset as u8, val.as_u8().unwrap()) 
            }
            bit_offset += val.as_u32().unwrap()
        }
        panic!("Failed to find Entry: {}", target)
    }
    pub fn get_max(&self, target: &str, data: &u32) -> u32 {
        let id = Manager::get_id(data);
        for (key, val) in self.json[id.to_string()]["structure"].entries() {
            if key == target {
                return pow(2, val.as_u32().unwrap()) - 1;
            }
        }
        panic!("Failed to find Type Entry: {}", target)
    }
    pub fn type_get(&self, target: &str, data: &u32) -> String {
        let id = Manager::get_id(data);
        for (key, val) in self.json[id.to_string()].entries() {
            if key == target {
                return val.to_string();
            }
        }
        panic!("Failed to find Type Entry: {}", target)
    }
    pub fn set(&self, target: &str, payload: u32, data: &mut u32) {
        let id = Manager::get_id(data);
        let mut bit_offset: u32 = 0;
        for (key, val) in self.json[id.to_string()]["structure"].entries() {
            if key == target {
                if payload > pow(2, val.as_u32().unwrap()) - 1 {
                    panic!("Overload: Attempted to set to value beyond maximum:\n
                    -> Attemted to set {} to {} but maximum is {}", target, payload, pow(2, val.as_u32().unwrap()) - 1)
                }
                set_data(data, bit_offset as u8, val.as_u8().unwrap(), payload);
                return;
            }
            bit_offset += val.as_u32().unwrap()
        }
        panic!("Failed to find Entry: {}", target)
    }
    pub fn type_entries(&self, data: &u32) -> Vec<&str> {
        let id = Manager::get_id(data);
        let mut entires: Vec<&str> = vec![];
        for (key, _) in self.json[id.to_string()].entries() {
            if key == "structure" { continue }
            entires.push(key);
        }
        entires
    }
    pub fn entires(&self, data: &u32) -> Vec<&str> {
        let id = Manager::get_id(data);
        let mut entires: Vec<&str> = vec![];
        for (key, _) in self.json[id.to_string()]["structure"].entries() {
            entires.push(key);
        }
        entires
    }
    pub fn print_data(&self, data: &u32) {
        let id = Manager::get_id(data);
        for (key, _) in self.json[id.to_string()]["structure"].entries() {
            println!("{}: {}", key, self.get(key, data));
        }
    }
    fn get_id(data: &u32) -> u32 {
        return get_data(data, 0, 8);
    }
}


fn pow(num: u32, n: u32) -> u32 {
    let mut res = num;
    if n == 0 {
        return 1;
    }
    for _ in 0..n-1 {
        res = res * num
    }
    res
}

fn get_data(data: &u32, index: u8, count: u8) -> u32 {
    
    let list = get_bits_rev(&data, index, count);
    let len = list.len() as u32;
    let mut power = 0;
    let mut end_num = 0;
    for i in 0..len {
        end_num += list[i as usize] as u32 * pow(2, power);
        power += 1;
    }
    end_num
}
fn get_bits(data: &u32, index: u8, mut count: u8) -> Vec<u8> {
    let mut bits: Vec<u8> = vec![0; count as usize];
    let len = bits.len();
    if count == 0 {
        count = (data.count_ones() + data.count_zeros() - index as u32) as u8
    }
    for n in 0..count {
        bits[len - 1 - n as usize] = get_bit_at(data, n + index);
    }
    bits
}
fn get_bits_rev(data: &u32, index: u8, mut count: u8) -> Vec<u8> {
    let mut bits: Vec<u8> = vec![0; count as usize];
    if count == 0 {
        count = (data.count_ones() + data.count_zeros() - index as u32) as u8
    }
    for n in 0..count {
        bits[n as usize] = get_bit_at(data, n + index);
    }
    bits
}

fn set_data(data: &mut u32, index: u8, mut count: u8, payload: u32) {
    if count == 0 {
        count = (data.count_ones() + data.count_zeros() - index as u32) as u8
    }
    for i in 0..count {
        set_bit_at(data, i + index, get_bit_at(&payload, i) != 0)
    }
}
fn get_bits_str(data: &u32, index: u8, count: u8) -> String {
    let mut str: String = String::new();
    let bit_vec = get_bits(data, index, count);
    for bit in bit_vec {
        str.insert_str(0, &bit.to_string())
    }
    str
}

fn get_bit_at(input: &u32, n: u8) -> u8 {
    if n < 32 {
        (input & (1 << n) != 0) as u8
    } else {
        0
    }
}
fn set_bit_at(input: &mut u32, n: u8, payload: bool) {
    if payload {
        *input |= 1 << n
    } else {
        *input &= !(1 << n)
    }
}