use lazy_static::*;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey};

lazy_static! {
    static ref KEY: RSAPrivateKey = {
        #[allow(deprecated)]
        let mut rng = ChaChaRng::from_seed([0; 32]);
        RSAPrivateKey::new(&mut rng, 256).unwrap()
    };
}

#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(16);

    #[allow(deprecated)]
    let mut rng = ChaChaRng::from_seed([0; 32]);

    sidefuzz::black_box(
        KEY.encrypt(&mut rng, PaddingScheme::PKCS1v15, input)
            .unwrap(),
    );
}
