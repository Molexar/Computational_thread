pub mod test_func;
pub mod lib;

fn main(){
	let input: Vec<u32> = (0..100).collect();
	let _results: Vec<u32> = lib::split_work::<u32, u32>(input, test_func::test_func);
}
