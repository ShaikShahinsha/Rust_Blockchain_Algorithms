// use ecdsa::{Signature, VerifyingKey, SigningKey};
// use p256::NistP256;
// use p256::elliptic_curve::rand_core::OsRng;

// fn main() {
//     // Generate a random signing key
//     let signing_key = SigningKey::<NistP256>::random(&mut OsRng);

//     // Derive the corresponding verifying key (public key)
//     let verifying_key = VerifyingKey::from(&signing_key);

//     // Message to be signed
//     let message = b"ECDSA proves this message is authentic";

//     // Sign the message
//     let signature: Signature<NistP256> = signing_key.sign(message);

//     // Verify the signature
//     assert!(verifying_key.verify(message, &signature).is_ok());

//     println!("Signature is valid!");
// }

use ecdsa::{Signature, VerifyingKey, SigningKey};
use ecdsa::signature::{Signer,Verifier};
use p256::NistP256;
use p256::elliptic_curve::rand_core::OsRng;

fn main() {
    // Generate a random signing key
    let signing_key = SigningKey::<NistP256>::random(&mut OsRng);

    // Derive the corresponding verifying key (public key)
    let verifying_key = VerifyingKey::from(&signing_key);

    // Message to be signed
    let message = b"ECDSA proves this message is authentic";

    // Sign the message
    let signature = Signer.sign(&signing_key, message);

    // Verify the signature
    //assert!(verifying_key.verify(message, &signature).is_ok());
    let result = Verifier::verify(&verifying_key, message, &signature);

    assert!(result.is_ok());
    println!("Signature is valid!");
}

/*

use ecdsa::{Signature, VerifyingKey, SigningKey};
use p256::NistP256;
use p256::elliptic_curve::rand_core::OsRng;

fn main() {
    // Generate a random signing key
    let signing_key = SigningKey::<NistP256>::random(&mut OsRng);

    // Derive the corresponding verifying key (public key)
    let verifying_key = VerifyingKey::from(&signing_key);

    // Message to be signed
    let message = b"ECDSA proves this message is authentic";

    // Sign the message
    let signature = signing_key.sign(message);

    // Verify the signature
    assert!(verifying_key.verify(message, &signature).is_ok());

    println!("Signature is valid!");
}

*/