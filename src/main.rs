//!
//! Use Python from Rust ()
//!
//! main.rs
//! 
//! ref: https://pyo3.rs/v0.23.4/
//! 


use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;
// use pyo3::ffi::c_str;

/// use python from rust
fn main() -> PyResult<()>{
    Python::with_gil(|py| {
        println!("starting python stuff...");
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        println!("version: {}", version);
        Ok(())
    })

}


