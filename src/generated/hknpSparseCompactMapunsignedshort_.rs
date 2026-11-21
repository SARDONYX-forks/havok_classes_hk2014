use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpSparseCompactMapunsignedshort`
/// - version: `0`
/// - signature: `0x4558127c`
/// - size: `  0`(x86)/` 40`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpSparseCompactMapunsignedshort<'a> {
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
    /// - name: `secondaryKeyMask`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "secondaryKeyMask"))]
    #[cfg_attr(feature = "serde", serde(rename = "secondaryKeyMask"))]
    pub m_secondaryKeyMask: U32<'a>,
    /// # C++ Info
    /// - name: `sencondaryKeyBits`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sencondaryKeyBits"))]
    #[cfg_attr(feature = "serde", serde(rename = "sencondaryKeyBits"))]
    pub m_sencondaryKeyBits: U32<'a>,
    /// # C++ Info
    /// - name: `primaryKeyToIndex`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "primaryKeyToIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "primaryKeyToIndex"))]
    pub m_primaryKeyToIndex: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `valueAndSecondaryKeys`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "valueAndSecondaryKeys"))]
    #[cfg_attr(feature = "serde", serde(rename = "valueAndSecondaryKeys"))]
    pub m_valueAndSecondaryKeys: Vec<U16<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpSparseCompactMapunsignedshort<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpSparseCompactMapunsignedshort"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4558127c)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hknpSparseCompactMapunsignedshort<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x4558127c)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hknpSparseCompactMapunsignedshort",
                    class_meta,
                    (0u64, 40u64),
                )?;
            serializer.serialize_field("secondaryKeyMask", &self.m_secondaryKeyMask)?;
            serializer.serialize_field("sencondaryKeyBits", &self.m_sencondaryKeyBits)?;
            serializer
                .serialize_array_field(
                    "primaryKeyToIndex",
                    &self.m_primaryKeyToIndex,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "valueAndSecondaryKeys",
                    &self.m_valueAndSecondaryKeys,
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
    impl<'de> _serde::Deserialize<'de> for hknpSparseCompactMapunsignedshort<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_secondaryKeyMask,
                m_sencondaryKeyBits,
                m_primaryKeyToIndex,
                m_valueAndSecondaryKeys,
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
                        "secondaryKeyMask" => Ok(__Field::m_secondaryKeyMask),
                        "sencondaryKeyBits" => Ok(__Field::m_sencondaryKeyBits),
                        "primaryKeyToIndex" => Ok(__Field::m_primaryKeyToIndex),
                        "valueAndSecondaryKeys" => Ok(__Field::m_valueAndSecondaryKeys),
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
            struct __hknpSparseCompactMapunsignedshortVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hknpSparseCompactMapunsignedshort<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hknpSparseCompactMapunsignedshortVisitor<'de> {
                type Value = hknpSparseCompactMapunsignedshort<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpSparseCompactMapunsignedshort",
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
                    let mut m_secondaryKeyMask: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_sencondaryKeyBits: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_primaryKeyToIndex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_valueAndSecondaryKeys: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_secondaryKeyMask) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "secondaryKeyMask",
                                        ),
                                    );
                                }
                                m_secondaryKeyMask = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sencondaryKeyBits,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sencondaryKeyBits",
                                        ),
                                    );
                                }
                                m_sencondaryKeyBits = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_primaryKeyToIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primaryKeyToIndex",
                                        ),
                                    );
                                }
                                m_primaryKeyToIndex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_valueAndSecondaryKeys,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "valueAndSecondaryKeys",
                                        ),
                                    );
                                }
                                m_valueAndSecondaryKeys = _serde::__private::Some(
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
                    let m_secondaryKeyMask = match m_secondaryKeyMask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "secondaryKeyMask",
                                ),
                            );
                        }
                    };
                    let m_sencondaryKeyBits = match m_sencondaryKeyBits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sencondaryKeyBits",
                                ),
                            );
                        }
                    };
                    let m_primaryKeyToIndex = match m_primaryKeyToIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primaryKeyToIndex",
                                ),
                            );
                        }
                    };
                    let m_valueAndSecondaryKeys = match m_valueAndSecondaryKeys {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "valueAndSecondaryKeys",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpSparseCompactMapunsignedshort {
                        __ptr,
                        m_secondaryKeyMask,
                        m_sencondaryKeyBits,
                        m_primaryKeyToIndex,
                        m_valueAndSecondaryKeys,
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
                    let mut m_secondaryKeyMask: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_sencondaryKeyBits: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_primaryKeyToIndex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_valueAndSecondaryKeys: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_secondaryKeyMask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_secondaryKeyMask) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "secondaryKeyMask",
                                        ),
                                    );
                                }
                                m_secondaryKeyMask = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sencondaryKeyBits => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sencondaryKeyBits,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sencondaryKeyBits",
                                        ),
                                    );
                                }
                                m_sencondaryKeyBits = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_primaryKeyToIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_primaryKeyToIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primaryKeyToIndex",
                                        ),
                                    );
                                }
                                m_primaryKeyToIndex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_valueAndSecondaryKeys => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_valueAndSecondaryKeys,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "valueAndSecondaryKeys",
                                        ),
                                    );
                                }
                                m_valueAndSecondaryKeys = _serde::__private::Some(
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
                    let m_secondaryKeyMask = match m_secondaryKeyMask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "secondaryKeyMask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sencondaryKeyBits = match m_sencondaryKeyBits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sencondaryKeyBits",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_primaryKeyToIndex = match m_primaryKeyToIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primaryKeyToIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_valueAndSecondaryKeys = match m_valueAndSecondaryKeys {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "valueAndSecondaryKeys",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpSparseCompactMapunsignedshort {
                        __ptr: __ptr.clone(),
                        m_secondaryKeyMask,
                        m_sencondaryKeyBits,
                        m_primaryKeyToIndex,
                        m_valueAndSecondaryKeys,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "secondaryKeyMask",
                "sencondaryKeyBits",
                "primaryKeyToIndex",
                "valueAndSecondaryKeys",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpSparseCompactMapunsignedshort",
                FIELDS,
                __hknpSparseCompactMapunsignedshortVisitor {
                    marker: _serde::__private::PhantomData::<
                        hknpSparseCompactMapunsignedshort,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
