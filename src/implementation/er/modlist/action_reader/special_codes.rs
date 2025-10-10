//! There's a mismatch between VK code sources. Note the lack of entries for 0x04-0x06 for the second one.
//! <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>
//! <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-tvtt/261ddfb0-ce10-4380-9b7a-4b50f482b8ec>
//! These will be added in manually.
//! left/right are added because they're covered by rdev which catches them slightly faster than `win_key_event`.
pub const LEFT:i32 = 0x01;
pub const RIGHT:i32 = 0x02;
pub const MIDDLE:i32 = 0x04;
pub const M4:i32 = 0x05;
pub const M5:i32 = 0x06;
//VK codes cover 0x01..0xFE. For simplicity custom codes will be kept in 0x100 and beyond.    
pub const SCROLL_UP:i32 = 0x100;
pub const SCROLL_DOWN:i32 = 0x101;
pub const SCROLL_LEFT:i32 = 0x102;
pub const SCROLL_RIGHT:i32 = 0x103;
//Additionally, +0x1000 means you can't have it held down.

