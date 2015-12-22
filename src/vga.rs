/// Enumerates the colors available in VGA text mode.
#[allow(dead_code)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

static mut x_pos: usize = 0;
static mut y_pos: usize = 0;

/// Clears the screen to the given background color.
///
/// # Parameters
///
/// * background - The color to clear the screen to.
pub fn clear_screen(background: u16) {
    let max = SCREEN_WIDTH * SCREEN_HEIGHT;

    for i in 0..max {
        unsafe {
            *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
        }
    }
}

/// Writes a character at the given position.
///
/// # Parameters
/// 
/// * c - The character to write.
pub fn write_char(c: char) {
    unsafe {

        // Handle newlines
        if c == '\n' {
            x_pos = 0;
            y_pos = y_pos + 1;
            if y_pos > SCREEN_HEIGHT {
                y_pos = 0;
            }
            return
        }

        // Get the current state of the entry.
        let mut entry = get_entry(x_pos, y_pos);

        // Update the character data.
        entry.data = c as u8;

        write_entry(entry, x_pos, y_pos);

        x_pos = x_pos + 1;
        if x_pos > SCREEN_WIDTH {
            x_pos = 0;
            y_pos = y_pos + 1;

            if y_pos > SCREEN_HEIGHT {
                y_pos = 0;
            }
        }
    }
}

/// Writes a string to the terminal.
///
/// # Parameters
///
/// * s - The string to write.
pub fn write_str(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
}

struct Entry {
    data: u8,
    attrib: u8,
}

impl Entry {
    fn new(c: char, bg: Color, fg: Color) -> Entry {
        Entry{ data: c as u8, attrib: ((bg as u8) << 4) | ((fg as u8) & 0x0F) }
    }

    fn from_raw(raw: u16) -> Entry {
        let data: u8 = (raw & 0x00FF) as u8;
        let attrib: u8 = ((raw & 0xFF00) >> 8) as u8;

        Entry { data: data, attrib: attrib }
    }

    fn raw(self) -> u16 {
        self.data as u16 | ((self.attrib as u16) << 8)
    }
}

#[test]
fn entry_from_raw_test() {
    let bg = Color::LightCyan;
    let fg = Color::Black;
    let c: char = 'n';

    let entry = Entry::new(c, bg, fg);
    assert_eq!(0x6E, entry.data);
    assert_eq!(0xB0, entry.attrib);

    let raw = entry.raw();
    assert_eq!(0xB06E, raw);

    let entry = Entry::from_raw(raw);
    assert_eq!(0x6E, entry.data);
    assert_eq!(0xB0, entry.attrib);
}


fn write_entry(entry: Entry, x: usize, y: usize) {
    let pos = x + y * 80;

    unsafe {
        *((0xb8000 + pos*2) as *mut u16) = entry.raw();
    }
}

fn get_entry(x: usize, y: usize) -> Entry {
    let raw: u16;
    let pos = x + y * 80;

    unsafe {
        raw = *((0xb8000 + pos*2) as *mut u16);
    }

    Entry::from_raw(raw)
}