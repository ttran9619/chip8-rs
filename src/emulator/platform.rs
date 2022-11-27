use derive_more::{From, Into};

#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct Pixel {
    pub column: u8,
    pub row: u8,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PixelState {
    On,
    Off,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyState {
    On,
    Off,
}
#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct KeypadNumber(pub u8);
pub const KEYPAD_COUNT: u8 = 16;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuzzerState {
    On,
    Off,
}

#[async_trait::async_trait]
pub trait Platform {
    // Display
    async fn get_display_width(&self) -> u8;
    async fn get_display_height(&self) -> u8;

    async fn clear_display(&mut self);
    async fn get_pixel(&self, pixel: Pixel) -> PixelState;
    async fn set_pixel(&mut self, pixel: Pixel, state: PixelState);

    // Keypad
    async fn block_for_any_keypress(&mut self) -> KeyState;
    async fn read_keypress_state(&self, key: KeypadNumber) -> KeyState;

    // Buzzer
    async fn set_buzzer(&mut self, state: BuzzerState);
}


impl From<KeypadNumber> for usize {
    fn from(item: KeypadNumber) -> Self {
        let value: u8 = item.0.into();
        value as usize
    }
}