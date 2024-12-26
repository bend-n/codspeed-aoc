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
pub mod day25 {
    use super::util::prelude::*;
    use std::simd::prelude::*;
    pub fn part1(x: &str) -> u32 {
        unsafe {
            let i = x.as_bytes().as_ptr();
            static mut keys: [u32; 256] = [u32::MAX; 256];
            let mut ki = 0;
            static mut locks: [u32; 250] = [u32::MAX; 250];
            let mut li = 0;
            for j in 0..500 {
                let acc = i
                    .add(j * (7 * 6 + 1) + 3)
                    .cast::<u8x32>()
                    .read_unaligned()
                    .simd_eq(Simd::splat(b'#'))
                    .to_bitmask() as u32;
                if acc & 1 == 0 {
                    C! { keys[ki] = acc };
                    ki += 1;
                } else {
                    C! { locks[li] = acc };
                    li += 1;
                }
            }
            let mut sum = i32x8::splat(0);
            for l in locks {
                for &k in keys.as_chunks_unchecked::<8>() {
                    sum += (u32x8::splat(l) & u32x8::from_array(k))
                        .simd_eq(Simd::splat(0))
                        .to_int();
                }
            }
            -sum.reduce_sum() as u32
        }
    }
    pub fn part2(_: &str) -> u32 {
        0
    }
}
