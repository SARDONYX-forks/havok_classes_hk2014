use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbVariableBindingSet`
/// - version: `2`
/// - signature: `0xe942f339`
/// - size: `  0`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableBindingSet<'a> {
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
    /// - name: `bindings`(ctype: `hkArray<struct hkbVariableBindingSetBinding>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bindings"))]
    #[cfg_attr(feature = "serde", serde(rename = "bindings"))]
    pub m_bindings: Vec<hkbVariableBindingSetBinding<'a>>,
    /// # C++ Info
    /// - name: `indexOfBindingToEnable`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexOfBindingToEnable"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexOfBindingToEnable"))]
    pub m_indexOfBindingToEnable: I32<'a>,
    /// # C++ Info
    /// - name: `hasOutputBinding`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "hasOutputBinding"))]
    #[cfg_attr(feature = "serde", serde(rename = "hasOutputBinding"))]
    pub m_hasOutputBinding: bool,
    /// # C++ Info
    /// - name: `initializedOffsets`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 37`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "initializedOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "initializedOffsets"))]
    pub m_initializedOffsets: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbVariableBindingSet<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableBindingSet"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe942f339)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_bindings
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkbVariableBindingSet<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xe942f339)));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableBindingSet", class_meta, (0u64, 40u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "bindings",
                    &self.m_bindings,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 40u64,
                    },
                )?;
            serializer
                .serialize_field(
                    "indexOfBindingToEnable",
                    &self.m_indexOfBindingToEnable,
                )?;
            serializer.skip_field("hasOutputBinding", &self.m_hasOutputBinding)?;
            serializer.skip_field("initializedOffsets", &self.m_initializedOffsets)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbVariableBindingSet<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_bindings,
                m_indexOfBindingToEnable,
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
                        "bindings" => Ok(__Field::m_bindings),
                        "indexOfBindingToEnable" => Ok(__Field::m_indexOfBindingToEnable),
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
            struct __hkbVariableBindingSetVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbVariableBindingSet<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbVariableBindingSetVisitor<'de> {
                type Value = hkbVariableBindingSet<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbVariableBindingSet",
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
                    let mut m_bindings: _serde::__private::Option<
                        Vec<hkbVariableBindingSetBinding<'de>>,
                    > = _serde::__private::None;
                    let mut m_indexOfBindingToEnable: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_hasOutputBinding: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_initializedOffsets: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_bindings) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bindings",
                                        ),
                                    );
                                }
                                m_bindings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbVariableBindingSetBinding<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_indexOfBindingToEnable,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexOfBindingToEnable",
                                        ),
                                    );
                                }
                                m_indexOfBindingToEnable = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_hasOutputBinding) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hasOutputBinding",
                                        ),
                                    );
                                }
                                m_hasOutputBinding = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_initializedOffsets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initializedOffsets",
                                        ),
                                    );
                                }
                                m_initializedOffsets = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 2usize)?;
                    let m_bindings = match m_bindings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bindings"),
                            );
                        }
                    };
                    let m_indexOfBindingToEnable = match m_indexOfBindingToEnable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexOfBindingToEnable",
                                ),
                            );
                        }
                    };
                    let m_hasOutputBinding = match m_hasOutputBinding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hasOutputBinding",
                                ),
                            );
                        }
                    };
                    let m_initializedOffsets = match m_initializedOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initializedOffsets",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbVariableBindingSet {
                        __ptr,
                        parent,
                        m_bindings,
                        m_indexOfBindingToEnable,
                        m_hasOutputBinding,
                        m_initializedOffsets,
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
                    let mut m_bindings: _serde::__private::Option<
                        Vec<hkbVariableBindingSetBinding<'de>>,
                    > = _serde::__private::None;
                    let mut m_indexOfBindingToEnable: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_bindings => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bindings) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bindings",
                                        ),
                                    );
                                }
                                m_bindings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbVariableBindingSetBinding<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexOfBindingToEnable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_indexOfBindingToEnable,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexOfBindingToEnable",
                                        ),
                                    );
                                }
                                m_indexOfBindingToEnable = _serde::__private::Some(
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
                    let m_bindings = match m_bindings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bindings"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexOfBindingToEnable = match m_indexOfBindingToEnable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexOfBindingToEnable",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbVariableBindingSet {
                        __ptr: __ptr.clone(),
                        parent,
                        m_bindings,
                        m_indexOfBindingToEnable,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "bindings",
                "indexOfBindingToEnable",
                "hasOutputBinding",
                "initializedOffsets",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbVariableBindingSet",
                FIELDS,
                __hkbVariableBindingSetVisitor {
                    marker: _serde::__private::PhantomData::<hkbVariableBindingSet>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
