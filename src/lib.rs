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
pub mod day23 {
    use super::util;
    use super::util::prelude::*;
    #[no_mangle]
    pub fn part2(x: &str) -> String {
        let g = Graph::load(x);
        let x = g.mxq();
        let mut i = 0;
        const c: u8 = b',';
        static mut out: [u8; 38] = [
            0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0, c, 0, 0,
            c, 0, 0, c, 0, 0, c, 0, 0,
        ];
        for j in 0..WORDS {
            let mut x = x[j];
            while x != 0 {
                let bit = x.trailing_zeros();
                unsafe {
                    out[i + i * 2..i + i * 2 + 2]
                        .copy_from_slice(&C! { NAMES[64 * j + bit as usize] })
                };
                i += 1;
                x &= !(1 << bit);
            }
        }
        unsafe { String::from_utf8_unchecked(out.to_vec()) }
    }

    fn part1(x: &str) -> u32 {
        let adj = Graph::load(x).adj;
        let mut has = [false; 676];
        let mut sum = 0;
        for computer in 494..=519 {
            has[computer] = true;
            let nbors = Graph::adj_on(adj[computer]);
            for (&elem2, i) in nbors.iter().ι::<usize>() {
                for &elem3 in &nbors[i..] {
                    if !has[elem2] && !has[elem3] && adj[elem2][elem3 / 64] & 1 << (elem3 % 64) != 0
                    {
                        sum += 1;
                    }
                }
            }
        }
        sum
    }
    struct Graph {
        // vert: [[u8; 2]; SIZE],
        adj: Box<[[u64; WORDS]; SIZE]>,
    }
    const SIZE: usize = 676;
    const WORDS: usize = (SIZE + 63) / 64;
    fn h([a, b]: [u8; 2]) -> usize {
        a as usize + 26 * b as usize
    }
    const NAMES: [[u8; 2]; 676] = include!("../lut2");

    impl Graph {
        fn load(content: &str) -> Self {
            const INDEX: [u16; 3295] = {
                let mut l = [0; 3295];
                include!("../lut");
                l
            };
            let mut i = content.as_ptr();
            let mut adj = Box::new([[0u64; WORDS]; SIZE]);
            for _ in 0..3380 {
                unsafe {
                    let a = *(i as *const [u8; 2]);
                    let b = *(i.add(3) as *const [u8; 2]);
                    let ha = h(a);
                    let hb = h(b);
                    i = i.add(6);
                    let i = INDEX[ha] as usize;
                    let j = INDEX[hb] as usize;
                    *adj.get_unchecked_mut(i).get_unchecked_mut(j / 64) |= 1u64 << (j % 64);
                    *adj.get_unchecked_mut(j).get_unchecked_mut(i / 64) |= 1u64 << (i % 64);
                }
            }
            Graph { adj }
        }

        fn print_mat(x: [u64; WORDS], l: [u8; 2]) {
            let n = Self::adj_on(x);
            print!("{}: ", l.p());
            for neighbor in n {
                print!("{} ", NAMES[neighbor].p());
            }
            println!();
        }

        fn first_2_bits(x: [u64; WORDS]) -> [usize; 2] {
            let mut out = [0; 2];
            let mut index = 0;
            for j in 0..WORDS {
                let mut x = x[j];
                while x != 0 {
                    let bit = x.trailing_zeros();
                    out[index] = 64 * j + bit as usize;
                    index += 1;
                    if index == 2 {
                        return out;
                    }
                    x &= !(1 << bit);
                }
            }
            panic!()
        }

        fn adj_on(x: [u64; WORDS]) -> Vec<usize> {
            let mut n = Vec::with_capacity(13);
            for j in 0..WORDS {
                let mut x = x[j];
                while x != 0 {
                    let bit = x.trailing_zeros();
                    n.push(64 * j + bit as usize);
                    x &= !(1 << bit);
                }
            }
            n
        }

        fn mxq(&self) -> [u64; WORDS] {
            'out: for computer in 0..SIZE {
                let neighbors = self.adj[computer];
                if neighbors == [0; 11] {
                    continue;
                }
                // neighbors[computer / 64] |= 1 << (computer % 64);
                // self.print_mat(neighbors, *b"nh");
                for node in Self::first_2_bits(neighbors) {
                    let inter = (0..WORDS)
                        .map(|i| (self.adj[node][i] & neighbors[i]).count_ones())
                        .sum::<u32>();
                    // check that the current node has 11 neighbours in common with either its first or second neighbour
                    if inter == 11 {
                        let mut v = [0; 11];
                        let mut pop = 0;
                        for j in 0..WORDS {
                            // self.print_mat(neighbors, *b"nh");
                            let mut x = neighbors[j];
                            while x != 0 {
                                let bit = x.trailing_zeros();
                                let n = 64 * j + bit as usize;
                                let inter = (0..WORDS)
                                    .map(|i| (self.adj[n][i] & neighbors[i]).count_ones())
                                    .sum::<u32>();
                                // they all have 11 neighbours in common with the current node
                                if inter == 11 {
                                    v[j] |= 1 << bit;
                                    pop += 1;
                                }
                                x &= !(1 << bit);
                            }
                        }
                        // self.print_mat(v, *b"ot");
                        if pop != 12 {
                            continue 'out;
                        }
                        v[computer / 64] |= 1 << computer % 64;
                        // v.push(computer);
                        // println!("whoa");
                        return v;
                    }
                }
            }
            panic!()
        }
    }
}
