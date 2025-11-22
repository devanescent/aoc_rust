use crate::aoc_result::AoCResult;

make_day!(Day22);

pub fn solve_part1(input: &String) -> AoCResult {
    let shufflings = input
        .trim_end()
        .lines()
        .map(|l| {
            if l == "deal into new stack" {
                // new stack = reverse deck
                ShuffleTechnique::DealNewStack
            } else if let Some(cut_val) = l.strip_prefix("cut ") {
                // cut
                ShuffleTechnique::Cut(cut_val.parse().unwrap())
            } else {
                // deal with increment
                ShuffleTechnique::DealWithIncrement(l.strip_prefix("deal with increment ").unwrap().parse().unwrap())
            }
        });

    let mut deck: Vec<u32> = (0..10007).collect();
    
    for s in shufflings {
        match s {
            ShuffleTechnique::DealNewStack => { 
                deck.reverse();
            }
            ShuffleTechnique::Cut(cut) => {
                let cut_size = if cut >= 0 { cut as usize } else { (deck.len() as i32 + cut) as usize };
                deck = deck.iter().skip(cut_size).chain(deck.iter().take(cut_size)).cloned().collect();
            }
            ShuffleTechnique::DealWithIncrement(inc) => {
                let mut deck_copy = deck.clone();
                let mut i = 0;
                for d in deck.iter() {
                    deck_copy[i] = *d;
                    i = (i + inc) % deck.len();
                }
                deck = deck_copy;
            }
        }
    }

    let card2019_index = deck.into_iter().enumerate().find(|(_, c)| c == &2019).unwrap().0;
    AoCResult::Num(card2019_index as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let shufflings = input
        .trim_end()
        .lines()
        .map(|l| {
            if l == "deal into new stack" {
                // new stack = reverse deck
                ShuffleTechnique::DealNewStack
            } else if let Some(cut_val) = l.strip_prefix("cut ") {
                // cut
                ShuffleTechnique::Cut(cut_val.parse().unwrap())
            } else {
                // deal with increment
                ShuffleTechnique::DealWithIncrement(l.strip_prefix("deal with increment ").unwrap().parse().unwrap())
            }
        });

    let deck_len: i128 = 119315717514047;

    // Represent deck as pair of offset and increment:
    // -> solution taken from: https://www.reddit.com/r/adventofcode/comments/ee0rqi/comment/fbnkaju/
    let (mut offset, mut increment) = (0i128, 1i128);
    
    for s in shufflings {
        match s {
            ShuffleTechnique::DealNewStack => { 
                increment = -increment;
                increment %= deck_len;
                offset += increment;
                offset %= deck_len;
            }
            ShuffleTechnique::Cut(cut) => {
                offset += cut as i128 * increment;
                offset %= deck_len;
            }
            ShuffleTechnique::DealWithIncrement(a) => {
                /* Notes:
                  - Offset is kept the same during the operation, because the first card (index = 0) is always placed at new index 0 also.
                  - To get the new increment, use the following reasoning:
                    1) Card at index i gets placed at position (i * a) % DECK_SIZE
                    2) To get the new increment, find out which card gets placed at index 1, so find the solution to: (x * a) % DECK_SIZE = 1 (which is the modular multiplicative inverse)
                    3) Use Fermat's little theorem: for a prime number p and a number a not divisble by p the following is true:
                           a ^ (p - 1) - 1  is divisible by p or  a ^ (p - 1) % p = 1
                    4) Written for the deck (which has a size that is a prime number, and all values for a in the input are not divisible by this size):
                           a ^ (DECK_SIZE - 1) % DECK_SIZE = 1
                    5) Together with the formular above:
                           a ^ (DECK_SIZE - 1) % DECK_SIZE = 1
                           (x * a) % DECK_SIZE = 1
                           => (x * a) = a ^ (DECK_SIZE - 1)       | divide both sides by a
                           => x = a ^ (DECK_SIZE - 2)
                    6) Actual position inside the deck is then x % DECK_SIZE, calculate a ^ (DECK_SIZE - 2) % DECK_SIZE by using modular exponentiation
                       via the following pseudo code (from https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method):
                            if modulus = 1 then
                                return 0
                            Assert :: (modulus - 1) * (modulus - 1) does not overflow base
                            result := 1
                            base := base mod modulus
                            while exponent > 0 do
                                if (exponent mod 2 == 1) then
                                    result := (result * base) mod modulus
                                exponent := exponent >> 1
                                base := (base * base) mod modulus
                            return result
                    7) Get new increment:
                        Card at (new) index 0: offset
                        Card at (new) index 1: offset + inc_old * x, with x being the solution for the modular multiplicative inverse
                        new_inc = card[1] - card[0] = (offset + inc_old * x) - offset = inc_old * x
                */
                increment *= mod_pow(a as i128, deck_len - 2, deck_len);
                increment %= deck_len;
            }
        };
    }

    // After one shuffle, the values for offset and increment are:
    let (offset_diff, incr_diff) = (offset % deck_len, increment % deck_len);
    println!("Offset: {}, incr.: {}", offset_diff, incr_diff);

    // After each shuffle, both values with change by:
    // - increment is only increased by multiplication with a constant number (the factor above depends neither on the old increment nor the offset)
    // - offset is incresed by some constant multiples of increments:
    //   -> For example, during one shuffle, offset is increased by (a * inc_0) + (b * inc_1) + (c * inc_2)
    //      where inc_1 = inc_0 * k and inc_2 = inc_1 * l = inc_0 * k * l
    //   -> offset_diff = (a * inc_0) + (b * inc_1) + (c * inc_2)
    //                  = (a * inc_0) * (b * inc_0 * k) + (c * inc_0 * k * l)
    //                  = inc_0 * c
    //   -> Initial increment is 1 so c = offset_diff
    
    // After N shuffles:
    let shuffles: i128 = 101741582076661;

    // Increment will be: increment_new = incr_diff ^ N
    let final_increment = mod_pow(incr_diff, shuffles, deck_len);
    
    // Offset after 1. shuffle is: offset_1 = inc_0 * offset_diff = 1 * offset_diff = (incr_diff ^ 0) * offset_diff
    // Offset after 2. shuffle is: offset_2 = inc_0 * offset_diff + inc_0 * incr_diff * offset_diff = (incr_diff ^ 0) * offset_diff + (incr_diff ^ 1) * offset_diff
    // Offset after 3. shuffle is: offset_3 = (incr_diff ^ 0) * offset_diff + (incr_diff ^ 1) * offset_diff + (incr_diff ^ 2) * offset_diff
    // Offset after Nth shuffle is: offset_n = (incr_diff ^ 0) * offset_diff + (incr_diff ^ 1) * offset_diff + (incr_diff ^ 2) * offset_diff + ... + (incr_diff ^ (n - 1)) * offset_diff
    
    // -> This is a geometric series, for which the sum for (n-1) terms can be calculated by:
    //      S = a * ((1 - r ^ n) / (1 - r)), where a = offset_diff, r = incr_diff, n = shuffles
    //        = offset_diff * (1 - incr_diff ^ n) / (1 - incr_diff)
    // -> Replace division by fomular for modular inverse:
    //      => offset_diff * (1 - pow(incr_diff, iterations, DECK_SIZE)) * pow(1 - incr_diff, DECK_SIZE-2, DECK_SIZE)


    // Final formula taken from: https://github.com/mcpower/adventofcode/blob/501b66084b0060e0375fc3d78460fb549bc7dfab/2019/22/a-p2.py#L34
    // final_offset = offset_diff * 1 - mod_pow(incr_diff, shuffles, deck_len) * mod_pow(1 - incr_diff, deck_len - 2, deck_len)
    
    // Calculating this directly will lead to an overflow, so use following equation to multiply:
    // (A * B) % C = ((A % C) * (B % C)) % C

    let factor_0 = offset_diff;
    let factor_1 = 1 - mod_pow(incr_diff, shuffles, deck_len);
    let factor_2 = mod_pow(1 - incr_diff, deck_len - 2, deck_len);

    let a = factor_0 * factor_1;
    let b = factor_2;
    let final_offset = ((a % deck_len) * (b % deck_len)) % deck_len;

    // Card at position 2020:
    let res = (final_increment * 2020 + final_offset) % deck_len;

    AoCResult::Num(res as u64)
}

enum ShuffleTechnique {
    DealNewStack,
    Cut(i32),
    DealWithIncrement(usize),
}

fn mod_pow(mut base: i128, mut exp: i128, modulo: i128) -> i128 {
    if modulo == 1 {
        return 0;
    }

    let mut result = 1;
    base %= modulo;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        exp >>= 1;
        base = (base * base) % modulo;
    }
    return result;
}