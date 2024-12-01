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
    core_intrinsics
)]
mod util;
pub mod day1 {
    use crate::util::prelude::*;
    pub fn part1(input: &str) -> impl std::fmt::Display {
        static mut a: [u32; 1000] = [0; 1000];
        static mut b: [u32; 1000] = [0; 1000];

        input
            .行()
            .map(crate::util::nail::<{ 5 + 5 + 3 }>)
            .map(|x| (reading::all(&x[0..5]), reading::all(&x[8..13])))
            .enumerate()
            .for_each(|(i, (x, y))| {
                C! { a[i] = x };
                C! { b[i] = y };
            });
        unsafe {
            a.sort_unstable();
            b.sort_unstable();
            a.iter()
                .copied()
                .zip(b)
                .map(|(x, y)| (x as i32 - y as i32).abs() as u32)
                .sum::<u32>()
        }
    }

    pub fn part2(i: &str) -> impl Display {
        static mut a: [u32; 1000] = [0; 1000];
        let mut map = HashMap::<u32, u32>::with_capacity_and_hasher(
            1000,
            rustc_hash::FxBuildHasher::default(),
        );
        i.行()
            .map(crate::util::nail::<{ 5 + 5 + 3 }>)
            .map(|x| (reading::all(&x[0..5]), reading::all(&x[8..13])))
            .enumerate()
            .for_each(|(i, (x, y))| {
                C! { a[i] = x };
                map.entry(y).and_modify(|x| *x += 1).or_insert(1);
            });
        unsafe {
            a.iter()
                .map(|x| x * map.get(x).copied().unwrap_or(0))
                .sum::<u32>()
        }
    }
}
