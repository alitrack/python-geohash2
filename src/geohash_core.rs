use std::error::Error;
use std::fmt;

pub const INT_UNIT: usize = 64;

const BASE32: &[u8; 32] = b"0123456789bcdefghjkmnpqrstuvwxyz";
const INVALID_BASE32: u8 = 0xff;
const MIDPOINT: u64 = 0x8000_0000_0000_0000;
const MANTISSA_MASK: u64 = 0x000f_ffff_ffff_ffff;
const HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;
// IEEE 754 binary64 for nextafter(90.0, -inf).
const NORTH_POLE_ADJACENT_LATITUDE_BITS: u64 = 0x4056_7fff_ffff_ffff;
// Build the hot-path lookup tables at compile time instead of storing literals.
const INTERLEAVE_MAP: [u16; 256] = build_interleave_map();
// 256 bytes, packed as upper nibble = longitude bits, lower nibble = latitude bits.
const DEINTERLEAVE_BYTE_MAP: [u8; 256] = build_deinterleave_byte_map();
const BASE32_DECODE_MAP: [u8; 128] = build_base32_decode_map();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeohashError {
    InvalidCode,
    InvalidArgument,
}

impl fmt::Display for GeohashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCode => write!(f, "geohash code is [0123456789bcdefghjkmnpqrstuvwxyz]+"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
        }
    }
}

impl Error for GeohashError {}

type Result<T> = std::result::Result<T, GeohashError>;

pub fn normalize_latitude(latitude: f64) -> Result<(f64, bool)> {
    if latitude >= 90.0 {
        if latitude == 90.0 {
            // Geohash latitude is half-open; use the nearest representable cell below 90.
            return Ok((f64::from_bits(NORTH_POLE_ADJACENT_LATITUDE_BITS), true));
        }
        return Err(GeohashError::InvalidArgument);
    }
    if latitude < -90.0 {
        return Err(GeohashError::InvalidArgument);
    }
    Ok((latitude, false))
}

fn double_to_i64(input: f64) -> Option<u64> {
    if !(-1.0..1.0).contains(&input) {
        return None;
    }

    let bits = input.to_bits();
    let sign = bits >> 63;
    let exp = ((bits >> 52) & 0x7ff) as i32;
    if exp == 0 {
        return Some(MIDPOINT);
    }
    if exp == 0x7ff {
        return None;
    }

    let mut value = (bits & MANTISSA_MASK) | HIDDEN_BIT;
    let shift = exp - 0x3ff + 11;
    if shift > 0 {
        value <<= shift as u32;
    } else {
        value >>= (-shift) as u32;
    }

    if sign != 0 {
        Some(MIDPOINT - value)
    } else {
        Some(MIDPOINT + value)
    }
}

fn i64_to_double(mut input: u64) -> f64 {
    if input == MIDPOINT {
        return 0.0;
    }

    let mut sign = 0_u64;
    if input < MIDPOINT {
        sign = 1;
        input = MIDPOINT - input;
    } else {
        input -= MIDPOINT;
    }

    let mut leading_zero_count = 0_u64;
    for i in 0..64 {
        if input >> (63 - i) != 0 {
            leading_zero_count = i;
            break;
        }
    }

    let mantissa_source = if leading_zero_count > 11 {
        input << (leading_zero_count - 11)
    } else {
        input >> (11 - leading_zero_count)
    };
    let mut bits = ((0x3ff - leading_zero_count) << 52) | (mantissa_source & MANTISSA_MASK);
    if sign != 0 {
        bits |= 0x8000_0000_0000_0000;
    }
    f64::from_bits(bits)
}

const fn spread_byte(value: u8) -> u16 {
    let mut output = 0_u16;
    let mut bit = 0;
    while bit < 8 {
        // Place each source bit into every other bit position: abcdefgh -> 0a0b0c...
        output |= (((value as u16) >> bit) & 1) << (bit * 2);
        bit += 1;
    }
    output
}

const fn build_interleave_map() -> [u16; 256] {
    let mut map = [0_u16; 256];
    let mut i = 0;
    while i < 256 {
        map[i] = spread_byte(i as u8);
        i += 1;
    }
    map
}

