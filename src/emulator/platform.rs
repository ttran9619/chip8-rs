use derive_more::{From, Into};

#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct Pixel{
    pub column: u8,
    pub row: u8,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PixelState{
    On,
    Off
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyState{
    On,
    Off
}
#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct KeypadNumber(pub super::FourBitValue);
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuzzerState{
    On,
    Off
}

pub trait Platform{
    // Display
    fn get_display_width(&self) -> u8;
    fn get_display_height(&self) -> u8;

    fn clear_display(&self);
    fn get_pixel(&self,pixel: Pixel) -> PixelState;
    fn set_pixel(&self,pixel: Pixel, state: PixelState);

    // Keypad
    fn block_for_any_keypress(&self) -> KeyState;
    fn read_keypress_state(&self, key: KeypadNumber) -> KeyState;

    // Buzzer
    fn set_buzzer(&self, state: BuzzerState);

}