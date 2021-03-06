#![feature(test)]

extern crate test;
extern crate rand;
extern crate sarkara;

use test::Bencher;
use rand::{ Rng, thread_rng, ChaChaRng };
use sarkara::kex::{ NewHope, KeyExchange };
use sarkara::aead::{ Ascon, General, RivGeneral, AeadCipher };
use sarkara::stream::HC256;
use sarkara::auth::HMAC;
use sarkara::hash::Blake2b;

type HHBB = General<HC256, HMAC<Blake2b>, Blake2b>;
type HRHB = RivGeneral<HC256, HMAC<Blake2b>, Blake2b>;


macro_rules! bench_box {
    ( secretbox $name:ident $ty:ident, $len:expr ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use sarkara::secretbox::SecretBox;

            let mut rng = thread_rng();
            let mut key = vec![0; $ty::KEY_LENGTH];
            let mut data = [0; $len];
            rng.fill_bytes(&mut key);
            rng.fill_bytes(&mut data);

            b.bytes = data.len() as u64;
            b.iter(|| {
                let ciphertext = $ty::seal_with_nonce(&mut rng, &key, &data);
                $ty::open(&key, &ciphertext)
            });
        }
    };
    ( sealedbox $name:ident $kty:ident - $cty:ident, $len:expr ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use sarkara::sealedbox::SealedBox;

            let (sk, pk) = $kty::keygen::<ChaChaRng>();
            let mut data = [0; $len];
            thread_rng().fill_bytes(&mut data);

            b.bytes = data.len() as u64;
            b.iter(|| {
                let ciphertext = $cty::seal::<$kty, ChaChaRng>(&pk, &data);
                $cty::open::<$kty>(&sk, &ciphertext)
            });
        }
    }
}

bench_box!(secretbox bench_secretbox_ascon_10   Ascon,      10);
bench_box!(secretbox bench_secretbox_ascon_1k   Ascon,      1024);
bench_box!(secretbox bench_secretbox_ascon_64k  Ascon,      65536);
bench_box!(secretbox bench_secretbox_hhbb_10    HHBB,       10);
bench_box!(secretbox bench_secretbox_hhbb_1k    HHBB,       1024);
bench_box!(secretbox bench_secretbox_hhbb_64k   HHBB,       65536);
bench_box!(secretbox bench_secretbox_hrhb_10    HRHB,       10);
bench_box!(secretbox bench_secretbox_hrhb_1k    HRHB,       1024);
bench_box!(secretbox bench_secretbox_hrhb_64k   HRHB,       65536);

bench_box!(sealedbox bench_sealedbox_ascon_10   NewHope-Ascon,      10);
bench_box!(sealedbox bench_sealedbox_ascon_1k   NewHope-Ascon,      1024);
bench_box!(sealedbox bench_sealedbox_ascon_64k  NewHope-Ascon,      65536);
bench_box!(sealedbox bench_sealedbox_hhbb_10    NewHope-HHBB,       10);
bench_box!(sealedbox bench_sealedbox_hhbb_1k    NewHope-HHBB,       1024);
bench_box!(sealedbox bench_sealedbox_hhbb_64k   NewHope-HHBB,       65536);
bench_box!(sealedbox bench_sealedbox_hrhb_10    NewHope-HRHB,       10);
bench_box!(sealedbox bench_sealedbox_hrhb_1k    NewHope-HRHB,       1024);
bench_box!(sealedbox bench_sealedbox_hrhb_64k   NewHope-HRHB,       65536);
