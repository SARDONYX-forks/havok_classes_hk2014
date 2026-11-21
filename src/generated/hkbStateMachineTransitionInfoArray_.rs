use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachineTransitionInfoArray`
/// - version: `0`
/// - signature: `0x704a19af`
/// - size: `  0`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineTransitionInfoArray<'a> {
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
    /// - name: `transitions`(ctype: `hkArray<struct hkbStateMachineTransitionInfo>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "transitions"))]
    pub m_transitions: Vec<hkbStateMachineTransitionInfo<'a>>,
    /// # C++ Info
    /// - name: `hasEventlessTransitions`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "hasEventlessTransitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "hasEventlessTransitions"))]
    pub m_hasEventlessTransitions: bool,
    /// # C++ Info
    /// - name: `hasTimeBoundedTransitions`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 33`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "hasTimeBoundedTransitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "hasTimeBoundedTransitions"))]
    pub m_hasTimeBoundedTransitions: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbStateMachineTransitionInfoArray<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineTransitionInfoArray"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x704a19af)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_transitions
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkbStateMachineTransitionInfoArray<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x704a19af)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbStateMachineTransitionInfoArray",
                    class_meta,
                    (0u64, 40u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "transitions",
                    &self.m_transitions,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 72u64,
                    },
                )?;
            serializer
                .skip_field("hasEventlessTransitions", &self.m_hasEventlessTransitions)?;
            serializer
                .skip_field(
                    "hasTimeBoundedTransitions",
                    &self.m_hasTimeBoundedTransitions,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineTransitionInfoArray<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transitions,
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
                        "transitions" => Ok(__Field::m_transitions),
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
            struct __hkbStateMachineTransitionInfoArrayVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbStateMachineTransitionInfoArray<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbStateMachineTransitionInfoArrayVisitor<'de> {
                type Value = hkbStateMachineTransitionInfoArray<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbStateMachineTransitionInfoArray",
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
                    let mut m_transitions: _serde::__private::Option<
                        Vec<hkbStateMachineTransitionInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_hasEventlessTransitions: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_hasTimeBoundedTransitions: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_transitions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitions",
                                        ),
                                    );
                                }
                                m_transitions = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbStateMachineTransitionInfo<'de>>,
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
                                    &m_hasEventlessTransitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hasEventlessTransitions",
                                        ),
                                    );
                                }
                                m_hasEventlessTransitions = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_hasTimeBoundedTransitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hasTimeBoundedTransitions",
                                        ),
                                    );
                                }
                                m_hasTimeBoundedTransitions = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 6usize)?;
                    let m_transitions = match m_transitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitions",
                                ),
                            );
                        }
                    };
                    let m_hasEventlessTransitions = match m_hasEventlessTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hasEventlessTransitions",
                                ),
                            );
                        }
                    };
                    let m_hasTimeBoundedTransitions = match m_hasTimeBoundedTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hasTimeBoundedTransitions",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbStateMachineTransitionInfoArray {
                        __ptr,
                        parent,
                        m_transitions,
                        m_hasEventlessTransitions,
                        m_hasTimeBoundedTransitions,
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
                    let mut m_transitions: _serde::__private::Option<
                        Vec<hkbStateMachineTransitionInfo<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_transitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transitions) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitions",
                                        ),
                                    );
                                }
                                m_transitions = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbStateMachineTransitionInfo<'de>>,
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
                    let m_transitions = match m_transitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitions",
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
                    _serde::__private::Ok(hkbStateMachineTransitionInfoArray {
                        __ptr: __ptr.clone(),
                        parent,
                        m_transitions,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transitions",
                "hasEventlessTransitions",
                "hasTimeBoundedTransitions",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineTransitionInfoArray",
                FIELDS,
                __hkbStateMachineTransitionInfoArrayVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineTransitionInfoArray,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
