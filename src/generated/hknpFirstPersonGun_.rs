use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpFirstPersonGun`
/// - version: `0`
/// - signature: `0xd2140666`
/// - size: `  0`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpFirstPersonGun<'a> {
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
    /// - name: `type`(ctype: `enum unknown`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub m_type: U8<'a>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `keyboardKey`(ctype: `enum KeyboardKey`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "keyboardKey"))]
    #[cfg_attr(feature = "serde", serde(rename = "keyboardKey"))]
    pub m_keyboardKey: KeyboardKey,
    /// # C++ Info
    /// - name: `listeners`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "listeners"))]
    #[cfg_attr(feature = "serde", serde(rename = "listeners"))]
    pub m_listeners: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionFilterInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionFilterInfo"))]
    pub m_collisionFilterInfo: U32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpFirstPersonGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpFirstPersonGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd2140666)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_listeners.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hknpFirstPersonGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xd2140666)));
            let mut serializer = __serializer
                .serialize_struct("hknpFirstPersonGun", class_meta, (0u64, 64u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("keyboardKey", &self.m_keyboardKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .skip_array_field("listeners", &self.m_listeners, TypeSize::NonPtr)?;
            serializer.skip_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
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
    impl<'de> _serde::Deserialize<'de> for hknpFirstPersonGun<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_keyboardKey,
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
                        "name" => Ok(__Field::m_name),
                        "keyboardKey" => Ok(__Field::m_keyboardKey),
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
            struct __hknpFirstPersonGunVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpFirstPersonGun<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpFirstPersonGunVisitor<'de> {
                type Value = hknpFirstPersonGun<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpFirstPersonGun",
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
                    let mut m_type: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_keyboardKey: _serde::__private::Option<KeyboardKey> = _serde::__private::None;
                    let mut m_listeners: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_type) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_name) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_keyboardKey) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keyboardKey",
                                        ),
                                    );
                                }
                                m_keyboardKey = _serde::__private::Some(
                                    match __A::next_value::<KeyboardKey>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_listeners) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "listeners",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_listeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_keyboardKey = match m_keyboardKey {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keyboardKey",
                                ),
                            );
                        }
                    };
                    let m_listeners = match m_listeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "listeners",
                                ),
                            );
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpFirstPersonGun {
                        __ptr,
                        parent,
                        m_type,
                        m_name,
                        m_keyboardKey,
                        m_listeners,
                        m_collisionFilterInfo,
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_keyboardKey: _serde::__private::Option<KeyboardKey> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
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
                            __Field::m_keyboardKey => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_keyboardKey) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keyboardKey",
                                        ),
                                    );
                                }
                                m_keyboardKey = _serde::__private::Some(
                                    match __A::next_value::<KeyboardKey>(&mut __map) {
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
                    let m_keyboardKey = match m_keyboardKey {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keyboardKey",
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
                    _serde::__private::Ok(hknpFirstPersonGun {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        m_keyboardKey,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "type",
                "name",
                "keyboardKey",
                "listeners",
                "collisionFilterInfo",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpFirstPersonGun",
                FIELDS,
                __hknpFirstPersonGunVisitor {
                    marker: _serde::__private::PhantomData::<hknpFirstPersonGun>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `KeyboardKey`(ctype: `hkEnum<KeyboardKey, hkUint8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum KeyboardKey {
    #[default]
    KEY_F1 = 112isize,
    KEY_F2 = 113isize,
    KEY_F3 = 114isize,
    KEY_F4 = 115isize,
    KEY_F5 = 116isize,
    KEY_F6 = 117isize,
    KEY_F7 = 118isize,
    KEY_F8 = 119isize,
    KEY_F9 = 120isize,
    KEY_F10 = 121isize,
    KEY_F11 = 122isize,
    KEY_F12 = 123isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for KeyboardKey {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::KEY_F1 => __serializer.serialize_field("KEY_F1", &112u64),
                Self::KEY_F2 => __serializer.serialize_field("KEY_F2", &113u64),
                Self::KEY_F3 => __serializer.serialize_field("KEY_F3", &114u64),
                Self::KEY_F4 => __serializer.serialize_field("KEY_F4", &115u64),
                Self::KEY_F5 => __serializer.serialize_field("KEY_F5", &116u64),
                Self::KEY_F6 => __serializer.serialize_field("KEY_F6", &117u64),
                Self::KEY_F7 => __serializer.serialize_field("KEY_F7", &118u64),
                Self::KEY_F8 => __serializer.serialize_field("KEY_F8", &119u64),
                Self::KEY_F9 => __serializer.serialize_field("KEY_F9", &120u64),
                Self::KEY_F10 => __serializer.serialize_field("KEY_F10", &121u64),
                Self::KEY_F11 => __serializer.serialize_field("KEY_F11", &122u64),
                Self::KEY_F12 => __serializer.serialize_field("KEY_F12", &123u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum KeyboardKey to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for KeyboardKey {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(112u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(113u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(114u8) => _serde::__private::Ok(__Field::__field2),
                        U8::Number(115u8) => _serde::__private::Ok(__Field::__field3),
                        U8::Number(116u8) => _serde::__private::Ok(__Field::__field4),
                        U8::Number(117u8) => _serde::__private::Ok(__Field::__field5),
                        U8::Number(118u8) => _serde::__private::Ok(__Field::__field6),
                        U8::Number(119u8) => _serde::__private::Ok(__Field::__field7),
                        U8::Number(120u8) => _serde::__private::Ok(__Field::__field8),
                        U8::Number(121u8) => _serde::__private::Ok(__Field::__field9),
                        U8::Number(122u8) => _serde::__private::Ok(__Field::__field10),
                        U8::Number(123u8) => _serde::__private::Ok(__Field::__field11),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "112" || v.eq_ignore_ascii_case("KEY_F1") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "113" || v.eq_ignore_ascii_case("KEY_F2") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "114" || v.eq_ignore_ascii_case("KEY_F3") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "115" || v.eq_ignore_ascii_case("KEY_F4") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "116" || v.eq_ignore_ascii_case("KEY_F5") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "117" || v.eq_ignore_ascii_case("KEY_F6") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "118" || v.eq_ignore_ascii_case("KEY_F7") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "119" || v.eq_ignore_ascii_case("KEY_F8") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "120" || v.eq_ignore_ascii_case("KEY_F9") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "121" || v.eq_ignore_ascii_case("KEY_F10") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "122" || v.eq_ignore_ascii_case("KEY_F11") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "123" || v.eq_ignore_ascii_case("KEY_F12") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<KeyboardKey>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = KeyboardKey;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum KeyboardKey",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F1)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F2)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F3)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F4)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F5)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F6)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F7)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F8)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F9)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F10)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F11)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F12)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "KEY_F1",
                "KEY_F2",
                "KEY_F3",
                "KEY_F4",
                "KEY_F5",
                "KEY_F6",
                "KEY_F7",
                "KEY_F8",
                "KEY_F9",
                "KEY_F10",
                "KEY_F11",
                "KEY_F12",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "KeyboardKey",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<KeyboardKey>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
