use lazy_static::*;
use num_bigint_dig::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;
use rsa::{RSAPrivateKey, RSAPublicKey};

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
    let c = BigUint::from_bytes_be(input);

    let key: RSAPublicKey = KEY.clone().into();
    sidefuzz::black_box(rsa::internals::encrypt(&key, &c));
}
