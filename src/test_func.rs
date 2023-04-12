use std::time::Duration;
use std::thread;

pub fn test_func(input: u32) -> u32{
     if input % 2 == 0{
         thread::sleep(Duration::from_millis(100));
     }     input
}
