use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair`
/// - version: `0`
/// - signature: `0xfc87019`
/// - size: `  0`(x86)/` 24`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<'a> {
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
    /// - name: `elem`(ctype: `hkArray<struct hkIntRealPair>`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "elem"))]
    #[cfg_attr(feature = "serde", serde(rename = "elem"))]
    pub m_elem: Vec<hkIntRealPair<'a>>,
    /// # C++ Info
    /// - name: `numElems`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numElems"))]
    #[cfg_attr(feature = "serde", serde(rename = "numElems"))]
    pub m_numElems: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass
    for hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfc87019)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_elem
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize
    for hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xfc87019)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair",
                    class_meta,
                    (0u64, 24u64),
                )?;
            serializer
                .serialize_array_field(
                    "elem",
                    &self.m_elem,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 8u64,
                    },
                )?;
            serializer.serialize_field("numElems", &self.m_numElems)?;
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
    for hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_elem,
                m_numElems,
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
                        "elem" => Ok(__Field::m_elem),
                        "numElems" => Ok(__Field::m_numElems),
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
            struct __hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPairVisitor<
                'de,
            > {
                marker: _serde::__private::PhantomData<
                    hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<
                        'de,
                    >,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPairVisitor<
                'de,
            > {
                type Value = hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<
                    'de,
                >;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair",
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
                    let mut m_elem: _serde::__private::Option<Vec<hkIntRealPair<'de>>> = _serde::__private::None;
                    let mut m_numElems: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_elem) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("elem"),
                                    );
                                }
                                m_elem = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkIntRealPair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_numElems) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numElems",
                                        ),
                                    );
                                }
                                m_numElems = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    let m_elem = match m_elem {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("elem"),
                            );
                        }
                    };
                    let m_numElems = match m_numElems {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numElems"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair {
                        __ptr,
                        m_elem,
                        m_numElems,
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
                    let mut m_elem: _serde::__private::Option<Vec<hkIntRealPair<'de>>> = _serde::__private::None;
                    let mut m_numElems: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_elem => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_elem) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("elem"),
                                    );
                                }
                                m_elem = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkIntRealPair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numElems => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numElems) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numElems",
                                        ),
                                    );
                                }
                                m_numElems = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    let m_elem = match m_elem {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("elem"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numElems = match m_numElems {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numElems"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair {
                        __ptr: __ptr.clone(),
                        m_elem,
                        m_numElems,
                    })
                }
            }
            const FIELDS: &[&str] = &["elem", "numElems"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair",
                FIELDS,
                __hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPairVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
