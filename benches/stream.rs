#![feature(test)]

extern crate test;
extern crate rand;
#[macro_use] extern crate sarkara;

use test::Bencher;
use sarkara::utils::Bytes;
use sarkara::stream::{ HC128, StreamCipher };


#[bench]
fn bench_stream_hc128(b: &mut Bencher) {
    let (key, nonce) = (
        Bytes::random(HC128::key_length()),
        Bytes::random(HC128::nonce_length())
    );
    let data = rand!(bytes 4096);
    b.bytes = data.len() as u64;
    b.iter(|| HC128::new(&key).process(&nonce, &data));
}
