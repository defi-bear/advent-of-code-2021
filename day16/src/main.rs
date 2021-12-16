// use std::fs;

// fn to_binary(c: char) -> &'static str {
//     match c {
//         '0' => "0000",
//         '1' => "0001",
//         '2' => "0010",
//         '3' => "0011",
//         '4' => "0100",
//         '5' => "0101",
//         '6' => "0110",
//         '7' => "0111",
//         '8' => "1000",
//         '9' => "1001",
//         'A' => "1010",
//         'B' => "1011",
//         'C' => "1100",
//         'D' => "1101",
//         'E' => "1110",
//         'F' => "1111",
//         _ => "",
//     }
// }

// fn convert_to_binary_from_hex(hex: &str) -> String {
//     hex[0..].chars().map(to_binary).collect()
// }

// fn recursive(binary: &mut &str, sum: &mut i32, flag: &mut i32) {
//     if binary.chars().count() <= 0 {
//         return;
//     }
//     println!("");
//     println!("flag: {}", flag);
//     println!("first: {}", binary);
//     let version = &binary[0..3];
//     let version_decimal = i32::from_str_radix(version, 2).unwrap();
//     *sum += version_decimal;
//     println!("sum: {}", sum);

//     *binary = &binary[3..];
    
//     let type_id = &binary[0..3];
//     *binary = &binary[3..];

//     let type_decimal = i32::from_str_radix(type_id, 2).unwrap();
//     println!("type_decimal: {}", type_decimal);

//     if type_decimal == 4 {
//         let mut lit_value: String = "\0".to_owned();
//         loop {
//             let first_bit = &binary[0..1];
//             let four_bits = &binary[1..5];
//             lit_value = lit_value + four_bits;
//             *binary = &binary[5..];
//             if *flag > 0 {
//                 *flag = *flag - 1;
//             }
//             if first_bit == "0" {
//                 break;
//             }
//         }
//         recursive(binary, sum, flag);
//         // println!("{}", i32::from_str_radix(lit_value.as_str(), 2).unwrap());
//     } else {
//         let length_type = &binary[0..1];
//         println!("length_type: {}", length_type);
//         if *flag > 0 {
//             *flag = *flag - 1;
//         }
//         if length_type == "0" {
//             let length = &binary[1..16];
//             *binary = &binary[16..];
//             let mut length_decimal = i32::from_str_radix(length, 2).unwrap();
//             println!("length: {}", length);
//             println!("length_decimal: {}", length_decimal);

//             let mut sub_sum: i32 = 0;
//             // length_decimal = length_decimal.min(binary.chars().count() as);
//             recursive(&mut &binary[0..length_decimal as usize], &mut sub_sum, &mut 1);
//             *sum += sub_sum;
//             *binary = &binary[length_decimal as usize..];
//             recursive(binary, sum, flag);
//         } else {
//             let length = &binary[1..12];
//             let length_decimal = i32::from_str_radix(length, 2).unwrap();
//             println!("length: {}", length);
//             println!("length_decimal: {}", length_decimal);
//             *binary = &binary[12..];
//             *flag = length_decimal;
//             recursive(binary, sum, flag);
//         }
//     }
// }

// fn main() {
//     let content = fs::read_to_string("input.txt").expect("something went wrong");
//     let binary_string = convert_to_binary_from_hex(content.as_str()).clone();
//     let mut binary = binary_string.as_str();
//     let mut sum: i32 = 0;
//     let mut flag: i32 = 0;
//     recursive(&mut binary, &mut sum, &mut flag);
//     println!("{}", sum);
// }

use std::fs;

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex[0..].chars().map(to_binary).collect()
}

fn recursive(binary: &mut &str, sum: &mut i32, flag: &mut i32) {
    if binary.chars().count() <= 0 {
        return;
    }
    println!("");
    println!("flag: {}", flag);
    println!("first: {}", binary);
    let version = &binary[0..3];
    let version_decimal = i32::from_str_radix(version, 2).unwrap();
    *sum += version_decimal;
    println!("sum: {}", sum);

    *binary = &binary[3..];
    
    let type_id = &binary[0..3];
    *binary = &binary[3..];

    let type_decimal = i32::from_str_radix(type_id, 2).unwrap();
    println!("type_decimal: {}", type_decimal);

    if type_decimal == 4 {
        let mut lit_value: String = "\0".to_owned();
        loop {
            let first_bit = &binary[0..1];
            let four_bits = &binary[1..5];
            lit_value = lit_value + four_bits;
            *binary = &binary[5..];
            if *flag > 0 {
                *flag = *flag - 1;
            }
            if first_bit == "0" {
                break;
            }
        }
        recursive(binary, sum, flag);
        // println!("{}", i32::from_str_radix(lit_value.as_str(), 2).unwrap());
    } else {
        let length_type = &binary[0..1];
        println!("length_type: {}", length_type);
        if *flag > 0 {
            *flag = *flag - 1;
        }
        if length_type == "0" {
            let length = &binary[1..16];
            *binary = &binary[16..];
            let mut length_decimal = i32::from_str_radix(length, 2).unwrap();
            println!("length: {}", length);
            println!("length_decimal: {}", length_decimal);

            let mut sub_sum: i32 = 0;
            // length_decimal = length_decimal.min(binary.chars().count() as);
            recursive(&mut &binary[0..length_decimal as usize], &mut sub_sum, &mut 1);
            *sum += sub_sum;
            *binary = &binary[length_decimal as usize..];
            recursive(binary, sum, flag);
        } else {
            let length = &binary[1..12];
            let length_decimal = i32::from_str_radix(length, 2).unwrap();
            println!("length: {}", length);
            println!("length_decimal: {}", length_decimal);
            *binary = &binary[12..];
            *flag = length_decimal;
            recursive(binary, sum, flag);
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("something went wrong");
    let binary_string = convert_to_binary_from_hex(content.as_str()).clone();
    let mut binary = binary_string.as_str();
    let mut sum: i32 = 0;
    let mut flag: i32 = 0;
    recursive(&mut binary, &mut sum, &mut flag);
    println!("{}", sum);
}