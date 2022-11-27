



#[cfg(test)]
mod test{
    use super::super::*;
    pub struct TestPlatform {
        display: Display,
        buzzer: BuzzerState,
        keypad: Keypad,
        waiters: Vec<futures::channel::oneshot::Sender<KeyState>>
    }

    use std::u8;

    use crate::emulator;

    use super::super::platform::*;
    use super::super::Emulator;

    const WIDTH: u8 = 32;
    const HEIGHT: u8 = 64;
    type Row = [PixelState; WIDTH as usize];
    type Display = [Row; HEIGHT as usize];

    type Keypad = [KeyState; KEYPAD_COUNT as usize];


    #[async_trait::async_trait]
    impl Platform for TestPlatform {
        // Display
        async fn get_display_width(&self) -> u8 {
            WIDTH
        }
        async fn get_display_height(&self) -> u8 {
            HEIGHT
        }
        async fn clear_display(&mut self) {
            self.display.fill([PixelState::Off; WIDTH as usize])
        }
        async fn get_pixel(&self, pixel: Pixel) -> PixelState {
            self.display[pixel.row as usize][pixel.row as usize]
        }
        async fn set_pixel(&mut self, pixel: Pixel, state: PixelState) {
            self.display[pixel.row as usize][pixel.row as usize] = state
        }

        // Keypad
        async fn block_for_any_keypress(&mut self) -> KeyState {
            let (sender, receive) = futures::channel::oneshot::channel::<KeyState>();

            self.waiters.push(sender);

            receive.await.unwrap()
        }
        async fn read_keypress_state(&self, key: KeypadNumber) -> KeyState {
            let index: usize = key.into();
            self.keypad[index]
        }

        // Buzzer
        async fn set_buzzer(&mut self, state: BuzzerState) {
            self.buzzer = state
        }
    }

    impl TestPlatform{
        pub fn new() -> Self{
            TestPlatform{
                 display: [[PixelState::Off; WIDTH as usize]; HEIGHT as usize],
                 buzzer: BuzzerState::Off,
                 keypad: [KeyState::Off; KEYPAD_COUNT as usize],
                 waiters: vec!()
            }
        }

        pub fn set_keypress(&mut self, key: KeypadNumber, key_state: KeyState){
            let index: usize = key.into();
            self.keypad[index] = key_state;

            for sender in self.waiters.drain(..){
                sender.send(key_state).unwrap();
            }
            assert!(self.waiters.len() == 0);
        }

        pub fn get_buzzer(& self) -> BuzzerState{
            self.buzzer
        }
    }

    fn new_test_emulator() -> Emulator<TestPlatform>{
        Emulator::new(TestPlatform::new())
    }

    #[tokio::test]
    async fn my_test() {
        let mut emulator = new_test_emulator();
        emulator.start_program().await;
    }
}

