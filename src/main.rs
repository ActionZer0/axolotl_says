use clap::Parser;

#[derive (Debug, Parser)]
///A little CLI that outputs a message
struct Arguments {
	///The thing to say
	message:String,
	///The axolotl's mood
	mood:String,
}

fn main() {
    let args = Arguments::parse();
	//dbg!(args.message);
	let text = &args.message;
	let mood = &args.mood;
	let text_len = text.len();

	print!("+");
	for _ in 0..text_len + 2 {
		print!("-");
	}
	println!("+");
	
	println!("| {} |", text);
	
	print!("+");
	for _ in 0..text_len + 2 {
		print!("-");
	}
	println!("+");
	
	println!("     /");
	print!("≽(◕ ");
	
	if mood == "sad" { print!("ᴖ") }
	else if mood == "ambivalent" { print!("_") }
	else { print!("ᴗ") }
	println!(" ◕)≼");
}
