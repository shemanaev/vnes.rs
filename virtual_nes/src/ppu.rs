use super::mapper::Mapper;
use super::memory::Memory;
use super::rom::Mirroring;
use std::cell::RefCell;
use std::rc::Rc;
use image::{RgbImage, Rgb};

#[derive(Debug)]
pub struct PPU {
    mapper: Rc<RefCell<Box<dyn Mapper>>>,
    palette: [u8; 32],
    colors: RgbImage,
    name_table: Vec<u8>,
    pub oam_data: Vec<u8>,

    // back: Box<RgbImage>,
    // front: Box<RgbImage>,
    current_front: usize,
    buffers: [RgbImage; 2],

    // background temporary variables
	name_table_byte: u8,
	attribute_table_byte: u8,
	low_tile_byte: u8,
	high_tile_byte: u8,
    tile_data: u64,

    // sprite temporary variables
	sprite_count: usize,
	sprite_patterns: [u32; 8],
	sprite_positions: [u8; 8],
	sprite_priorities: [u8; 8],
    sprite_indexes: [u8; 8],

    cycle: usize,     // 0-340
    scan_line: i32, // 0-261, 0-239=visible, 240=post, 241-260=vblank, 261=pre
    frame: usize,

    // PPU registers
	v: u16,  // current vram address (15 bit)
	t: u16,  // temporary vram address (15 bit)
	x: u8,   // fine x scroll (3 bit)
	w: u8,   // write toggle (1 bit)
    f: u8,   // even/odd frame flag (1 bit)
    register: u8,

    // NMI
    nmi_occurred: bool,
    nmi_output: bool,
    nmi_previous: bool,
    nmi_delay: u8,

    // $2000
    flag_name_table: u8,
    flag_increment: u8,
    flag_sprite_table: u8,
    flag_background_table: u8,
    flag_sprite_size: u8,
    flag_master_slave: u8,

    // $2001
    flag_grayscale: u8,
    flag_show_left_background: u8,
    flag_show_left_sprites: u8,
    flag_show_background: u8,
    flag_show_sprites: u8,
    flag_red_tint: u8,
    flag_green_tint: u8,
    flag_blue_tint: u8,

    // $2002
    flag_sprite_zero_hit: u8,
    flag_sprite_overflow: u8,

    // $2003
    pub oam_address: usize,

    // $2007
    buffered_data: u8,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..0x2000 => self.mapper.borrow().read(address),
            0x2000..0x3F00 => {
                let address = mirror_address(self.mapper.borrow().get_mirroring(), address);
                self.name_table[address % 2048]
            }
            0x3F00..0x4000 => self.read_palette(address % 32),
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..0x2000 => self.mapper.borrow_mut().write(address, value),
            0x2000..0x3F00 => {
                let address = mirror_address(self.mapper.borrow().get_mirroring(), address);
                self.name_table[address % 2048] = value;
            }
            0x3F00..0x4000 => self.write_palette(address % 32, value),
            _ => {}
        }
    }
}

const MIRROR_LOOKUP: [[usize; 4]; 5] = [
    [0, 0, 1, 1],
	[0, 1, 0, 1],
	[0, 0, 0, 0],
	[1, 1, 1, 1],
    [0, 1, 2, 3],
];

fn mirror_address(mirroring: &Mirroring, address: u16) -> usize {
    let address = (address - 0x2000) % 0x1000;
    let table = address as usize / 0x4000;
    let offset = address as usize % 0x4000;
    0x2000 + MIRROR_LOOKUP[*mirroring as usize][table] * 0x4000 + offset
}

