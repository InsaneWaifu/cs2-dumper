use memflow::prelude::v1::*;

use super::*;

pub type SchemaClassBinding = SchemaClassInfoData;

#[rustfmt::skip]
#[derive(Pod)]
#[repr(C)]
pub struct SchemaClassInfoData {
    pub base: Pointer64<SchemaClassInfoData>,                  // 0x0000
    pub name: Pointer64<ReprCString>,                          // 0x0008
    pub module_name: Pointer64<ReprCString>,                   // 0x0010
    pub size: i32,                                             // 0x0018
    pub field_count: i16,                                      // 0x001C
    pub static_metadata_count: i16,                            // 0x001E
    pad_0020: [u8; 0x2],                                       // 0x0020
    pub align_of: u8,                                          // 0x0022
    pub has_base_class: u8,                                    // 0x0023
    pub total_class_size: i16,                                 // 0x0024
    pub derived_class_size: i16,                               // 0x0026
    pub fields: Pointer64<[SchemaClassFieldData]>,             // 0x0028
    pad_0038: [u8; 0x8],                                       // 0x0030
    pub base_classes: Pointer64<SchemaBaseClassInfoData>,      // 0x0038
    pub static_metadata: Pointer64<[SchemaMetadataEntryData]>, // 0x0040
    pub type_scope: Pointer64<SchemaSystemTypeScope>,          // 0x0050
    pub r#type: Pointer64<SchemaType>,                         // 0x0058
    pad_0060: [u8; 0x10],                                      // 0x0060
}
