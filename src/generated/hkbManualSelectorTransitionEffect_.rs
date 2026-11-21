use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbManualSelectorTransitionEffect`
/// - version: `0`
/// - signature: `0x9c02b0c0`
/// - size: `  0`(x86)/`232`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbManualSelectorTransitionEffect<'a> {
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
    pub parent: hkbTransitionEffect<'a>,
    /// # C++ Info
    /// - name: `transitionEffects`(ctype: `hkArray<hkbTransitionEffect*>`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transitionEffects"))]
    #[cfg_attr(feature = "serde", serde(rename = "transitionEffects"))]
    pub m_transitionEffects: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `selectedIndex`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "selectedIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "selectedIndex"))]
    pub m_selectedIndex: U8<'a>,
    /// # C++ Info
    /// - name: `indexSelector`(ctype: `struct hkbCustomIdSelector*`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexSelector"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexSelector"))]
    pub m_indexSelector: Pointer<'a>,
    /// # C++ Info
    /// - name: `currentTransitionEffectIndex`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "currentTransitionEffectIndex")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "currentTransitionEffectIndex"))]
    pub m_currentTransitionEffectIndex: U8<'a>,
    /// # C++ Info
    /// - name: `currentTransitionEffect`(ctype: `void*`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentTransitionEffect"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentTransitionEffect"))]
    pub m_currentTransitionEffect: Pointer<'a>,
    /// # C++ Info
    /// - name: `parentStateMachine`(ctype: `void*`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "parentStateMachine"))]
    #[cfg_attr(feature = "serde", serde(rename = "parentStateMachine"))]
    pub m_parentStateMachine: Pointer<'a>,
    /// # C++ Info
    /// - name: `referenceBehavior`(ctype: `void*`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "referenceBehavior"))]
    #[cfg_attr(feature = "serde", serde(rename = "referenceBehavior"))]
    pub m_referenceBehavior: Pointer<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbManualSelectorTransitionEffect<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbManualSelectorTransitionEffect"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9c02b0c0)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.parent.m_nodeInfo);
            v.extend(self.parent.parent.m_partitionInfo.deps_indexes());
            v.push(&self.parent.parent.m_syncInfo);
            v.push(&self.parent.m_patchedBindingInfo);
            v.push(&self.parent.m_fromGenerator);
            v.push(&self.parent.m_toGenerator);
            v.extend(self.m_transitionEffects.iter());
            v.push(&self.m_indexSelector);
            v.push(&self.m_currentTransitionEffect);
            v.push(&self.m_parentStateMachine);
            v.push(&self.m_referenceBehavior);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbManualSelectorTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x9c02b0c0)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbManualSelectorTransitionEffect",
                    class_meta,
                    (0u64, 232u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self.parent.parent.parent.parent.m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("nodeInfo", &self.parent.parent.parent.m_nodeInfo)?;
            serializer.skip_field("partitionInfo", &self.parent.parent.m_partitionInfo)?;
            serializer.skip_field("syncInfo", &self.parent.parent.m_syncInfo)?;
            serializer
                .skip_fixed_array_field(
                    "pad",
                    self.parent.parent.m_pad.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "selfTransitionMode",
                    &self.parent.m_selfTransitionMode,
                )?;
            serializer.serialize_field("eventMode", &self.parent.m_eventMode)?;
            serializer.skip_field("defaultEventMode", &self.parent.m_defaultEventMode)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .skip_field("patchedBindingInfo", &self.parent.m_patchedBindingInfo)?;
            serializer.skip_field("fromGenerator", &self.parent.m_fromGenerator)?;
            serializer.skip_field("toGenerator", &self.parent.m_toGenerator)?;
            serializer
                .serialize_array_field(
                    "transitionEffects",
                    &self.m_transitionEffects,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("selectedIndex", &self.m_selectedIndex)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("indexSelector", &self.m_indexSelector)?;
            serializer
                .skip_field(
                    "currentTransitionEffectIndex",
                    &self.m_currentTransitionEffectIndex,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .skip_field("currentTransitionEffect", &self.m_currentTransitionEffect)?;
            serializer.skip_field("parentStateMachine", &self.m_parentStateMachine)?;
            serializer.skip_field("referenceBehavior", &self.m_referenceBehavior)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbManualSelectorTransitionEffect<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_selfTransitionMode,
                m_eventMode,
                m_transitionEffects,
                m_selectedIndex,
                m_indexSelector,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "selfTransitionMode" => Ok(__Field::m_selfTransitionMode),
                        "eventMode" => Ok(__Field::m_eventMode),
                        "transitionEffects" => Ok(__Field::m_transitionEffects),
                        "selectedIndex" => Ok(__Field::m_selectedIndex),
                        "indexSelector" => Ok(__Field::m_indexSelector),
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
            struct __hkbManualSelectorTransitionEffectVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbManualSelectorTransitionEffect<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbManualSelectorTransitionEffectVisitor<'de> {
                type Value = hkbManualSelectorTransitionEffect<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbManualSelectorTransitionEffect",
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
                    let mut m_transitionEffects: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_selectedIndex: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_indexSelector: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_currentTransitionEffectIndex: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_currentTransitionEffect: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_parentStateMachine: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_referenceBehavior: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transitionEffects,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionEffects",
                                        ),
                                    );
                                }
                                m_transitionEffects = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_selectedIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selectedIndex",
                                        ),
                                    );
                                }
                                m_selectedIndex = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_indexSelector) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexSelector",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_indexSelector = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentTransitionEffectIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentTransitionEffectIndex",
                                        ),
                                    );
                                }
                                m_currentTransitionEffectIndex = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentTransitionEffect,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentTransitionEffect",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_currentTransitionEffect = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_parentStateMachine,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "parentStateMachine",
                                        ),
                                    );
                                }
                                m_parentStateMachine = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_referenceBehavior,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referenceBehavior",
                                        ),
                                    );
                                }
                                m_referenceBehavior = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                    let m_transitionEffects = match m_transitionEffects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionEffects",
                                ),
                            );
                        }
                    };
                    let m_selectedIndex = match m_selectedIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selectedIndex",
                                ),
                            );
                        }
                    };
                    let m_indexSelector = match m_indexSelector {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexSelector",
                                ),
                            );
                        }
                    };
                    let m_currentTransitionEffectIndex = match m_currentTransitionEffectIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentTransitionEffectIndex",
                                ),
                            );
                        }
                    };
                    let m_currentTransitionEffect = match m_currentTransitionEffect {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentTransitionEffect",
                                ),
                            );
                        }
                    };
                    let m_parentStateMachine = match m_parentStateMachine {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "parentStateMachine",
                                ),
                            );
                        }
                    };
                    let m_referenceBehavior = match m_referenceBehavior {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referenceBehavior",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbManualSelectorTransitionEffect {
                        __ptr,
                        parent,
                        m_transitionEffects,
                        m_selectedIndex,
                        m_indexSelector,
                        m_currentTransitionEffectIndex,
                        m_currentTransitionEffect,
                        m_parentStateMachine,
                        m_referenceBehavior,
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
                    let mut m_variableBindingSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_selfTransitionMode: _serde::__private::Option<
                        SelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_eventMode: _serde::__private::Option<EventMode> = _serde::__private::None;
                    let mut m_transitionEffects: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_selectedIndex: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_indexSelector: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_selfTransitionMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_selfTransitionMode,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selfTransitionMode",
                                        ),
                                    );
                                }
                                m_selfTransitionMode = _serde::__private::Some(
                                    match __A::next_value::<SelfTransitionMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eventMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventMode",
                                        ),
                                    );
                                }
                                m_eventMode = _serde::__private::Some(
                                    match __A::next_value::<EventMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionEffects => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transitionEffects,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionEffects",
                                        ),
                                    );
                                }
                                m_transitionEffects = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_selectedIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_selectedIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selectedIndex",
                                        ),
                                    );
                                }
                                m_selectedIndex = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexSelector => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indexSelector) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexSelector",
                                        ),
                                    );
                                }
                                m_indexSelector = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_selfTransitionMode = match m_selfTransitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selfTransitionMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eventMode = match m_eventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionEffects = match m_transitionEffects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionEffects",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_selectedIndex = match m_selectedIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selectedIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexSelector = match m_indexSelector {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexSelector",
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
                    let parent = hkbBindable {
                        __ptr: __ptr.clone(),
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr: __ptr.clone(),
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        m_selfTransitionMode,
                        m_eventMode,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbManualSelectorTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        m_transitionEffects,
                        m_selectedIndex,
                        m_indexSelector,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transitionEffects",
                "selectedIndex",
                "indexSelector",
                "currentTransitionEffectIndex",
                "currentTransitionEffect",
                "parentStateMachine",
                "referenceBehavior",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbManualSelectorTransitionEffect",
                FIELDS,
                __hkbManualSelectorTransitionEffectVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbManualSelectorTransitionEffect,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
