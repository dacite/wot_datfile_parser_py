use pyo3::exceptions::PyException as Err;
use pyo3::prelude::*;
use wot_datfile_parser as dp;

/// A Python module implemented in Rust.
#[pymodule]
fn wot_datfile_parser_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DatfileParser>()?;

    Ok(())
}

#[pyclass]
struct DatfileParser {
    dp: dp::DatFileParser,
}

#[pymethods]
impl DatfileParser {
    #[new]
    fn new() -> Self {
        Self {
            dp: dp::DatFileParser::new(),
        }
    }

    fn parse(&self, file: &[u8]) -> PyResult<String> {
        let battle = self.dp.parse(file).map_err(convert_err)?;
        let json_string = serde_json::to_string(&battle).map_err(convert_err)?;

        Ok(json_string)
    }
}

fn convert_err(err: impl ToString) -> PyErr {
    Err::new_err(err.to_string())
}
