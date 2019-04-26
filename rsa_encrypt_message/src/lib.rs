use rand::chacha::ChaChaRng;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey};

use sidefuzz::black_box;
use std::slice;

#[no_mangle]
pub extern "C" fn len() -> i32 {
    return 32;
}

#[no_mangle]
pub extern "C" fn sidefuzz(ptr: i32, len: i32) {
    let input: &[u8] = unsafe { slice::from_raw_parts(ptr as _, len as _) };

    #[allow(deprecated)]
    let mut rng = ChaChaRng::new_unseeded();
    let key = RSAPrivateKey::new(&mut rng, 256).unwrap();

    black_box(
        key.encrypt(&mut rng, PaddingScheme::PKCS1v15, input)
            .unwrap(),
    );
}