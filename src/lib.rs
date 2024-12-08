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
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics,
    portable_simd,
    hint_assert_unchecked
)]
mod util;
pub mod day7 {
    use super::util::prelude::*;

    #[inline]
    fn do_(i: &str, search: fn(&[u64], u64) -> bool) -> u64 {
        let mut v = [0u64; 12];
        let mut i = i.as_bytes();
        let mut sum = 0;
        while !i.is_empty() {
            let should = reading::until(&mut i, b':');
            i.skip(1);
            let i = i.take_line().ψ();
            let read = reading::κ(i, &mut v);
            sum += should * search(C! { &v[..read] }, should) as u64;
        }
        sum
    }

    pub fn part1(i: &str) -> impl Display {
        // go backwards for extreme pruning ability
        fn search(nums: &[u64], tv: u64) -> bool {
            match nums {
                &[tail] => tv == tail,
                [head @ .., tail] => {
                    let tail = *tail;
                    unsafe { core::hint::assert_unchecked(tail != 0) };
                    (tv % tail == 0 && search(head, tv / tail))
                        || (tv > tail && search(head, tv - tail))
                }
                [] => shucks!(),
            }
        }
        do_(i, search)
    }

    pub fn part2(i: &str) -> impl Display {
        fn search(nums: &[u64], tv: u64) -> bool {
            match nums {
                &[tail] => tv == tail,
                [head @ .., tail] => {
                    let &d = unsafe {
                        crate::util::powers
                            .get_unchecked((((tail + 0xbf6) & (tail + 0x79c)) >> 10) as usize + 1)
                    };
                    let tail = *tail;
                    (tv % tail == 0 && search(head, tv / tail))
                        || ((tv - tail) % d == 0 && search(head, tv / d))
                        || (tv > tail && search(head, tv - tail))
                }
                [] => shucks!(),
            }
        }
        do_(i, |n, should| search(n, should))
    }
}
