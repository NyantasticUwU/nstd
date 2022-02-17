use num_derive::*;
use num_traits::*;
use winit::event::VirtualKeyCode;

/// The shift key.
pub const NSTD_INPUT_KEY_SHIFT_BIT: u8 = 0b00000001;

/// The ctrl key.
pub const NSTD_INPUT_KEY_CTRL_BIT: u8 = 0b00000010;

/// The alt key.
pub const NSTD_INPUT_KEY_ALT_BIT: u8 = 0b00000100;

/// The logo key.
pub const NSTD_INPUT_KEY_LOGO_BIT: u8 = 0b00001000;

/// Represents a virtual key code.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum NSTDKey {
    /// An unknown keyboard key.
    NSTD_KEY_NONE,
    /// The 1 keyboard key.
    NSTD_KEY_1,
    /// The 2 keyboard key.
    NSTD_KEY_2,
    /// The 3 keyboard key.
    NSTD_KEY_3,
    /// The 4 keyboard key.
    NSTD_KEY_4,
    /// The 5 keyboard key.
    NSTD_KEY_5,
    /// The 6 keyboard key.
    NSTD_KEY_6,
    /// The 7 keyboard key.
    NSTD_KEY_7,
    /// The 8 keyboard key.
    NSTD_KEY_8,
    /// The 9 keyboard key.
    NSTD_KEY_9,
    /// The 0 keyboard key.
    NSTD_KEY_0,
    /// The A keyboard key.
    NSTD_KEY_A,
    /// The B keyboard key.
    NSTD_KEY_B,
    /// The C keyboard key.
    NSTD_KEY_C,
    /// The D keyboard key.
    NSTD_KEY_D,
    /// The E keyboard key.
    NSTD_KEY_E,
    /// The F keyboard key.
    NSTD_KEY_F,
    /// The G keyboard key.
    NSTD_KEY_G,
    /// The H keyboard key.
    NSTD_KEY_H,
    /// The I keyboard key.
    NSTD_KEY_I,
    /// The J keyboard key.
    NSTD_KEY_J,
    /// The K keyboard key.
    NSTD_KEY_K,
    /// The L keyboard key.
    NSTD_KEY_L,
    /// The M keyboard key.
    NSTD_KEY_M,
    /// The N keyboard key.
    NSTD_KEY_N,
    /// The O keyboard key.
    NSTD_KEY_O,
    /// The P keyboard key.
    NSTD_KEY_P,
    /// The Q keyboard key.
    NSTD_KEY_Q,
    /// The R keyboard key.
    NSTD_KEY_R,
    /// The S keyboard key.
    NSTD_KEY_S,
    /// The T keyboard key.
    NSTD_KEY_T,
    /// The U keyboard key.
    NSTD_KEY_U,
    /// The V keyboard key.
    NSTD_KEY_V,
    /// The W keyboard key.
    NSTD_KEY_W,
    /// The X keyboard key.
    NSTD_KEY_X,
    /// The Y keyboard key.
    NSTD_KEY_Y,
    /// The Z keyboard key.
    NSTD_KEY_Z,
    /// The ESCAPE keyboard key.
    NSTD_KEY_ESCAPE,
    /// The F1 keyboard key.
    NSTD_KEY_F1,
    /// The F2 keyboard key.
    NSTD_KEY_F2,
    /// The F3 keyboard key.
    NSTD_KEY_F3,
    /// The F4 keyboard key.
    NSTD_KEY_F4,
    /// The F5 keyboard key.
    NSTD_KEY_F5,
    /// The F6 keyboard key.
    NSTD_KEY_F6,
    /// The F7 keyboard key.
    NSTD_KEY_F7,
    /// The F8 keyboard key.
    NSTD_KEY_F8,
    /// The F9 keyboard key.
    NSTD_KEY_F9,
    /// The F10 keyboard key.
    NSTD_KEY_F10,
    /// The F11 keyboard key.
    NSTD_KEY_F11,
    /// The F12 keyboard key.
    NSTD_KEY_F12,
    /// The F13 keyboard key.
    NSTD_KEY_F13,
    /// The F14 keyboard key.
    NSTD_KEY_F14,
    /// The F15 keyboard key.
    NSTD_KEY_F15,
    /// The F16 keyboard key.
    NSTD_KEY_F16,
    /// The F17 keyboard key.
    NSTD_KEY_F17,
    /// The F18 keyboard key.
    NSTD_KEY_F18,
    /// The F19 keyboard key.
    NSTD_KEY_F19,
    /// The F20 keyboard key.
    NSTD_KEY_F20,
    /// The F21 keyboard key.
    NSTD_KEY_F21,
    /// The F22 keyboard key.
    NSTD_KEY_F22,
    /// The F23 keyboard key.
    NSTD_KEY_F23,
    /// The F24 keyboard key.
    NSTD_KEY_F24,
    /// The SNAPSHOT keyboard key.
    NSTD_KEY_SNAPSHOT,
    /// The SCROLL keyboard key.
    NSTD_KEY_SCROLL,
    /// The PAUSE keyboard key.
    NSTD_KEY_PAUSE,
    /// The INSERT keyboard key.
    NSTD_KEY_INSERT,
    /// The HOME keyboard key.
    NSTD_KEY_HOME,
    /// The DELETE keyboard key.
    NSTD_KEY_DELETE,
    /// The END keyboard key.
    NSTD_KEY_END,
    /// The PAGE_DOWN keyboard key.
    NSTD_KEY_PAGE_DOWN,
    /// The PAGE_UP keyboard key.
    NSTD_KEY_PAGE_UP,
    /// The LEFT keyboard key.
    NSTD_KEY_LEFT,
    /// The UP keyboard key.
    NSTD_KEY_UP,
    /// The RIGHT keyboard key.
    NSTD_KEY_RIGHT,
    /// The DOWN keyboard key.
    NSTD_KEY_DOWN,
    /// The BACK keyboard key.
    NSTD_KEY_BACK,
    /// The RETURN keyboard key.
    NSTD_KEY_RETURN,
    /// The SPACE keyboard key.
    NSTD_KEY_SPACE,
    /// The COMPOSE keyboard key.
    NSTD_KEY_COMPOSE,
    /// The CARET keyboard key.
    NSTD_KEY_CARET,
    /// The NUMLOCK keyboard key.
    NSTD_KEY_NUMLOCK,
    /// The NUMPAD_0 keyboard key.
    NSTD_KEY_NUMPAD_0,
    /// The NUMPAD_1 keyboard key.
    NSTD_KEY_NUMPAD_1,
    /// The NUMPAD_2 keyboard key.
    NSTD_KEY_NUMPAD_2,
    /// The NUMPAD_3 keyboard key.
    NSTD_KEY_NUMPAD_3,
    /// The NUMPAD_4 keyboard key.
    NSTD_KEY_NUMPAD_4,
    /// The NUMPAD_5 keyboard key.
    NSTD_KEY_NUMPAD_5,
    /// The NUMPAD_6 keyboard key.
    NSTD_KEY_NUMPAD_6,
    /// The NUMPAD_7 keyboard key.
    NSTD_KEY_NUMPAD_7,
    /// The NUMPAD_8 keyboard key.
    NSTD_KEY_NUMPAD_8,
    /// The NUMPAD_9 keyboard key.
    NSTD_KEY_NUMPAD_9,
    /// The NUMPAD_ADD keyboard key.
    NSTD_KEY_NUMPAD_ADD,
    /// The NUMPAD_DIVIDE keyboard key.
    NSTD_KEY_NUMPAD_DIVIDE,
    /// The NUMPAD_DECIMAL keyboard key.
    NSTD_KEY_NUMPAD_DECIMAL,
    /// The NUMPAD_COMMA keyboard key.
    NSTD_KEY_NUMPAD_COMMA,
    /// The NUMPAD_ENTER keyboard key.
    NSTD_KEY_NUMPAD_ENTER,
    /// The NUMPAD_EQUALS keyboard key.
    NSTD_KEY_NUMPAD_EQUALS,
    /// The NUMPAD_MULTIPLY keyboard key.
    NSTD_KEY_NUMPAD_MULTIPLY,
    /// The NUMPAD_SUBTRACT keyboard key.
    NSTD_KEY_NUMPAD_SUBTRACT,
    /// The ABNTC1 keyboard key.
    NSTD_KEY_ABNTC1,
    /// The ABNTC2 keyboard key.
    NSTD_KEY_ABNTC2,
    /// The APOSTROPHE keyboard key.
    NSTD_KEY_APOSTROPHE,
    /// The APPS keyboard key.
    NSTD_KEY_APPS,
    /// The ASTERISK keyboard key.
    NSTD_KEY_ASTERISK,
    /// The AT keyboard key.
    NSTD_KEY_AT,
    /// The AX keyboard key.
    NSTD_KEY_AX,
    /// The BACKSLASH keyboard key.
    NSTD_KEY_BACKSLASH,
    /// The CALCULATOR keyboard key.
    NSTD_KEY_CALCULATOR,
    /// The CAPITAL keyboard key.
    NSTD_KEY_CAPITAL,
    /// The COLON keyboard key.
    NSTD_KEY_COLON,
    /// The COMMA keyboard key.
    NSTD_KEY_COMMA,
    /// The CONVERT keyboard key.
    NSTD_KEY_CONVERT,
    /// The EQUALS keyboard key.
    NSTD_KEY_EQUALS,
    /// The GRAVE keyboard key.
    NSTD_KEY_GRAVE,
    /// The KANA keyboard key.
    NSTD_KEY_KANA,
    /// The KANJI keyboard key.
    NSTD_KEY_KANJI,
    /// The LEFT_ALT keyboard key.
    NSTD_KEY_LEFT_ALT,
    /// The LEFT_BRACKET keyboard key.
    NSTD_KEY_LEFT_BRACKET,
    /// The LEFT_CONTROL keyboard key.
    NSTD_KEY_LEFT_CONTROL,
    /// The LEFT_SHIFT keyboard key.
    NSTD_KEY_LEFT_SHIFT,
    /// The LEFT_WIN keyboard key.
    NSTD_KEY_LEFT_WIN,
    /// The MAIL keyboard key.
    NSTD_KEY_MAIL,
    /// The MEDIA_SELECT keyboard key.
    NSTD_KEY_MEDIA_SELECT,
    /// The MEDIA_STOP keyboard key.
    NSTD_KEY_MEDIA_STOP,
    /// The MINUS keyboard key.
    NSTD_KEY_MINUS,
    /// The MUTE keyboard key.
    NSTD_KEY_MUTE,
    /// The MY_COMPUTER keyboard key.
    NSTD_KEY_MY_COMPUTER,
    /// The NAVIGATE_FORWARD keyboard key.
    NSTD_KEY_NAVIGATE_FORWARD,
    /// The NAVIGATE_BACKWARD keyboard key.
    NSTD_KEY_NAVIGATE_BACKWARD,
    /// The NEXT_TRACK keyboard key.
    NSTD_KEY_NEXT_TRACK,
    /// The NO_CONVERT keyboard key.
    NSTD_KEY_NO_CONVERT,
    /// The OEM102 keyboard key.
    NSTD_KEY_OEM102,
    /// The PERIOD keyboard key.
    NSTD_KEY_PERIOD,
    /// The PLAY_PAUSE keyboard key.
    NSTD_KEY_PLAY_PAUSE,
    /// The PLUS keyboard key.
    NSTD_KEY_PLUS,
    /// The POWER keyboard key.
    NSTD_KEY_POWER,
    /// The PREV_TRACK keyboard key.
    NSTD_KEY_PREV_TRACK,
    /// The RIGHT_ALT keyboard key.
    NSTD_KEY_RIGHT_ALT,
    /// The RIGHT_BRACKET keyboard key.
    NSTD_KEY_RIGHT_BRACKET,
    /// The RIGHT_CONTROL keyboard key.
    NSTD_KEY_RIGHT_CONTROL,
    /// The RIGHT_SHIFT keyboard key.
    NSTD_KEY_RIGHT_SHIFT,
    /// The RIGHT_WIN keyboard key.
    NSTD_KEY_RIGHT_WIN,
    /// The SEMICOLON keyboard key.
    NSTD_KEY_SEMICOLON,
    /// The SLASH keyboard key.
    NSTD_KEY_SLASH,
    /// The SLEEP keyboard key.
    NSTD_KEY_SLEEP,
    /// The STOP keyboard key.
    NSTD_KEY_STOP,
    /// The SYSRQ keyboard key.
    NSTD_KEY_SYSRQ,
    /// The TAB keyboard key.
    NSTD_KEY_TAB,
    /// The UNDERLINE keyboard key.
    NSTD_KEY_UNDERLINE,
    /// The UNLABELED keyboard key.
    NSTD_KEY_UNLABELED,
    /// The VOLUME_DOWN keyboard key.
    NSTD_KEY_VOLUME_DOWN,
    /// The VOLUME_UP keyboard key.
    NSTD_KEY_VOLUME_UP,
    /// The WAKE keyboard key.
    NSTD_KEY_WAKE,
    /// The WEB_BACK keyboard key.
    NSTD_KEY_WEB_BACK,
    /// The WEB_FAVORITES keyboard key.
    NSTD_KEY_WEB_FAVORITES,
    /// The WEB_FORWARD keyboard key.
    NSTD_KEY_WEB_FORWARD,
    /// The WEB_HOME keyboard key.
    NSTD_KEY_WEB_HOME,
    /// The WEB_REFRESH keyboard key.
    NSTD_KEY_WEB_REFRESH,
    /// The WEB_SEARCH keyboard key.
    NSTD_KEY_WEB_SEARCH,
    /// The WEB_STOP keyboard key.
    NSTD_KEY_WEB_STOP,
    /// The YEN keyboard key.
    NSTD_KEY_YEN,
    /// The COPY keyboard key.
    NSTD_KEY_COPY,
    /// The PASTE keyboard key.
    NSTD_KEY_PASTE,
    /// The CUT keyboard key.
    NSTD_KEY_CUT,
}
impl Default for NSTDKey {
    #[inline]
    fn default() -> Self {
        Self::NSTD_KEY_NONE
    }
}
impl From<VirtualKeyCode> for NSTDKey {
    #[inline]
    fn from(key: VirtualKeyCode) -> Self {
        match key {
            // English alphabet.
            VirtualKeyCode::A => Self::NSTD_KEY_A,
            VirtualKeyCode::B => Self::NSTD_KEY_B,
            VirtualKeyCode::C => Self::NSTD_KEY_C,
            VirtualKeyCode::D => Self::NSTD_KEY_D,
            VirtualKeyCode::E => Self::NSTD_KEY_E,
            VirtualKeyCode::F => Self::NSTD_KEY_F,
            VirtualKeyCode::G => Self::NSTD_KEY_G,
            VirtualKeyCode::H => Self::NSTD_KEY_H,
            VirtualKeyCode::I => Self::NSTD_KEY_I,
            VirtualKeyCode::J => Self::NSTD_KEY_J,
            VirtualKeyCode::K => Self::NSTD_KEY_K,
            VirtualKeyCode::L => Self::NSTD_KEY_L,
            VirtualKeyCode::M => Self::NSTD_KEY_M,
            VirtualKeyCode::N => Self::NSTD_KEY_N,
            VirtualKeyCode::O => Self::NSTD_KEY_O,
            VirtualKeyCode::P => Self::NSTD_KEY_P,
            VirtualKeyCode::Q => Self::NSTD_KEY_Q,
            VirtualKeyCode::R => Self::NSTD_KEY_R,
            VirtualKeyCode::S => Self::NSTD_KEY_S,
            VirtualKeyCode::T => Self::NSTD_KEY_T,
            VirtualKeyCode::U => Self::NSTD_KEY_U,
            VirtualKeyCode::V => Self::NSTD_KEY_V,
            VirtualKeyCode::W => Self::NSTD_KEY_W,
            VirtualKeyCode::X => Self::NSTD_KEY_X,
            VirtualKeyCode::Y => Self::NSTD_KEY_Y,
            VirtualKeyCode::Z => Self::NSTD_KEY_Z,
            // Row 1.
            VirtualKeyCode::Escape => Self::NSTD_KEY_ESCAPE,
            VirtualKeyCode::F1 => Self::NSTD_KEY_F1,
            VirtualKeyCode::F2 => Self::NSTD_KEY_F2,
            VirtualKeyCode::F3 => Self::NSTD_KEY_F3,
            VirtualKeyCode::F4 => Self::NSTD_KEY_F4,
            VirtualKeyCode::F5 => Self::NSTD_KEY_F5,
            VirtualKeyCode::F6 => Self::NSTD_KEY_F6,
            VirtualKeyCode::F7 => Self::NSTD_KEY_F7,
            VirtualKeyCode::F8 => Self::NSTD_KEY_F8,
            VirtualKeyCode::F9 => Self::NSTD_KEY_F9,
            VirtualKeyCode::F10 => Self::NSTD_KEY_F10,
            VirtualKeyCode::F11 => Self::NSTD_KEY_F11,
            VirtualKeyCode::F12 => Self::NSTD_KEY_F12,
            VirtualKeyCode::F13 => Self::NSTD_KEY_F13,
            VirtualKeyCode::F14 => Self::NSTD_KEY_F14,
            VirtualKeyCode::F15 => Self::NSTD_KEY_F15,
            VirtualKeyCode::F16 => Self::NSTD_KEY_F16,
            VirtualKeyCode::F17 => Self::NSTD_KEY_F17,
            VirtualKeyCode::F18 => Self::NSTD_KEY_F18,
            VirtualKeyCode::F19 => Self::NSTD_KEY_F19,
            VirtualKeyCode::F20 => Self::NSTD_KEY_F20,
            VirtualKeyCode::F21 => Self::NSTD_KEY_F21,
            VirtualKeyCode::F22 => Self::NSTD_KEY_F22,
            VirtualKeyCode::F23 => Self::NSTD_KEY_F23,
            VirtualKeyCode::F24 => Self::NSTD_KEY_F24,
            // Row 2.
            VirtualKeyCode::Grave => Self::NSTD_KEY_GRAVE,
            VirtualKeyCode::Key1 => Self::NSTD_KEY_1,
            VirtualKeyCode::Key2 => Self::NSTD_KEY_2,
            VirtualKeyCode::Key3 => Self::NSTD_KEY_3,
            VirtualKeyCode::Key4 => Self::NSTD_KEY_4,
            VirtualKeyCode::Key5 => Self::NSTD_KEY_5,
            VirtualKeyCode::Key6 => Self::NSTD_KEY_6,
            VirtualKeyCode::Key7 => Self::NSTD_KEY_7,
            VirtualKeyCode::Key8 => Self::NSTD_KEY_8,
            VirtualKeyCode::Key9 => Self::NSTD_KEY_9,
            VirtualKeyCode::Key0 => Self::NSTD_KEY_0,
            VirtualKeyCode::Minus => Self::NSTD_KEY_MINUS,
            VirtualKeyCode::Equals => Self::NSTD_KEY_EQUALS,
            VirtualKeyCode::Back => Self::NSTD_KEY_BACK,
            // Row 3.
            VirtualKeyCode::Tab => Self::NSTD_KEY_TAB,
            VirtualKeyCode::LBracket => Self::NSTD_KEY_LEFT_BRACKET,
            VirtualKeyCode::RBracket => Self::NSTD_KEY_RIGHT_BRACKET,
            VirtualKeyCode::Backslash => Self::NSTD_KEY_BACKSLASH,
            // Row 4.
            VirtualKeyCode::Capital => Self::NSTD_KEY_CAPITAL,
            VirtualKeyCode::Semicolon => Self::NSTD_KEY_SEMICOLON,
            VirtualKeyCode::Apostrophe => Self::NSTD_KEY_APOSTROPHE,
            VirtualKeyCode::Return => Self::NSTD_KEY_RETURN,
            // Row 5.
            VirtualKeyCode::LShift => Self::NSTD_KEY_LEFT_SHIFT,
            VirtualKeyCode::RShift => Self::NSTD_KEY_RIGHT_SHIFT,
            VirtualKeyCode::Comma => Self::NSTD_KEY_COMMA,
            VirtualKeyCode::Period => Self::NSTD_KEY_PERIOD,
            VirtualKeyCode::Slash => Self::NSTD_KEY_SLASH,
            // Row 6.
            VirtualKeyCode::LControl => Self::NSTD_KEY_LEFT_CONTROL,
            VirtualKeyCode::RControl => Self::NSTD_KEY_RIGHT_CONTROL,
            VirtualKeyCode::LAlt => Self::NSTD_KEY_LEFT_ALT,
            VirtualKeyCode::RAlt => Self::NSTD_KEY_RIGHT_ALT,
            VirtualKeyCode::Space => Self::NSTD_KEY_SPACE,
            // 60%.
            VirtualKeyCode::Insert => Self::NSTD_KEY_INSERT,
            VirtualKeyCode::Home => Self::NSTD_KEY_HOME,
            VirtualKeyCode::PageUp => Self::NSTD_KEY_PAGE_UP,
            VirtualKeyCode::PageDown => Self::NSTD_KEY_PAGE_DOWN,
            VirtualKeyCode::Delete => Self::NSTD_KEY_DELETE,
            VirtualKeyCode::End => Self::NSTD_KEY_END,
            VirtualKeyCode::Up => Self::NSTD_KEY_UP,
            VirtualKeyCode::Down => Self::NSTD_KEY_DOWN,
            VirtualKeyCode::Left => Self::NSTD_KEY_LEFT,
            VirtualKeyCode::Right => Self::NSTD_KEY_RIGHT,
            // Numpad.
            VirtualKeyCode::Numlock => Self::NSTD_KEY_NUMLOCK,
            VirtualKeyCode::NumpadDivide => Self::NSTD_KEY_NUMPAD_DIVIDE,
            VirtualKeyCode::NumpadMultiply => Self::NSTD_KEY_NUMPAD_MULTIPLY,
            VirtualKeyCode::NumpadSubtract => Self::NSTD_KEY_NUMPAD_SUBTRACT,
            VirtualKeyCode::NumpadAdd => Self::NSTD_KEY_NUMPAD_ADD,
            VirtualKeyCode::NumpadEnter => Self::NSTD_KEY_NUMPAD_ENTER,
            VirtualKeyCode::NumpadDecimal => Self::NSTD_KEY_NUMPAD_DECIMAL,
            VirtualKeyCode::Numpad1 => Self::NSTD_KEY_NUMPAD_1,
            VirtualKeyCode::Numpad2 => Self::NSTD_KEY_NUMPAD_2,
            VirtualKeyCode::Numpad3 => Self::NSTD_KEY_NUMPAD_3,
            VirtualKeyCode::Numpad4 => Self::NSTD_KEY_NUMPAD_4,
            VirtualKeyCode::Numpad5 => Self::NSTD_KEY_NUMPAD_5,
            VirtualKeyCode::Numpad6 => Self::NSTD_KEY_NUMPAD_6,
            VirtualKeyCode::Numpad7 => Self::NSTD_KEY_NUMPAD_7,
            VirtualKeyCode::Numpad8 => Self::NSTD_KEY_NUMPAD_8,
            VirtualKeyCode::Numpad9 => Self::NSTD_KEY_NUMPAD_9,
            VirtualKeyCode::Numpad0 => Self::NSTD_KEY_NUMPAD_0,
            // Fallback to implicit conversion.
            _ => match FromPrimitive::from_i32((key as i32) + 1) {
                Some(key) => key,
                _ => Self::NSTD_KEY_NONE,
            },
        }
    }
}
impl TryInto<VirtualKeyCode> for NSTDKey {
    type Error = u32;

