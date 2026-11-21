use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations`
/// - version: `0`
/// - signature: `0x7a67e9de`
/// - size: `  0`(x86)/` 24`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
    'a,
> {
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
    /// - name: `elements`(ctype: `hkArray<struct hknpMaterial>`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "elements"))]
    #[cfg_attr(feature = "serde", serde(rename = "elements"))]
    pub m_elements: Vec<hknpMaterial<'a>>,
    /// # C++ Info
    /// - name: `firstFree`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "firstFree"))]
    #[cfg_attr(feature = "serde", serde(rename = "firstFree"))]
    pub m_firstFree: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass
    for hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
        'a,
    > {
        #[inline]
        fn name(&self) -> &'static str {
            "hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7a67e9de)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_elements
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize
    for hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
        'a,
    > {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x7a67e9de)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations",
                    class_meta,
                    (0u64, 24u64),
                )?;
            serializer
                .serialize_array_field(
                    "elements",
                    &self.m_elements,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 80u64,
                    },
                )?;
            serializer.serialize_field("firstFree", &self.m_firstFree)?;
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
    for hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
        'de,
    > {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_elements,
                m_firstFree,
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
                        "elements" => Ok(__Field::m_elements),
                        "firstFree" => Ok(__Field::m_firstFree),
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
            struct __hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperationsVisitor<
                'de,
            > {
                marker: _serde::__private::PhantomData<
                    hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                        'de,
                    >,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperationsVisitor<
                'de,
            > {
                type Value = hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                    'de,
                >;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations",
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
                    let mut m_elements: _serde::__private::Option<
                        Vec<hknpMaterial<'de>>,
                    > = _serde::__private::None;
                    let mut m_firstFree: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_elements) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elements",
                                        ),
                                    );
                                }
                                m_elements = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMaterial<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_firstFree) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstFree",
                                        ),
                                    );
                                }
                                m_firstFree = _serde::__private::Some(
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
                    let m_elements = match m_elements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("elements"),
                            );
                        }
                    };
                    let m_firstFree = match m_firstFree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstFree",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations {
                        __ptr,
                        m_elements,
                        m_firstFree,
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
                    let mut m_elements: _serde::__private::Option<
                        Vec<hknpMaterial<'de>>,
                    > = _serde::__private::None;
                    let mut m_firstFree: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_elements => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_elements) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elements",
                                        ),
                                    );
                                }
                                m_elements = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMaterial<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_firstFree => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_firstFree) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstFree",
                                        ),
                                    );
                                }
                                m_firstFree = _serde::__private::Some(
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
                    let m_elements = match m_elements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("elements"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_firstFree = match m_firstFree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstFree",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations {
                        __ptr: __ptr.clone(),
                        m_elements,
                        m_firstFree,
                    })
                }
            }
            const FIELDS: &[&str] = &["elements", "firstFree"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations",
                FIELDS,
                __hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperationsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
