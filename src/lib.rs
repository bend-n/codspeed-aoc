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
pub mod day12 {
    use super::util::prelude::*;

    const SIZE: usize = 140;
    #[no_mangle]
    pub fn part2(i: &str) -> impl Display {
        fn explore(
            (x, y): (usize, usize),
            handled: &mut [[bool; 140]; 140],
            char: u8,
            get: &mut impl FnMut(usize, usize) -> Option<u8>,
            tot: &mut u32,
            count: &mut u32,
        ) {
            if get(x, y) == Some(char) && handled[y][x] == false {
                handled[y][x] = true;
                // αbβ
                // a.c
                // γdδ
                let α = get(x.wrapping_sub(1), y.wrapping_sub(1)) != Some(char);
                let β = get(x.wrapping_add(1), y.wrapping_sub(1)) != Some(char);
                let γ = get(x.wrapping_sub(1), y.wrapping_add(1)) != Some(char);
                let δ = get(x.wrapping_add(1), y.wrapping_add(1)) != Some(char);

                let a = get(x.wrapping_sub(1), y) != Some(char);
                let b = get(x, y.wrapping_sub(1)) != Some(char);
                let c = get(x.wrapping_add(1), y) != Some(char);
                let d = get(x, y.wrapping_add(1)) != Some(char);
                fn u(a: bool) -> u32 {
                    a as u32
                }
                // *tot += u(a) + u(b) + u(c) + u(d);

                *tot += u(a & b) + u(b & c) + u(c & d) + u(a & d);
                *tot += u(!a & !b & α) + u(!b & !c & β) + u(!c & !d & δ) + u(!a & !d & γ);
                *count += 1;

                explore((x.wrapping_sub(1), y), handled, char, get, tot, count);
                explore((x + 1, y), handled, char, get, tot, count);
                explore((x, y + 1), handled, char, get, tot, count);
                explore((x, y.wrapping_sub(1)), handled, char, get, tot, count);
            }
        }

        let grid = unsafe { i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>() };
        let handled = &mut [[false; SIZE]; SIZE];
        let mut get = |x: usize, y: usize| {
            unsafe { core::hint::assert_unchecked(grid.len() == SIZE) };
            (x < SIZE && y < SIZE).then(|| grid[y][x])
        };
        (0..SIZE)
            .flat_map(move |y| (0..SIZE).map(move |x| (x, y)))
            .filter_map(|(x, y)| {
                let mut sides = 0;
                let mut area = 0;
                (!handled[y][x]).then(|| {
                    let char = C! { grid[y][x]};
                    explore((x, y), handled, char, &mut get, &mut sides, &mut area);
                    area * sides
                })
            })
            .sum::<u32>()
    }
    #[no_mangle]
    pub fn part1(i: &str) -> impl Display {
        fn explore(
            (x, y): (usize, usize),
            handled: &mut [[bool; 140]; 140],
            char: u8,
            get: &mut impl FnMut(usize, usize) -> Option<u8>,
            tot: &mut u32,
            count: &mut u32,
        ) {
            if get(x, y) == Some(char) && handled[y][x] == false {
                handled[y][x] = true;
                // αbβ
                // a.c
                // γdδ
                let a = get(x.wrapping_sub(1), y) != Some(char);
                let b = get(x, y.wrapping_sub(1)) != Some(char);
                let c = get(x.wrapping_add(1), y) != Some(char);
                let d = get(x, y.wrapping_add(1)) != Some(char);
                fn u(a: bool) -> u32 {
                    a as u32
                }
                *tot += u(a) + u(b) + u(c) + u(d);

                // *tot += u(a & b) + u(b & c) + u(c & d) + u(a & d);
                // *tot += u(!a & !b & α) + u(!b & !c & β) + u(!c & !d & δ) + u(!a & !d & γ);
                *count += 1;

                explore((x.wrapping_sub(1), y), handled, char, get, tot, count);
                explore((x + 1, y), handled, char, get, tot, count);
                explore((x, y + 1), handled, char, get, tot, count);
                explore((x, y.wrapping_sub(1)), handled, char, get, tot, count);
            }
        }

        let grid = unsafe { i.as_bytes().as_chunks_unchecked::<{ SIZE + 1 }>() };
        let handled = &mut [[false; SIZE]; SIZE];
        let mut get = |x: usize, y: usize| {
            unsafe { core::hint::assert_unchecked(grid.len() == SIZE) };
            (x < SIZE && y < SIZE).then(|| grid[y][x])
        };
        (0..SIZE)
            .flat_map(move |y| (0..SIZE).map(move |x| (x, y)))
            .filter_map(|(x, y)| {
                let mut sides = 0;
                let mut area = 0;
                (!handled[y][x]).then(|| {
                    let char = C! { grid[y][x]};
                    explore((x, y), handled, char, &mut get, &mut sides, &mut area);
                    area * sides
                })
            })
            .sum::<u32>()
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
