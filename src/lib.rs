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
pub mod day21 {
    use super::util;
    use super::util::prelude::*;

    extern crate test;
    static P2: [u64; 592138] = {
        let mut l = [0; 592138];
        include!("../lut");
        l
    };
    static P1: [u64; 592138] = {
        let mut l = [0; 592138];
        include!("../lut2");
        l
    };
    #[inline(always)]
    pub fn part1(x: &str) -> impl Display {
        let i = x.as_bytes();
        let codes: &[[u8; 5]; 5] = unsafe { i.as_chunks_unchecked::<5>().try_into().ψ() };
        codes
            .into_iter()
            .map(|x| C! { P1[u32::from_le_bytes(x[..4].try_into().ψ()) as usize & 0x0f0f0f] })
            .sum::<u64>()
    }
    #[inline(always)]
    pub fn part2(x: &str) -> impl Display {
        let i = x.as_bytes();
        let codes: &[[u8; 5]; 5] = unsafe { i.as_chunks_unchecked::<5>().try_into().ψ() };
        codes
            .into_iter()
            .map(|x| C! { P2[u32::from_le_bytes(x[..4].try_into().ψ()) as usize & 0x0f0f0f] })
            .sum::<u64>()
    }
}
