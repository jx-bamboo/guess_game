use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
	
	let secret_number = rand::thread_rng().gen_range(1..101);
	
	println!("The secret number is: {}", secret_number);
	
	loop {
		println!("Please input your guess.");
		
		let mut guess = String::new();
		
		io::stdin().read_line(&mut guess).expect("Failed to read line");
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("You guessed: {}",guess);
		
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
	
	//###
	let y = {
		let x = 5;
		x + 1
	};
	
	println!("This is a {}", y);
	
	let x: (i32,f32,u32) = (12,1.3,88);
	println!("元祖是：{}",x.2);
	
	let mut counter = 0; 
	let result = loop { 
		counter += 1;
		if counter == 10 {
			break counter * 2;
		} 
	};
	println!("The result is {}", result);
	test();
	println!("{}",test());

	let mut number = 3;
	while number != 0 {
		println!("{}!", number);
		number = number -1;
	}
	println!("LIFTOFF!!!");

	
}

fn test() -> i32 {
	return 28
}
