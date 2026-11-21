use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBoneSpaceDeformerLocalBlockPNT`
/// - version: `0`
/// - signature: `0xa943d812`
/// - size: `  0`(x86)/`512`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBoneSpaceDeformerLocalBlockPNT<'a> {
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
    /// - name: `localPosition`(ctype: `hkVector4[16]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`256`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localPosition"))]
    #[cfg_attr(feature = "serde", serde(rename = "localPosition"))]
    pub m_localPosition: [Vector4; 16usize],
    /// # C++ Info
    /// - name: `localNormal`(ctype: `hkInt16[64]`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/`128`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(schema_with = "make_large_int_array_schema")
    )]
    #[cfg_attr(feature = "serde_default", serde(default = "crate::default_array"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<[::serde_with::Same; 64]>")
    )]
    #[educe(Default(expression = core::array::from_fn(|_idx|Default::default())))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localNormal"))]
    #[cfg_attr(feature = "serde", serde(rename = "localNormal"))]
    pub m_localNormal: [I16<'a>; 64usize],
    /// # C++ Info
    /// - name: `localTangent`(ctype: `hkInt16[64]`)
    /// - offset: `  0`(x86)/`384`(x86_64)
    /// - type_size: `  0`(x86)/`128`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(schema_with = "make_large_int_array_schema")
    )]
    #[cfg_attr(feature = "serde_default", serde(default = "crate::default_array"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<[::serde_with::Same; 64]>")
    )]
    #[educe(Default(expression = core::array::from_fn(|_idx|Default::default())))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localTangent"))]
    #[cfg_attr(feature = "serde", serde(rename = "localTangent"))]
    pub m_localTangent: [I16<'a>; 64usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBoneSpaceDeformerLocalBlockPNT<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBoneSpaceDeformerLocalBlockPNT"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa943d812)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclBoneSpaceDeformerLocalBlockPNT<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xa943d812)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBoneSpaceDeformerLocalBlockPNT",
                    class_meta,
                    (0u64, 512u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "localPosition",
                    self.m_localPosition.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "localNormal",
                    self.m_localNormal.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "localTangent",
                    self.m_localTangent.as_slice(),
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
    impl<'de> _serde::Deserialize<'de> for hclBoneSpaceDeformerLocalBlockPNT<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_localPosition,
                m_localNormal,
                m_localTangent,
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
                        "localPosition" => Ok(__Field::m_localPosition),
                        "localNormal" => Ok(__Field::m_localNormal),
                        "localTangent" => Ok(__Field::m_localTangent),
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
            struct __hclBoneSpaceDeformerLocalBlockPNTVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBoneSpaceDeformerLocalBlockPNT<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBoneSpaceDeformerLocalBlockPNTVisitor<'de> {
                type Value = hclBoneSpaceDeformerLocalBlockPNT<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBoneSpaceDeformerLocalBlockPNT",
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
                    let mut m_localPosition: _serde::__private::Option<
                        [Vector4; 16usize],
                    > = _serde::__private::None;
                    let mut m_localNormal: _serde::__private::Option<
                        [I16<'de>; 64usize],
                    > = _serde::__private::None;
                    let mut m_localTangent: _serde::__private::Option<
                        [I16<'de>; 64usize],
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_localPosition) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localPosition",
                                        ),
                                    );
                                }
                                m_localPosition = _serde::__private::Some(
                                    match __A::next_value::<[Vector4; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_localNormal) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localNormal",
                                        ),
                                    );
                                }
                                m_localNormal = _serde::__private::Some(
                                    match __A::next_value::<[I16<'de>; 64usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_localTangent) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localTangent",
                                        ),
                                    );
                                }
                                m_localTangent = _serde::__private::Some(
                                    match __A::next_value::<[I16<'de>; 64usize]>(&mut __map) {
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
                    let m_localPosition = match m_localPosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localPosition",
                                ),
                            );
                        }
                    };
                    let m_localNormal = match m_localNormal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localNormal",
                                ),
                            );
                        }
                    };
                    let m_localTangent = match m_localTangent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localTangent",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBoneSpaceDeformerLocalBlockPNT {
                        __ptr,
                        m_localPosition,
                        m_localNormal,
                        m_localTangent,
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
                    let mut m_localPosition: _serde::__private::Option<
                        [Vector4; 16usize],
                    > = _serde::__private::None;
                    let mut m_localNormal: _serde::__private::Option<
                        [I16<'de>; 64usize],
                    > = _serde::__private::None;
                    let mut m_localTangent: _serde::__private::Option<
                        [I16<'de>; 64usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_localPosition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localPosition) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localPosition",
                                        ),
                                    );
                                }
                                m_localPosition = _serde::__private::Some(
                                    match __A::next_value::<[Vector4; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localNormal => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localNormal) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localNormal",
                                        ),
                                    );
                                }
                                m_localNormal = _serde::__private::Some(
                                    match __A::next_value::<[I16<'de>; 64usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localTangent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localTangent) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localTangent",
                                        ),
                                    );
                                }
                                m_localTangent = _serde::__private::Some(
                                    match __A::next_value::<[I16<'de>; 64usize]>(&mut __map) {
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
                    let m_localPosition = match m_localPosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localPosition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_localNormal = match m_localNormal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localNormal",
                                ),
                            );
                            #[cfg(not(feature = "strict"))]
                            core::array::from_fn(|_idx| Default::default())
                        }
                    };
                    let m_localTangent = match m_localTangent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localTangent",
                                ),
                            );
                            #[cfg(not(feature = "strict"))]
                            core::array::from_fn(|_idx| Default::default())
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBoneSpaceDeformerLocalBlockPNT {
                        __ptr: __ptr.clone(),
                        m_localPosition,
                        m_localNormal,
                        m_localTangent,
                    })
                }
            }
            const FIELDS: &[&str] = &["localPosition", "localNormal", "localTangent"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBoneSpaceDeformerLocalBlockPNT",
                FIELDS,
                __hclBoneSpaceDeformerLocalBlockPNTVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBoneSpaceDeformerLocalBlockPNT,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
