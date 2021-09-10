#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

lazy_static! {
    static ref CLIENT: reqwest::Client = {
        println!("Creating a new client...will be used all over.");
        reqwest::Client::new()
    };
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn make_request(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        // Ok(String::from("Hello"))
        let response = CLIENT.get("https://httpbin.org/ip").send().await.unwrap().text().await.unwrap();

        Ok(Python::with_gil(|py| response.into_py(py)))
        // Ok(Python::with_gil(|p   y| py.None()))
    })
    // Ok(CLIENT.get("https://httpbin.org/ip").send().unwrap().text().unwrap())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn maturin_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(make_request, m)?)?;

    Ok(())
}