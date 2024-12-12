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
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics,
    portable_simd,
    hint_assert_unchecked
)]
mod util;
pub mod day10 {
    use super::util::prelude::*;

    #[no_mangle]
    pub fn part1(i: &str) -> impl Display {
        let i = i.as_bytes();
        let size = memchr::memchr(b'\n', i).ψ();
        let max = size * (size + 1);
        unsafe { core::hint::assert_unchecked(i.len() == max) };
        let get = |j: u16| (j < max as u16).then(|| i[j as usize]);

        // fimg::Image::<Vec<u8>, 1>::build(SIZE as _, SIZE as _)
        //     .buf(
        //         grid.iter()
        //             .flat_map(|x| &x[..SIZE])
        //             .map(|x| (((*x - b'0') as f32 / 10.0) * 255.0) as u8)
        //             .collect_vec(),
        //     )
        //     .scale::<fimg::scale::Nearest>(SIZE as u32 * 8, SIZE as u32 * 8)
        //     .save("hmm.png");
        let mut tot = 0;
        pub fn p1(
            start: u16,
            graph: &mut impl Fn(u16) -> [u16; 4],
            ok: &mut impl Fn(u16, u16) -> bool,
            end: &mut impl Fn(u16) -> bool,
            reached: &mut [u8],
        ) -> u16 {
            if end(start) && reached[start as usize] == 0 {
                reached[start as usize] = 1;
                return 1;
            } else {
                graph(start)
                    .into_iter()
                    .map(|x| {
                        if ok(start, x) {
                            p1(x, graph, ok, end, reached)
                        } else {
                            0
                        }
                    })
                    .sum()
            }
        }

        let mut dp = vec![0; max];
        // let mut dp = vec![u16::MAX; max];
        for i in memchr::memchr_iter(b'9', i) {
            dp.fill(0);
            tot += p1(
                i as u16,
                &mut |i| {
                    [
                        i.wrapping_add(1),
                        i.wrapping_sub(1),
                        i.wrapping_add(size as u16 + 1),
                        i.wrapping_sub(size as u16 + 1),
                    ]
                },
                &mut |i1, i2| match get(i2) {
                    Some(x) => get(i1).ψ().wrapping_sub(x) == 1,
                    None => false,
                },
                &mut |i| get(i) == Some(b'0'),
                &mut dp,
            );
        }
        tot
    }

    #[no_mangle]
    pub fn part2(i: &str) -> impl Display {
        let i = i.as_bytes();
        let size = memchr::memchr(b'\n', i).ψ();
        let max = size * (size + 1);
        unsafe { core::hint::assert_unchecked(i.len() == max) };
        let get = |j: u16| (j < max as u16).then(|| i[j as usize]);
        let mut tot = 0;

        pub fn p2(
            start: u16,
            graph: &mut impl Fn(u16) -> [u16; 4],
            ok: &mut impl Fn(u16, u16) -> bool,
            end: &mut impl Fn(u16) -> bool,
            dp: &mut [u16],
        ) -> u16 {
            if end(start) {
                return 1;
            } else {
                graph(start)
                    .into_iter()
                    .map(|x| {
                        if ok(start, x) {
                            #[allow(irrefutable_let_patterns)]
                            if let x = dp[x as usize]
                                && x != 0xffff
                            {
                                return x;
                            }
                            let n = p2(x, graph, ok, end, dp);
                            dp[x as usize] = n;
                            n
                        } else {
                            0
                        }
                    })
                    .sum()
            }
        }
        let mut dp = vec![u16::MAX; max];
        for i in memchr::memchr_iter(b'9', i) {
            tot += p2(
                i as u16,
                &mut |i| {
                    [
                        i.wrapping_add(1),
                        i.wrapping_sub(1),
                        i.wrapping_add(size as u16 + 1),
                        i.wrapping_sub(size as u16 + 1),
                    ]
                },
                &mut |i1, i2| match get(i2) {
                    Some(x) => get(i1).ψ().wrapping_sub(x) == 1,
                    None => false,
                },
                &mut |i| get(i) == Some(b'0'),
                &mut dp,
            );
        }
        tot
    }
}
