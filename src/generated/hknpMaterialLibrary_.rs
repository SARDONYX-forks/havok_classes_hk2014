use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpMaterialLibrary`
/// - version: `0`
/// - signature: `0x5c07fd6a`
/// - size: `  0`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpMaterialLibrary<'a> {
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
    /// - name: `materialAddedSignal`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialAddedSignal"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialAddedSignal"))]
    pub m_materialAddedSignal: Pointer<'a>,
    /// # C++ Info
    /// - name: `materialModifiedSignal`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialModifiedSignal"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialModifiedSignal"))]
    pub m_materialModifiedSignal: Pointer<'a>,
    /// # C++ Info
    /// - name: `materialRemovedSignal`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialRemovedSignal"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialRemovedSignal"))]
    pub m_materialRemovedSignal: Pointer<'a>,
    /// # C++ Info
    /// - name: `entries`(ctype: `struct hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "entries"))]
    #[cfg_attr(feature = "serde", serde(rename = "entries"))]
    pub m_entries: hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
        'a,
    >,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpMaterialLibrary<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpMaterialLibrary"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5c07fd6a)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_materialAddedSignal);
            v.push(&self.m_materialModifiedSignal);
            v.push(&self.m_materialRemovedSignal);
            v.extend(self.m_entries.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hknpMaterialLibrary<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x5c07fd6a)));
            let mut serializer = __serializer
                .serialize_struct("hknpMaterialLibrary", class_meta, (0u64, 64u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialAddedSignal", &self.m_materialAddedSignal)?;
            serializer
                .skip_field("materialModifiedSignal", &self.m_materialModifiedSignal)?;
            serializer
                .skip_field("materialRemovedSignal", &self.m_materialRemovedSignal)?;
            serializer.serialize_field("entries", &self.m_entries)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpMaterialLibrary<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_entries,
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
                        "entries" => Ok(__Field::m_entries),
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
            struct __hknpMaterialLibraryVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpMaterialLibrary<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpMaterialLibraryVisitor<'de> {
                type Value = hknpMaterialLibrary<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpMaterialLibrary",
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
                    let mut m_materialAddedSignal: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_materialModifiedSignal: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_materialRemovedSignal: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_entries: _serde::__private::Option<
                        hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                            'de,
                        >,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_materialAddedSignal,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialAddedSignal",
                                        ),
                                    );
                                }
                                m_materialAddedSignal = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_materialModifiedSignal,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialModifiedSignal",
                                        ),
                                    );
                                }
                                m_materialModifiedSignal = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_materialRemovedSignal,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialRemovedSignal",
                                        ),
                                    );
                                }
                                m_materialRemovedSignal = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_entries) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "entries",
                                        ),
                                    );
                                }
                                m_entries = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                                            'de,
                                        >,
                                    >(&mut __map) {
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
                    let m_materialAddedSignal = match m_materialAddedSignal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialAddedSignal",
                                ),
                            );
                        }
                    };
                    let m_materialModifiedSignal = match m_materialModifiedSignal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialModifiedSignal",
                                ),
                            );
                        }
                    };
                    let m_materialRemovedSignal = match m_materialRemovedSignal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialRemovedSignal",
                                ),
                            );
                        }
                    };
                    let m_entries = match m_entries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("entries"),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpMaterialLibrary {
                        __ptr,
                        parent,
                        m_materialAddedSignal,
                        m_materialModifiedSignal,
                        m_materialRemovedSignal,
                        m_entries,
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
                    let mut m_entries: _serde::__private::Option<
                        hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                            'de,
                        >,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_entries => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_entries) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "entries",
                                        ),
                                    );
                                }
                                m_entries = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                                            'de,
                                        >,
                                    >(&mut __map) {
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
                    let m_entries = match m_entries {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("entries"),
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpMaterialLibrary {
                        __ptr: __ptr.clone(),
                        parent,
                        m_entries,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "materialAddedSignal",
                "materialModifiedSignal",
                "materialRemovedSignal",
                "entries",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpMaterialLibrary",
                FIELDS,
                __hknpMaterialLibraryVisitor {
                    marker: _serde::__private::PhantomData::<hknpMaterialLibrary>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
