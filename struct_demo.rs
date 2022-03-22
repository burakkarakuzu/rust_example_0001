fn main(){
struct S{
	isim:String,
	soyisim:String,
	email:String,
};
let s=S{
	isim:String::from("Burak"),
	soyisim:String::from("Karakuzu"),
	email:String::from("a@gmail.com"),
};
if s.isim == "Burak" {
	println!("oldu.");
}else{
	println!("olmadı.");
}
}