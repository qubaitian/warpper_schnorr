use pyo3::prelude::*;

/// Formats
#[pyfunction]
fn sum_as(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


fn private_to_public(private_key: String) -> anyhow::Result<String> {
    let private_key_bytes = bs58::decode(private_key).into_vec()?;
    let private_key = schnorrkel::SecretKey::from_bytes(&private_key_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid Schnorr private key"))?;
    let public_key = private_key.to_public();
    Ok(bs58::encode(public_key.to_bytes()).into_string())
}

#[pyfunction]
fn private_key_to_public_key (private_key: String) -> String{
    let result = private_to_public(private_key).expect("111");
    result
}

/// please
#[pyfunction]
fn other_function(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn warpper_schnorr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as, m)?)?;
    m.add_function(wrap_pyfunction!(other_function, m)?)?;
    m.add_function(wrap_pyfunction!(private_key_to_public_key, m)?)?;
    Ok(())
}

