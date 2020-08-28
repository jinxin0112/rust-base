fn main() {
    let v = vec![String::from("jin"), String::from("xin")];
    println!("v is {:?}", v);

    fn pig_latin_format(s: &String) -> String {
        let vowels = vec!["a", "e", "i", "o", "u"];
        let first_char = &s[..1];
        let res_char = &s[1..];

        match vowels.contains(&first_char) {
            true => format!("{}-hay", &s),
            false => format!("{}-{}ay", &res_char, first_char),
        }
    }

    println!("{}", pig_latin_format(&String::from("first")));
    println!("{}", pig_latin_format(&String::from("apple")));
}
