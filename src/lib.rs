use pyo3::prelude::*;
use rand::{rngs::OsRng, rngs::StdRng, SeedableRng};
use schnorrkel::{signing_context, verify_batch, Keypair};
use sha2::{Digest, Sha256};

pub fn _sign_message(
    message: String,
    private_key: String,
    context: String,
) -> anyhow::Result<String> {
    let private_key_bytes = bs58::decode(private_key).into_vec()?;
    let private_key = schnorrkel::SecretKey::from_bytes(&private_key_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid Schnorr private key"))?;
    let keypair = Keypair::from(private_key);
    let sign_context = schnorrkel::signing_context(context.as_bytes());

    let signature = keypair.sign(sign_context.bytes(message.as_bytes()));
    Ok(bs58::encode(signature.to_bytes().to_vec()).into_string())
}

#[pyfunction]
fn sign_message(message: String, private_key: String, context: String) -> String {
    _sign_message(message, private_key, context).expect("internal exception")
}

pub fn _generate_keys_with_seed(seed: String) -> anyhow::Result<(String, String)> {
    // Hash the seed to create a 32-byte array
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let seed_hash = hasher.finalize();
    let seed_array: [u8; 32] = seed_hash
        .try_into()
        .expect("Hash algorithm changed output size");

    // Generate the key pair
    let mut rng = StdRng::from_seed(seed_array);
    let keypair = Keypair::generate_with(&mut rng);
    let public_key = bs58::encode(keypair.public.to_bytes()).into_string();
    let private_key = bs58::encode(keypair.secret.to_bytes()).into_string();
    Ok((public_key, private_key))
}

#[pyfunction]
fn generate_keys_with_seed(seed: String) -> (String, String) {
    let (public_key, private_key) = _generate_keys_with_seed(seed).expect("111");
    (public_key, private_key)
}

fn private_to_public(private_key: String) -> anyhow::Result<String> {
    let private_key_bytes = bs58::decode(private_key).into_vec()?;
    let private_key = schnorrkel::SecretKey::from_bytes(&private_key_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid Schnorr private key"))?;
    let public_key = private_key.to_public();
    Ok(bs58::encode(public_key.to_bytes()).into_string())
}

#[pyfunction]
fn private_key_to_public_key(private_key: String) -> String {
    let result = private_to_public(private_key).expect("internal exception");
    result
}


pub fn _verify_signature(
    message: String,
    signature: String,
    public_key: String,
    context: String,
) -> anyhow::Result<bool> {
    let signature_bytes = bs58::decode(signature).into_vec()?;
    let signature = schnorrkel::Signature::from_bytes(&signature_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid Schnorr signature"))?;
    let public_key_bytes = bs58::decode(public_key).into_vec()?;
    let public_key = schnorrkel::PublicKey::from_bytes(&public_key_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid Schnorr public key"))?;

    let sign_context = schnorrkel::signing_context(context.as_bytes());

    Ok(public_key
        .verify(sign_context.bytes(&message.as_bytes()), &signature)
        .is_ok())
}

#[pyfunction]
pub fn verify_signature(
    message: String,
    signature: String,
    public_key: String,
    context: String,
) -> bool {
    _verify_signature(message,signature,public_key,context).expect("internal exception")
}



/// A Python module implemented in Rust.
#[pymodule]
fn warpper_schnorr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_signature, m)?)?;
    m.add_function(wrap_pyfunction!(sign_message, m)?)?;
    m.add_function(wrap_pyfunction!(generate_keys_with_seed, m)?)?;
    m.add_function(wrap_pyfunction!(private_key_to_public_key, m)?)?;
    Ok(())
}
