use core::any::TypeId;

const U8_TYPE_ID: TypeId = TypeId::of::<u8>();
const U16_TYPE_ID: TypeId = TypeId::of::<u16>();
// const U32_TYPE_ID: TypeId = TypeId::of::<u32>();
// const U64_TYPE_ID: TypeId = TypeId::of::<u64>();
// const U128_TYPE_ID: TypeId = TypeId::of::<u128>();
// const USIZE_TYPE_ID: TypeId = TypeId::of::<usize>();

const I8_TYPE_ID: TypeId = TypeId::of::<i8>();
const I16_TYPE_ID: TypeId = TypeId::of::<i16>();
// const I32_TYPE_ID: TypeId = TypeId::of::<i32>();
// const I64_TYPE_ID: TypeId = TypeId::of::<i64>();
// const I128_TYPE_ID: TypeId = TypeId::of::<i128>();
// const ISIZE_TYPE_ID: TypeId = TypeId::of::<isize>();

#[expect(non_camel_case_types)]
pub(crate) enum Type {
    u8,
    u16,
    i8,
    i16,
}

pub(crate) const fn type_of<T: 'static>() -> Option<Type> {
    let type_id: TypeId = TypeId::of::<T>();

    if type_id == U8_TYPE_ID {
        Some(Type::u8)
    } else if type_id == U16_TYPE_ID {
        Some(Type::u16)
    } else if type_id == I8_TYPE_ID {
        Some(Type::i8)
    } else if type_id == I16_TYPE_ID {
        Some(Type::i16)
    } else {
        None
    }
}
