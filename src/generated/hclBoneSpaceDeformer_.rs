use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBoneSpaceDeformer`
/// - version: `0`
/// - signature: `0x694fac94`
/// - size: `  0`(x86)/` 88`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBoneSpaceDeformer<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// # C++ Info
    /// - name: `fourBlendEntries`(ctype: `hkArray<struct hclBoneSpaceDeformerFourBlendEntryBlock>`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fourBlendEntries"))]
    #[cfg_attr(feature = "serde", serde(rename = "fourBlendEntries"))]
    pub m_fourBlendEntries: Vec<hclBoneSpaceDeformerFourBlendEntryBlock<'a>>,
    /// # C++ Info
    /// - name: `threeBlendEntries`(ctype: `hkArray<struct hclBoneSpaceDeformerThreeBlendEntryBlock>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "threeBlendEntries"))]
    #[cfg_attr(feature = "serde", serde(rename = "threeBlendEntries"))]
    pub m_threeBlendEntries: Vec<hclBoneSpaceDeformerThreeBlendEntryBlock<'a>>,
    /// # C++ Info
    /// - name: `twoBlendEntries`(ctype: `hkArray<struct hclBoneSpaceDeformerTwoBlendEntryBlock>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "twoBlendEntries"))]
    #[cfg_attr(feature = "serde", serde(rename = "twoBlendEntries"))]
    pub m_twoBlendEntries: Vec<hclBoneSpaceDeformerTwoBlendEntryBlock<'a>>,
    /// # C++ Info
    /// - name: `oneBlendEntries`(ctype: `hkArray<struct hclBoneSpaceDeformerOneBlendEntryBlock>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "oneBlendEntries"))]
    #[cfg_attr(feature = "serde", serde(rename = "oneBlendEntries"))]
    pub m_oneBlendEntries: Vec<hclBoneSpaceDeformerOneBlendEntryBlock<'a>>,
    /// # C++ Info
    /// - name: `controlBytes`(ctype: `hkArray<hkUint8>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "controlBytes"))]
    #[cfg_attr(feature = "serde", serde(rename = "controlBytes"))]
    pub m_controlBytes: Vec<U8<'a>>,
    /// # C++ Info
    /// - name: `startVertexIndex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "startVertexIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "startVertexIndex"))]
    pub m_startVertexIndex: U16<'a>,
    /// # C++ Info
    /// - name: `endVertexIndex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 82`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "endVertexIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "endVertexIndex"))]
    pub m_endVertexIndex: U16<'a>,
    /// # C++ Info
    /// - name: `batchSizeSpu`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 84`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "batchSizeSpu"))]
    #[cfg_attr(feature = "serde", serde(rename = "batchSizeSpu"))]
    pub m_batchSizeSpu: U16<'a>,
    /// # C++ Info
    /// - name: `partialWrite`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 86`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "partialWrite"))]
    #[cfg_attr(feature = "serde", serde(rename = "partialWrite"))]
    pub m_partialWrite: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBoneSpaceDeformer<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBoneSpaceDeformer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x694fac94)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_fourBlendEntries
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_threeBlendEntries
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_twoBlendEntries
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_oneBlendEntries
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclBoneSpaceDeformer<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x694fac94)));
            let mut serializer = __serializer
                .serialize_struct("hclBoneSpaceDeformer", class_meta, (0u64, 88u64))?;
            serializer
                .serialize_array_field(
                    "fourBlendEntries",
                    &self.m_fourBlendEntries,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 48u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "threeBlendEntries",
                    &self.m_threeBlendEntries,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 48u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "twoBlendEntries",
                    &self.m_twoBlendEntries,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 48u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "oneBlendEntries",
                    &self.m_oneBlendEntries,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 64u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "controlBytes",
                    &self.m_controlBytes,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("startVertexIndex", &self.m_startVertexIndex)?;
            serializer.serialize_field("endVertexIndex", &self.m_endVertexIndex)?;
            serializer.serialize_field("batchSizeSpu", &self.m_batchSizeSpu)?;
            serializer.serialize_field("partialWrite", &self.m_partialWrite)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclBoneSpaceDeformer<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_fourBlendEntries,
                m_threeBlendEntries,
                m_twoBlendEntries,
                m_oneBlendEntries,
                m_controlBytes,
                m_startVertexIndex,
                m_endVertexIndex,
                m_batchSizeSpu,
                m_partialWrite,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "fourBlendEntries" => Ok(__Field::m_fourBlendEntries),
                        "threeBlendEntries" => Ok(__Field::m_threeBlendEntries),
                        "twoBlendEntries" => Ok(__Field::m_twoBlendEntries),
                        "oneBlendEntries" => Ok(__Field::m_oneBlendEntries),
                        "controlBytes" => Ok(__Field::m_controlBytes),
                        "startVertexIndex" => Ok(__Field::m_startVertexIndex),
                        "endVertexIndex" => Ok(__Field::m_endVertexIndex),
                        "batchSizeSpu" => Ok(__Field::m_batchSizeSpu),
                        "partialWrite" => Ok(__Field::m_partialWrite),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hclBoneSpaceDeformerVisitor<'de> {
                marker: _serde::__private::PhantomData<hclBoneSpaceDeformer<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclBoneSpaceDeformerVisitor<'de> {
                type Value = hclBoneSpaceDeformer<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBoneSpaceDeformer",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_fourBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerFourBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_threeBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerThreeBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_twoBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerTwoBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_oneBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerOneBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_controlBytes: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_startVertexIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_endVertexIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_batchSizeSpu: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_partialWrite: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_fourBlendEntries) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fourBlendEntries",
                                        ),
                                    );
                                }
                                m_fourBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerFourBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_threeBlendEntries,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "threeBlendEntries",
                                        ),
                                    );
                                }
                                m_threeBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerThreeBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_twoBlendEntries) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "twoBlendEntries",
                                        ),
                                    );
                                }
                                m_twoBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerTwoBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_oneBlendEntries) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oneBlendEntries",
                                        ),
                                    );
                                }
                                m_oneBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerOneBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_controlBytes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "controlBytes",
                                        ),
                                    );
                                }
                                m_controlBytes = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_startVertexIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startVertexIndex",
                                        ),
                                    );
                                }
                                m_startVertexIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_endVertexIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endVertexIndex",
                                        ),
                                    );
                                }
                                m_endVertexIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_batchSizeSpu) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "batchSizeSpu",
                                        ),
                                    );
                                }
                                m_batchSizeSpu = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_partialWrite) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "partialWrite",
                                        ),
                                    );
                                }
                                m_partialWrite = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 0usize, 1usize)?;
                    let m_fourBlendEntries = match m_fourBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fourBlendEntries",
                                ),
                            );
                        }
                    };
                    let m_threeBlendEntries = match m_threeBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "threeBlendEntries",
                                ),
                            );
                        }
                    };
                    let m_twoBlendEntries = match m_twoBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "twoBlendEntries",
                                ),
                            );
                        }
                    };
                    let m_oneBlendEntries = match m_oneBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oneBlendEntries",
                                ),
                            );
                        }
                    };
                    let m_controlBytes = match m_controlBytes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "controlBytes",
                                ),
                            );
                        }
                    };
                    let m_startVertexIndex = match m_startVertexIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startVertexIndex",
                                ),
                            );
                        }
                    };
                    let m_endVertexIndex = match m_endVertexIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endVertexIndex",
                                ),
                            );
                        }
                    };
                    let m_batchSizeSpu = match m_batchSizeSpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "batchSizeSpu",
                                ),
                            );
                        }
                    };
                    let m_partialWrite = match m_partialWrite {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "partialWrite",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBoneSpaceDeformer {
                        __ptr,
                        m_fourBlendEntries,
                        m_threeBlendEntries,
                        m_twoBlendEntries,
                        m_oneBlendEntries,
                        m_controlBytes,
                        m_startVertexIndex,
                        m_endVertexIndex,
                        m_batchSizeSpu,
                        m_partialWrite,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_fourBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerFourBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_threeBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerThreeBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_twoBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerTwoBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_oneBlendEntries: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerOneBlendEntryBlock<'de>>,
                    > = _serde::__private::None;
                    let mut m_controlBytes: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_startVertexIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_endVertexIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_batchSizeSpu: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_partialWrite: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_fourBlendEntries => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fourBlendEntries) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fourBlendEntries",
                                        ),
                                    );
                                }
                                m_fourBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerFourBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_threeBlendEntries => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_threeBlendEntries,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "threeBlendEntries",
                                        ),
                                    );
                                }
                                m_threeBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerThreeBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_twoBlendEntries => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_twoBlendEntries) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "twoBlendEntries",
                                        ),
                                    );
                                }
                                m_twoBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerTwoBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_oneBlendEntries => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_oneBlendEntries) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oneBlendEntries",
                                        ),
                                    );
                                }
                                m_oneBlendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBoneSpaceDeformerOneBlendEntryBlock<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_controlBytes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_controlBytes) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "controlBytes",
                                        ),
                                    );
                                }
                                m_controlBytes = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startVertexIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_startVertexIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startVertexIndex",
                                        ),
                                    );
                                }
                                m_startVertexIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_endVertexIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_endVertexIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endVertexIndex",
                                        ),
                                    );
                                }
                                m_endVertexIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_batchSizeSpu => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_batchSizeSpu) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "batchSizeSpu",
                                        ),
                                    );
                                }
                                m_batchSizeSpu = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_partialWrite => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_partialWrite) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "partialWrite",
                                        ),
                                    );
                                }
                                m_partialWrite = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_fourBlendEntries = match m_fourBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fourBlendEntries",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_threeBlendEntries = match m_threeBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "threeBlendEntries",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_twoBlendEntries = match m_twoBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "twoBlendEntries",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_oneBlendEntries = match m_oneBlendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oneBlendEntries",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_controlBytes = match m_controlBytes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "controlBytes",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startVertexIndex = match m_startVertexIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startVertexIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_endVertexIndex = match m_endVertexIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endVertexIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_batchSizeSpu = match m_batchSizeSpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "batchSizeSpu",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_partialWrite = match m_partialWrite {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "partialWrite",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBoneSpaceDeformer {
                        __ptr: __ptr.clone(),
                        m_fourBlendEntries,
                        m_threeBlendEntries,
                        m_twoBlendEntries,
                        m_oneBlendEntries,
                        m_controlBytes,
                        m_startVertexIndex,
                        m_endVertexIndex,
                        m_batchSizeSpu,
                        m_partialWrite,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "fourBlendEntries",
                "threeBlendEntries",
                "twoBlendEntries",
                "oneBlendEntries",
                "controlBytes",
                "startVertexIndex",
                "endVertexIndex",
                "batchSizeSpu",
                "partialWrite",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBoneSpaceDeformer",
                FIELDS,
                __hclBoneSpaceDeformerVisitor {
                    marker: _serde::__private::PhantomData::<hclBoneSpaceDeformer>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
