fn main(){
	let mut v=Vec::new();
	v.push(2);
	v.push(4);
	v.push(6);
for v in &v{
	println!("v:[{}]",v);
}
	v.pop();
for v in &v{
	println!("Yeni v:[{}]",v);
}
}