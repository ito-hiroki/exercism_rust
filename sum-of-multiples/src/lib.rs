pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().any(|y| *y != 0 && x % y == 0))
        .sum()
}

// fn is_multiple(x: &u32, factors: &[u32]) -> bool {
//     for factor in factors {
//         if *factor != 0 {
//             if x % factor == 0 {
//                 return true;
//             }
//         }
//     }
//     false
// }
