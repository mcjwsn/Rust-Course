// use rand::Rng;
//
// pub fn generate(length : i32,charsets: &[&str]) -> String{
//     let mut rng = rand::rng(); // creates a new random number generator
//     let mut lowercase = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m',
//         'n','o','p','q','r','s','t','u','v','w','x','y','z'];
//     let mut uppercase= vec!['A','B','C','D','E','F','G','H','I','J','K','L','M',
//     'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
//     let mut digits = vec!['0','1','2','3','4','5','6','7','8','9'];
//     let mut special  = vec![
//         '!','@','#','$','%','^','&','*','(',')',
//         '_','+','-','=','[',']','{','}','|',';',
//         ':','\'','"',',','.','<','>','/','?'];
//     let mut res: Vec<char> = Vec::new();
//     let mut vals: Vec<char> = Vec::new();
//
//     for charset in charsets{
//         match *charset{
//             "lowercase" =>res.append(&mut lowercase),
//             "uppercase" => res.append(&mut uppercase),
//             "digits" => res.append(&mut digits),
//             "special" => res.append(&mut special),
//             _ => {}
//         }
//     }
//     if res == vec![]{
//         res.append(&mut lowercase);
//         res.append(&mut uppercase);
//         res.append(&mut digits);
//         res.append(&mut special); }
//
//     for _ in 0..length{
//         vals.push(res[rng.random_range(0..res.len())]);
//     }
//     vals.iter().collect()
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashSet;
//
//     fn get_charset_set(name: &str) -> HashSet<char> {
//         match name {
//             "lowercase" => "abcdefghijklmnopqrstuvwxyz".chars().collect(),
//             "uppercase" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
//             "digits"    => "0123456789".chars().collect(),
//             "special"   => "!@#$%^&*()_+-=[]{}|;:'\",.<>/?".chars().collect(),
//             _ => HashSet::new(),
//         }
//     }
//
//     #[test]
//     fn test_generate_charset_validity() {
//         let charsets = ["lowercase", "digits"];
//         let password = generate(50, &charsets);
//
//         let allowed: HashSet<char> = charsets
//             .iter()
//             .flat_map(|c| get_charset_set(c))
//             .collect();
//
//         for ch in password.chars() {
//             assert!(
//                 allowed.contains(&ch),
//                 "Bad char: '{}'", ch
//             );
//         }
//     }
// }