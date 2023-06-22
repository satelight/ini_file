use ini::Ini;

struct IniFile;

impl IniFile {
    pub fn create(&self, hinmoku_codes: Vec<&str>) {
        let mut conf = Ini::new();
        for hinmoku_code in hinmoku_codes {
            conf.with_section(Some(hinmoku_code))
                .set("FileName", hinmoku_code);
        }
        conf.write_to_file("conf.ini").unwrap();
    }

    pub fn read(&self) {
        let conf = Ini::load_from_file("conf.ini").unwrap();
        for (sec, prop) in &conf {
            println!("Section: {:?}", sec);
            for (key, value) in prop.iter() {
                println!("{:?}:{:?}", key, value);
            }
        }
    }

    pub fn search_section(&self, section_word: &str) -> String {
        let conf = Ini::load_from_file("conf.ini").unwrap();
        let search_word: String = String::from("FileName=") + section_word;
        println!("{search_word}");
        let section = conf.section(Some(section_word)).unwrap();
        String::from(section.get(&search_word).unwrap()).replace("FileName=", "")
    }
}

fn main() {
    let hinmoku_code = vec!["CO66901-C0", "CO66901-C1"];
    let ini_file = IniFile;
    ini_file.create(hinmoku_code);
    ini_file.read();
    ini_file.search_section("CO66901-C0");
}
