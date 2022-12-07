use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("module.rs");
    let source = fs::read_to_string("src/lib.rs").unwrap();

    let functions = Regex::new(r"#\[pyfunction]\s*(?:\w\s+)*?fn\s+([\w0-9]+)").unwrap();
    let structs = Regex::new(r"#\[pyclass]\s*(?:\w\s+)*?(?:struct|enum)\s+([\w0-9]+)").unwrap();

    let mut modules = "#[pymodule]
    fn tkmandel(_py: Python, m: &PyModule) -> PyResult<()> {\n".to_string();

    for cap in functions.captures_iter(&source) {
        modules += &format!("m.add_function(wrap_pyfunction!({}, m)?)?;\n", &cap[1]);
    }

    for cap in structs.captures_iter(&source) {
        modules += &format!("m.add_class::<{}>()?;\n", &cap[1]);
    }

    modules += "Ok(())
    }";
    
    fs::write(&dest_path, modules).unwrap();
}