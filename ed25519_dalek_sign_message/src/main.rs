use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use sha2::Sha512;

fn main() {
    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);

    let fuzzer = sidefuzz::SideFuzz::new(32, #[inline(never)] |message: &[u8]| {
        sidefuzz::black_box(keypair.sign::<Sha512>(message));
    });

    fuzzer.run();
}
