pub fn check(isbn:String) -> bool{
    let mut vals: Vec<u32> = Vec::new();
    for ch in isbn.chars(){
        if ch == '-'{continue;}
            else if ch == 'X'{
                vals.push(10);
            }
        else if let Some(digit) = ch.to_digit(10){
            vals.push(digit);
        }else {
            return false;
        }
    }
    if vals.len() != 10 {
        return false;
    }
    let mut sum:u32 = 0;
    for i in 0..10{
        sum += vals[i] * (10-i as u32);
    }
    if sum % 11 == 0{
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_isbn() {
        assert!(check("0-306-40615-2".to_string()));
        assert!(check("0306406152".to_string()));
        assert!(check("0-7167-0344-0".to_string()));
        assert!(check("0-19-852663-6".to_string()));
    }

    #[test]
    fn test_valid_isbn_with_x() {
        assert!(check("3-16-148410-X".to_string()));
        assert!(check("316148410X".to_string()));
        assert!(check("0-8044-2957-X".to_string()));
    }

    #[test]
    fn test_invalid_isbn_wrong_checksum() {
        assert!(!check("0-306-40615-3".to_string()));
        assert!(!check("0-306-40615-0".to_string()));
        assert!(!check("3-16-148410-9".to_string()));
    }

    #[test]
    fn test_invalid_isbn_wrong_length() {
        assert!(!check("0-306-4061".to_string()));
        assert!(!check("".to_string()));
    }

    #[test]
    fn test_invalid_isbn_invalid_characters() {
        assert!(!check("0-3A6-40615-2".to_string()));
        assert!(!check("0-306-4061B-2".to_string()));
        assert!(!check("0=306=40615=2".to_string()));
    }

    #[test]
    fn test_invalid_isbn_x_not_in_last_position() {
        assert!(!check("X-306-40615-2".to_string()));
        assert!(!check("0-X06-40615-2".to_string()));
    }
}