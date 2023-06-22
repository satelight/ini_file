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
}

fn main() {
    let hinmoku_code = vec!["CO66901-C0", "CO66901-C1"];
    let ini_file = IniFile;
    ini_file.create(hinmoku_code)
}
