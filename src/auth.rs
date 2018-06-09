use rand;
use rand::Rng;
use byteorder::{ByteOrder, BigEndian};
use base64;
use sha1::Sha1;
use openssl::rsa::{Rsa, PKCS1_OAEP_PADDING};
use openssl::bn::BigNum;

static GOOGLE_PUBLIC_KEY: &'static str = "AAAAgMom/1a/v0lblO2Ubrt60J2gcuXSljGFQXgcyZWveWLEwo6prwgi3iJIZdodyhKZQrNWp5nKJ3srRXcUW+F1BD3baEVGcmEgqaLZUNBjm057pKRI16kB0YppeGx5qIQ5QjKzsR8ETQbKLNWgRY0QRNVz34kMJR3P/LgHax/6rmf5AAAAAwEAAQ==";

pub fn create_android_id() -> String {
    let mut bytes = vec![0; 8];
    let mut rng = rand::OsRng::new().unwrap();
    rng.fill_bytes(&mut bytes);
    bytes
        .iter()
        .map(|byte| format!("{:x}", byte))
        .collect()
}

fn decompose(public_key: Vec<u8>) -> (BigNum, BigNum) {
    let i = BigEndian::read_i32(&public_key) as usize;
    let modulus = &public_key[4 .. (4 + i)];
    let j = BigEndian::read_i32(&public_key[i + 4..]) as usize;
    let exponent = &public_key[(i + 8) .. (i + j + 8)];

    let modulus = BigNum::from_slice(modulus).unwrap();
    let exponent = BigNum::from_slice(exponent).unwrap();

    (modulus, exponent)
}

pub fn encrypt_login(email: String, password: String) -> String {
    let mut data = String::new();
    data.push_str(email.as_str());
    data.push_str("\u{0000}");
    data.push_str(password.as_str());

    let public_key = base64::decode(GOOGLE_PUBLIC_KEY).unwrap();

    let mut hash = Sha1::new();
    hash.update(&public_key);
    let digest = hash.digest();
    let digest = digest.bytes();
    let mut signature = vec![0; 1];
    '\x00'.encode_utf8(&mut signature);
    signature.extend(digest[0..4].iter());

    let (modulus, exponent) = decompose(public_key);
    let rsa = Rsa::from_public_components(modulus, exponent).unwrap();
    let mut encrypted : Vec<u8> = vec![0; rsa.size()];
    rsa.public_encrypt(data.as_bytes(), &mut encrypted, PKCS1_OAEP_PADDING).unwrap();

    let mut res: Vec<u8> = vec![];
    res.extend(signature.iter());
    res.extend(encrypted.iter());

    base64::encode(&res)
}
