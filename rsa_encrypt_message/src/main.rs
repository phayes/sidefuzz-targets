use rsa::padding::PaddingScheme;
use rsa::{PublicKey, RSAPrivateKey};

fn main() {
    let mut rng = rand::thread_rng();
    let key = RSAPrivateKey::new(&mut rng, 512).unwrap();

    let fuzzer = sidefuzz::SideFuzz::new(32, |message: &[u8]| {
        let result = sidefuzz::black_box(key.encrypt(
            &mut rand::thread_rng(),
            PaddingScheme::PKCS1v15,
            message,
        ));
        if result.is_err() {
            return Err(());
        }
        Ok(())
    });

    fuzzer.run();
}
