pub fn check(isbn:String) -> bool{
    let mut vals: Vec<i8> = Vec::new();
    for ch in isbn{
        if ch == "-"{continue;}
            else if ch == "X"{
                vals.append(&mut vec![10]);
            }
        else{
            ch.to_digit();
            vals.append(ch);
        }
    }
    let mut sum:i32 = 0;
    for i in 0..10{
        sum += vals[i] * (10-i);
    }
    if sum % 11 == 0{
        return true
    }
    false
}