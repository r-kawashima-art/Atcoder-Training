use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;

fn main() {
    input! {t: Chars, p: Chars}
    let t_len: usize = t.len();
    let p_len: usize = p.len();
    let mut power10: Vec<usize> = vec![1; t_len + 2];
    for i in 1..=t_len + 1 {
        power10[i] = power10[i - 1] * 10 % MOD;
    }
    let mut t_hash: Vec<usize> = vec![0; t_len + 1];
    for i in 0..t_len {
        t_hash[i + 1] = (t_hash[i] + (t[i] as usize - 'a' as usize) * power10[i + 1]) % MOD;
    }
    let p_hash: usize = p
        .iter()
        .enumerate()
        .map(|(i, c)| (*c as usize - 'a' as usize) * power10[i])
        .sum();
    let get_hash = |left: usize, right: usize| -> usize {
        (t_hash[right + 1] - t_hash[left] + MOD) % MOD * power10[t_len - right] % MOD
    };
    for left in 0..t_len {
        if left + p_len > t_len {
            break;
        }
        if get_hash(left, left + p_len - 1) == p_hash {
            println!("{}", left);
        }
    }
}
