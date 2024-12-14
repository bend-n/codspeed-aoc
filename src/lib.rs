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
pub mod day13 {
    use super::util;
    use super::util::prelude::*;
    const SIZE: usize = 140;

    fn two([a, b]: [u8; 2]) -> i64 {
        (a - b'0') as i64 * 10 + (b - b'0') as i64
    }

    #[no_mangle]
    pub fn part2(i: &str) -> impl Display {
        let mut i = i.as_bytes();
        // let i = i.as_chunks_unchecked::<{ SIZE + 1 }>();
        // let get = |x, y| (x < SIZE && y < SIZE).then(|| i[y][x]);
        let mut sum = 0;
        for _ in 0..340 {
            let a_x = two(util::nail(C! { &i["button a: x+".len()..]}));
            let a_y = two(util::nail(C! { &i["button a: x+55, y+".len()..]}));
            let b_x = two(util::nail(
                C! { &i["button a: x+55, y+jj\nbutton b: x+".len()..]},
            ));
            let b_y = two(util::nail(
                C! { &i["button a: x+55, y+jj\nbutton b: x+44, y+".len()..]},
            ));
            i.skip("button a: x+55, y+jj\nbutton b: x+44, y+jj\nprize: x=".len());
            let p_x: i64 = reading::until::<i64>(&mut i, b',') + 10000000000000;
            i.skip(" y=".len());
            let p_y: i64 = reading::until::<i64>(&mut i, b'\n') + 10000000000000;
            #[inline]
            fn dmod(a: i64, b: i64) -> (i64, i64) {
                unsafe {
                    (
                        core::intrinsics::unchecked_div(a, b),
                        core::intrinsics::unchecked_rem(a, b),
                    )
                }
            }
            // a_x * α + b_x * β = p_x
            // a_y * α + b_y * β = p_y
            let (β, ok) = dmod(
                a_y * p_x - a_x * p_y, //
                a_y * b_x - a_x * b_y,
            );
            if ok == 0 {
                let (α, ok) = {
                    dmod(
                        b_y * p_x - b_x * p_y, //
                        a_x * b_y - a_y * b_x,
                    )
                };
                if ok == 0 {
                    sum += 3 * α + β;
                }
            }

            if i.is_empty() {
                break;
            }
            i.skip(1);
        }
        sum
    }

    #[no_mangle]
    pub fn part1(i: &str) -> impl Display {
        let mut i = i.as_bytes();
        // let i = i.as_chunks_unchecked::<{ SIZE + 1 }>();
        // let get = |x, y| (x < SIZE && y < SIZE).then(|| i[y][x]);
        let mut sum = 0;
        for _ in 0..340 {
            let a_x = two(util::nail(C! { &i["button a: x+".len()..]}));
            let a_y = two(util::nail(C! { &i["button a: x+55, y+".len()..]}));
            let b_x = two(util::nail(
                C! { &i["button a: x+55, y+jj\nbutton b: x+".len()..]},
            ));
            let b_y = two(util::nail(
                C! { &i["button a: x+55, y+jj\nbutton b: x+44, y+".len()..]},
            ));
            i.skip("button a: x+55, y+jj\nbutton b: x+44, y+jj\nprize: x=".len());
            let p_x: i64 = reading::until(&mut i, b',');
            i.skip(" y=".len());
            let p_y: i64 = reading::until(&mut i, b'\n');
            #[inline]
            fn dmod(a: i64, b: i64) -> (i64, i64) {
                unsafe {
                    (
                        core::intrinsics::unchecked_div(a, b),
                        core::intrinsics::unchecked_rem(a, b),
                    )
                }
            }
            // a_x * α + b_x * β = p_x
            // a_y * α + b_y * β = p_y
            let (β, ok) = dmod(
                a_y * p_x - a_x * p_y, //
                a_y * b_x - a_x * b_y,
            );
            if ok == 0 {
                let α = unsafe {
                    core::intrinsics::unchecked_div(
                        b_y * p_x - b_x * p_y, //
                        a_x * b_y - a_y * b_x,
                    )
                };
                sum += 3 * α + β;
            }

            if i.is_empty() {
                break;
            }
            i.skip(1);
        }
        sum
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
