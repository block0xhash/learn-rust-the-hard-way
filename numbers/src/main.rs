
pub mod demo; // look for a file called demo or dir demo/mod.rs
pub mod libs;
use libs::numbers; 

fn main() ->() {
    
    demo::numbers::say_hello();
    numbers::say_hello();
    
    numbers::print(100);
    


}
