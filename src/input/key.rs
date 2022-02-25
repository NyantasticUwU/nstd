use winit::event::VirtualKeyCode;

/// Represents a virtual key code.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDKey {
    /// An unknown keyboard key.
    NSTD_KEY_UNKNOWN,

    /// The `esc` key.
    NSTD_KEY_ESC,
    /// The `space` key.
    NSTD_KEY_SPACE,

    /// The `0` key.
    NSTD_KEY_0,
    /// The `1` key.
    NSTD_KEY_1,
    /// The `2` key.
    NSTD_KEY_2,
    /// The `3` key.
    NSTD_KEY_3,
    /// The `4` key.
    NSTD_KEY_4,
    /// The `5` key.
    NSTD_KEY_5,
    /// The `6` key.
    NSTD_KEY_6,
    /// The `7` key.
    NSTD_KEY_7,
    /// The `8` key.
    NSTD_KEY_8,
    /// The `9` key.
    NSTD_KEY_9,

    /// The `A` key.
    NSTD_KEY_A,
    /// The `B` key.
    NSTD_KEY_B,
    /// The `C` key.
    NSTD_KEY_C,
    /// The `D` key.
    NSTD_KEY_D,
    /// The `E` key.
    NSTD_KEY_E,
    /// The `F` key.
    NSTD_KEY_F,
    /// The `G` key.
    NSTD_KEY_G,
    /// The `H` key.
    NSTD_KEY_H,
    /// The `I` key.
    NSTD_KEY_I,
    /// The `J` key.
    NSTD_KEY_J,
    /// The `K` key.
    NSTD_KEY_K,
    /// The `L` key.
    NSTD_KEY_L,
    /// The `M` key.
    NSTD_KEY_M,
    /// The `N` key.
    NSTD_KEY_N,
    /// The `O` key.
    NSTD_KEY_O,
    /// The `P` key.
    NSTD_KEY_P,
    /// The `Q` key.
    NSTD_KEY_Q,
    /// The `R` key.
    NSTD_KEY_R,
    /// The `S` key.
    NSTD_KEY_S,
    /// The `T` key.
    NSTD_KEY_T,
    /// The `U` key.
    NSTD_KEY_U,
    /// The `V` key.
    NSTD_KEY_V,
    /// The `W` key.
    NSTD_KEY_W,
    /// The `X` key.
    NSTD_KEY_X,
    /// The `Y` key.
    NSTD_KEY_Y,
    /// The `Z` key.
    NSTD_KEY_Z,

    /// The `F1` key.
    NSTD_KEY_F1,
    /// The `F2` key.
    NSTD_KEY_F2,
    /// The `F3` key.
    NSTD_KEY_F3,
    /// The `F4` key.
    NSTD_KEY_F4,
    /// The `F5` key.
    NSTD_KEY_F5,
    /// The `F6` key.
    NSTD_KEY_F6,
    /// The `F7` key.
    NSTD_KEY_F7,
    /// The `F8` key.
    NSTD_KEY_F8,
    /// The `F9` key.
    NSTD_KEY_F9,
    /// The `F10` key.
    NSTD_KEY_F10,
    /// The `F11` key.
    NSTD_KEY_F11,
    /// The `F12` key.
    NSTD_KEY_F12,

    /// The ``` key.
    NSTD_KEY_GRAVE,
    /// The `-` key.
    NSTD_KEY_HYPHEN,
    /// The `=` key.
    NSTD_KEY_EQUALS,
    /// The `backspace` or `delete` key.
    NSTD_KEY_BACKSPACE,
    /// The `tab` key.
    NSTD_KEY_TAB,
    /// The `[` key.
    NSTD_KEY_LEFT_BRACKET,
    /// The `]` key.
    NSTD_KEY_RIGHT_BRACKET,
    /// The `/` key.
    NSTD_KEY_SLASH,
    /// The `\` key.
    NSTD_KEY_BACKSLASH,
    /// The `caps lock` key.
    NSTD_KEY_CAPS_LOCK,
    /// The `;` key.
    NSTD_KEY_SEMICOLON,
    /// The `'` key.
    NSTD_KEY_APOSTROPHE,
    /// The `enter` key.
    NSTD_KEY_ENTER,
    /// The `,` key.
    NSTD_KEY_COMMA,
    /// The `.` key.
    NSTD_KEY_PERIOD,

    /// The left `shift` key.
    NSTD_KEY_LEFT_SHIFT,
    /// The right `shift` key.
    NSTD_KEY_RIGHT_SHIFT,
    /// The left `ctrl` key.
    NSTD_KEY_LEFT_CTRL,
    /// The right `ctrl` key.
    NSTD_KEY_RIGHT_CTRL,
    /// The left `alt` key.
    NSTD_KEY_LEFT_ALT,
    /// The right `alt` key.
    NSTD_KEY_RIGHT_ALT,
}
impl Default for NSTDKey {
    #[inline]
    fn default() -> Self {
        Self::NSTD_KEY_UNKNOWN
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
            VirtualKeyCode::Escape => Self::NSTD_KEY_ESC,
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
            VirtualKeyCode::Minus => Self::NSTD_KEY_HYPHEN,
            VirtualKeyCode::Equals => Self::NSTD_KEY_EQUALS,
            VirtualKeyCode::Back => Self::NSTD_KEY_BACKSPACE,
            // Row 3.
            VirtualKeyCode::Tab => Self::NSTD_KEY_TAB,
            VirtualKeyCode::LBracket => Self::NSTD_KEY_LEFT_BRACKET,
            VirtualKeyCode::RBracket => Self::NSTD_KEY_RIGHT_BRACKET,
            VirtualKeyCode::Backslash => Self::NSTD_KEY_BACKSLASH,
            // Row 4.
            VirtualKeyCode::Capital => Self::NSTD_KEY_CAPS_LOCK,
            VirtualKeyCode::Semicolon => Self::NSTD_KEY_SEMICOLON,
            VirtualKeyCode::Apostrophe => Self::NSTD_KEY_APOSTROPHE,
            VirtualKeyCode::Return => Self::NSTD_KEY_ENTER,
            // Row 5.
            VirtualKeyCode::LShift => Self::NSTD_KEY_LEFT_SHIFT,
            VirtualKeyCode::RShift => Self::NSTD_KEY_RIGHT_SHIFT,
            VirtualKeyCode::Comma => Self::NSTD_KEY_COMMA,
            VirtualKeyCode::Period => Self::NSTD_KEY_PERIOD,
            VirtualKeyCode::Slash => Self::NSTD_KEY_SLASH,
            // Row 6.
            VirtualKeyCode::LControl => Self::NSTD_KEY_LEFT_CTRL,
            VirtualKeyCode::RControl => Self::NSTD_KEY_RIGHT_CTRL,
            VirtualKeyCode::LAlt => Self::NSTD_KEY_LEFT_ALT,
            VirtualKeyCode::RAlt => Self::NSTD_KEY_RIGHT_ALT,
            VirtualKeyCode::Space => Self::NSTD_KEY_SPACE,
            // Unknown key.
            _ => Self::default(),
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
            Self::NSTD_KEY_ESC => Ok(VirtualKeyCode::Escape),
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
            Self::NSTD_KEY_HYPHEN => Ok(VirtualKeyCode::Minus),
            Self::NSTD_KEY_EQUALS => Ok(VirtualKeyCode::Equals),
            Self::NSTD_KEY_BACKSPACE => Ok(VirtualKeyCode::Back),
            // Row 3.
            Self::NSTD_KEY_TAB => Ok(VirtualKeyCode::Tab),
            Self::NSTD_KEY_LEFT_BRACKET => Ok(VirtualKeyCode::LBracket),
            Self::NSTD_KEY_RIGHT_BRACKET => Ok(VirtualKeyCode::RBracket),
            Self::NSTD_KEY_BACKSLASH => Ok(VirtualKeyCode::Backslash),
            // Row 4.
            Self::NSTD_KEY_CAPS_LOCK => Ok(VirtualKeyCode::Capital),
            Self::NSTD_KEY_SEMICOLON => Ok(VirtualKeyCode::Semicolon),
            Self::NSTD_KEY_APOSTROPHE => Ok(VirtualKeyCode::Apostrophe),
            Self::NSTD_KEY_ENTER => Ok(VirtualKeyCode::Return),
            // Row 5.
            Self::NSTD_KEY_LEFT_SHIFT => Ok(VirtualKeyCode::LShift),
            Self::NSTD_KEY_RIGHT_SHIFT => Ok(VirtualKeyCode::RShift),
            Self::NSTD_KEY_COMMA => Ok(VirtualKeyCode::Comma),
            Self::NSTD_KEY_PERIOD => Ok(VirtualKeyCode::Period),
            Self::NSTD_KEY_SLASH => Ok(VirtualKeyCode::Slash),
            // Row 6.
            Self::NSTD_KEY_LEFT_CTRL => Ok(VirtualKeyCode::LControl),
            Self::NSTD_KEY_RIGHT_CTRL => Ok(VirtualKeyCode::RControl),
            Self::NSTD_KEY_LEFT_ALT => Ok(VirtualKeyCode::LAlt),
            Self::NSTD_KEY_RIGHT_ALT => Ok(VirtualKeyCode::RAlt),
            Self::NSTD_KEY_SPACE => Ok(VirtualKeyCode::Space),
            // Fallback to implicit conversion.
            _ => Err(Self::Error::MAX),
        }
    }
}

/// Represents a key state.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDKeyState {
    /// A key is/was released.
    NSTD_KEY_STATE_RELEASED,
    /// A key is/was pressed.
    NSTD_KEY_STATE_PRESSED,
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
