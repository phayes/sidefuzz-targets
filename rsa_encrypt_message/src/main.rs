use rsa::padding::PaddingScheme;
use rsa::{PublicKey, RSAPrivateKey};

fn main() {
    let mut rng = rand::thread_rng();
    let key = RSAPrivateKey::new(&mut rng, 512).unwrap();

    let fuzzer = sidefuzz::SideFuzz::new(32, #[inline(never)]
    |message: &[u8]| {
        sidefuzz::black_box(
            key.encrypt(&mut rand::thread_rng(), PaddingScheme::PKCS1v15, message)
                .unwrap(),
        );
    });

    fuzzer.run();
}
