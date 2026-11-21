use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkcdStaticMeshTreeBase`
/// - version: `0`
/// - signature: `0xf885dcd0`
/// - size: `  0`(x86)/`112`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkcdStaticMeshTreeBase<'a> {
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
    pub parent: hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5<'a>,
    /// # C++ Info
    /// - name: `numPrimitiveKeys`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numPrimitiveKeys"))]
    #[cfg_attr(feature = "serde", serde(rename = "numPrimitiveKeys"))]
    pub m_numPrimitiveKeys: I32<'a>,
    /// # C++ Info
    /// - name: `bitsPerKey`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bitsPerKey"))]
    #[cfg_attr(feature = "serde", serde(rename = "bitsPerKey"))]
    pub m_bitsPerKey: I32<'a>,
    /// # C++ Info
    /// - name: `maxKeyValue`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxKeyValue"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxKeyValue"))]
    pub m_maxKeyValue: U32<'a>,
    /// # C++ Info
    /// - name: `sections`(ctype: `hkArray<struct hkcdStaticMeshTreeBaseSection>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sections"))]
    #[cfg_attr(feature = "serde", serde(rename = "sections"))]
    pub m_sections: Vec<hkcdStaticMeshTreeBaseSection<'a>>,
    /// # C++ Info
    /// - name: `primitives`(ctype: `hkArray<struct hkcdStaticMeshTreeBasePrimitive>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "primitives"))]
    #[cfg_attr(feature = "serde", serde(rename = "primitives"))]
    pub m_primitives: Vec<hkcdStaticMeshTreeBasePrimitive<'a>>,
    /// # C++ Info
    /// - name: `sharedVerticesIndex`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sharedVerticesIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "sharedVerticesIndex"))]
    pub m_sharedVerticesIndex: Vec<U16<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkcdStaticMeshTreeBase<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkcdStaticMeshTreeBase"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf885dcd0)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .m_nodes
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.parent.m_domain.deps_indexes());
            v.extend(
                self
                    .m_sections
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_primitives
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkcdStaticMeshTreeBase<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xf885dcd0)));
            let mut serializer = __serializer
                .serialize_struct("hkcdStaticMeshTreeBase", class_meta, (0u64, 112u64))?;
            serializer
                .serialize_array_field(
                    "nodes",
                    &self.parent.parent.parent.m_nodes,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 5u64,
                    },
                )?;
            serializer.serialize_field("domain", &self.parent.m_domain)?;
            serializer.serialize_field("numPrimitiveKeys", &self.m_numPrimitiveKeys)?;
            serializer.serialize_field("bitsPerKey", &self.m_bitsPerKey)?;
            serializer.serialize_field("maxKeyValue", &self.m_maxKeyValue)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "sections",
                    &self.m_sections,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 96u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "primitives",
                    &self.m_primitives,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 4u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "sharedVerticesIndex",
                    &self.m_sharedVerticesIndex,
                    TypeSize::NonPtr,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkcdStaticMeshTreeBase<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_nodes,
                m_domain,
                m_numPrimitiveKeys,
                m_bitsPerKey,
                m_maxKeyValue,
                m_sections,
                m_primitives,
                m_sharedVerticesIndex,
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
                        "nodes" => Ok(__Field::m_nodes),
                        "domain" => Ok(__Field::m_domain),
                        "numPrimitiveKeys" => Ok(__Field::m_numPrimitiveKeys),
                        "bitsPerKey" => Ok(__Field::m_bitsPerKey),
                        "maxKeyValue" => Ok(__Field::m_maxKeyValue),
                        "sections" => Ok(__Field::m_sections),
                        "primitives" => Ok(__Field::m_primitives),
                        "sharedVerticesIndex" => Ok(__Field::m_sharedVerticesIndex),
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
            struct __hkcdStaticMeshTreeBaseVisitor<'de> {
                marker: _serde::__private::PhantomData<hkcdStaticMeshTreeBase<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkcdStaticMeshTreeBaseVisitor<'de> {
                type Value = hkcdStaticMeshTreeBase<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkcdStaticMeshTreeBase",
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
                    let mut m_numPrimitiveKeys: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_bitsPerKey: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_maxKeyValue: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_sections: _serde::__private::Option<
                        Vec<hkcdStaticMeshTreeBaseSection<'de>>,
                    > = _serde::__private::None;
                    let mut m_primitives: _serde::__private::Option<
                        Vec<hkcdStaticMeshTreeBasePrimitive<'de>>,
                    > = _serde::__private::None;
                    let mut m_sharedVerticesIndex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_numPrimitiveKeys) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numPrimitiveKeys",
                                        ),
                                    );
                                }
                                m_numPrimitiveKeys = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bitsPerKey) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitsPerKey",
                                        ),
                                    );
                                }
                                m_bitsPerKey = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_maxKeyValue) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxKeyValue",
                                        ),
                                    );
                                }
                                m_maxKeyValue = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_sections) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sections",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_sections = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticMeshTreeBaseSection<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_primitives) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primitives",
                                        ),
                                    );
                                }
                                m_primitives = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticMeshTreeBasePrimitive<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sharedVerticesIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sharedVerticesIndex",
                                        ),
                                    );
                                }
                                m_sharedVerticesIndex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
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
                    let m_numPrimitiveKeys = match m_numPrimitiveKeys {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numPrimitiveKeys",
                                ),
                            );
                        }
                    };
                    let m_bitsPerKey = match m_bitsPerKey {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bitsPerKey",
                                ),
                            );
                        }
                    };
                    let m_maxKeyValue = match m_maxKeyValue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxKeyValue",
                                ),
                            );
                        }
                    };
                    let m_sections = match m_sections {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("sections"),
                            );
                        }
                    };
                    let m_primitives = match m_primitives {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primitives",
                                ),
                            );
                        }
                    };
                    let m_sharedVerticesIndex = match m_sharedVerticesIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sharedVerticesIndex",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkcdStaticMeshTreeBase {
                        __ptr,
                        parent,
                        m_numPrimitiveKeys,
                        m_bitsPerKey,
                        m_maxKeyValue,
                        m_sections,
                        m_primitives,
                        m_sharedVerticesIndex,
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
                    let mut m_nodes: _serde::__private::Option<
                        Vec<hkcdStaticTreeCodec3Axis5<'de>>,
                    > = _serde::__private::None;
                    let mut m_domain: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_numPrimitiveKeys: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_bitsPerKey: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_maxKeyValue: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_sections: _serde::__private::Option<
                        Vec<hkcdStaticMeshTreeBaseSection<'de>>,
                    > = _serde::__private::None;
                    let mut m_primitives: _serde::__private::Option<
                        Vec<hkcdStaticMeshTreeBasePrimitive<'de>>,
                    > = _serde::__private::None;
                    let mut m_sharedVerticesIndex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_nodes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nodes) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("nodes"),
                                    );
                                }
                                m_nodes = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticTreeCodec3Axis5<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_domain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_domain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("domain"),
                                    );
                                }
                                m_domain = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numPrimitiveKeys => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numPrimitiveKeys) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numPrimitiveKeys",
                                        ),
                                    );
                                }
                                m_numPrimitiveKeys = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bitsPerKey => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bitsPerKey) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitsPerKey",
                                        ),
                                    );
                                }
                                m_bitsPerKey = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxKeyValue => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxKeyValue) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxKeyValue",
                                        ),
                                    );
                                }
                                m_maxKeyValue = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sections => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sections) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sections",
                                        ),
                                    );
                                }
                                m_sections = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticMeshTreeBaseSection<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_primitives => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_primitives) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primitives",
                                        ),
                                    );
                                }
                                m_primitives = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticMeshTreeBasePrimitive<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sharedVerticesIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sharedVerticesIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sharedVerticesIndex",
                                        ),
                                    );
                                }
                                m_sharedVerticesIndex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
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
                    let m_nodes = match m_nodes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("nodes"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_domain = match m_domain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("domain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numPrimitiveKeys = match m_numPrimitiveKeys {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numPrimitiveKeys",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bitsPerKey = match m_bitsPerKey {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bitsPerKey",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxKeyValue = match m_maxKeyValue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxKeyValue",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sections = match m_sections {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("sections"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_primitives = match m_primitives {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primitives",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sharedVerticesIndex = match m_sharedVerticesIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sharedVerticesIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5 {
                        __ptr: __ptr.clone(),
                        m_nodes,
                    };
                    let parent = hkcdStaticTreeDynamicStorage5 {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let parent = hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5 {
                        __ptr: __ptr.clone(),
                        parent,
                        m_domain,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkcdStaticMeshTreeBase {
                        __ptr: __ptr.clone(),
                        parent,
                        m_numPrimitiveKeys,
                        m_bitsPerKey,
                        m_maxKeyValue,
                        m_sections,
                        m_primitives,
                        m_sharedVerticesIndex,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "numPrimitiveKeys",
                "bitsPerKey",
                "maxKeyValue",
                "sections",
                "primitives",
                "sharedVerticesIndex",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkcdStaticMeshTreeBase",
                FIELDS,
                __hkcdStaticMeshTreeBaseVisitor {
                    marker: _serde::__private::PhantomData::<hkcdStaticMeshTreeBase>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
