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

pub fn clear_screen(background: u16) {
    let max = 80 * 25;

    for i in 0..max {
        unsafe {
            *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
        }
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

pub fn write_char(c: char, x: usize, y: usize) {
    // Get the current state of the entry.
    let mut entry = get_entry(x, y);

    // Update the character data.
    entry.data = c as u8;

    write_entry(entry, x, y);
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