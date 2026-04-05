use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub version: [u8; 4],
    pub platform: [u8; 4],
    pub file_type: [u8; 4],
    pub file_version: [u8; 4],
    pub string_count: u32,
    pub string_offset: u32,
    pub data_offset: u32,
    pub structs: Vec<RawStructDef>,
}

#[derive(Debug, Clone)]
pub struct RawStructDef {
    pub type_code: [u8; 4],
    pub size: u32,
    pub fields: Vec<RawFieldDef>,
}

#[derive(Debug, Clone)]
pub struct RawFieldDef {
    pub label: u32,
    pub type_id: u16,
    pub is_list: bool,
    pub is_struct: bool,
    pub is_reference: bool,
    pub offset: u32,
}

fn is_x360(platform: &[u8; 4]) -> bool {
    platform == b"X360"
}

fn is_big_endian_platform(platform: &[u8; 4]) -> bool {
    platform != b"PC  "
}

pub fn real_version(version: &[u8; 4], platform: &[u8; 4]) -> [u8; 4] {
    if is_x360(platform) && version == b"V4.0" {
        *b"V4.1"
    } else {
        *version
    }
}

fn read_u32<R: Read>(r: &mut R, endian: Endian) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(match endian {
        Endian::Little => u32::from_le_bytes(buf),
        Endian::Big => u32::from_be_bytes(buf),
    })
}

fn unpack_flags(flags: u16) -> io::Result<(bool, bool, bool)> {
    if flags & 0x1FFF != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unknown flag bits: {:04x}", flags & 0x1FFF),
        ));
    }
    Ok((
        flags & 0x8000 != 0,
        flags & 0x4000 != 0,
        flags & 0x2000 != 0,
    ))
}

pub fn read_raw_header<R: Read + Seek>(r: &mut R) -> io::Result<Header> {
    r.seek(SeekFrom::Start(0))?;

    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != b"GFF " {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid magic: {:?}", magic),
        ));
    }

    let mut version = [0u8; 4];
    r.read_exact(&mut version)?;
    if &version != b"V4.0" && &version != b"V4.1" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsupported version: {:?}", version),
        ));
    }

    let mut platform = [0u8; 4];
    r.read_exact(&mut platform)?;

    let endian = if is_big_endian_platform(&platform) {
        Endian::Big
    } else {
        Endian::Little
    };

    let effective_version = real_version(&version, &platform);

    let (file_type, file_version, struct_count, string_count, string_offset, data_offset) =
        if &effective_version == b"V4.0" {
            let mut file_type = [0u8; 4];
            let mut file_version = [0u8; 4];
            r.read_exact(&mut file_type)?;
            r.read_exact(&mut file_version)?;
            let struct_count = read_u32(r, endian)?;
            let data_offset = read_u32(r, endian)?;
            (
                file_type,
                file_version,
                struct_count,
                0,
                data_offset,
                data_offset,
            )
        } else {
            let mut file_type = [0u8; 4];
            let mut file_version = [0u8; 4];
            r.read_exact(&mut file_type)?;
            r.read_exact(&mut file_version)?;
            let struct_count = read_u32(r, endian)?;
            let string_count = read_u32(r, endian)?;
            let string_offset = read_u32(r, endian)?;
            let data_offset = read_u32(r, endian)?;
            (
                file_type,
                file_version,
                struct_count,
                string_count,
                string_offset,
                data_offset,
            )
        };

    let mut struct_headers = Vec::new();
    for _ in 0..struct_count {
        let mut type_code = [0u8; 4];
        r.read_exact(&mut type_code)?;
        let field_count = read_u32(r, endian)?;
        let field_offset = read_u32(r, endian)?;
        let size = read_u32(r, endian)?;
        struct_headers.push((type_code, field_count, field_offset, size));
    }

    let mut structs = Vec::new();
    for (type_code, field_count, field_offset, size) in struct_headers {
        r.seek(SeekFrom::Start(field_offset as u64))?;
        let mut fields = Vec::new();

        for _ in 0..field_count {
            let label = read_u32(r, endian)?;
            let type_and_flags = read_u32(r, endian)?;
            let offset = read_u32(r, endian)?;

            let flags = (type_and_flags >> 16) as u16;
            let type_id = (type_and_flags & 0xFFFF) as u16;
            let (is_list, is_struct, is_reference) = unpack_flags(flags)?;

            fields.push(RawFieldDef {
                label,
                type_id,
                is_list,
                is_struct,
                is_reference,
                offset,
            });
        }

        structs.push(RawStructDef {
            type_code,
            size,
            fields,
        });
    }

    Ok(Header {
        version,
        platform,
        file_type,
        file_version,
        string_count,
        string_offset,
        data_offset,
        structs,
    })
}