const fn deinterleave_byte(interleaved: u8) -> u8 {
    let mut upper = 0_u8;
    let mut lower = 0_u8;
    let mut bit = 4;
    while bit > 0 {
        bit -= 1;
        upper = (upper << 1) | ((interleaved >> (bit * 2 + 1)) & 1);
        lower = (lower << 1) | ((interleaved >> (bit * 2)) & 1);
    }
    (upper << 4) | lower
}

const fn build_deinterleave_byte_map() -> [u8; 256] {
    let mut map = [0_u8; 256];
    let mut i = 0;
    while i < 256 {
        map[i] = deinterleave_byte(i as u8);
        i += 1;
    }
    map
}

const fn build_base32_decode_map() -> [u8; 128] {
    let mut map = [INVALID_BASE32; 128];
    let mut i = 0;
    while i < BASE32.len() {
        let lowercase = BASE32[i] as usize;
        map[lowercase] = i as u8;
        if lowercase >= b'a' as usize && lowercase <= b'z' as usize {
            map[lowercase - 32] = i as u8;
        }
        i += 1;
    }
    map
}

fn interleave(upper: u8, lower: u8) -> u16 {
    (INTERLEAVE_MAP[upper as usize] << 1) | INTERLEAVE_MAP[lower as usize]
}

fn deinterleave(interleaved: u16) -> (u8, u8) {
    let upper_packed = DEINTERLEAVE_BYTE_MAP[(interleaved >> 8) as usize];
    let lower_packed = DEINTERLEAVE_BYTE_MAP[(interleaved & 0xff) as usize];
    let upper = (upper_packed & 0xf0) | (lower_packed >> 4);
    let lower = ((upper_packed & 0x0f) << 4) | (lower_packed & 0x0f);
    (upper, lower)
}

fn base32_value(byte: u8) -> Option<u8> {
    if byte as usize >= BASE32_DECODE_MAP.len() {
        return None;
    }
    match BASE32_DECODE_MAP[byte as usize] {
        INVALID_BASE32 => None,
        value => Some(value),
    }
}

fn push_geohash_digit(output: &mut String, value: u16) {
    output.push(BASE32[(value & 0x1f) as usize] as char);
}

fn bit_mask(bit_count: usize) -> u16 {
    ((1_u32 << bit_count) - 1) as u16
}

fn read_interleaved_bits(interleaved: &[u16], bit_index: usize, bit_count: usize) -> u32 {
    let mut output = 0_u32;
    let mut remaining = bit_count;
    let mut word_index = bit_index / 16;
    let mut word_offset = bit_index % 16;

    while remaining > 0 {
        let available = 16 - word_offset;
        let take = remaining.min(available);
        let shift = available - take;
        let word = interleaved.get(word_index).copied().unwrap_or(0);
        output = (output << take) | u32::from((word >> shift) & bit_mask(take));
        remaining -= take;
        word_index += 1;
        word_offset = 0;
    }
    output
}

fn write_interleaved_bits(interleaved: &mut [u16], bit_index: usize, bit_count: usize, value: u32) {
    let mut remaining = bit_count;
    let mut source_shift = bit_count;
    let mut word_index = bit_index / 16;
    let mut word_offset = bit_index % 16;

    while remaining > 0 {
        let available = 16 - word_offset;
        let take = remaining.min(available);
        source_shift -= take;
        let bits = ((value >> source_shift) as u16) & bit_mask(take);
        interleaved[word_index] |= bits << (available - take);
        remaining -= take;
        word_index += 1;
        word_offset = 0;
    }
}

fn interleaved_to_geohash(interleaved: &[u16], precision: usize) -> String {
    let mut output = String::with_capacity(precision);
    let full_chunks = precision / 4;
    for chunk in 0..full_chunks {
        let value = read_interleaved_bits(interleaved, chunk * 20, 20);
        push_geohash_digit(&mut output, (value >> 15) as u16);
        push_geohash_digit(&mut output, (value >> 10) as u16);
        push_geohash_digit(&mut output, (value >> 5) as u16);
        push_geohash_digit(&mut output, value as u16);
    }

    for digit in (full_chunks * 4)..precision {
        let value = read_interleaved_bits(interleaved, digit * 5, 5);
        output.push(BASE32[value as usize] as char);
    }
    output
}

