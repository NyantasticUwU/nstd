#ifndef NSTD_INPUT_KEY_H_INCLUDED
#define NSTD_INPUT_KEY_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// Represents a virtual key code.
typedef enum
{
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
    NSTD_KEY_RIGHT_ALT
} NSTDKey;

/// Represents a key state.
typedef enum
{
    /// A key is/was released.
    NSTD_KEY_STATE_RELEASED,
    /// A key is/was pressed.
    NSTD_KEY_STATE_PRESSED
} NSTDKeyState;

/// Represents a key event.
typedef struct
{
    /// The keyboard key.
    NSTDKey key;
    /// The state of the key.
    NSTDKeyState state;
    /// The key's scan code.
    NSTDUInt32 scan_code;
} NSTDKeyEvent;

#endif
