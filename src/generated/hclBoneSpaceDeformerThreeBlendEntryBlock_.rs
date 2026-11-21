use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBoneSpaceDeformerThreeBlendEntryBlock`
/// - version: `1`
/// - signature: `0xcb5ad1e2`
/// - size: `  0`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBoneSpaceDeformerThreeBlendEntryBlock<'a> {
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
    /// - name: `vertexIndices`(ctype: `hkUint16[5]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 10`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexIndices"))]
    pub m_vertexIndices: [U16<'a>; 5usize],
    /// # C++ Info
    /// - name: `boneIndices`(ctype: `hkUint16[15]`)
    /// - offset: `  0`(x86)/` 10`(x86_64)
    /// - type_size: `  0`(x86)/` 30`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneIndices"))]
    pub m_boneIndices: [U16<'a>; 15usize],
    /// # C++ Info
    /// - name: `padding`(ctype: `hkUint8[8]`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "padding"))]
    #[cfg_attr(feature = "serde", serde(rename = "padding"))]
    pub m_padding: [U8<'a>; 8usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBoneSpaceDeformerThreeBlendEntryBlock<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBoneSpaceDeformerThreeBlendEntryBlock"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcb5ad1e2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclBoneSpaceDeformerThreeBlendEntryBlock<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xcb5ad1e2)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBoneSpaceDeformerThreeBlendEntryBlock",
                    class_meta,
                    (0u64, 48u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "vertexIndices",
                    self.m_vertexIndices.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "boneIndices",
                    self.m_boneIndices.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_fixed_array_field(
                    "padding",
                    self.m_padding.as_slice(),
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
    impl<'de> _serde::Deserialize<'de>
    for hclBoneSpaceDeformerThreeBlendEntryBlock<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_vertexIndices,
                m_boneIndices,
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
                        "vertexIndices" => Ok(__Field::m_vertexIndices),
                        "boneIndices" => Ok(__Field::m_boneIndices),
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
            struct __hclBoneSpaceDeformerThreeBlendEntryBlockVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBoneSpaceDeformerThreeBlendEntryBlock<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBoneSpaceDeformerThreeBlendEntryBlockVisitor<'de> {
                type Value = hclBoneSpaceDeformerThreeBlendEntryBlock<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBoneSpaceDeformerThreeBlendEntryBlock",
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
                    let mut m_vertexIndices: _serde::__private::Option<
                        [U16<'de>; 5usize],
                    > = _serde::__private::None;
                    let mut m_boneIndices: _serde::__private::Option<
                        [U16<'de>; 15usize],
                    > = _serde::__private::None;
                    let mut m_padding: _serde::__private::Option<[U8<'de>; 8usize]> = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_vertexIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexIndices",
                                        ),
                                    );
                                }
                                m_vertexIndices = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 5usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_boneIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndices",
                                        ),
                                    );
                                }
                                m_boneIndices = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 15usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_padding) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "padding",
                                        ),
                                    );
                                }
                                m_padding = _serde::__private::Some(
                                    match __A::next_value::<[U8<'de>; 8usize]>(&mut __map) {
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
                    let m_vertexIndices = match m_vertexIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexIndices",
                                ),
                            );
                        }
                    };
                    let m_boneIndices = match m_boneIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndices",
                                ),
                            );
                        }
                    };
                    let m_padding = match m_padding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("padding"),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBoneSpaceDeformerThreeBlendEntryBlock {
                        __ptr,
                        m_vertexIndices,
                        m_boneIndices,
                        m_padding,
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
                    let mut m_vertexIndices: _serde::__private::Option<
                        [U16<'de>; 5usize],
                    > = _serde::__private::None;
                    let mut m_boneIndices: _serde::__private::Option<
                        [U16<'de>; 15usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_vertexIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexIndices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexIndices",
                                        ),
                                    );
                                }
                                m_vertexIndices = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 5usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneIndices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndices",
                                        ),
                                    );
                                }
                                m_boneIndices = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 15usize]>(&mut __map) {
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
                    let m_vertexIndices = match m_vertexIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneIndices = match m_boneIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBoneSpaceDeformerThreeBlendEntryBlock {
                        __ptr: __ptr.clone(),
                        m_vertexIndices,
                        m_boneIndices,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &["vertexIndices", "boneIndices", "padding"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBoneSpaceDeformerThreeBlendEntryBlock",
                FIELDS,
                __hclBoneSpaceDeformerThreeBlendEntryBlockVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBoneSpaceDeformerThreeBlendEntryBlock,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
