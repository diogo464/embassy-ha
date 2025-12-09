use core::{mem::MaybeUninit, ops::Deref};

use super::varint;

const DEFAULT_FIELD_BUFFER_CAP: usize = 32;

pub enum Field<'a> {
    U8(u8),
    U16(u16),
    VarInt(u32),
    Buffer(&'a [u8]),
    LenPrefixedBuffer(&'a [u8]),
    LenPrefixedString(&'a str),
}

pub struct FieldBuffer<'a, const N: usize = DEFAULT_FIELD_BUFFER_CAP> {
    data: [MaybeUninit<Field<'a>>; N],
    len: usize,
}

impl<'a, const N: usize> Default for FieldBuffer<'a, N> {
    fn default() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }
}

impl<'a, const N: usize> FieldBuffer<'a, N> {
    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, field: Field<'a>) {
        assert!(self.len < N, "field buffer lenght limit exceeded");
        self.data[self.len].write(field);
        self.len += 1;
    }

    pub fn set(&mut self, n: usize, field: Field<'a>) {
        assert!(self.len > n);
        self.data[n].write(field);
    }

    pub fn as_slice<'s>(&'s self) -> &'s [Field<'a>] {
        unsafe {
            core::mem::transmute::<&'s [MaybeUninit<Field<'a>>], &'s [Field<'a>]>(
                &self.data[..self.len],
            )
        }
    }
}

impl<'a, const N: usize> AsRef<[Field<'a>]> for FieldBuffer<'a, N> {
    fn as_ref(&self) -> &[Field<'a>] {
        self.as_slice()
    }
}

impl<'a, const N: usize> Deref for FieldBuffer<'a, N> {
    type Target = [Field<'a>];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

pub fn field_size(field: &Field) -> usize {
    match field {
        Field::U8(_) => 1,
        Field::U16(_) => 2,
        Field::VarInt(v) => {
            let (_, n) = varint::encode(*v);
            n
        }
        Field::Buffer(v) => v.len(),
        Field::LenPrefixedBuffer(v) => v.len().strict_add(2),
        Field::LenPrefixedString(v) => v.len().strict_add(2),
    }
}

pub fn fields_size(fields: &[Field]) -> usize {
    let mut total_size = 0usize;
    for field in fields {
        total_size = total_size.strict_add(field_size(field));
    }
    total_size
}
