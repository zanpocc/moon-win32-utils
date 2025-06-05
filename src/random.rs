use rand::Rng;

pub fn random_file_name(len: Option<u8>) -> String {
    let arr = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut rng = rand::thread_rng();
    

    let name_len = len.unwrap_or_else(||15);
    
    let mut file_name = String::new();
    for _ in 0..name_len{
        let random_index = rng.gen_range(0..arr.len());
        file_name.push(arr[random_index]);
    }

    return file_name;
}


#[cfg(test)]
mod tests {
    use crate::random::random_file_name;


    #[test]
    fn test_random_file_name() {
        println!("{}",random_file_name(None));
    }

}