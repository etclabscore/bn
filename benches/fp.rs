// TODO: later move all these benchmarks to within the code itself.

#![feature(test)]
extern crate test;
extern crate rand;
extern crate bn;

use bn::*;

#[bench]
fn fr_addition(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] + v2[ctr % SAMPLES]
    });
}

#[bench]
fn fr_subtraction(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] - v2[ctr % SAMPLES]
    });
}

#[bench]
fn fr_multiplication(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] * v2[ctr % SAMPLES]
    });
}

#[bench]
fn fr_inverses(b: &mut test::Bencher) {
	const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES].inverse()
    });
}

#[bench]
fn fq_addition(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] + v2[ctr % SAMPLES]
    });
}

#[bench]
fn fq_subtraction(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] - v2[ctr % SAMPLES]
    });
}

#[bench]
fn fq_multiplication(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] * v2[ctr % SAMPLES]
    });
}

#[bench]
fn fq_inverses(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES].inverse()
    });
}

#[bench]
fn fq2_multiplication(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] * v2[ctr % SAMPLES]
    });
}

#[bench]
fn fq2_addition(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] + v2[ctr % SAMPLES]
    });
}

#[bench]
fn fq2_subtraction(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] - v2[ctr % SAMPLES]
    });
}

#[bench]
fn fq2_inverses(b: &mut test::Bencher) {
    const SAMPLES: usize = 1000;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| Fq2::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES].inverse()
    });
}

#[bench]
fn g1_addition(b: &mut test::Bencher) {
    const SAMPLES: usize = 100;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| G1::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| G1::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] + v2[ctr % SAMPLES]
    });
}

#[bench]
fn g1_subtraction(b: &mut test::Bencher) {
    const SAMPLES: usize = 100;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| G1::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| G1::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] - v2[ctr % SAMPLES]
    });
}

#[bench]
fn g1_scalar_multiplication(b: &mut test::Bencher) {
    const SAMPLES: usize = 100;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| G1::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] * v2[ctr % SAMPLES]
    });
}

#[bench]
fn g2_addition(b: &mut test::Bencher) {
    const SAMPLES: usize = 100;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| G2::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| G2::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] + v2[ctr % SAMPLES]
    });
}

#[bench]
fn g2_subtraction(b: &mut test::Bencher) {
    const SAMPLES: usize = 100;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| G2::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| G2::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] - v2[ctr % SAMPLES]
    });
}

#[bench]
fn g2_scalar_multiplication(b: &mut test::Bencher) {
    const SAMPLES: usize = 100;

    let rng = &mut rand::thread_rng();

    let v1: Vec<_> = (0..SAMPLES).map(|_| G2::random(rng)).collect();
    let v2: Vec<_> = (0..SAMPLES).map(|_| Fr::random(rng)).collect();

    let mut ctr = 0;

    b.iter(|| {
        ctr += 1;

        v1[ctr % SAMPLES] * v2[ctr % SAMPLES]
    });
}
