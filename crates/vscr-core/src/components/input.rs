bitflags::bitflags! {
    #[derive(Reflect)]
    pub struct Buttons: u16 {
        const LEFT  = 0b00000001;
        const RIGHT = 0b00000010;
        const UP    = 0b00000100;
        const DOWN  = 0b00001000;

        const A     = 0b00010000; // light
        const B     = 0b00100000; // medium
        const C     = 0b01000000; // heavy
        const D     = 0b10000000; // special, etc.
    }
}

#[derive(Component, Reflect, Default)]
pub struct InputBuffer {
    pub current: Buttons,
    pub frames_held: u8,
    pub history: heapless::Vec<Buttons, 32>, // or Vec if you prefer
}
