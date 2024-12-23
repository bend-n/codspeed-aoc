#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    incomplete_features
)]
#![feature(
    slice_swap_unchecked,
    iter_repeat_n,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    vec_into_raw_parts,
    array_windows,
    slice_take,
    test,
    maybe_uninit_array_assume_init,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics,
    portable_simd,
    hint_assert_unchecked
)]
mod util;
pub mod day22 {
    use super::util;
    use super::util::prelude::*;

    pub fn mod10(a: u32) -> u32 {
        const D: u32 = 10;
        const M: u64 = (u64::MAX / D as u64) + 1;
        (M.wrapping_mul(a as u64) as u128 * D as u128 >> 64) as u32
    }

    fn next(mut x: u32) -> u32 {
        x ^= (x * 64) & 16777215;
        x ^= x / 32;
        x ^= (x * 2048) & 16777215;
        x
    }

    #[rustfmt::skip]
// 8051
fn next2000(n: u32) -> u32 {
    let n = n as u64;
    let m = n | n << 24;
    let r = (m & 0x61a765) ^ (m >> 1 & 0xc2f82d) ^ (m >> 2 & 0x286d53) ^ (m >> 3 & 0x44f679)
    ^ (m >> 4 & 0x4d6be8) ^ (m >> 5 & 0x118005) ^ (m >> 6 & 0x5f19f2) ^ (m >> 7 & 0xf03667)
    ^ (m >> 8 & 0xcea653) ^ (m >> 9 & 0xafa201) ^ (m >> 10 & 0xfd0d29) ^ (m >> 11 & 0x949200)
    ^ (m >> 12 & 0x49a994) ^ (m >> 13 & 0x21673) ^ (m >> 14 & 0xb4c5bf) ^ (m >> 15 & 0x1e0aaf)
    ^ (m >> 16 & 0x7cab00) ^ (m >> 17 & 0x95ba48) ^ (m >> 18 & 0x49f04c) ^ (m >> 19 & 0x9a8320)
    ^ (m >> 20 & 0xb69d39) ^ (m >> 21 & 0x6a2085) ^ (m >> 22 & 0xd13c84) ^ (m >> 23 & 0x1c9e15);
    r as u32

}

    fn batch((mut x, j): (u32, u16), map: &mut [u16; 130321], seen: &mut Vec<u16>) {
        let 〇 = x;
        let 一 = next(x);
        let 二 = next(一);
        let 三 = next(二);
        let mut ⅰ;
        let [mut ⅱ, mut ⅲ, mut ⅳ] =
            [[〇, 一], [一, 二], [二, 三]].map(|[a, b]| (9 + mod10(b) - mod10(a)) as u32);

        x = 三;
        let mut l = mod10(三);
        for _ in 3..2000 {
            x = next(x);
            let p = mod10(x);
            (ⅰ, ⅱ, ⅲ, ⅳ) = (ⅱ, ⅲ, ⅳ, (9 + p - l) as u32);
            let i = (ⅰ * 19 * 19 * 19 + ⅱ * 19 * 19 + ⅲ * 19 + ⅳ) as usize;
            if seen[i] != j {
                map[i] += p as u16;
                seen[i] = j;
            }
            l = p;
        }
    }

    static retval: std::sync::Mutex<[u16; 130321]> = std::sync::Mutex::new([0; 130321]);
    #[inline(always)]
    pub fn part2(x: &str) -> u16 {
        let mut i = x.as_bytes();
        let ints = reading::Integer::<u32>::new(&mut i);
        let mut chunks = ints.array_chunks::<128>();
        std::thread::scope(|scope| {
            for chunk in chunks.by_ref() {
                scope.spawn(move || {
                    let mut map = [0; 130321];
                    let mut seen = vec![0; 130321];
                    for elem in chunk.into_iter().ι1::<u16>() {
                        batch(elem, &mut map, &mut seen);
                    }
                    let mut upmap = retval.lock().ψ();
                    for (a, b) in map.into_iter().zip(upmap.iter_mut()) {
                        *b += a;
                    }

                    drop(upmap);
                });
            }

            let mut map = [0; 130321];
            let mut seen = vec![0; 130321];
            for elem in chunks.into_remainder().into_iter().flatten().ι1::<u16>() {
                batch(elem, &mut map, &mut seen);
            }
            let mut upmap = retval.lock().ψ();
            for (a, b) in map.into_iter().zip(upmap.iter_mut()) {
                *b += a;
            }
        });
        retval.lock().ψ().into_iter().max().ψ()
    }

    use std::simd::prelude::*;
    #[inline(always)]
    pub fn part1(x: &str) -> u64 {
        let mut x = x.as_bytes();
        let mut i = reading::Integer::<u32>::new(&mut x).array_chunks::<8>();
        i.by_ref()
            .map(|x| u32x8::from_array(x.map(next2000)))
            .fold(u32x8::splat(0), |acc, x| acc + x)
            .cast::<u64>()
            .reduce_sum()
            + i.into_remainder()
                .map_or(0, |x| x.map(next2000).map(|x| x as u64).sum())
    }
}
