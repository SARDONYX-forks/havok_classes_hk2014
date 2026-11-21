use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpDeformableAngConstraintAtom`
/// - version: `1`
/// - signature: `0x490ca3fc`
/// - size: `  0`(x86)/` 96`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpDeformableAngConstraintAtom<'a> {
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
    pub parent: hkpConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `offset`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offset"))]
    #[cfg_attr(feature = "serde", serde(rename = "offset"))]
    pub m_offset: Quaternion,
    /// # C++ Info
    /// - name: `yieldStrengthDiag`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "yieldStrengthDiag"))]
    #[cfg_attr(feature = "serde", serde(rename = "yieldStrengthDiag"))]
    pub m_yieldStrengthDiag: Vector4,
    /// # C++ Info
    /// - name: `yieldStrengthOffDiag`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "yieldStrengthOffDiag"))]
    #[cfg_attr(feature = "serde", serde(rename = "yieldStrengthOffDiag"))]
    pub m_yieldStrengthOffDiag: Vector4,
    /// # C++ Info
    /// - name: `ultimateStrengthDiag`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ultimateStrengthDiag"))]
    #[cfg_attr(feature = "serde", serde(rename = "ultimateStrengthDiag"))]
    pub m_ultimateStrengthDiag: Vector4,
    /// # C++ Info
    /// - name: `ultimateStrengthOffDiag`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ultimateStrengthOffDiag"))]
    #[cfg_attr(feature = "serde", serde(rename = "ultimateStrengthOffDiag"))]
    pub m_ultimateStrengthOffDiag: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpDeformableAngConstraintAtom<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpDeformableAngConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x490ca3fc)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpDeformableAngConstraintAtom<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x490ca3fc)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpDeformableAngConstraintAtom",
                    class_meta,
                    (0u64, 96u64),
                )?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_field("yieldStrengthDiag", &self.m_yieldStrengthDiag)?;
            serializer
                .serialize_field("yieldStrengthOffDiag", &self.m_yieldStrengthOffDiag)?;
            serializer
                .serialize_field("ultimateStrengthDiag", &self.m_ultimateStrengthDiag)?;
            serializer
                .serialize_field(
                    "ultimateStrengthOffDiag",
                    &self.m_ultimateStrengthOffDiag,
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
    impl<'de> _serde::Deserialize<'de> for hkpDeformableAngConstraintAtom<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_offset,
                m_yieldStrengthDiag,
                m_yieldStrengthOffDiag,
                m_ultimateStrengthDiag,
                m_ultimateStrengthOffDiag,
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
                        "type" => Ok(__Field::m_type),
                        "offset" => Ok(__Field::m_offset),
                        "yieldStrengthDiag" => Ok(__Field::m_yieldStrengthDiag),
                        "yieldStrengthOffDiag" => Ok(__Field::m_yieldStrengthOffDiag),
                        "ultimateStrengthDiag" => Ok(__Field::m_ultimateStrengthDiag),
                        "ultimateStrengthOffDiag" => {
                            Ok(__Field::m_ultimateStrengthOffDiag)
                        }
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
            struct __hkpDeformableAngConstraintAtomVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpDeformableAngConstraintAtom<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpDeformableAngConstraintAtomVisitor<'de> {
                type Value = hkpDeformableAngConstraintAtom<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpDeformableAngConstraintAtom",
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
                    let mut m_offset: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_yieldStrengthDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_yieldStrengthOffDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ultimateStrengthDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ultimateStrengthOffDiag: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_offset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 14usize)?;
                                m_offset = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_yieldStrengthDiag,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "yieldStrengthDiag",
                                        ),
                                    );
                                }
                                m_yieldStrengthDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_yieldStrengthOffDiag,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "yieldStrengthOffDiag",
                                        ),
                                    );
                                }
                                m_yieldStrengthOffDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_ultimateStrengthDiag,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ultimateStrengthDiag",
                                        ),
                                    );
                                }
                                m_ultimateStrengthDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_ultimateStrengthOffDiag,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ultimateStrengthOffDiag",
                                        ),
                                    );
                                }
                                m_ultimateStrengthOffDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_offset = match m_offset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offset"),
                            );
                        }
                    };
                    let m_yieldStrengthDiag = match m_yieldStrengthDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "yieldStrengthDiag",
                                ),
                            );
                        }
                    };
                    let m_yieldStrengthOffDiag = match m_yieldStrengthOffDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "yieldStrengthOffDiag",
                                ),
                            );
                        }
                    };
                    let m_ultimateStrengthDiag = match m_ultimateStrengthDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ultimateStrengthDiag",
                                ),
                            );
                        }
                    };
                    let m_ultimateStrengthOffDiag = match m_ultimateStrengthOffDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ultimateStrengthOffDiag",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpDeformableAngConstraintAtom {
                        __ptr,
                        parent,
                        m_offset,
                        m_yieldStrengthDiag,
                        m_yieldStrengthOffDiag,
                        m_ultimateStrengthDiag,
                        m_ultimateStrengthOffDiag,
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
                    let mut m_type: _serde::__private::Option<AtomType> = _serde::__private::None;
                    let mut m_offset: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_yieldStrengthDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_yieldStrengthOffDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ultimateStrengthDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ultimateStrengthOffDiag: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<AtomType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_offset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                                    );
                                }
                                m_offset = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_yieldStrengthDiag => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_yieldStrengthDiag,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "yieldStrengthDiag",
                                        ),
                                    );
                                }
                                m_yieldStrengthDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_yieldStrengthOffDiag => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_yieldStrengthOffDiag,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "yieldStrengthOffDiag",
                                        ),
                                    );
                                }
                                m_yieldStrengthOffDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ultimateStrengthDiag => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_ultimateStrengthDiag,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ultimateStrengthDiag",
                                        ),
                                    );
                                }
                                m_ultimateStrengthDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ultimateStrengthOffDiag => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_ultimateStrengthOffDiag,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ultimateStrengthOffDiag",
                                        ),
                                    );
                                }
                                m_ultimateStrengthOffDiag = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_offset = match m_offset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offset"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_yieldStrengthDiag = match m_yieldStrengthDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "yieldStrengthDiag",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_yieldStrengthOffDiag = match m_yieldStrengthOffDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "yieldStrengthOffDiag",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ultimateStrengthDiag = match m_ultimateStrengthDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ultimateStrengthDiag",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ultimateStrengthOffDiag = match m_ultimateStrengthOffDiag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ultimateStrengthOffDiag",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkpConstraintAtom {
                        __ptr: __ptr.clone(),
                        m_type,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpDeformableAngConstraintAtom {
                        __ptr: __ptr.clone(),
                        parent,
                        m_offset,
                        m_yieldStrengthDiag,
                        m_yieldStrengthOffDiag,
                        m_ultimateStrengthDiag,
                        m_ultimateStrengthOffDiag,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "offset",
                "yieldStrengthDiag",
                "yieldStrengthOffDiag",
                "ultimateStrengthDiag",
                "ultimateStrengthOffDiag",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpDeformableAngConstraintAtom",
                FIELDS,
                __hkpDeformableAngConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpDeformableAngConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
