// Copyright 2018 pdb Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// PDBs contain PE section headers in one or two streams. `pdb::pe` is responsible for parsing them.

use common::*;

/// A PE `IMAGE_SECTION_HEADER`, as described in [the Microsoft documentation](https://msdn.microsoft.com/en-us/library/windows/desktop/ms680341(v=vs.85).aspx).
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct ImageSectionHeader {
    /// An 8-byte, null-padded UTF-8 string. There is no terminating null character if the string is
    /// exactly eight characters long. For longer names, this member contains a forward slash (`/`)
    /// followed by an ASCII representation of a decimal number that is an offset into the string
    /// table. Executable images do not use a string table and do not support section names longer
    /// than eight characters.
    pub name: [u8; 8],

    /// The file address.
    pub physical_address: u32,

    /// The address of the first byte of the section when loaded into memory, relative to the image
    /// base. For object files, this is the address of the first byte before relocation is applied.
    pub virtual_address: u32,

    /// The size of the initialized data on disk, in bytes. This value must be a multiple of the
    /// `FileAlignment` member of the `IMAGE_OPTIONAL_HEADER` structure. If this value is less than
    /// the `VirtualSize` member, the remainder of the section is filled with zeroes. If the section
    /// contains only uninitialized data, the member is zero.
    pub size_of_raw_data: u32,

    /// A file pointer to the first page within the COFF file. This value must be a multiple of the
    /// `FileAlignment` member of the `IMAGE_OPTIONAL_HEADER` structure. If a section contains only
    /// uninitialized data, set this member is zero.
    pub pointer_to_raw_data: u32,

    /// A file pointer to the beginning of the relocation entries for the section. If there are no
    /// relocations, this value is zero.
    pub pointer_to_relocations: u32,

    /// A file pointer to the beginning of the line-number entries for the section. If there are no
    /// COFF line numbers, this value is zero.
    pub pointer_to_line_numbers: u32,

    /// The number of relocation entries for the section. This value is zero for executable images.
    pub number_of_relocations: u16,

    /// The number of line-number entries for the section.
    pub number_of_line_numbers: u16,

    /// The characteristics of the image.
    pub characteristics: u32,
}

impl ImageSectionHeader {
    pub fn parse(parse_buffer: &mut ParseBuffer) -> Result<Self> {
        let name_bytes = parse_buffer.take(8)?;

        Ok(ImageSectionHeader{
            name: [
                name_bytes[0], name_bytes[1], name_bytes[2], name_bytes[3],
                name_bytes[4], name_bytes[5], name_bytes[6], name_bytes[7]
            ],
            physical_address: parse_buffer.parse_u32()?,
            virtual_address: parse_buffer.parse_u32()?,
            size_of_raw_data: parse_buffer.parse_u32()?,
            pointer_to_raw_data: parse_buffer.parse_u32()?,
            pointer_to_relocations: parse_buffer.parse_u32()?,
            pointer_to_line_numbers: parse_buffer.parse_u32()?,
            number_of_relocations: parse_buffer.parse_u16()?,
            number_of_line_numbers: parse_buffer.parse_u16()?,
            characteristics: parse_buffer.parse_u32()?,
        })
    }

    pub fn name(&self) -> RawString {
        let first_nul = self.name.iter().position(|ch| *ch == 0);
        let name_bytes = &self.name[0..first_nul.unwrap_or(self.name.len())];
        RawString::from(name_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_section_header() {
        let bytes: Vec<u8> = vec![
            0x2E, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00,
            0x48, 0x35, 0x09, 0x00, 0x00, 0xD0, 0x1E, 0x00,
            0x00, 0xFE, 0x00, 0x00, 0x00, 0xA2, 0x1E, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0xC8
        ];

        let mut parse_buffer = ParseBuffer::from(bytes.as_slice());

        let ish = ImageSectionHeader::parse(&mut parse_buffer).expect("parse");
        assert_eq!(&ish.name, b".data\0\0\0");
        assert_eq!(ish.name(), RawString::from(".data"));
        assert_eq!(ish.physical_address, 0x93548);
        assert_eq!(ish.virtual_address, 0x1ed000);
        assert_eq!(ish.size_of_raw_data, 0xfe00);
        assert_eq!(ish.pointer_to_raw_data, 0x1ea200);
        assert_eq!(ish.pointer_to_relocations, 0);
        assert_eq!(ish.pointer_to_line_numbers, 0);
        assert_eq!(ish.number_of_relocations, 0);
        assert_eq!(ish.number_of_line_numbers, 0);
        assert_eq!(ish.characteristics, 0xc8000040);
    }
}