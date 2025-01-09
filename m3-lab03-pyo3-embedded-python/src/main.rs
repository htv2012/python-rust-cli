use pyo3::types::PyString;
use pyo3::types::PyModule;
use pyo3::prelude::*;




fn marco_python(input: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            r#"
def marco(value):
    if value == "marco":
        return "head"
    return "tail"
"#,
            "marco.py",
            "marco",
)?;

        let func = module.getattr("marco")?;
        let result = func.call1((input,))?;
        let str_result: &PyString = result.extract()?;
        Ok(str_result.to_string())
    })
}

fn main() {
    println!("marco => {}", marco_python("marco").unwrap());
    println!("polo => {}", marco_python("polo").unwrap());
}
