use std::char;

pub fn rot13(message: String) -> String {
    let alphabet: Vec<char> = (97..123).map(|n| char::from_u32(n).unwrap()).collect();
    let upper_alphabet: Vec<char> = (65..91).map(|n| char::from_u32(n).unwrap()).collect();

    message.chars()
        .map(|c| *alphabet.iter()
            .chain(alphabet.iter())
            .chain(upper_alphabet.iter())
            .chain(upper_alphabet.iter())
            .skip_while(|&x| *x != c)
            .nth(13)
            .unwrap_or(&c))
        .collect()
}