impl PPU {
    pub fn new(mapper: Rc<RefCell<Box<dyn Mapper>>>) -> PPU {
        PPU {
            mapper: mapper,
            palette: [0; 32],
            colors: Self::init_palette(),
            name_table: vec![0; 2048],
            oam_data: vec![0; 256],

            // back: Box::new(RgbImage::new(256, 240)),
            // front: Box::new(RgbImage::new(256, 240)),
            current_front: 0,
            buffers: [RgbImage::new(256, 240), RgbImage::new(256, 240)],

            name_table_byte: 0,
            attribute_table_byte: 0,
            low_tile_byte: 0,
            high_tile_byte: 0,
            tile_data: 0,

            sprite_count: 0,
            sprite_patterns: [0; 8],
            sprite_positions: [0; 8],
            sprite_priorities: [0; 8],
            sprite_indexes: [0; 8],

            cycle: 0,
            scan_line: 0,
            frame: 0,

            v: 0,
            t: 0,
            x: 0,
            w: 0,
            f: 0,
            register: 0,

            // NMI
            nmi_occurred: true,
            nmi_output: false,
            nmi_previous: false,
            nmi_delay: 0,

            // $2000
            flag_name_table: 0,
            flag_increment: 0,
            flag_sprite_table: 0,
            flag_background_table: 0,
            flag_sprite_size: 0,
            flag_master_slave: 0,

            // $2001
            flag_grayscale: 0,
            flag_show_left_background: 0,
            flag_show_left_sprites: 0,
            flag_show_background: 0,
            flag_show_sprites: 0,
            flag_red_tint: 0,
            flag_green_tint: 0,
            flag_blue_tint: 0,

            // $2002
            flag_sprite_zero_hit: 0,
            flag_sprite_overflow: 1,

            // $2003
            oam_address: 0,

            // $2007
            buffered_data: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cycle = 0; // 340
        self.scan_line = 241; // 240
        self.frame = 0;
        self.write_control(0);
        self.write_mask(0);
        self.write_oam_address(0);
    }

    pub fn get_pixels(&self) -> &[u8] {
        self.buffers[self.current_front].as_ref()
    }

    #[cfg(test)]
    pub fn get_cycle(&self) -> usize {
        self.cycle
    }

    #[cfg(test)]
    pub fn get_sl(&self) -> i32 {
        match self.scan_line {
            261 => -1,
            v => v,
        }
    }

    fn read_palette(&self, address: u16) -> u8 {
        // let address = if address >= 16 && address % 4 == 0 {
        //     address - 16
        // } else {
        //     address
        // } as usize;
        let address = if address & 0x0F == 0 {
            0
        } else {
            address
        } as usize;
        self.palette[address & 0x1F]
    }

    fn write_palette(&mut self, address: u16, value: u8) {
        // let address = if address >= 16 && address % 4 == 0 {
        //     address - 16
        // } else {
        //     address
        // } as usize;
        let address = if address & 0x0F == 0 {
            0
        } else {
            address
        } as usize;
        self.palette[address & 0x1F] = value;
    }

    pub fn read_register(&mut self, address: u16) -> u8 {
        match address {
            0x2002 => self.read_status(),
            0x2004 => self.read_oam_data(),
            0x2007 => self.read_data(),
            _ => 0
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        self.register = value;
        match address {
            0x2000 => self.write_control(value),
            0x2001 => self.write_mask(value),
            0x2003 => self.write_oam_address(value),
            0x2004 => self.write_oam_data(value),
            0x2005 => self.write_scroll(value),
            0x2006 => self.write_address(value),
            0x2007 => self.write_data(value),
            // 0x4014 => self.write_dma(value),
            _ => {}
        }
    }

    fn write_control(&mut self, value: u8) {
        self.flag_name_table = (value >> 0) & 3;
        self.flag_increment = (value >> 2) & 1;
        self.flag_sprite_table = (value >> 3) & 1;
        self.flag_background_table = (value >> 4) & 1;
        self.flag_sprite_size = (value >> 5) & 1;
        self.flag_master_slave = (value >> 6) & 1;
        self.nmi_output = (value >> 7) & 1 == 1;
        self.nmi_change();
        self.t = (self.t & 0xF3FF) | ((value as u16 & 0x03) << 10);
    }

    fn write_mask(&mut self, value: u8) {
        // println!("WRITEMASK: {:02X}", value);
        self.flag_grayscale = (value >> 0) & 1;
        self.flag_show_left_background = (value >> 1) & 1;
        self.flag_show_left_sprites = (value >> 2) & 1;
        self.flag_show_background = (value >> 3) & 1;
        self.flag_show_sprites = (value >> 4) & 1;
        self.flag_red_tint = (value >> 5) & 1;
        self.flag_green_tint = (value >> 6) & 1;
        self.flag_blue_tint = (value >> 7) & 1;
    }

    fn read_status(&mut self) -> u8 {
        let mut result = self.register & 0x1F;
        result |= self.flag_sprite_overflow << 5;
        result |= self.flag_sprite_zero_hit << 6;
        if self.nmi_occurred {
            result |= 1 << 7;
        }
        self.nmi_occurred = false;
        self.nmi_change();
        self.w = 0;
        result
    }

    fn write_oam_address(&mut self, value: u8) {
        self.oam_address = value as usize;
    }

    fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_address]
    }