fn geohash_to_interleaved(hashcode: &str) -> Result<Vec<u16>> {
    let bytes = hashcode.as_bytes();
    let mut interleaved = vec![0_u16; bytes.len() * 5 / 16 + 1];
    let full_chunks = bytes.len() / 4;

    for chunk in 0..full_chunks {
        let src = chunk * 4;
        let value = (u32::from(base32_value(bytes[src]).ok_or(GeohashError::InvalidCode)?) << 15)
            | (u32::from(base32_value(bytes[src + 1]).ok_or(GeohashError::InvalidCode)?) << 10)
            | (u32::from(base32_value(bytes[src + 2]).ok_or(GeohashError::InvalidCode)?) << 5)
            | u32::from(base32_value(bytes[src + 3]).ok_or(GeohashError::InvalidCode)?);
        write_interleaved_bits(&mut interleaved, chunk * 20, 20, value);
    }

    for (offset, byte) in bytes[(full_chunks * 4)..].iter().copied().enumerate() {
        let char_index = full_chunks * 4 + offset;
        let value = u32::from(base32_value(byte).ok_or(GeohashError::InvalidCode)?);
        write_interleaved_bits(&mut interleaved, char_index * 5, 5, value);
    }
    Ok(interleaved)
}

fn interleave_coordinates(latitude: f64, longitude: f64) -> Result<[u16; 8]> {
    let lat64 = double_to_i64(latitude / 90.0).ok_or(GeohashError::InvalidArgument)?;
    let lon64 = double_to_i64(longitude / 180.0).ok_or(GeohashError::InvalidArgument)?;
    let mut interleaved = [0_u16; 8];
    for i in 0..8 {
        interleaved[7 - i] = interleave((lon64 >> (i * 8)) as u8, (lat64 >> (i * 8)) as u8);
    }
    Ok(interleaved)
}

pub(crate) fn encode_normalized_latitude(latitude: f64, longitude: f64) -> Result<String> {
    let interleaved = interleave_coordinates(latitude, longitude)?;
    Ok(interleaved_to_geohash(&interleaved, 26))
}

pub fn encode(latitude: f64, longitude: f64) -> Result<String> {
    let (latitude, _) = normalize_latitude(latitude)?;
    encode_normalized_latitude(latitude, longitude)
}

pub fn decode(hashcode: &str) -> Result<(f64, f64, usize, usize)> {
    let interleaved = geohash_to_interleaved(hashcode)?;
    let mut lat64 = 0_u64;
    let mut lon64 = 0_u64;
    for word in interleaved
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(8)
    {
        let (upper, lower) = deinterleave(word);
        lon64 = (lon64 << 8) + u64::from(upper);
        lat64 = (lat64 << 8) + u64::from(lower);
    }

    let code_len = hashcode.len();
    Ok((
        i64_to_double(lat64) * 90.0,
        i64_to_double(lon64) * 180.0,
        code_len / 2 * 5 + code_len % 2 * 2,
        code_len / 2 * 5 + code_len % 2 * 3,
    ))
}

fn decode_c2i(hashcode: &str) -> Result<(u128, u128, usize, usize)> {
    let mut lon = 0_u128;
    let mut lat = 0_u128;
    let mut bit_length = 0_usize;
    let mut lat_length = 0_usize;
    let mut lon_length = 0_usize;

    for byte in hashcode.bytes() {
        let value = u128::from(base32_value(byte).ok_or(GeohashError::InvalidCode)?);
        if bit_length % 2 == 0 {
            lon = (lon << 3) + ((value >> 2) & 4) + ((value >> 1) & 2) + (value & 1);
            lat = (lat << 2) + ((value >> 2) & 2) + ((value >> 1) & 1);
            lon_length += 3;
            lat_length += 2;
        } else {
            lon = (lon << 2) + ((value >> 2) & 2) + ((value >> 1) & 1);
            lat = (lat << 3) + ((value >> 2) & 4) + ((value >> 1) & 2) + (value & 1);
            lon_length += 2;
            lat_length += 3;
        }
        bit_length += 5;
    }

    Ok((lat, lon, lat_length, lon_length))
}

