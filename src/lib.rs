pub mod asciifier {

    use std::collections::HashMap;

    pub struct Asciifier {
        character_map: HashMap<char, &'static str>,
    }

    impl Asciifier {

        pub fn new() -> Asciifier {
            let mut char_map = HashMap::with_capacity(20);
            char_map.insert('ä', "ae");
            char_map.insert('ö', "oe");
            char_map.insert('ü', "ue");
            char_map.insert('Ä', "Ae");
            char_map.insert('Ö', "Oe");
            char_map.insert('Ü', "Ue");
            char_map.insert('ß', "ss");
            Asciifier {
                character_map: char_map,
            }
        }

        pub fn to_ascii(&self, from: String) -> String {
            let mut to = from;
            for (special_char, replacement) in &self.character_map {
                to = to.replace(*special_char, replacement);
            }
            to
        }
}

}


#[cfg(test)]
mod tests {
    #[test]
    fn replace_single_char() {
        let asciifier = super::Asciifier::new();
        let ascii_string = asciifier.to_ascii("ä".to_string());
        assert_eq!(&ascii_string, "ae");
    }

    #[test]
    fn replace_lots() {
        let asciifier = super::Asciifier::new();
        let utf8_str = "ÄÖÜßäöü";
        //let utf8_str = "ŠŒŽšœžŸ¥µÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖØÙÚÛÜÝßàáâãäåæçèéêëìíîïðñòóôõöøùúûüýÿ";
        let ascii_string = asciifier.to_ascii(utf8_str.to_string());
        assert_eq!(&ascii_string, "AeOeUessaeoeue");
        //assert_eq!(&ascii_string, "SOZsozYYuAAAAAAeACEEEEIIIIDNOOOOOeOeUUUUeYssaaaaaaeaceeeeiiiionoooooeoeuuuueyy");
    }
}
