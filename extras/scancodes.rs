// static mut CAPS_LOCK_STATE: bool = false;

// let key = match scancode  {

//     0x1C => Some("\n"),
//     0x09 => Some("8"),
//     0x1E => Some("A"),
//     0x3A => {
//         CAPS_LOCK_STATE = !CAPS_LOCK_STATE;
//         None
//     },

//     _ => None
// };

// if CAPS_LOCK_STATE {
//     if let Some(scancode) = key {
//         print!("{}U", scancode);
//     };
// } else {
//     if let Some(scancode) = key {
//         print!("{}u", scancode);
//     }; 
// }




// lazy_static! {
//     static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = {
//         let keyboard = Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
//         keyboard
//     };
// }
// let mut keyboard = KEYBOARD.lock();
// if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//     if let Some(key) = keyboard.process_keyevent(key_event) {
//         match key {
//             DecodedKey::Unicode(character) => print!("{}", character),
//             DecodedKey::RawKey(key) => print!("{:?}", key),
//         }
//     }
// }