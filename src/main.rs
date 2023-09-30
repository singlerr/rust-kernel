#![no_std]
#![no_main]

use handler;
use io::vgabuffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    
    let mut writer = vgabuffer::Writer::new(
         0,
         vgabuffer::ColorCode::new(vgabuffer::Color::Yellow, vgabuffer::Color::Black),
         unsafe{ &mut *(0xb800 as *mut vgabuffer::Buffer)}
    );
    
    writer.write_string("test");

    loop {
        
    }
}


