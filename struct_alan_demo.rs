struct Kare{
	uzunluk:i32,
	genislik:i32,
}
fn main(){
	impl Kare{
	fn alan(&self) ->i32 {
		self.uzunluk*self.genislik
	}
}
	let k=Kare{
		uzunluk:20,
		genislik:30,
	};	
	println!("Karenin alanı:{}",k.alan());
}