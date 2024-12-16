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
pub mod day15 {
    use super::util;
    use super::util::prelude::*;
    const SIZE: usize = 50;
    pub fn part2(i: &str) -> impl Display {
        let i = i.as_bytes();
        let bot = memchr::memchr(b'@', i).ψ();
        let (mut x, mut y) = ((bot % (SIZE + 1)) * 2, bot / (SIZE + 1));
        let grid = unsafe {
            i[..(SIZE + 1) * SIZE]
                .array_chunks::<{ SIZE + 1 }>()
                .flat_map(|x| {
                    x.iter().take(SIZE).copied().flat_map(|x| match x {
                        b'#' => [x; 2],
                        b'O' => *b"[]",
                        b'@' | b'.' => *b"..",
                        _ => shucks!(),
                    })
                })
                .collect::<Vec<_>>()
                .leak()
                .as_chunks_unchecked_mut::<{ SIZE * 2 }>()
        };
        // for y in 0..SIZE {
        //     for x in 0..SIZE * 2 {
        //         if (px, py) == (x, y) {
        //             print!("@");
        //         } else {
        //             print!("{}", grid[y][x] as char);
        //         }
        //     }
        //     println!();
        // }
        // println!("{grid/:?}");
        // let grid = i[..(SIZE + 1) * SIZE]
        //     .to_vec()
        //     .leak()
        //     .as_chunks_unchecked_mut::<{ SIZE + 1 }>();
        // grid[y][x * 2] = b'.';
        let i = &i[((SIZE + 1) * SIZE) + 1..];
        #[no_mangle]
        fn push(
            (x, y): (usize, usize),
            dir: Dir,
            grid: &mut [[u8; SIZE * 2]],
            commit: bool,
        ) -> bool {
            match dir {
                Dir::N => {
                    macro_rules! set {
                        () => {{
                            grid[y - 1][x] = b'[';
                            grid[y - 1][x + 1] = b']';
                            grid[y][x] = b'.';
                            grid[y][x + 1] = b'.';
                        }};
                    }
                    match [grid[y - 1][x], grid[y - 1][x + 1]] {
                        [_, b'#'] | [b'#', _] => {}
                        [b'.', b'.'] => {
                            if commit {
                                set!()
                            }
                            return true;
                        }
                        [b']', b'['] => {
                            let val = push((x - 1, y - 1), dir, grid, false)
                                && push((x + 1, y - 1), dir, grid, false);
                            if commit && val {
                                push((x - 1, y - 1), dir, grid, commit);
                                push((x + 1, y - 1), dir, grid, commit);
                                set!();
                            }
                            return val;
                        }
                        [b']', b'.'] => {
                            let val = push((x - 1, y - 1), dir, grid, commit);
                            if commit && val {
                                set!()
                            }
                            return val;
                        }
                        [b'.', b'['] => {
                            let val = push((x + 1, y - 1), dir, grid, commit);
                            if commit && val {
                                set!()
                            }
                            return val;
                        }
                        // "simple" case
                        [b'[', b']'] => {
                            let val = push((x, y - 1), dir, grid, commit);
                            if val && commit {
                                set!()
                            }
                            return val;
                        }
                        x => shucks!("{x:?}"),
                    }
                }
                Dir::S => {
                    macro_rules! set {
                        () => {{
                            grid[y + 1][x] = b'[';
                            grid[y + 1][x + 1] = b']';
                            grid[y][x] = b'.';
                            grid[y][x + 1] = b'.';
                        }};
                    }
                    match [grid[y + 1][x], grid[y + 1][x + 1]] {
                        [_, b'#'] | [b'#', _] => {}
                        [b'.', b'.'] => {
                            if commit {
                                set!()
                            }
                            return true;
                            // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                        }
                        [b']', b'['] => {
                            let val = push((x - 1, y + 1), dir, grid, false)
                                && push((x + 1, y + 1), dir, grid, false);
                            if commit && val {
                                push((x - 1, y + 1), dir, grid, commit);
                                push((x + 1, y + 1), dir, grid, commit);
                                set!()
                            }
                            return val;
                        }
                        [b']', b'.'] => {
                            let val = push((x - 1, y + 1), dir, grid, commit);
                            if commit && val {
                                set!()
                            }
                            return val;
                        }
                        [b'.', b'['] => {
                            let val = push((x + 1, y + 1), dir, grid, commit);
                            if commit && val {
                                set!()
                            }
                            return val;
                        }
                        [b'[', b']'] => {
                            let val = push((x, y + 1), dir, grid, commit);
                            if val && commit {
                                set!()
                            }
                            return val;
                        }
                        x => shucks!("{x:?}"),
                    }
                }
                Dir::E => {
                    macro_rules! set {
                        () => {{
                            grid[y][x + 2] = b']';
                            grid[y][x + 1] = b'[';
                            grid[y][x] = b'.';
                        }};
                    }
                    match grid[y][x + 2] {
                        b'.' => {
                            set!();
                            return true;
                            // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                        }
                        b'[' => {
                            if push((x + 2, y), dir, grid, true) {
                                set!();
                                return true;
                            }
                        }
                        b'#' => {}
                        x => shucks!("{}", x as char),
                    }
                }

                Dir::W => {
                    macro_rules! set {
                        () => {{
                            grid[y][x - 1] = b'[';
                            grid[y][x] = b']';
                            grid[y][x + 1] = b'.';
                        }};
                    }
                    match grid[y][x - 1] {
                        b'.' => {
                            set!();
                            return true;
                            // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                        }
                        b']' => {
                            if push((x - 2, y), dir, grid, commit) {
                                set!();
                                return true;
                            }
                        }
                        b'#' => {}
                        x => shucks!("{}", x as char),
                    }
                }
            }
            false
        }
        for input in i {
            // println!("{}", *input as char);
            match input {
                b'<' => match grid[y][x - 1] {
                    b'.' => x = x - 1,
                    b'#' => (),
                    b']' => {
                        if push((x - 2, y), Dir::W, grid, true) {
                            x = x - 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },
                b'>' => match grid[y][x + 1] {
                    b'.' => x = x + 1,
                    b'#' => (),
                    b'[' => {
                        if push((x + 1, y), Dir::E, grid, true) {
                            x = x + 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },

                b'^' => match grid[y - 1][x] {
                    b'.' => y = y - 1,
                    b'#' => (),
                    b']' => {
                        if push((x - 1, y - 1), Dir::N, grid, true) {
                            y = y - 1;
                        }
                    }
                    b'[' => {
                        if push((x, y - 1), Dir::N, grid, true) {
                            y = y - 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },
                b'v' => match grid[y + 1][x] {
                    b'.' => y = y + 1,
                    b'#' => (),
                    b'[' => {
                        if push((x, y + 1), Dir::S, grid, true) {
                            y = y + 1;
                        }
                    }
                    b']' => {
                        if push((x - 1, y + 1), Dir::S, grid, true) {
                            y = y + 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },
                _ => {}
            }
        }
        // grid[y][x] = b'@';
        // for row in &*grid {
        //     println!("{}", row.p());
        // }
        // grid[y][x] = b'.';
        (0..SIZE)
            .flat_map(|y| (0..SIZE * 2).map(move |x| (x, y)))
            .filter(|&(x, y)| grid[y][x] == b'[')
            .map(|(x, y)| 100 * y + x)
            .sum::<usize>()
    }

    #[no_mangle]
    pub fn part1(i: &str) -> impl Display {
        let i = i.as_bytes();
        let bot = memchr::memchr(b'@', i).ψ();
        let (mut x, mut y) = (bot % (SIZE + 1), bot / (SIZE + 1));
        let grid = unsafe {
            i[..(SIZE + 1) * SIZE]
                .to_vec()
                .leak()
                .as_chunks_unchecked_mut::<{ SIZE + 1 }>()
        };
        grid[y][x] = b'.';
        let i = &i[((SIZE + 1) * SIZE) + 1..];
        fn push((x, y): (usize, usize), dir: Dir, grid: &mut [[u8; SIZE + 1]]) -> bool {
            match dir {
                Dir::N => match grid[y - 1][x] {
                    b'.' => {
                        grid[y - 1][x] = b'O';
                        grid[y][x] = b'.';
                        return true;
                        // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                    }
                    b'O' => {
                        if push((x, y - 1), dir, grid) {
                            grid[y - 1][x] = b'O';
                            grid[y][x] = b'.';
                            return true;
                        }
                    }
                    b'#' => {}
                    x => shucks!("{}", x as char),
                },
                Dir::E => match grid[y][x + 1] {
                    b'.' => {
                        grid[y][x + 1] = b'O';
                        grid[y][x] = b'.';
                        return true;
                        // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                    }
                    b'O' => {
                        if push((x + 1, y), dir, grid) {
                            grid[y][x + 1] = b'O';
                            grid[y][x] = b'.';
                            return true;
                        }
                    }
                    b'#' => {}
                    x => shucks!("{}", x as char),
                },
                Dir::S => match grid[y + 1][x] {
                    b'.' => {
                        grid[y + 1][x] = b'O';
                        grid[y][x] = b'.';
                        return true;
                        // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                    }
                    b'O' => {
                        if push((x, y + 1), dir, grid) {
                            grid[y + 1][x] = b'O';
                            grid[y][x] = b'.';
                            return true;
                        }
                    }
                    b'#' => {}
                    x => shucks!("{}", x as char),
                },
                Dir::W => match grid[y][x - 1] {
                    b'.' => {
                        grid[y][x - 1] = b'O';
                        grid[y][x] = b'.';
                        return true;
                        // swap(&mut grid[y - 1][x], &mut grid[y][x]),
                    }
                    b'O' => {
                        if push((x - 1, y), dir, grid) {
                            grid[y][x - 1] = b'O';
                            grid[y][x] = b'.';
                            return true;
                        }
                    }
                    b'#' => {}
                    x => shucks!("{}", x as char),
                },
            }
            false
        }
        for input in i {
            match input {
                b'<' => match grid[y][x - 1] {
                    b'.' => x = x - 1,
                    b'#' => (),
                    b'O' => {
                        if push((x - 1, y), Dir::W, grid) {
                            x = x - 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },
                b'>' => match grid[y][x + 1] {
                    b'.' => x = x + 1,
                    b'#' => (),
                    b'O' => {
                        if push((x + 1, y), Dir::E, grid) {
                            x = x + 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },

                b'^' => match grid[y - 1][x] {
                    b'.' => y = y - 1,
                    b'#' => (),
                    b'O' => {
                        if push((x, y - 1), Dir::N, grid) {
                            y = y - 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },
                b'v' => match grid[y + 1][x] {
                    b'.' => y = y + 1,
                    b'#' => (),
                    b'O' => {
                        if push((x, y + 1), Dir::S, grid) {
                            y = y + 1;
                        }
                    }
                    x => shucks!("{}", x as char),
                },
                _ => {}
            }
        }
        let mut sum = 0;
        for (row, y) in grid.into_iter().ι::<u32>() {
            for (col, x) in row.into_iter().ι::<u32>() {
                if *col == b'O' {
                    sum += 100 * y + x
                }
            }
        }

        sum
    }
}
