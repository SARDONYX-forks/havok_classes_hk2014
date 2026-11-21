use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBlendSomeVerticesOperator`
/// - version: `1`
/// - signature: `0x2b45e438`
/// - size: `  0`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBlendSomeVerticesOperator<'a> {
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
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hclOperator<'a>,
    /// # C++ Info
    /// - name: `blendEntries`(ctype: `hkArray<struct hclBlendSomeVerticesOperatorBlendEntry>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendEntries"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendEntries"))]
    pub m_blendEntries: Vec<hclBlendSomeVerticesOperatorBlendEntry<'a>>,
    /// # C++ Info
    /// - name: `bufferIdx_A`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bufferIdx_A"))]
    #[cfg_attr(feature = "serde", serde(rename = "bufferIdx_A"))]
    pub m_bufferIdx_A: U32<'a>,
    /// # C++ Info
    /// - name: `bufferIdx_B`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bufferIdx_B"))]
    #[cfg_attr(feature = "serde", serde(rename = "bufferIdx_B"))]
    pub m_bufferIdx_B: U32<'a>,
    /// # C++ Info
    /// - name: `bufferIdx_C`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bufferIdx_C"))]
    #[cfg_attr(feature = "serde", serde(rename = "bufferIdx_C"))]
    pub m_bufferIdx_C: U32<'a>,
    /// # C++ Info
    /// - name: `blendNormals`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 60`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendNormals"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendNormals"))]
    pub m_blendNormals: bool,
    /// # C++ Info
    /// - name: `blendTangents`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 61`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendTangents"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendTangents"))]
    pub m_blendTangents: bool,
    /// # C++ Info
    /// - name: `blendBitangents`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 62`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendBitangents"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendBitangents"))]
    pub m_blendBitangents: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBlendSomeVerticesOperator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBlendSomeVerticesOperator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2b45e438)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_blendEntries
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclBlendSomeVerticesOperator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x2b45e438)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBlendSomeVerticesOperator",
                    class_meta,
                    (0u64, 64u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.parent.m_name)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "blendEntries",
                    &self.m_blendEntries,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 8u64,
                    },
                )?;
            serializer.serialize_field("bufferIdx_A", &self.m_bufferIdx_A)?;
            serializer.serialize_field("bufferIdx_B", &self.m_bufferIdx_B)?;
            serializer.serialize_field("bufferIdx_C", &self.m_bufferIdx_C)?;
            serializer.serialize_field("blendNormals", &self.m_blendNormals)?;
            serializer.serialize_field("blendTangents", &self.m_blendTangents)?;
            serializer.serialize_field("blendBitangents", &self.m_blendBitangents)?;
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
    impl<'de> _serde::Deserialize<'de> for hclBlendSomeVerticesOperator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_blendEntries,
                m_bufferIdx_A,
                m_bufferIdx_B,
                m_bufferIdx_C,
                m_blendNormals,
                m_blendTangents,
                m_blendBitangents,
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
                        "name" => Ok(__Field::m_name),
                        "blendEntries" => Ok(__Field::m_blendEntries),
                        "bufferIdx_A" => Ok(__Field::m_bufferIdx_A),
                        "bufferIdx_B" => Ok(__Field::m_bufferIdx_B),
                        "bufferIdx_C" => Ok(__Field::m_bufferIdx_C),
                        "blendNormals" => Ok(__Field::m_blendNormals),
                        "blendTangents" => Ok(__Field::m_blendTangents),
                        "blendBitangents" => Ok(__Field::m_blendBitangents),
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
            struct __hclBlendSomeVerticesOperatorVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBlendSomeVerticesOperator<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBlendSomeVerticesOperatorVisitor<'de> {
                type Value = hclBlendSomeVerticesOperator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBlendSomeVerticesOperator",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_blendEntries: _serde::__private::Option<
                        Vec<hclBlendSomeVerticesOperatorBlendEntry<'de>>,
                    > = _serde::__private::None;
                    let mut m_bufferIdx_A: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_bufferIdx_B: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_bufferIdx_C: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_blendNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_blendTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_blendBitangents: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_blendEntries) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendEntries",
                                        ),
                                    );
                                }
                                m_blendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBlendSomeVerticesOperatorBlendEntry<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bufferIdx_A) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx_A",
                                        ),
                                    );
                                }
                                m_bufferIdx_A = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_bufferIdx_B) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx_B",
                                        ),
                                    );
                                }
                                m_bufferIdx_B = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_bufferIdx_C) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx_C",
                                        ),
                                    );
                                }
                                m_bufferIdx_C = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_blendNormals) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendNormals",
                                        ),
                                    );
                                }
                                m_blendNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_blendTangents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendTangents",
                                        ),
                                    );
                                }
                                m_blendTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_blendBitangents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendBitangents",
                                        ),
                                    );
                                }
                                m_blendBitangents = _serde::__private::Some(
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
                    let m_blendEntries = match m_blendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendEntries",
                                ),
                            );
                        }
                    };
                    let m_bufferIdx_A = match m_bufferIdx_A {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx_A",
                                ),
                            );
                        }
                    };
                    let m_bufferIdx_B = match m_bufferIdx_B {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx_B",
                                ),
                            );
                        }
                    };
                    let m_bufferIdx_C = match m_bufferIdx_C {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx_C",
                                ),
                            );
                        }
                    };
                    let m_blendNormals = match m_blendNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendNormals",
                                ),
                            );
                        }
                    };
                    let m_blendTangents = match m_blendTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendTangents",
                                ),
                            );
                        }
                    };
                    let m_blendBitangents = match m_blendBitangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendBitangents",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBlendSomeVerticesOperator {
                        __ptr,
                        parent,
                        m_blendEntries,
                        m_bufferIdx_A,
                        m_bufferIdx_B,
                        m_bufferIdx_C,
                        m_blendNormals,
                        m_blendTangents,
                        m_blendBitangents,
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_blendEntries: _serde::__private::Option<
                        Vec<hclBlendSomeVerticesOperatorBlendEntry<'de>>,
                    > = _serde::__private::None;
                    let mut m_bufferIdx_A: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_bufferIdx_B: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_bufferIdx_C: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_blendNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_blendTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_blendBitangents: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blendEntries => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendEntries) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendEntries",
                                        ),
                                    );
                                }
                                m_blendEntries = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclBlendSomeVerticesOperatorBlendEntry<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bufferIdx_A => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bufferIdx_A) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx_A",
                                        ),
                                    );
                                }
                                m_bufferIdx_A = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bufferIdx_B => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bufferIdx_B) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx_B",
                                        ),
                                    );
                                }
                                m_bufferIdx_B = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bufferIdx_C => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bufferIdx_C) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx_C",
                                        ),
                                    );
                                }
                                m_bufferIdx_C = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blendNormals => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendNormals) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendNormals",
                                        ),
                                    );
                                }
                                m_blendNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blendTangents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendTangents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendTangents",
                                        ),
                                    );
                                }
                                m_blendTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blendBitangents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendBitangents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendBitangents",
                                        ),
                                    );
                                }
                                m_blendBitangents = _serde::__private::Some(
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendEntries = match m_blendEntries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendEntries",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bufferIdx_A = match m_bufferIdx_A {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx_A",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bufferIdx_B = match m_bufferIdx_B {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx_B",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bufferIdx_C = match m_bufferIdx_C {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx_C",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendNormals = match m_blendNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendNormals",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendTangents = match m_blendTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendTangents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendBitangents = match m_blendBitangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendBitangents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hclOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBlendSomeVerticesOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_blendEntries,
                        m_bufferIdx_A,
                        m_bufferIdx_B,
                        m_bufferIdx_C,
                        m_blendNormals,
                        m_blendTangents,
                        m_blendBitangents,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "blendEntries",
                "bufferIdx_A",
                "bufferIdx_B",
                "bufferIdx_C",
                "blendNormals",
                "blendTangents",
                "blendBitangents",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBlendSomeVerticesOperator",
                FIELDS,
                __hclBlendSomeVerticesOperatorVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBlendSomeVerticesOperator,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
