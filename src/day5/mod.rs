// pub fn part1(input: &str) -> String {
//     let mut converts = Vec::<Vec<(u64, u64, u64)>>::new();
//
//     let mut split = input.split("\n\n");
//
//     let desired_seeds = split
//         .next()
//         .unwrap()
//         .split(':')
//         .nth(1)
//         .unwrap()
//         .split_whitespace()
//         .map(|num| num.parse::<u64>().unwrap())
//         .collect::<Vec<u64>>();
//
//     for conversion in split {
//         let mut possible_conversions = Vec::<(u64, u64, u64)>::new();
//         for line in conversion.lines().skip(1).map(|ln| {
//             ln.split_whitespace()
//                 .map(|num| num.parse::<u64>().unwrap())
//                 .collect::<Vec<u64>>()
//         }) {
//             let store = (line[0], line[1], line[2]);
//             possible_conversions.push(store);
//         }
//
//         converts.push(possible_conversions);
//     }
//
//     let mut converted_seeds = Vec::<u64>::new();
//
//     for seed in desired_seeds {
//         let mut converted = seed;
//         for conversion in &converts {
//             for range in conversion {
//                 if let Some(converted_seed) = try_converting(converted, *range) {
//                     converted = converted_seed;
//                 }
//             }
//         }
//
//         converted_seeds.push(converted);
//     }
//
//     converted_seeds.sort();
//     converted_seeds.first().unwrap().to_string()
// }

////////////
// part 2 //
////////////

//
// pub fn try_converting(current: u128, range: (u128, u128, u128)) -> Option<u128> {
//     if current >= range.1 && current < range.1 + range.2 {
//         Some(current - range.1 + range.0)
//     } else {
//         None
//     }
// }
//
// pub fn part2(input: &str) -> String {
//     let mut converts = Vec::<Vec<(u128, u128, u128)>>::new();
//
//     let mut split = input.split("\n\n");
//
//     let mut desired_seeds_temp = split
//         .next()
//         .unwrap()
//         .split(':')
//         .nth(1)
//         .unwrap()
//         .split_whitespace()
//         .map(|num| num.parse::<u128>().unwrap())
//         .collect::<Vec<u128>>()
//         .iter();
//
//     let mut desired_seeds = Vec::<u128>::new();
//
//     while desired_seeds_temp.count() > 0 {
//         let beginning = desired_seeds_temp.next().unwrap();
//         let how_many = desired_seeds_temp.next().unwrap();
//     }
//
//     for conversion in split {
//         let mut possible_conversions = Vec::<(u128, u128, u128)>::new();
//         for line in conversion.lines().skip(1).map(|ln| {
//             ln.split_whitespace()
//                 .map(|num| num.parse::<u128>().unwrap())
//                 .collect::<Vec<u128>>()
//         }) {
//             let store = (line[0], line[1], line[2]);
//             possible_conversions.push(store);
//         }
//
//         converts.push(possible_conversions);
//     }
//
//     let mut converted_seeds = Vec::<u128>::new();
//
//     for seed in desired_seeds {
//         let mut converted = seed;
//         for conversion in &converts {
//             for range in conversion {
//                 if let Some(converted_seed) = try_converting(converted, *range) {
//                     converted = converted_seed;
//                 }
//             }
//         }
//
//         converted_seeds.push(converted);
//     }
//
//     converted_seeds.sort();
//     converted_seeds.first().unwrap().to_string()
// }
