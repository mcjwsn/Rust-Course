use rand::Rng;

pub fn generate(length : i32) -> String{
    let mut rng = rand::rng(); // creates a new random number generator
    let chars: [char; 91] = [
        'a','b','c','d','e','f','g','h','i','j','k','l','m',
        'n','o','p','q','r','s','t','u','v','w','x','y','z',
        'A','B','C','D','E','F','G','H','I','J','K','L','M',
        'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
        '0','1','2','3','4','5','6','7','8','9',
        '!','@','#','$','%','^','&','*','(',')',
        '_','+','-','=','[',']','{','}','|',';',
        ':','\'','"',',','.','<','>','/','?'];
    let mut res: Vec<char> = Vec::new();
    for i in 0..length{
        res.push(chars[rng.random_range(0..91)]);
    }
    res.iter().collect()

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test(){
        let a:i32 = 6;
        let b = generate(a);
        assert!(a == b.len() as i32, "a: {} nie jest równe długości b: {}", a, b.len());

    }
}