    #[inline]
    fn try_into(self) -> Result<VirtualKeyCode, Self::Error> {
        match self {
            // English alphabet.
            Self::NSTD_KEY_A => Ok(VirtualKeyCode::A),
            Self::NSTD_KEY_B => Ok(VirtualKeyCode::B),
            Self::NSTD_KEY_C => Ok(VirtualKeyCode::C),
            Self::NSTD_KEY_D => Ok(VirtualKeyCode::D),
            Self::NSTD_KEY_E => Ok(VirtualKeyCode::E),
            Self::NSTD_KEY_F => Ok(VirtualKeyCode::F),
            Self::NSTD_KEY_G => Ok(VirtualKeyCode::G),
            Self::NSTD_KEY_H => Ok(VirtualKeyCode::H),
            Self::NSTD_KEY_I => Ok(VirtualKeyCode::I),
            Self::NSTD_KEY_J => Ok(VirtualKeyCode::J),
            Self::NSTD_KEY_K => Ok(VirtualKeyCode::K),
            Self::NSTD_KEY_L => Ok(VirtualKeyCode::L),
            Self::NSTD_KEY_M => Ok(VirtualKeyCode::M),
            Self::NSTD_KEY_N => Ok(VirtualKeyCode::N),
            Self::NSTD_KEY_O => Ok(VirtualKeyCode::O),
            Self::NSTD_KEY_P => Ok(VirtualKeyCode::P),
            Self::NSTD_KEY_Q => Ok(VirtualKeyCode::Q),
            Self::NSTD_KEY_R => Ok(VirtualKeyCode::R),
            Self::NSTD_KEY_S => Ok(VirtualKeyCode::S),
            Self::NSTD_KEY_T => Ok(VirtualKeyCode::T),
            Self::NSTD_KEY_U => Ok(VirtualKeyCode::U),
            Self::NSTD_KEY_V => Ok(VirtualKeyCode::V),
            Self::NSTD_KEY_W => Ok(VirtualKeyCode::W),
            Self::NSTD_KEY_X => Ok(VirtualKeyCode::X),
            Self::NSTD_KEY_Y => Ok(VirtualKeyCode::Y),
            Self::NSTD_KEY_Z => Ok(VirtualKeyCode::Z),
            // Row 1.
            Self::NSTD_KEY_ESCAPE => Ok(VirtualKeyCode::Escape),
            Self::NSTD_KEY_F1 => Ok(VirtualKeyCode::F1),
            Self::NSTD_KEY_F2 => Ok(VirtualKeyCode::F2),
            Self::NSTD_KEY_F3 => Ok(VirtualKeyCode::F3),
            Self::NSTD_KEY_F4 => Ok(VirtualKeyCode::F4),
            Self::NSTD_KEY_F5 => Ok(VirtualKeyCode::F5),
            Self::NSTD_KEY_F6 => Ok(VirtualKeyCode::F6),
            Self::NSTD_KEY_F7 => Ok(VirtualKeyCode::F7),
            Self::NSTD_KEY_F8 => Ok(VirtualKeyCode::F8),
            Self::NSTD_KEY_F9 => Ok(VirtualKeyCode::F9),
            Self::NSTD_KEY_F10 => Ok(VirtualKeyCode::F10),
            Self::NSTD_KEY_F11 => Ok(VirtualKeyCode::F11),
            Self::NSTD_KEY_F12 => Ok(VirtualKeyCode::F12),
            Self::NSTD_KEY_F13 => Ok(VirtualKeyCode::F13),
            Self::NSTD_KEY_F14 => Ok(VirtualKeyCode::F14),
            Self::NSTD_KEY_F15 => Ok(VirtualKeyCode::F15),
            Self::NSTD_KEY_F16 => Ok(VirtualKeyCode::F16),
            Self::NSTD_KEY_F17 => Ok(VirtualKeyCode::F17),
            Self::NSTD_KEY_F18 => Ok(VirtualKeyCode::F18),
            Self::NSTD_KEY_F19 => Ok(VirtualKeyCode::F19),
            Self::NSTD_KEY_F20 => Ok(VirtualKeyCode::F20),
            Self::NSTD_KEY_F21 => Ok(VirtualKeyCode::F21),
            Self::NSTD_KEY_F22 => Ok(VirtualKeyCode::F22),
            Self::NSTD_KEY_F23 => Ok(VirtualKeyCode::F23),
            Self::NSTD_KEY_F24 => Ok(VirtualKeyCode::F24),
            // Row 2.
            Self::NSTD_KEY_GRAVE => Ok(VirtualKeyCode::Grave),
            Self::NSTD_KEY_1 => Ok(VirtualKeyCode::Key1),
            Self::NSTD_KEY_2 => Ok(VirtualKeyCode::Key2),
            Self::NSTD_KEY_3 => Ok(VirtualKeyCode::Key3),
            Self::NSTD_KEY_4 => Ok(VirtualKeyCode::Key4),
            Self::NSTD_KEY_5 => Ok(VirtualKeyCode::Key5),
            Self::NSTD_KEY_6 => Ok(VirtualKeyCode::Key6),
            Self::NSTD_KEY_7 => Ok(VirtualKeyCode::Key7),
            Self::NSTD_KEY_8 => Ok(VirtualKeyCode::Key8),
            Self::NSTD_KEY_9 => Ok(VirtualKeyCode::Key9),
            Self::NSTD_KEY_0 => Ok(VirtualKeyCode::Key0),
            Self::NSTD_KEY_MINUS => Ok(VirtualKeyCode::Minus),
            Self::NSTD_KEY_EQUALS => Ok(VirtualKeyCode::Equals),
            Self::NSTD_KEY_BACK => Ok(VirtualKeyCode::Back),
            // Row 3.
            Self::NSTD_KEY_TAB => Ok(VirtualKeyCode::Tab),
            Self::NSTD_KEY_LEFT_BRACKET => Ok(VirtualKeyCode::LBracket),
            Self::NSTD_KEY_RIGHT_BRACKET => Ok(VirtualKeyCode::RBracket),
            Self::NSTD_KEY_BACKSLASH => Ok(VirtualKeyCode::Backslash),
            // Row 4.
            Self::NSTD_KEY_CAPITAL => Ok(VirtualKeyCode::Capital),
            Self::NSTD_KEY_SEMICOLON => Ok(VirtualKeyCode::Semicolon),
            Self::NSTD_KEY_APOSTROPHE => Ok(VirtualKeyCode::Apostrophe),
            Self::NSTD_KEY_RETURN => Ok(VirtualKeyCode::Return),
            // Row 5.
            Self::NSTD_KEY_LEFT_SHIFT => Ok(VirtualKeyCode::LShift),
            Self::NSTD_KEY_RIGHT_SHIFT => Ok(VirtualKeyCode::RShift),
            Self::NSTD_KEY_COMMA => Ok(VirtualKeyCode::Comma),
            Self::NSTD_KEY_PERIOD => Ok(VirtualKeyCode::Period),
            Self::NSTD_KEY_SLASH => Ok(VirtualKeyCode::Slash),
            // Row 6.
            Self::NSTD_KEY_LEFT_CONTROL => Ok(VirtualKeyCode::LControl),
            Self::NSTD_KEY_RIGHT_CONTROL => Ok(VirtualKeyCode::RControl),
            Self::NSTD_KEY_LEFT_ALT => Ok(VirtualKeyCode::LAlt),
            Self::NSTD_KEY_RIGHT_ALT => Ok(VirtualKeyCode::RAlt),
            Self::NSTD_KEY_SPACE => Ok(VirtualKeyCode::Space),
            // 60%.
            Self::NSTD_KEY_INSERT => Ok(VirtualKeyCode::Insert),
            Self::NSTD_KEY_HOME => Ok(VirtualKeyCode::Home),
            Self::NSTD_KEY_PAGE_UP => Ok(VirtualKeyCode::PageUp),
            Self::NSTD_KEY_PAGE_DOWN => Ok(VirtualKeyCode::PageDown),
            Self::NSTD_KEY_DELETE => Ok(VirtualKeyCode::Delete),
            Self::NSTD_KEY_END => Ok(VirtualKeyCode::End),
            Self::NSTD_KEY_UP => Ok(VirtualKeyCode::Up),
            Self::NSTD_KEY_DOWN => Ok(VirtualKeyCode::Down),
            Self::NSTD_KEY_LEFT => Ok(VirtualKeyCode::Left),
            Self::NSTD_KEY_RIGHT => Ok(VirtualKeyCode::Right),
            // Numpad.
            Self::NSTD_KEY_NUMLOCK => Ok(VirtualKeyCode::Numlock),
            Self::NSTD_KEY_NUMPAD_DIVIDE => Ok(VirtualKeyCode::NumpadDivide),
            Self::NSTD_KEY_NUMPAD_MULTIPLY => Ok(VirtualKeyCode::NumpadMultiply),
            Self::NSTD_KEY_NUMPAD_SUBTRACT => Ok(VirtualKeyCode::NumpadSubtract),
            Self::NSTD_KEY_NUMPAD_ADD => Ok(VirtualKeyCode::NumpadAdd),
            Self::NSTD_KEY_NUMPAD_ENTER => Ok(VirtualKeyCode::NumpadEnter),
            Self::NSTD_KEY_NUMPAD_DECIMAL => Ok(VirtualKeyCode::NumpadDecimal),
            Self::NSTD_KEY_NUMPAD_1 => Ok(VirtualKeyCode::Numpad1),
            Self::NSTD_KEY_NUMPAD_2 => Ok(VirtualKeyCode::Numpad2),
            Self::NSTD_KEY_NUMPAD_3 => Ok(VirtualKeyCode::Numpad3),
            Self::NSTD_KEY_NUMPAD_4 => Ok(VirtualKeyCode::Numpad4),
            Self::NSTD_KEY_NUMPAD_5 => Ok(VirtualKeyCode::Numpad5),
            Self::NSTD_KEY_NUMPAD_6 => Ok(VirtualKeyCode::Numpad6),
            Self::NSTD_KEY_NUMPAD_7 => Ok(VirtualKeyCode::Numpad7),
            Self::NSTD_KEY_NUMPAD_8 => Ok(VirtualKeyCode::Numpad8),
            Self::NSTD_KEY_NUMPAD_9 => Ok(VirtualKeyCode::Numpad9),
            Self::NSTD_KEY_NUMPAD_0 => Ok(VirtualKeyCode::Numpad0),
            // Fallback to implicit conversion.
            _ => Err(self as u32),
        }
    }
}

/// Represents a key state.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDKeyState {
    /// A key is/was pressed.
    NSTD_KEY_STATE_PRESSED,
    /// A key is/was released.
    NSTD_KEY_STATE_RELEASED,
}
impl Default for NSTDKeyState {
    #[inline]
    fn default() -> Self {
        Self::NSTD_KEY_STATE_RELEASED
    }
}

/// Represents a key event.
#[repr(C)]
#[derive(Default)]
pub struct NSTDKeyEvent {
    /// The keyboard key.
    pub key: NSTDKey,
    /// The state of the key.
    pub state: NSTDKeyState,
    /// The key's scan code.
    pub scan_code: u32,
}
