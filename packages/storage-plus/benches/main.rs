use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::Rng;
use std::mem;
use std::time::Duration;

use cw_storage_plus::CwIntKey;

fn bench_signed_int_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("Signed int keys");

    fn k() -> i32 {
        // let k: i32 = 0x42434445;
        // k
        let r = rand::thread_rng().gen_range(i32::MIN..i32::MAX);
        r
    }
    // For the asserts
    let k_check = k();

    type Buf = [u8; mem::size_of::<i32>()];

    group.bench_function("i32 to_cw_bytes: xored (u32) + to_be_bytes", |b| {
        #[inline]
        fn to_cw_bytes(value: &i32) -> Buf {
            (*value as u32 ^ i32::MIN as u32).to_be_bytes()
        }

        assert_eq!(to_cw_bytes(&0), i32::to_cw_bytes(&0));
        assert_eq!(to_cw_bytes(&k_check), i32::to_cw_bytes(&k_check));
        assert_eq!(to_cw_bytes(&-k_check), i32::to_cw_bytes(&-k_check));

        b.iter(|| {
            black_box(to_cw_bytes(&k()));
            black_box(to_cw_bytes(&-k()));
        });
    });

    group.bench_function("i32 to_cw_bytes: xored (u128) + to_be_bytes", |b| {
        #[inline]
        fn to_cw_bytes(value: &i32) -> Buf {
            ((*value as u128 ^ i32::MIN as u128) as i32).to_be_bytes()
        }

        assert_eq!(to_cw_bytes(&0), i32::to_cw_bytes(&0));
        assert_eq!(to_cw_bytes(&k_check), i32::to_cw_bytes(&k_check));
        assert_eq!(to_cw_bytes(&-k_check), i32::to_cw_bytes(&-k_check));

        b.iter(|| {
            black_box(to_cw_bytes(&k()));
            black_box(to_cw_bytes(&-k()));
        });
    });

    group.bench_function("i32 to_cw_bytes: mut to_be_bytes + xor", |b| {
        #[inline]
        fn to_cw_bytes(value: &i32) -> Buf {
            let mut buf = i32::to_be_bytes(*value);
            buf[0] ^= 0x80;
            buf
        }

        assert_eq!(to_cw_bytes(&0), i32::to_cw_bytes(&0));
        assert_eq!(to_cw_bytes(&k_check), i32::to_cw_bytes(&k_check));
        assert_eq!(to_cw_bytes(&-k_check), i32::to_cw_bytes(&-k_check));

        b.iter(|| {
            black_box(to_cw_bytes(&k()));
            black_box(to_cw_bytes(&-k()));
        });
    });

    group.bench_function("i32 to_cw_bytes: branching plus / minus", |b| {
        #[inline]
        fn to_cw_bytes(value: &i32) -> Buf {
            if value >= &0i32 {
                (*value as u32 - i32::MIN as u32).to_be_bytes()
            } else {
                (*value as u32 + i32::MIN as u32).to_be_bytes()
            }
        }

        assert_eq!(to_cw_bytes(&0), i32::to_cw_bytes(&0));
        assert_eq!(to_cw_bytes(&k_check), i32::to_cw_bytes(&k_check));
        assert_eq!(to_cw_bytes(&-k_check), i32::to_cw_bytes(&-k_check));

        b.iter(|| {
            black_box(to_cw_bytes(&k()));
            black_box(to_cw_bytes(&-k()));
        });
    });

    group.finish();
}

fn make_config() -> Criterion {
    Criterion::default()
        .without_plots()
        .measurement_time(Duration::new(5, 0))
        .sample_size(10)
        .configure_from_args()
}

criterion_group!(
    name = signed_int_key;
    config = make_config();
    targets = bench_signed_int_key
);
criterion_main!(signed_int_key);
