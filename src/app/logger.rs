use std::{
    env::current_exe,
    io::Write,
    fs::{File},
};

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn log(&mut self, str: String) {
        self.file.write(str.as_bytes()).unwrap();
        self.file.write("\n".as_bytes()).unwrap();
        self.file.flush().unwrap();
    }
}

pub fn build_logger() -> Logger {
    let mut l = Logger {
        file: File::create(get_log_path()).unwrap()
    };
    l.log("\n\n\nRESTARTING\n\n\n".to_string());
    l
}

pub fn get_log_path() -> String {
    let p = current_exe().unwrap();
    let pp = p.parent().unwrap();
    pp.join("gitbumr.log").as_os_str().to_os_string().into_string().unwrap()
}
