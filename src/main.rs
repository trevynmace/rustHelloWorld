fn main() 
{
	let x = "blah";
	foo(x);
}

fn foo(x: &str)
{
	println!("{}", x);
}
