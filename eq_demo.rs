fn main(){
	let vec1=vec![1,2,3];
	let vec2:Vec<_>=vec1.iter().map(|x| x*2).collect();
	assert_eq!(vec2,vec![2,4,6]);
}