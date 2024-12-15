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
pub mod day14 {
    use super::util;
    use super::util::prelude::*;
    use std::ops::Neg;
    const W: i32 = 101;
    const H: i32 = 103;
    #[no_mangle]
    pub fn run(i: &str) -> impl Display {
        let mut grids = [0; 4];
        let mut p = i.as_bytes().as_ptr();
        unsafe {
            for j in 0..500 {
                p = p.add(2);
                let px = match std::slice::from_raw_parts(p, 3) {
                    [a, b',', _] => {
                        p = p.add(2);
                        *a - b'0'
                    }
                    [a, b, b','] => {
                        p = p.add(3);
                        (*a - b'0') * 10 + (*b - b'0')
                    }
                    [a, b, c] => {
                        p = p.add(4);
                        ((*a - b'0') * 10 + (*b - b'0')) * 10 + (*c - b'0')
                    }
                    _ => shucks!(),
                };
                let py = match std::slice::from_raw_parts(p, 3) {
                    [a, b' ', _] => {
                        p = p.add(2 + 2);
                        *a - b'0'
                    }
                    [a, b, b' '] => {
                        p = p.add(3 + 2);
                        (*a - b'0') * 10 + (*b - b'0')
                    }
                    [a, b, c] => {
                        p = p.add(4 + 2);
                        ((*a - b'0') * 10 + (*b - b'0')) * 10 + (*c - b'0')
                    }
                    _ => shucks!(),
                };
                let vx = match std::slice::from_raw_parts(p, 3) {
                    [a, b',', _] => {
                        p = p.add(2);
                        (*a - b'0') as i32
                    }
                    [b'-', a, b','] => {
                        p = p.add(3);
                        ((*a - b'0') as i32).neg()
                    }
                    [b'-', a, b] => {
                        p = p.add(4);
                        (((*a - b'0') * 10 + (*b - b'0')) as i32).neg()
                    }
                    [a, b, b','] => {
                        p = p.add(3);
                        ((*a - b'0') * 10 + (*b - b'0')) as i32
                    }
                    _ => shucks!(),
                };
                let vy = match std::slice::from_raw_parts(p, 3) {
                    [a, b'\n', _] => {
                        p = p.add(2);
                        (*a - b'0') as i32
                    }
                    [b'-', a, b'\n'] => {
                        p = p.add(3);
                        ((*a - b'0') as i8).neg() as i32
                    }
                    [b'-', a, b] => {
                        p = p.add(4);
                        (((*a - b'0') * 10 + (*b - b'0')) as i32).neg()
                        // .rem_euclid(101) as u8
                    }
                    [a, b, b'\n'] => {
                        p = p.add(3);
                        ((*a - b'0') * 10 + (*b - b'0')) as i32
                    }
                    _ => shucks!(),
                };
                let x = (px as i32 + vx * 100).rem_euclid(W);
                let y = (py as i32 + vy * 100).rem_euclid(H);
                let w = W / 2;
                let h = H / 2;
                if x < w && y < h {
                    grids[0] += 1;
                } else if x < w && y > h {
                    grids[1] += 1;
                } else if x > w && y < h {
                    grids[2] += 1;
                } else if x > w && y > h {
                    grids[3] += 1;
                }
            }
        }
        grids.iter().product::<u32>()
    }