    fn write_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_address] = value;
        self.oam_address += 1;
    }

    fn write_scroll(&mut self, value: u8) {
        if self.w == 0 {
            self.t = (self.t & 0xFFE0) | (value as u16 >> 3);
            self.x = value & 0x07;
            self.w = 1;
        } else {
            self.t = (self.t & 0x8FFF) | ((value as u16 & 0x07) << 12);
            self.t = (self.t & 0xFC1F) | ((value as u16 & 0xF8) << 2);
            self.w = 0;
        }
    }

    fn write_address(&mut self, value: u8) {
        if self.w == 0 {
            self.t = (self.t & 0x80FF) | ((value as u16 & 0x3F) << 8);
            self.w = 1;
        } else {
            self.t = (self.t & 0xFF00) | (value as u16);
            self.v = self.t;
            self.w = 0;
        }
    }

    fn read_data(&mut self) -> u8 {
        let mut value = self.read(self.v);

        if self.v % 0x4000 < 0x3F00 {
            let buffered = self.buffered_data;
            self.buffered_data = value;
            value = buffered;
        } else {
            self.buffered_data = self.read(self.v - 0x1000);
        }

        if self.flag_increment == 0 {
            self.v += 1;
        } else {
            self.v += 0x20;
        }
        value
    }

    fn write_data(&mut self, value: u8) {
        self.write(self.v, value);
        if self.flag_increment == 0 {
            self.v += 1;
        } else {
            self.v += 0x20;
        }
    }

    // fn write_dma(&mut self, value: u8) {
    //     let address = (value as u16) << 8;
    //     for i in 0..256 {
    //         self.oam_data[self.oam_address as usize] = self.read(address);
    //     }
    // }

    fn nmi_change(&mut self) {
        let nmi = self.nmi_output && self.nmi_occurred;
        if nmi && !self.nmi_previous {
            self.nmi_delay = 15;
        }
        self.nmi_previous = nmi;
    }

    fn increment_x(&mut self) {
        if self.v & 0x001F == 31 {
            self.v &= 0xFFE0;
            self.v ^= 0x4000;
        } else {
            self.v = self.v.wrapping_add(1);
        }
    }

    fn increment_y(&mut self) {
        if self.v & 0x7000 != 0x7000 {
            self.v = self.v.wrapping_add(0x1000);
        } else {
            self.v &= 0x8FFF;
            let mut y = (self.v & 0x03E0) >> 5;
            match y { 
                29 => {
                    y = 0;
                    self.v ^= 0x0800;
                }
                31 => y = 0,
                _ => y += 1,
            }
            self.v = (self.v & 0xFC1F) | (y << 5);
        }
    }

    fn copy_x(&mut self) {
        self.v = (self.v & 0xFBE0) | (self.t & 0x041F);
    }

    fn copy_y(&mut self) {
        self.v = (self.v & 0x841F) | (self.t & 0x7BE0);
    }

    fn set_vblank(&mut self) {
        self.current_front = if self.current_front == 0 { 1 } else { 0 };
        self.nmi_occurred = true;
        self.nmi_change();
    }

    fn clear_vblank(&mut self) {
        self.nmi_occurred = false;
        self.nmi_change();
    }

    fn fetch_name_table_byte(&mut self) {
        let address = 0x2000 | (self.v & 0x0FFF);
        self.name_table_byte = self.read(address);
    }

    fn fetch_attribute_table_byte(&mut self) {
        let v = self.v;
        let address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        let shift = ((v >> 4) & 4) | (v & 2);
        self.attribute_table_byte = ((self.read(address) >> shift) & 3) << 2;
    }

    fn fetch_low_tile_byte(&mut self) {
        let y = (self.v >> 12) & 7;
        let table = self.flag_background_table as u16;
        let tile = self.name_table_byte as u16;
        let address = 0x1000 * table + tile * 16 + y;
        self.low_tile_byte = self.read(address);
    }

    fn fetch_high_tile_byte(&mut self) {
        let y = (self.v >> 12) & 7;
        let table = self.flag_background_table as u16;
        let tile = self.name_table_byte as u16;
        let address = 0x1000 * table + tile * 16 + y;
        self.high_tile_byte = self.read(address + 8);
    }

    fn store_tile_data(&mut self) {
        let mut data: u32 = 0;
        for _ in 0..8 {
            let a = self.attribute_table_byte;
            let p1 = (self.low_tile_byte & 0x80) >> 7;
            let p2 = (self.high_tile_byte & 0x80) >> 6;
            self.low_tile_byte <<= 1;
            self.high_tile_byte <<= 1;
            data <<= 4;
            data |= (a | p1 | p2) as u32;
        }
        self.tile_data |= data as u64;
    }

    fn fetch_tile_data(&self) -> u32 {
        (self.tile_data >> 32) as u32
    }

    fn background_pixel(&self) -> u8 {
        if self.flag_show_background == 0 {
            0
        } else {
            let data = self.fetch_tile_data() >> ((7 - self.x) * 4);
            (data & 0x0F) as u8
        }
    }

    fn sprite_pixel(&self) -> (u8, u8) {
        if self.flag_show_sprites == 0 {
            return (0, 0)
        }
        for i in 0..self.sprite_count {
            let mut offset = (self.cycle as i32 - 1) - self.sprite_positions[i] as i32;
            if offset < 0 || offset > 7 {
                continue
            }
            offset = 7 - offset;
            let color = (self.sprite_patterns[i] >> (offset * 4)) & 0x0F;
            if color % 4 == 0 {
                continue
            }
            return (i as u8, color as u8)
        }
        (0, 0)
    }

    fn render_pixel(&mut self) {
        let x = self.cycle - 1;
        let y = self.scan_line;
        let mut background = self.background_pixel();
        let (i, mut sprite) = self.sprite_pixel();
        if x < 8 && self.flag_show_left_background == 0 {
            background = 0;
        }
        if x < 8 && self.flag_show_left_sprites == 0 {
            sprite = 0;
        }
        let b = background % 4 != 0;
        let s = sprite % 4 != 0;

        let color: u8;
        if !b && !s {
            color = 0;
        } else if !b && s {
            color = sprite | 0x10;
        } else if b && !s {
            color = background;
        } else {
            if self.sprite_indexes[i as usize] == 0 && x < 255 {
                self.flag_sprite_zero_hit = 1;
            }
            if self.sprite_priorities[i as usize] == 0 {
                color = sprite | 0x10;
            } else {
                color = background;
            }
        }

        let address = self.read_palette(color as u16) % 64;
        let c = self.colors.get_pixel(address as u32, 0).clone();
        let back = if self.current_front == 0 { 1 } else { 0 };
        self.buffers[back].put_pixel(x as u32, y as u32, c);
        // println!("RENDER: {:02X} {}, {}, {:?}", address, x, y, c);
    }

    fn fetch_sprite_pattern(&mut self, i: usize, row: u16) -> u32 {
        let mut row = row;
        let mut tile = self.oam_data[i * 4 + 1] as u16;
        let attributes = self.oam_data[i * 4 + 2];
        let address: u16;
        if self.flag_sprite_size == 0 {
            if attributes & 0x80 == 0x80 {
                row = 7 - row;
            }
            let table = self.flag_sprite_table as u16;
            address = 0x1000 * table + tile * 16 + row;
        } else {
            if attributes & 0x80 == 0x80 {
                row = 15 - row;
            }
            let table = tile & 1;
            tile &= 0xFE;
            if row > 7 {
                tile += 1;
                row -= 8;
            }
            address = 0x1000 * table + tile * 16 + row;
        }
        let a = (attributes & 3) << 2;
        let mut low_byte = self.read(address);
        let mut high_byte = self.read(address);
        let mut data: u32 = 0;
        for _ in 0..8 {
            let p1: u8;
            let p2: u8;
            if attributes & 0x40 == 0x40 {
                p1 = (low_byte & 1) << 0;
                p2 = (high_byte & 1) << 1;
                low_byte >>= 1;
                high_byte >>= 1;
            } else {
                p1 = (low_byte & 0x80) >> 7;
                p2 = (high_byte & 0x80) >> 6;
                low_byte <<= 1;
                high_byte <<= 1;
            }
            data <<= 4;
            data |= (a | p1 | p2) as u32;
        }

        data
    }

    fn evaluate_sprites(&mut self) {
        let h = match self.flag_sprite_size {
            0 => 8,
            _ => 16,
        };
        let mut count = 0;
        for i in 0..64 {
            let y = self.oam_data[i * 4 + 0];
            let a = self.oam_data[i * 4 + 2];
            let x = self.oam_data[i * 4 + 3];
            let row = self.scan_line - y as i32;
            if row < 0 || row >= h {
                continue
            }
            if count < 8 {
                self.sprite_patterns[count] = self.fetch_sprite_pattern(i, row as u16);
                self.sprite_positions[count] = x;
                self.sprite_priorities[count] = (a >> 5) & 1;
                self.sprite_indexes[count] = i as u8;
            }
            count += 1;
        }
        if count > 8 {
            count = 8;
            self.flag_sprite_overflow = 1;
        }
        self.sprite_count = count;
    }

    // true if should trigger NMI
    pub fn tick(&mut self) -> bool {
        let mut result = false;

        // println!("delay: {}, output: {}, occured: {}", self.nmi_delay, self.nmi_output, self.nmi_occurred);

        if self.nmi_delay > 0 {
            self.nmi_delay -= 1;
            if self.nmi_delay == 0 && self.nmi_output && self.nmi_occurred {
                result = true;
            }
        }

        if self.flag_show_background != 0 || self.flag_show_sprites != 0 {
            if self.f == 1 && self.scan_line == 261 && self.cycle == 339 {
                self.cycle = 0;
                self.scan_line = 0;
                self.frame += 1;
                self.f ^= 1;
                return result
            }
        }

        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scan_line += 1;
            if self.scan_line > 261 {
                self.scan_line = 0;
                self.frame += 1;
                self.f ^= 1;
            }
        }

        result
    }

    pub fn step(&mut self) {
        let rendering_enabled = self.flag_show_background != 0 || self.flag_show_sprites != 0;
        let pre_line = self.scan_line == 261;
        let visible_line = self.scan_line < 240;
        let render_line = pre_line || visible_line;
        let pre_fetch_cycle = self.cycle >= 321 && self.cycle <= 336;
        let visible_cycle = self.cycle >= 1 && self.cycle <= 256;
        let fetch_cycle = pre_fetch_cycle || visible_cycle;

        if rendering_enabled {
            if visible_line && visible_cycle {
                self.render_pixel();
            }
            if render_line && fetch_cycle {
                self.tile_data <<= 4;
                match self.cycle % 8 {
                    1 => self.fetch_name_table_byte(),
                    3 => self.fetch_attribute_table_byte(),
                    5 => self.fetch_low_tile_byte(),
                    7 => self.fetch_high_tile_byte(),
                    0 => self.store_tile_data(),
                    _ => {}
                }
            }
            if pre_line && self.cycle >= 280 && self.cycle <= 304 {
                self.copy_y();
            }
            if render_line {
                if fetch_cycle && self.cycle % 8 == 0 {
                    self.increment_x();
                }
                if self.cycle == 256 {
                    self.increment_y();
                }
                if self.cycle == 257 {
                    self.copy_x();
                }
            }
        }

        if rendering_enabled {
            if self.cycle == 257 {
                if visible_line {
                    self.evaluate_sprites();
                } else {
                    self.sprite_count = 0;
                }
            }
        }

        if self.scan_line == 241 && self.cycle == 1 {
            // println!("VBLANK");
            self.set_vblank();
        }
        if pre_line && self.cycle == 1 {
            self.clear_vblank();
            self.flag_sprite_zero_hit = 0;
            self.flag_sprite_overflow = 0;
        }
    }

    fn init_palette() -> RgbImage {
        let colors = [
    		0x666666, 0x002A88, 0x1412A7, 0x3B00A4, 0x5C007E, 0x6E0040, 0x6C0600, 0x561D00,
    		0x333500, 0x0B4800, 0x005200, 0x004F08, 0x00404D, 0x000000, 0x000000, 0x000000,
    		0xADADAD, 0x155FD9, 0x4240FF, 0x7527FE, 0xA01ACC, 0xB71E7B, 0xB53120, 0x994E00,
    		0x6B6D00, 0x388700, 0x0C9300, 0x008F32, 0x007C8D, 0x000000, 0x000000, 0x000000,
	    	0xFFFEFF, 0x64B0FF, 0x9290FF, 0xC676FF, 0xF36AFF, 0xFE6ECC, 0xFE8170, 0xEA9E22,
		    0xBCBE00, 0x88D800, 0x5CE430, 0x45E082, 0x48CDDE, 0x4F4F4F, 0x000000, 0x000000,
		    0xFFFEFF, 0xC0DFFF, 0xD3D2FF, 0xE8C8FF, 0xFBC2FF, 0xFEC4EA, 0xFECCC5, 0xF7D8A5,
		    0xE4E594, 0xCFEF96, 0xBDF4AB, 0xB3F3CC, 0xB5EBF2, 0xB8B8B8, 0x000000, 0x000000,
        ];
        let mut result = RgbImage::new(64, 1);

        for (i, c) in colors.iter().enumerate() {
            let r = (c >> 16) as u8;
            let g = (c >> 8) as u8;
            let b = *c as u8;
            result.put_pixel(i as u32, 0, Rgb([r, g, b]));
        }

        result
    }
}
