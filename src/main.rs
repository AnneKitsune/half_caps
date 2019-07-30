use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Usage: fuckthecase 'string'");
		return;
	}
	let input = &args[1];
	let mut buf = String::with_capacity(input.len());
	let mut up = false;
	for c in input.chars(){
		if c.is_alphabetic(){
			let c = if up{
				c.to_uppercase().collect::<Vec<_>>()
			}else{
				c.to_lowercase().collect::<Vec<_>>()
			};
			for c in c{
				buf.push(c);
			}
			up = !up;
		}else{
			buf.push(c);
		}
	}
    println!("{}",buf);
}