    #[no_mangle]
    pub fn p2(i: &str) -> impl Display {
        const W: u8 = 101;
        const H: u8 = 103;
        let mut positions_x = [0; 500];
        let mut velocities_x = [0; 500];
        let mut positions_y = [0; 500];
        let mut velocities_y = [0; 500];
        let mut p = i.as_bytes().as_ptr();
        unsafe {
            for j in 0..500 {
                p = p.add(2);
                positions_x[j] = match std::slice::from_raw_parts(p, 3) {
                    [a, b',', _] => {
                        p = p.add(2);
                        *a - b'0'
                    }
                    [a, b, b','] => {
                        p = p.add(3);
                        (*a - b'0') * 10 + (*b - b'0')
                    }
                    [a, b, c] => {
                        p = p.add(4);
                        ((*a - b'0') * 10 + (*b - b'0')) * 10 + (*c - b'0')
                    }
                    _ => shucks!(),
                };
                positions_y[j] = match std::slice::from_raw_parts(p, 3) {
                    [a, b' ', _] => {
                        p = p.add(2 + 2);
                        *a - b'0'
                    }
                    [a, b, b' '] => {
                        p = p.add(3 + 2);
                        (*a - b'0') * 10 + (*b - b'0')
                    }
                    [a, b, c] => {
                        p = p.add(4 + 2);
                        ((*a - b'0') * 10 + (*b - b'0')) * 10 + (*c - b'0')
                    }
                    _ => shucks!(),
                };
                velocities_x[j] = match std::slice::from_raw_parts(p, 3) {
                    [a, b',', _] => {
                        p = p.add(2);
                        a - b'0'
                    }
                    [b'-', a, b','] => {
                        p = p.add(3);
                        ((*a - b'0') as i8).neg().rem_euclid(103) as u8
                    }
                    [b'-', a, b] => {
                        p = p.add(4);
                        ((((*a - b'0') * 10 + (*b - b'0')) as i8)
                            .neg()
                            .rem_euclid(103)) as u8
                    }
                    [a, b, b','] => {
                        p = p.add(3);
                        (*a - b'0') * 10 + (*b - b'0')
                    }
                    _ => shucks!(),
                };
                velocities_y[j] = match std::slice::from_raw_parts(p, 3) {
                    [a, b'\n', _] => {
                        p = p.add(2);
                        a - b'0'
                    }
                    [b'-', a, b'\n'] => {
                        p = p.add(3);
                        ((*a - b'0') as i8).neg().rem_euclid(101) as u8
                    }
                    [b'-', a, b] => {
                        p = p.add(4);
                        (((*a - b'0') * 10 + (*b - b'0')) as i8)
                            .neg()
                            .rem_euclid(101) as u8
                    }
                    [a, b, b'\n'] => {
                        p = p.add(3);
                        (*a - b'0') * 10 + (*b - b'0')
                    }
                    _ => shucks!(),
                };
            }
        }
        use std::simd::prelude::*;
        let (px, rpx) = positions_x.as_chunks_mut::<32>();
        let (vx, rvx) = velocities_x.as_chunks_mut::<32>();
        let mut px: [u8x32; 15] = std::array::from_fn(|i| u8x32::from_array(px[i]));
        let vx: [u8x32; 15] = std::array::from_fn(|i| u8x32::from_array(vx[i]));
        let bx = (1..W)
            .map(|_| {
                px.iter_mut()
                    .zip(vx)
                    .map(|(px, vx)| {
                        *px += vx;
                        *px = px
                            .simd_ge(u8x32::splat(W))
                            .select(*px - u8x32::splat(W), *px);
                        *px
                    })
                    .map(|x| unsafe {
                        u64x4::from(core::arch::x86_64::_mm256_sad_epu8(
                            x.into(),
                            u8x32::splat(W as u8 / 2).into(),
                        ))
                    })
                    .fold(u64x4::default(), |acc, x| acc + x)
                    .reduce_sum()
                    + rpx
                        .iter_mut()
                        .zip(&*rvx)
                        .map(move |(x, vx)| {
                            *x += vx;
                            *x %= W;
                            x.abs_diff(W / 2) as u64
                        })
                        .sum::<u64>()
            })
            .ι1::<u32>()
            .min_by_key(|&(x, _)| x)
            .unwrap()
            .1 as i32;

        let (py, rpy) = positions_y.as_chunks_mut::<32>();
        let (vy, rvy) = velocities_y.as_chunks_mut::<32>();
        let mut py: [u8x32; 15] = std::array::from_fn(|i| u8x32::from_array(py[i]));
        let vy: [u8x32; 15] = std::array::from_fn(|i| u8x32::from_array(vy[i]));

        let by = (1..H)
            .map(|_| {
                py.iter_mut()
                    .zip(vy)
                    .map(|(py, vy)| {
                        *py += vy;
                        *py = py
                            .simd_ge(u8x32::splat(H))
                            .select(*py - u8x32::splat(H), *py);
                        *py
                    })
                    .map(|x| unsafe {
                        u64x4::from(core::arch::x86_64::_mm256_sad_epu8(
                            x.into(),
                            u8x32::splat(H as u8 / 2).into(),
                        ))
                    })
                    .fold(u64x4::default(), |acc, x| acc + x)
                    .reduce_sum()
                    + rpy
                        .iter_mut()
                        .zip(&*rvy)
                        .map(move |(x, vx)| {
                            *x += vx;
                            *x %= H;
                            x.abs_diff(H / 2) as u64
                        })
                        .sum::<u64>()
            })
            .ι1::<u32>()
            .min_by_key(|&(x, _)| x)
            .unwrap()
            .1 as i32;
        bx + ((51 * (by - bx)) % H as i32) * W as i32
    }
}
