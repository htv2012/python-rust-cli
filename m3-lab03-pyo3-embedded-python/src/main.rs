use pyo3::types::PyString;
use pyo3::types::PyModule;
use pyo3::prelude::*;




fn marco_python(input: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let marco = PyModule::from_code(
            py,
            cr#"
def marco(value):
    if value == "marco":
        return "head"
    return "tail"
"#,
            cr#"marco.py"#,
            cr#"marco"#,
)?;

        let marco_func = marco.getattr("marco")?;
        let marco_result = marco_func.call1((input,))?;
        let marco_result: &PyString = marco_result.extract()?;
        Ok(marco_result.to_string())
    })
}

fn main() {
    println!("marco => {}", marco_python("marco").unwrap());
    println!("polo => {}", marco_python("polo").unwrap());
}
