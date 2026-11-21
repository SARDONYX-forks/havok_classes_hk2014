use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclClothData`
/// - version: `1`
/// - signature: `0xf943cea2`
/// - size: `  0`(x86)/`128`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclClothData<'a> {
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
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `simClothDatas`(ctype: `hkArray<hclSimClothData*>`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simClothDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "simClothDatas"))]
    pub m_simClothDatas: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `bufferDefinitions`(ctype: `hkArray<hclBufferDefinition*>`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bufferDefinitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "bufferDefinitions"))]
    pub m_bufferDefinitions: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `transformSetDefinitions`(ctype: `hkArray<hclTransformSetDefinition*>`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transformSetDefinitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "transformSetDefinitions"))]
    pub m_transformSetDefinitions: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `operators`(ctype: `hkArray<hclOperator*>`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "operators"))]
    #[cfg_attr(feature = "serde", serde(rename = "operators"))]
    pub m_operators: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `clothStateDatas`(ctype: `hkArray<hclClothState*>`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "clothStateDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "clothStateDatas"))]
    pub m_clothStateDatas: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `actions`(ctype: `hkArray<hclAction*>`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "actions"))]
    #[cfg_attr(feature = "serde", serde(rename = "actions"))]
    pub m_actions: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `targetPlatform`(ctype: `enum Platform`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetPlatform"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetPlatform"))]
    pub m_targetPlatform: Platform,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclClothData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclClothData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf943cea2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_simClothDatas.iter());
            v.extend(self.m_bufferDefinitions.iter());
            v.extend(self.m_transformSetDefinitions.iter());
            v.extend(self.m_operators.iter());
            v.extend(self.m_clothStateDatas.iter());
            v.extend(self.m_actions.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hclClothData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xf943cea2)));
            let mut serializer = __serializer
                .serialize_struct("hclClothData", class_meta, (0u64, 128u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer
                .serialize_array_field(
                    "simClothDatas",
                    &self.m_simClothDatas,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "bufferDefinitions",
                    &self.m_bufferDefinitions,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "transformSetDefinitions",
                    &self.m_transformSetDefinitions,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "operators",
                    &self.m_operators,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "clothStateDatas",
                    &self.m_clothStateDatas,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field("actions", &self.m_actions, TypeSize::NonPtr)?;
            serializer.serialize_field("targetPlatform", &self.m_targetPlatform)?;
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
    impl<'de> _serde::Deserialize<'de> for hclClothData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_simClothDatas,
                m_bufferDefinitions,
                m_transformSetDefinitions,
                m_operators,
                m_clothStateDatas,
                m_actions,
                m_targetPlatform,
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
                        "simClothDatas" => Ok(__Field::m_simClothDatas),
                        "bufferDefinitions" => Ok(__Field::m_bufferDefinitions),
                        "transformSetDefinitions" => {
                            Ok(__Field::m_transformSetDefinitions)
                        }
                        "operators" => Ok(__Field::m_operators),
                        "clothStateDatas" => Ok(__Field::m_clothStateDatas),
                        "actions" => Ok(__Field::m_actions),
                        "targetPlatform" => Ok(__Field::m_targetPlatform),
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
            struct __hclClothDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hclClothData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclClothDataVisitor<'de> {
                type Value = hclClothData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hclClothData")
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_simClothDatas: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_bufferDefinitions: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_transformSetDefinitions: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_operators: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_clothStateDatas: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_actions: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_targetPlatform: _serde::__private::Option<Platform> = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_name) {
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
                            1usize => {
                                if _serde::__private::Option::is_some(&m_simClothDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simClothDatas",
                                        ),
                                    );
                                }
                                m_simClothDatas = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bufferDefinitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferDefinitions",
                                        ),
                                    );
                                }
                                m_bufferDefinitions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transformSetDefinitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformSetDefinitions",
                                        ),
                                    );
                                }
                                m_transformSetDefinitions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_operators) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "operators",
                                        ),
                                    );
                                }
                                m_operators = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_clothStateDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clothStateDatas",
                                        ),
                                    );
                                }
                                m_clothStateDatas = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_actions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "actions",
                                        ),
                                    );
                                }
                                m_actions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_targetPlatform) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetPlatform",
                                        ),
                                    );
                                }
                                m_targetPlatform = _serde::__private::Some(
                                    match __A::next_value::<Platform>(&mut __map) {
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_simClothDatas = match m_simClothDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simClothDatas",
                                ),
                            );
                        }
                    };
                    let m_bufferDefinitions = match m_bufferDefinitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferDefinitions",
                                ),
                            );
                        }
                    };
                    let m_transformSetDefinitions = match m_transformSetDefinitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformSetDefinitions",
                                ),
                            );
                        }
                    };
                    let m_operators = match m_operators {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "operators",
                                ),
                            );
                        }
                    };
                    let m_clothStateDatas = match m_clothStateDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clothStateDatas",
                                ),
                            );
                        }
                    };
                    let m_actions = match m_actions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("actions"),
                            );
                        }
                    };
                    let m_targetPlatform = match m_targetPlatform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetPlatform",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclClothData {
                        __ptr,
                        parent,
                        m_name,
                        m_simClothDatas,
                        m_bufferDefinitions,
                        m_transformSetDefinitions,
                        m_operators,
                        m_clothStateDatas,
                        m_actions,
                        m_targetPlatform,
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
                    let mut m_simClothDatas: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_bufferDefinitions: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_transformSetDefinitions: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_operators: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_clothStateDatas: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_actions: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_targetPlatform: _serde::__private::Option<Platform> = _serde::__private::None;
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
                            __Field::m_simClothDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simClothDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simClothDatas",
                                        ),
                                    );
                                }
                                m_simClothDatas = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bufferDefinitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_bufferDefinitions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferDefinitions",
                                        ),
                                    );
                                }
                                m_bufferDefinitions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transformSetDefinitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transformSetDefinitions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformSetDefinitions",
                                        ),
                                    );
                                }
                                m_transformSetDefinitions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_operators => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_operators) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "operators",
                                        ),
                                    );
                                }
                                m_operators = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_clothStateDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_clothStateDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clothStateDatas",
                                        ),
                                    );
                                }
                                m_clothStateDatas = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_actions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_actions) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "actions",
                                        ),
                                    );
                                }
                                m_actions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetPlatform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_targetPlatform) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetPlatform",
                                        ),
                                    );
                                }
                                m_targetPlatform = _serde::__private::Some(
                                    match __A::next_value::<Platform>(&mut __map) {
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
                    let m_simClothDatas = match m_simClothDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simClothDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bufferDefinitions = match m_bufferDefinitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferDefinitions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transformSetDefinitions = match m_transformSetDefinitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformSetDefinitions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_operators = match m_operators {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "operators",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_clothStateDatas = match m_clothStateDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clothStateDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_actions = match m_actions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("actions"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetPlatform = match m_targetPlatform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetPlatform",
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
                    _serde::__private::Ok(hclClothData {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        m_simClothDatas,
                        m_bufferDefinitions,
                        m_transformSetDefinitions,
                        m_operators,
                        m_clothStateDatas,
                        m_actions,
                        m_targetPlatform,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "simClothDatas",
                "bufferDefinitions",
                "transformSetDefinitions",
                "operators",
                "clothStateDatas",
                "actions",
                "targetPlatform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclClothData",
                FIELDS,
                __hclClothDataVisitor {
                    marker: _serde::__private::PhantomData::<hclClothData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `Platform`(ctype: `hkEnum<Platform, hkUint32>`)
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
pub enum Platform {
    #[default]
    HCL_PLATFORM_INVALID = 0isize,
    HCL_PLATFORM_WIN32 = 1isize,
    HCL_PLATFORM_X64 = 2isize,
    HCL_PLATFORM_MACPPC = 4isize,
    HCL_PLATFORM_IOS = 8isize,
    HCL_PLATFORM_MAC386 = 16isize,
    HCL_PLATFORM_PS3 = 32isize,
    HCL_PLATFORM_XBOX360 = 64isize,
    HCL_PLATFORM_WII = 128isize,
    HCL_PLATFORM_LRB = 256isize,
    HCL_PLATFORM_LINUX = 512isize,
    HCL_PLATFORM_PSVITA = 1024isize,
    HCL_PLATFORM_ANDROID = 2048isize,
    HCL_PLATFORM_CTR = 4096isize,
    HCL_PLATFORM_WIIU = 8192isize,
    HCL_PLATFORM_PS4 = 16384isize,
    HCL_PLATFORM_XBOXONE = 32768isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Platform {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HCL_PLATFORM_INVALID => {
                    __serializer.serialize_field("HCL_PLATFORM_INVALID", &0u64)
                }
                Self::HCL_PLATFORM_WIN32 => {
                    __serializer.serialize_field("HCL_PLATFORM_WIN32", &1u64)
                }
                Self::HCL_PLATFORM_X64 => {
                    __serializer.serialize_field("HCL_PLATFORM_X64", &2u64)
                }
                Self::HCL_PLATFORM_MACPPC => {
                    __serializer.serialize_field("HCL_PLATFORM_MACPPC", &4u64)
                }
                Self::HCL_PLATFORM_IOS => {
                    __serializer.serialize_field("HCL_PLATFORM_IOS", &8u64)
                }
                Self::HCL_PLATFORM_MAC386 => {
                    __serializer.serialize_field("HCL_PLATFORM_MAC386", &16u64)
                }
                Self::HCL_PLATFORM_PS3 => {
                    __serializer.serialize_field("HCL_PLATFORM_PS3", &32u64)
                }
                Self::HCL_PLATFORM_XBOX360 => {
                    __serializer.serialize_field("HCL_PLATFORM_XBOX360", &64u64)
                }
                Self::HCL_PLATFORM_WII => {
                    __serializer.serialize_field("HCL_PLATFORM_WII", &128u64)
                }
                Self::HCL_PLATFORM_LRB => {
                    __serializer.serialize_field("HCL_PLATFORM_LRB", &256u64)
                }
                Self::HCL_PLATFORM_LINUX => {
                    __serializer.serialize_field("HCL_PLATFORM_LINUX", &512u64)
                }
                Self::HCL_PLATFORM_PSVITA => {
                    __serializer.serialize_field("HCL_PLATFORM_PSVITA", &1024u64)
                }
                Self::HCL_PLATFORM_ANDROID => {
                    __serializer.serialize_field("HCL_PLATFORM_ANDROID", &2048u64)
                }
                Self::HCL_PLATFORM_CTR => {
                    __serializer.serialize_field("HCL_PLATFORM_CTR", &4096u64)
                }
                Self::HCL_PLATFORM_WIIU => {
                    __serializer.serialize_field("HCL_PLATFORM_WIIU", &8192u64)
                }
                Self::HCL_PLATFORM_PS4 => {
                    __serializer.serialize_field("HCL_PLATFORM_PS4", &16384u64)
                }
                Self::HCL_PLATFORM_XBOXONE => {
                    __serializer.serialize_field("HCL_PLATFORM_XBOXONE", &32768u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u32()
                .ok_or(S::Error::custom("Failed enum Platform to_u32"))?;
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
    impl<'de> _serde::Deserialize<'de> for Platform {
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
                __field12,
                __field13,
                __field14,
                __field15,
                __field16,
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
                fn visit_uint32<__E>(
                    self,
                    __value: U32<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U32::Number(0u32) => _serde::__private::Ok(__Field::__field0),
                        U32::Number(1u32) => _serde::__private::Ok(__Field::__field1),
                        U32::Number(2u32) => _serde::__private::Ok(__Field::__field2),
                        U32::Number(4u32) => _serde::__private::Ok(__Field::__field3),
                        U32::Number(8u32) => _serde::__private::Ok(__Field::__field4),
                        U32::Number(16u32) => _serde::__private::Ok(__Field::__field5),
                        U32::Number(32u32) => _serde::__private::Ok(__Field::__field6),
                        U32::Number(64u32) => _serde::__private::Ok(__Field::__field7),
                        U32::Number(128u32) => _serde::__private::Ok(__Field::__field8),
                        U32::Number(256u32) => _serde::__private::Ok(__Field::__field9),
                        U32::Number(512u32) => _serde::__private::Ok(__Field::__field10),
                        U32::Number(1024u32) => _serde::__private::Ok(__Field::__field11),
                        U32::Number(2048u32) => _serde::__private::Ok(__Field::__field12),
                        U32::Number(4096u32) => _serde::__private::Ok(__Field::__field13),
                        U32::Number(8192u32) => _serde::__private::Ok(__Field::__field14),
                        U32::Number(16384u32) => {
                            _serde::__private::Ok(__Field::__field15)
                        }
                        U32::Number(32768u32) => {
                            _serde::__private::Ok(__Field::__field16)
                        }
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of 0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768",
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_WIN32") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_X64") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_MACPPC") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "8"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_IOS") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "16"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_MAC386") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "32"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_PS3") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "64"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_XBOX360") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "128"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_WII") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "256"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_LRB") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "512"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_LINUX") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "1024"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_PSVITA") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "2048"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_ANDROID") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "4096"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_CTR") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "8192"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_WIIU") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "16384"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_PS4") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "32768"
                                || v.eq_ignore_ascii_case("HCL_PLATFORM_XBOXONE") => {
                                _serde::__private::Ok(__Field::__field16)
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
                        _serde::de::ReadEnumSize::Uint32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Platform>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Platform;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Platform")
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
                            _serde::__private::Ok(Platform::HCL_PLATFORM_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_WIN32)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_X64)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_MACPPC)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_IOS)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_MAC386)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_PS3)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_XBOX360)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_WII)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_LRB)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_LINUX)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_PSVITA)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_ANDROID)
                        }
                        (__Field::__field13, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_CTR)
                        }
                        (__Field::__field14, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_WIIU)
                        }
                        (__Field::__field15, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_PS4)
                        }
                        (__Field::__field16, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Platform::HCL_PLATFORM_XBOXONE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HCL_PLATFORM_INVALID",
                "HCL_PLATFORM_WIN32",
                "HCL_PLATFORM_X64",
                "HCL_PLATFORM_MACPPC",
                "HCL_PLATFORM_IOS",
                "HCL_PLATFORM_MAC386",
                "HCL_PLATFORM_PS3",
                "HCL_PLATFORM_XBOX360",
                "HCL_PLATFORM_WII",
                "HCL_PLATFORM_LRB",
                "HCL_PLATFORM_LINUX",
                "HCL_PLATFORM_PSVITA",
                "HCL_PLATFORM_ANDROID",
                "HCL_PLATFORM_CTR",
                "HCL_PLATFORM_WIIU",
                "HCL_PLATFORM_PS4",
                "HCL_PLATFORM_XBOXONE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Platform",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Platform>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
