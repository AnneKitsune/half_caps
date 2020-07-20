use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut buffer = String::new();
    let inputs = if args.len() < 2 {
        std::io::stdin().lock().read_to_string(&mut buffer).expect("Failed to read from stdin");
        vec![buffer]
    } else {
        args[1..].to_vec()
    };
    let str_len: usize = inputs.iter().map(|s| s.len()).sum();
    let mut buf = String::with_capacity(str_len + inputs.len());
    let mut up = false;
    for input in inputs {
        for c in input.chars(){
            if c.is_alphabetic(){
                let c = if up{
                    c.to_uppercase().to_string()
                }else{
                    c.to_lowercase().to_string()
                };
                buf.push_str(&c);
                up = !up;
            }else{
                buf.push(c);
            }
        }
        buf.push(' ');
    }
    println!("{}",buf);
}
