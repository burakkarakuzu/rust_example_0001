use std::fs::File;
fn main(){
	let f=File::open("b.sh");
	let _f=match f{
		Ok(file) => file,
		Err(_error) => panic!("Hata"),
	};
	
}