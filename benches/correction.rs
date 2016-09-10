#![feature(test)]

extern crate ghoti;
extern crate test;

use ghoti::correction;
use test::Bencher;


#[bench]
fn bench_existing_word(b: &mut Bencher) {
    b.iter(|| correction("the"));
}

#[bench]
fn bench_single_edit(b: &mut Bencher) {
    b.iter(|| correction("speling"));
}

#[bench]
fn bench_double_edit(b: &mut Bencher) {
    b.iter(|| correction("inconvient"));
}
