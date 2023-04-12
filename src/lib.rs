use std::thread;
use std::sync::Arc;

const THRESHHOLD: usize = 10;

pub fn split_work<T:std::marker::Sync + std::marker::Copy + std::marker::Send +'static, R:std::marker::Send + std::marker::Sync + 'static>(input: Vec<T>, func: fn(T) -> R) -> Vec<R>{
	let mut childrens = vec![];
	let mut results = vec![];
	let length: usize = input.len();
	let input_data = Arc::new(input);
	
	for split in (0..length).step_by(THRESHHOLD){
		let input_data = Arc::clone(&input_data);
		childrens.push(thread::spawn(move || -> Vec<R> {
			let mut i: usize = 0;
			let mut result = vec![];
			while (i+split < length) && (i < THRESHHOLD){
				result.push(func(input_data[i+split]));
				i+=1;
			}
			result
		}));	
	}
	for children in childrens.into_iter(){
		results.extend(children.join().unwrap());
	}
	results
}
