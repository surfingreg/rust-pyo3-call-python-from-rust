//!
//! Use Python from Rust ()
//!
//! main.rs
//! 
//! ref: https://pyo3.rs/v0.23.4/
//! GIL = global interpreter lock
//! 


use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::ffi::c_str;

/// use python from rust
fn main() -> PyResult<()>{


    // Acquires the global interpreter lock, allowing access to the Python interpreter. The provided 
    // closure F will be executed with the acquired Python marker token.
    // 
    // If the auto-initialize feature is enabled and the Python runtime is not already initialized, 
    // this function will initialize it. See prepare_freethreaded_python for details.
    //
    // https://docs.rs/pyo3/latest/pyo3/marker/struct.Python.html#method.with_gil
    Python::with_gil(|py| {
    
        println!("running python stuff...");
        
        // Use python to get the system version:
        // >>> sys.version
        // ie. version: 3.13.1 (main, Dec  3 2024, 17:59:52) [Clang 16.0.0 (clang-1600.0.26.4)]
        let bound: Bound<'_, PyModule> = py.import("sys")?;
        let version: String = bound.getattr("version")?.extract()?;
        println!("version: {}", version);

        // >>> os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'
        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;
        println!("user: {}", user);


        // compare Python bytes to a Rust [u8] slice
        // https://docs.rs/pyo3/latest/pyo3/types/struct.PyBytes.html#equality
        let py_bytes = pyo3::types::PyBytes::new(py, b"wassup".as_slice());
        assert_eq!(py_bytes, b"wassup".as_slice());





        Ok(())
    })

}


