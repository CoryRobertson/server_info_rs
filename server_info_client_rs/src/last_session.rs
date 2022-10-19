

pub mod last_session {
    use std::fmt::Write;
    use std::fs;
    use std::fs::File;
    use std::io::Write as IOWrite;
    use std::path::Path;

    #[derive(Default)]
    pub struct LastSession {
        pub address: String,
        pub screen_dimension: (f32, f32),
    }

    impl LastSession {
        fn convert_to_string(&self) -> String {
            let mut s = String::new();
            let _ = s.write_str(&*self.address);
            let _ = s.write_str(",");
            let _ = s.write_str(&*format!("{},{}", self.screen_dimension.0.to_string(), self.screen_dimension.1.to_string()));

            return s;
        }
    }

    fn string_to_last_session(input: String) -> Result<LastSession, String> {

        let v: Vec<&str> = input.split(",").collect();
        let address = v[0].to_string();

        let x = match v[1].parse::<f32>() {
            Ok(x) => {x}
            Err(e) => {return Err(e.to_string());}
        };

        let y = match v[2].parse::<f32>() {
            Ok(y) => {y}
            Err(e) => {return Err(e.to_string());}
        };

        return Ok(LastSession{ address, screen_dimension: (x, y) })
    }
    
    pub fn write_to_file(file_name: &str, last_session: LastSession) -> Result<(), String>{
        let path = Path::new(file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Ok(f) => {f}
            Err(e) => {return Err(format!("Could not create file: {}, {}", e, display).to_string());}
        };

        match file.write_all(last_session.convert_to_string().as_bytes()) {
            Ok(_) => {}
            Err(e) => {return Err(format!("Could not write to file: {}, {}", e, display).to_string());}
        };

        return Ok(());
    }

    pub fn read_from_file(file_name: &str) -> Result<LastSession, String> {
        let file_as_string = match fs::read_to_string(file_name) {
            Ok(s) => {s}
            Err(e) => {return Err(e.to_string());}
        };
        return string_to_last_session(file_as_string);
    }
}