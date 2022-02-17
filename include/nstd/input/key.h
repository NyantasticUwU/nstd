#ifndef NSTD_INPUT_KEY_H_INCLUDED
#define NSTD_INPUT_KEY_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// The shift key.
#define NSTD_INPUT_KEY_SHIFT_BIT 0b00000001

/// The ctrl key.
#define NSTD_INPUT_KEY_CTRL_BIT 0b00000010

/// The alt key.
#define NSTD_INPUT_KEY_ALT_BIT 0b00000100

/// The logo key.
#define NSTD_INPUT_KEY_LOGO_BIT 0b00001000

/// Represents a virtual key code.
typedef enum
{
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
    NSTD_KEY_CUT
} NSTDKey;

/// Represents a key state.
typedef enum
{
    /// A key is/was pressed.
    NSTD_KEY_STATE_PRESSED,
    /// A key is/was released.
    NSTD_KEY_STATE_RELEASED
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
