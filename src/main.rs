fn main() {
    println!("{}",string_to_braille("test"));
    println!("{}",braille_to_text("⡴⡥⡳⡴"));
    println!("{}",braille_to_text(&string_to_braille("test")));
}

fn byte_to_braille(by:u8) -> u16 {
    by as u16 + 10240
}

fn braille_to_byte(br:u16) -> u8 {
    (br - 10240) as u8
}

fn string_to_braille(s:&str) -> String {
    let mut vec: Vec<u16> = Vec::new();
    for c in s.chars() {
        vec.push(byte_to_braille(c as u8));
    }
    String::from_utf16_lossy(&vec)
}

fn bytes_to_braille(b: Vec<u8>) -> String {
    let mut vec: Vec<u16> = Vec::new();
    for by in b {
        vec.push(byte_to_braille(by as u8));
    }
    String::from_utf16_lossy(&vec)    
}

fn braille_to_bytes(s: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for c in s.chars() {
        vec.push(braille_to_byte(c as u16));
    }
    vec
}

fn braille_to_text(s: &str) -> String {
    String::from_utf8_lossy(&braille_to_bytes(s)).to_string()
}