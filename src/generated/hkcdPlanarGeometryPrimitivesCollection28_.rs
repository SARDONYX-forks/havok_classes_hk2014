use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkcdPlanarGeometryPrimitivesCollection28`
/// - version: `0`
/// - signature: `0x3921c763`
/// - size: `  0`(x86)/`3472`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkcdPlanarGeometryPrimitivesCollection28<'a> {
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
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `storage`(ctype: `hkArray<hkInt32>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "storage"))]
    #[cfg_attr(feature = "serde", serde(rename = "storage"))]
    pub m_storage: Vec<I32<'a>>,
    /// # C++ Info
    /// - name: `primaryBitmap`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "primaryBitmap"))]
    #[cfg_attr(feature = "serde", serde(rename = "primaryBitmap"))]
    pub m_primaryBitmap: U32<'a>,
    /// # C++ Info
    /// - name: `secondaryBitmaps`(ctype: `hkUint32[26]`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`104`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "secondaryBitmaps"))]
    #[cfg_attr(feature = "serde", serde(rename = "secondaryBitmaps"))]
    pub m_secondaryBitmaps: [U32<'a>; 26usize],
    /// # C++ Info
    /// - name: `freeBlocks`(ctype: `hkUint32[832]`)
    /// - offset: `  0`(x86)/`140`(x86_64)
    /// - type_size: `  0`(x86)/`3328`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(schema_with = "make_large_int_array_schema")
    )]
    #[cfg_attr(feature = "serde_default", serde(default = "crate::default_array"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<[::serde_with::Same; 832]>")
    )]
    #[educe(Default(expression = core::array::from_fn(|_idx|Default::default())))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "freeBlocks"))]
    #[cfg_attr(feature = "serde", serde(rename = "freeBlocks"))]
    pub m_freeBlocks: [U32<'a>; 832usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkcdPlanarGeometryPrimitivesCollection28<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkcdPlanarGeometryPrimitivesCollection28"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3921c763)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkcdPlanarGeometryPrimitivesCollection28<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x3921c763)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkcdPlanarGeometryPrimitivesCollection28",
                    class_meta,
                    (0u64, 3472u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field("storage", &self.m_storage, TypeSize::NonPtr)?;
            serializer.serialize_field("primaryBitmap", &self.m_primaryBitmap)?;
            serializer
                .serialize_fixed_array_field(
                    "secondaryBitmaps",
                    self.m_secondaryBitmaps.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "freeBlocks",
                    self.m_freeBlocks.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
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
    for hkcdPlanarGeometryPrimitivesCollection28<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_storage,
                m_primaryBitmap,
                m_secondaryBitmaps,
                m_freeBlocks,
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
                        "storage" => Ok(__Field::m_storage),
                        "primaryBitmap" => Ok(__Field::m_primaryBitmap),
                        "secondaryBitmaps" => Ok(__Field::m_secondaryBitmaps),
                        "freeBlocks" => Ok(__Field::m_freeBlocks),
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
            struct __hkcdPlanarGeometryPrimitivesCollection28Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkcdPlanarGeometryPrimitivesCollection28<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkcdPlanarGeometryPrimitivesCollection28Visitor<'de> {
                type Value = hkcdPlanarGeometryPrimitivesCollection28<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkcdPlanarGeometryPrimitivesCollection28",
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
                    let mut m_storage: _serde::__private::Option<Vec<I32<'de>>> = _serde::__private::None;
                    let mut m_primaryBitmap: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_secondaryBitmaps: _serde::__private::Option<
                        [U32<'de>; 26usize],
                    > = _serde::__private::None;
                    let mut m_freeBlocks: _serde::__private::Option<
                        [U32<'de>; 832usize],
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_storage) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "storage",
                                        ),
                                    );
                                }
                                m_storage = _serde::__private::Some(
                                    match __A::next_value::<Vec<I32<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_primaryBitmap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primaryBitmap",
                                        ),
                                    );
                                }
                                m_primaryBitmap = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_secondaryBitmaps) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "secondaryBitmaps",
                                        ),
                                    );
                                }
                                m_secondaryBitmaps = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 26usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_freeBlocks) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "freeBlocks",
                                        ),
                                    );
                                }
                                m_freeBlocks = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 832usize]>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_storage = match m_storage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("storage"),
                            );
                        }
                    };
                    let m_primaryBitmap = match m_primaryBitmap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primaryBitmap",
                                ),
                            );
                        }
                    };
                    let m_secondaryBitmaps = match m_secondaryBitmaps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "secondaryBitmaps",
                                ),
                            );
                        }
                    };
                    let m_freeBlocks = match m_freeBlocks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "freeBlocks",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkcdPlanarGeometryPrimitivesCollection28 {
                        __ptr,
                        parent,
                        m_storage,
                        m_primaryBitmap,
                        m_secondaryBitmaps,
                        m_freeBlocks,
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
                    let mut m_storage: _serde::__private::Option<Vec<I32<'de>>> = _serde::__private::None;
                    let mut m_primaryBitmap: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_secondaryBitmaps: _serde::__private::Option<
                        [U32<'de>; 26usize],
                    > = _serde::__private::None;
                    let mut m_freeBlocks: _serde::__private::Option<
                        [U32<'de>; 832usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_storage => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_storage) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "storage",
                                        ),
                                    );
                                }
                                m_storage = _serde::__private::Some(
                                    match __A::next_value::<Vec<I32<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_primaryBitmap => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_primaryBitmap) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primaryBitmap",
                                        ),
                                    );
                                }
                                m_primaryBitmap = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_secondaryBitmaps => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_secondaryBitmaps) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "secondaryBitmaps",
                                        ),
                                    );
                                }
                                m_secondaryBitmaps = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 26usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_freeBlocks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_freeBlocks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "freeBlocks",
                                        ),
                                    );
                                }
                                m_freeBlocks = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 832usize]>(&mut __map) {
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
                    let m_storage = match m_storage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("storage"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_primaryBitmap = match m_primaryBitmap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primaryBitmap",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_secondaryBitmaps = match m_secondaryBitmaps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "secondaryBitmaps",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_freeBlocks = match m_freeBlocks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "freeBlocks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))]
                            core::array::from_fn(|_idx| Default::default())
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkcdPlanarGeometryPrimitivesCollection28 {
                        __ptr: __ptr.clone(),
                        parent,
                        m_storage,
                        m_primaryBitmap,
                        m_secondaryBitmaps,
                        m_freeBlocks,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "storage",
                "primaryBitmap",
                "secondaryBitmaps",
                "freeBlocks",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkcdPlanarGeometryPrimitivesCollection28",
                FIELDS,
                __hkcdPlanarGeometryPrimitivesCollection28Visitor {
                    marker: _serde::__private::PhantomData::<
                        hkcdPlanarGeometryPrimitivesCollection28,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
