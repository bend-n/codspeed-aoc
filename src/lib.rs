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
pub mod day11 {
    use super::util::prelude::*;

    pub fn part1(i: &str) -> impl Display {
        const LUT: [u32; 10000000] = unsafe {
            std::mem::transmute::<[u8; 10000000 * 4], _>(*include_bytes!("../beeg2-larger-basic"))
        };
        p8(i)
            .map(|stone| C! { LUT[stone as usize] })
            .into_iter()
            .sum::<u32>()
    }

    fn nat<const N: usize>(x: [&u8; N]) -> u32 {
        x.into_iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u32)
    }

    pub fn part2(i: &str) -> impl Display {
        static mut lut: [u64; 10000000] = unsafe {
            std::mem::transmute::<[u8; 10000000 * 8], _>(*include_bytes!("../beeg-basic"))
        };
        p8(i)
            .map(|stone| C! { lut[stone as usize] })
            .into_iter()
            .sum::<u64>()
    }

    fn p8(i: &str) -> [u32; 8] {
        let mut i = &i.as_bytes()[..i.len() - 1];
        use std::mem::MaybeUninit as MU;
        let mut arr = [const { MU::<u32>::uninit() }; 8];
        for j in 0..7 {
            arr[j].write(match i {
                [一, b' ', rest @ ..] => {
                    i = rest;
                    nat([一])
                }
                [一, 二, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二])
                }
                [一, 二, 三, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二, 三])
                }
                [一, 二, 三, 四, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二, 三, 四])
                }
                [一, 二, 三, 四, 五, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二, 三, 四, 五])
                }
                [一, 二, 三, 四, 五, 六, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二, 三, 四, 五, 六])
                }
                [一, 二, 三, 四, 五, 六, 七, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二, 三, 四, 五, 六, 七])
                }
                [一, 二, 三, 四, 五, 六, 七, 八, b' ', rest @ ..] => {
                    i = rest;
                    nat([一, 二, 三, 四, 五, 六, 七, 八])
                }
                _ => shucks!(),
            });
        }
        arr[7].write(match i {
            [一] => nat([一]),
            [一, 二] => nat([一, 二]),
            [一, 二, 三] => nat([一, 二, 三]),
            [一, 二, 三, 四] => nat([一, 二, 三, 四]),
            [一, 二, 三, 四, 五] => nat([一, 二, 三, 四, 五]),
            [一, 二, 三, 四, 五, 六] => nat([一, 二, 三, 四, 五, 六]),
            [一, 二, 三, 四, 五, 六, 七] => nat([一, 二, 三, 四, 五, 六, 七]),
            [一, 二, 三, 四, 五, 六, 七, 八] => nat([一, 二, 三, 四, 五, 六, 七, 八]),
            _ => shucks!(),
        });
        unsafe { MU::array_assume_init(arr) }
    }
}

// static mut map: std::sync::OnceLock<HashMap<(u64, u8), u64>> = std::sync::OnceLock::new();
// fn rocc(one: u64, iters: u8) -> u64 {
//     if let Some(&x) = unsafe { map.get_mut().ψ().get(&(one, iters)) } {
//         return x;
//     }
//     let answer = {
//         match iters.checked_sub(1) {
//             Some(iters) if one == 0 => rocc(1, iters),
//             Some(iters)
//                 if let ͱ = one.ͱ() as usize
//                     && ͱ % 2 == 0 =>
//             {
//                 let pow = util::powers[ͱ / 2];
//                 rocc(one / pow, iters) + rocc(one % pow, iters)
//             }
//             Some(iters) if iters > 1 && (one * 2024).ͱ() % 2 == 1 => {
//                 // skip
//                 let one = one * 2024 * 2024;
//                 let pow = util::powers[one.ͱ() as usize / 2];
//                 rocc(one / pow, iters - 2) + rocc(one % pow, iters - 2)
//             }
//             Some(iters) => rocc(one * 2024, iters),
//             None => 1,
//         }
//     };
//     unsafe { map.get_mut().ψ().insert((one, iters), answer) };
//     answer
// }
