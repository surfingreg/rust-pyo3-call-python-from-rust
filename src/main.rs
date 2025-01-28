//!
//! Use Python from Rust ()
//!
//! main.rs
//! 
//! ref: https://pyo3.rs/v0.23.4/
//! 


use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::ffi::c_str;

/// use python from rust
fn main() -> PyResult<()>{

    Python::with_gil(|py| {
    
        println!("running python stuff...");
        
        // Use python to get the system version:
        // >>> sys.version
        // ie. version: 3.13.1 (main, Dec  3 2024, 17:59:52) [Clang 16.0.0 (clang-1600.0.26.4)]
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        println!("version: {}", version);

        // >>> os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'
        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;
        println!("user: {}", user);

        Ok(())
    })

}


