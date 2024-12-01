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

    pub fn part1(i: &str) -> impl Display {
        static mut a: [i32; 1000] = [0; 1000];
        static mut b: [i32; 1000] = [0; 1000];

        unsafe {
            i.as_bytes()
                .as_chunks_unchecked::<14>()
                .into_iter()
                .map(|x| (reading::all(&x[0..5]), reading::all(&x[8..13])))
                .enumerate()
                .for_each(|(i, (x, y))| {
                    *a.get_unchecked_mut(i) = x;
                    *b.get_unchecked_mut(i) = y;
                });

            a.sort_unstable();
            b.sort_unstable();
            a.iter()
                .copied()
                .zip(b)
                .map(|(x, y)| (x - y).abs())
                .sum::<i32>()
        }
    }

    pub fn part2(i: &str) -> impl Display {
        static mut a: [u32; 1000] = [0; 1000];
        static mut map: [u32; 100000] = [0; 100000];
        let i = i.as_bytes();

        unsafe {
            let x = C! { &i[..14] };
            let (x, y) = (reading::all(&x[0..5]), reading::all::<u32>(&x[8..13]));
            *a.get_unchecked_mut(0) = x;
            *map.get_unchecked_mut(y as usize) += 1;

            for n in 1..1000 {
                let x = crate::util::reading::八(
                    u64::from_le_bytes(crate::util::nail::<8>(&i[n * 14 - 3..]))
                        & 0xffffffffff000000,
                );
                let y = crate::util::reading::八(u64::from_le_bytes(crate::util::nail::<8>(
                    &i[n * 14 + 5..],
                )));
                *a.get_unchecked_mut(n) = x as u32;
                *map.get_unchecked_mut(y as usize) += 1;
            }
            a.iter()
                .copied()
                .map(|x| x * map.get_unchecked(x as usize))
                .sum::<u32>()
        }
    }
}
