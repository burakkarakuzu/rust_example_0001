fn main(){
fn assert_demo(x:i32) -> i32{
	x*2
}
}
#[cfg(test)]
mod tests {
    #[test]
    fn fonk(){
	assert_nq!(4,assert_demo(2));
}	
}