fn encode_i2c(lat: u128, lon: u128, lat_length: usize, lon_length: usize) -> String {
    let precision = (lat_length + lon_length) / 5;
    let (mut a, mut b) = if lat_length < lon_length {
        (lon, lat)
    } else {
        (lat, lon)
    };

    let boost = [0_u8, 1, 4, 5, 16, 17, 20, 21];
    let mut output = Vec::with_capacity(precision);
    for _ in 0..precision {
        let value = (boost[(a & 7) as usize] + (boost[(b & 3) as usize] << 1)) & 0x1f;
        output.push(BASE32[value as usize]);
        let next_a = a >> 3;
        a = b >> 2;
        b = next_a;
    }
    output.reverse();
    String::from_utf8(output).expect("geohash alphabet is ASCII")
}

pub fn neighbors(hashcode: &str) -> Result<Vec<String>> {
    let (lat, lon, lat_length, lon_length) = decode_c2i(hashcode)?;
    if lat_length == 0 && lon_length == 0 {
        return Ok(Vec::new());
    }
    if lat_length >= 128 || lon_length >= 128 {
        return Err(GeohashError::InvalidArgument);
    }

    let lat_size = 1_u128 << lat_length;
    let lon_size = 1_u128 << lon_length;
    let lats = [
        Some(lat),
        lat.checked_sub(1),
        if lat + 1 < lat_size {
            Some(lat + 1)
        } else {
            None
        },
    ];
    let lons = [lon, (lon + lon_size - 1) % lon_size, (lon + 1) % lon_size];

    let mut output = Vec::with_capacity(8);
    let mut previous_lat = None;
    for (lat_index, candidate_lat) in lats.iter().copied().enumerate() {
        let Some(candidate_lat) = candidate_lat else {
            continue;
        };
        if previous_lat == Some(candidate_lat) {
            continue;
        }
        previous_lat = Some(candidate_lat);

        let mut previous_lon = None;
        for (lon_index, candidate_lon) in lons.iter().copied().enumerate() {
            if previous_lon == Some(candidate_lon) {
                continue;
            }
            previous_lon = Some(candidate_lon);
            if lat_index == 0 && lon_index == 0 {
                continue;
            }
            output.push(encode_i2c(
                candidate_lat,
                candidate_lon,
                lat_length,
                lon_length,
            ));
        }
    }
    Ok(output)
}

pub(crate) fn encode_uint128_parts_normalized_latitude(
    latitude: f64,
    longitude: f64,
) -> Result<(u64, u64)> {
    let interleaved = interleave_coordinates(latitude, longitude)?;
    let high = (u64::from(interleaved[0]) << 48)
        | (u64::from(interleaved[1]) << 32)
        | (u64::from(interleaved[2]) << 16)
        | u64::from(interleaved[3]);
    let low = (u64::from(interleaved[4]) << 48)
        | (u64::from(interleaved[5]) << 32)
        | (u64::from(interleaved[6]) << 16)
        | u64::from(interleaved[7]);
    Ok((high, low))
}

pub fn encode_int(latitude: f64, longitude: f64) -> Result<(u64, u64)> {
    // Keep the historical encode_int name for compatibility; the parts are unsigned.
    let (latitude, _) = normalize_latitude(latitude)?;
    encode_uint128_parts_normalized_latitude(latitude, longitude)
}

pub fn decode_int_parts(parts: &[u64]) -> Result<(f64, f64)> {
    let mut interleaved = [0_u16; 8];
    match parts.len() {
        2 => {
            let high = parts[0];
            let low = parts[1];
            interleaved[0] = (high >> 48) as u16;
            interleaved[1] = (high >> 32) as u16;
            interleaved[2] = (high >> 16) as u16;
            interleaved[3] = high as u16;
            interleaved[4] = (low >> 48) as u16;
            interleaved[5] = (low >> 32) as u16;
            interleaved[6] = (low >> 16) as u16;
            interleaved[7] = low as u16;
        }
        4 => {
            for i in 0..4 {
                let value = parts[i];
                interleaved[i * 2] = (value >> 16) as u16;
                interleaved[i * 2 + 1] = value as u16;
            }
        }
        8 => {
            for i in 0..8 {
                interleaved[i] = parts[i] as u16;
            }
        }
        _ => return Err(GeohashError::InvalidArgument),
    }

    let mut lat64 = 0_u64;
    let mut lon64 = 0_u64;
    for word in interleaved {
        let (upper, lower) = deinterleave(word);
        lon64 = (lon64 << 8) + u64::from(upper);
        lat64 = (lat64 << 8) + u64::from(lower);
    }
    Ok((i64_to_double(lat64) * 90.0, i64_to_double(lon64) * 180.0))
}
