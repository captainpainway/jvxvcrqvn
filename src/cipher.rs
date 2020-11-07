pub fn rot13(message: String) -> String {
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];
    let upper_alphabet: Vec<_> = alphabet.iter().map(|c| c.to_ascii_uppercase()).collect();

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
