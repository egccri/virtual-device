// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum DataType {
//     /// Null type
//     Null,
//     /// A boolean datatype representing the values `true` and `false`.
//     Boolean,
//     /// A signed 8-bit integer.
//     Int8,
//     /// A signed 16-bit integer.
//     Int16,
//     /// A signed 32-bit integer.
//     Int32,
//     /// A signed 64-bit integer.
//     Int64,
//     /// An unsigned 8-bit integer.
//     UInt8,
//     /// An unsigned 16-bit integer.
//     UInt16,
//     /// An unsigned 32-bit integer.
//     UInt32,
//     /// An unsigned 64-bit integer.
//     UInt64,
//     /// A 16-bit floating point number.
//     Float16,
//     /// A 32-bit floating point number.
//     Float32,
//     /// A 64-bit floating point number.
//     Float64,
// }

// pub struct Int32Type {
//     data: i32,
// }

// impl Int32Type {
//     pub fn from_i32(val: i32) -> Self {
//         Int32Type {
//             data: val,
//         }
//     }
//
//     pub fn get_i32(self) -> i32 {
//         self.data
//     }
// }

// pub trait PrimitiveType: 'static {
//     /// Corresponding Rust native type for the primitive type.
//     type Native: JsonSerializable;
//
//     /// the corresponding Arrow data type of this primitive type.
//     const DATA_TYPE: DataType;
// }

// pub trait JsonSerializable: 'static {
//     fn into_json_value(self) -> Option<Value>;
// }
//
// impl JsonSerializable for i32 {
//     fn into_json_value(self) -> Option<Value> {
//         Some(self.into())
//     }
// }
//
// impl JsonSerializable for f32 {
//     fn into_json_value(self) -> Option<Value> {
//         todo!()
//     }
// }
//
// impl <T> JsonSerializable for Box<T>
// where
//     T : 'static
// {
//     fn into_json_value(self) -> Option<Value> {
//         todo!()
//     }
// }

// impl PrimitiveType for Int32Type {
//     type Native = i32;
//     const DATA_TYPE: DataType = DataType::Int32;
// }
