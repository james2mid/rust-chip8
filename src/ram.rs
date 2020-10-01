// Code is able to access up to 4KB (4, 096 bytes) of RAM
// Start: 0x000 (0)
//   End: 0x1FF (4095)

pub struct Ram {
  mem: [u8; 4096]
}

// Sprites of "0" through "F" stored between 0x00 and 0x50
// Each sprite is 8x5 pixels
const SPRITES: [[u8; 5]; 16] = [
  [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
  [0x20, 0x60, 0x20, 0x20, 0x70], // 1
  [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
  [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
  [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
  [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
  [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
  [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
  [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
  [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
  [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
  [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
  [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
  [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
  [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
  [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

impl Ram {
  pub fn new() -> Ram {
    let mut ram = Ram { mem: [0; 4096] };

    let mut i = 0;
    for sprite in SPRITES.iter() {
      for ch in sprite {
        ram.mem[i] = *ch;
        i += 1;
      }
    }

    return ram
  }

  pub fn write_byte(&mut self, address: u16, value: u8) {
    self.mem[address as usize] = value;
  }

  pub fn read_byte(&mut self, address: u16) -> u8 {
    return self.mem[address as usize]
  }
}