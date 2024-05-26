#[derive(Debug, Clone, thiserror::Error)]
#[error("Convert Enum <-> Primitive failed, source type: '{src_t}', destination type: '{dest_t}', value: {value}")]
pub struct EnumPrimitiveConversionError<S: core::fmt::Debug> {
    pub value: S,
    pub src_t: String,
    pub dest_t: String,
}
