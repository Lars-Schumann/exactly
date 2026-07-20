use core::any::TypeId;

const U8: TypeId = TypeId::of::<u8>();
const U16: TypeId = TypeId::of::<u16>();
const U32: TypeId = TypeId::of::<u32>();
const U64: TypeId = TypeId::of::<u64>();
const U128: TypeId = TypeId::of::<u128>();
const USIZE: TypeId = TypeId::of::<usize>();

const I8: TypeId = TypeId::of::<i8>();
const I16: TypeId = TypeId::of::<i16>();
const I32: TypeId = TypeId::of::<i32>();
const I64: TypeId = TypeId::of::<i64>();
const I128: TypeId = TypeId::of::<i128>();
const ISIZE: TypeId = TypeId::of::<isize>();

#[expect(non_camel_case_types)]
pub(crate) enum Type {
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
}

#[rustfmt::skip]
pub(crate) const fn type_of<T: 'static>() -> Option<Type> {
    let type_id: TypeId = TypeId::of::<T>();

         if type_id == U8    { Some(Type::u8   ) } 
    else if type_id == U16   { Some(Type::u16  ) } 
    else if type_id == U32   { Some(Type::u32  ) } 
    else if type_id == U64   { Some(Type::u64  ) } 
    else if type_id == U128  { Some(Type::u128 ) } 
    else if type_id == USIZE { Some(Type::usize) } 
    else if type_id == I8    { Some(Type::i8   ) } 
    else if type_id == I16   { Some(Type::i16  ) } 
    else if type_id == I32   { Some(Type::i32  ) } 
    else if type_id == I64   { Some(Type::i64  ) } 
    else if type_id == I128  { Some(Type::i128 ) } 
    else if type_id == ISIZE { Some(Type::isize) } 
    else                     { None              }
}
