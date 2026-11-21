use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbToolNodeType`
/// - version: `0`
/// - signature: `0xad59b7df`
/// - size: `  0`(x86)/`  1`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbToolNodeType<'a> {
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
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbToolNodeType<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbToolNodeType"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xad59b7df)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbToolNodeType<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xad59b7df)));
            let mut serializer = __serializer
                .serialize_struct("hkbToolNodeType", class_meta, (0u64, 1u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbToolNodeType<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
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
            struct __hkbToolNodeTypeVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbToolNodeType<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbToolNodeTypeVisitor<'de> {
                type Value = hkbToolNodeType<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbToolNodeType",
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
                    for i in 0..0usize {
                        match i {
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 0usize, 1usize)?;
                    _serde::__private::Ok(hkbToolNodeType { __ptr })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbToolNodeType {
                        __ptr: __ptr.clone(),
                    })
                }
            }
            const FIELDS: &[&str] = &[];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbToolNodeType",
                FIELDS,
                __hkbToolNodeTypeVisitor {
                    marker: _serde::__private::PhantomData::<hkbToolNodeType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `NodeType`(ctype: `hkEnum<NodeType, hkUint16>`)
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
pub enum NodeType {
    #[default]
    NODE_TYPE_UNKNOWN = 0isize,
    NODE_TYPE_STATE_MACHINE = 1isize,
    NODE_TYPE_CLIP = 2isize,
    NODE_TYPE_BLEND = 4isize,
    NODE_TYPE_MODIFIER = 8isize,
    NODE_TYPE_GENERATOR = 16isize,
    NODE_TYPE_MODIFIER_GENERATOR = 32isize,
    NODE_TYPE_TRANSITION_EFFECT = 64isize,
    NODE_TYPE_BEHAVIOR_FILE_REFERENCE = 128isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for NodeType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NODE_TYPE_UNKNOWN => {
                    __serializer.serialize_field("NODE_TYPE_UNKNOWN", &0u64)
                }
                Self::NODE_TYPE_STATE_MACHINE => {
                    __serializer.serialize_field("NODE_TYPE_STATE_MACHINE", &1u64)
                }
                Self::NODE_TYPE_CLIP => {
                    __serializer.serialize_field("NODE_TYPE_CLIP", &2u64)
                }
                Self::NODE_TYPE_BLEND => {
                    __serializer.serialize_field("NODE_TYPE_BLEND", &4u64)
                }
                Self::NODE_TYPE_MODIFIER => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER", &8u64)
                }
                Self::NODE_TYPE_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_GENERATOR", &16u64)
                }
                Self::NODE_TYPE_MODIFIER_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER_GENERATOR", &32u64)
                }
                Self::NODE_TYPE_TRANSITION_EFFECT => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION_EFFECT", &64u64)
                }
                Self::NODE_TYPE_BEHAVIOR_FILE_REFERENCE => {
                    __serializer
                        .serialize_field("NODE_TYPE_BEHAVIOR_FILE_REFERENCE", &128u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum NodeType to_u16"))?;
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
    impl<'de> _serde::Deserialize<'de> for NodeType {
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
                fn visit_uint16<__E>(
                    self,
                    __value: U16<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U16::Number(0u16) => _serde::__private::Ok(__Field::__field0),
                        U16::Number(1u16) => _serde::__private::Ok(__Field::__field1),
                        U16::Number(2u16) => _serde::__private::Ok(__Field::__field2),
                        U16::Number(4u16) => _serde::__private::Ok(__Field::__field3),
                        U16::Number(8u16) => _serde::__private::Ok(__Field::__field4),
                        U16::Number(16u16) => _serde::__private::Ok(__Field::__field5),
                        U16::Number(32u16) => _serde::__private::Ok(__Field::__field6),
                        U16::Number(64u16) => _serde::__private::Ok(__Field::__field7),
                        U16::Number(128u16) => _serde::__private::Ok(__Field::__field8),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint16(__value),
                                    &"value(u16) of variant is one of 0, 1, 2, 4, 8, 16, 32, 64, 128",
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
                                || v.eq_ignore_ascii_case("NODE_TYPE_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("NODE_TYPE_STATE_MACHINE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("NODE_TYPE_CLIP") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("NODE_TYPE_BLEND") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "8"
                                || v.eq_ignore_ascii_case("NODE_TYPE_MODIFIER") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "16"
                                || v.eq_ignore_ascii_case("NODE_TYPE_GENERATOR") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "32"
                                || v
                                    .eq_ignore_ascii_case("NODE_TYPE_MODIFIER_GENERATOR") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "64"
                                || v.eq_ignore_ascii_case("NODE_TYPE_TRANSITION_EFFECT") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "128"
                                || v
                                    .eq_ignore_ascii_case(
                                        "NODE_TYPE_BEHAVIOR_FILE_REFERENCE",
                                    ) => _serde::__private::Ok(__Field::__field8),
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
                        _serde::de::ReadEnumSize::Uint16,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<NodeType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = NodeType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum NodeType")
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
                            _serde::__private::Ok(NodeType::NODE_TYPE_UNKNOWN)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_STATE_MACHINE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_CLIP)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_BLEND)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_MODIFIER)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_GENERATOR)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_MODIFIER_GENERATOR)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_TRANSITION_EFFECT)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                NodeType::NODE_TYPE_BEHAVIOR_FILE_REFERENCE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "NODE_TYPE_UNKNOWN",
                "NODE_TYPE_STATE_MACHINE",
                "NODE_TYPE_CLIP",
                "NODE_TYPE_BLEND",
                "NODE_TYPE_MODIFIER",
                "NODE_TYPE_GENERATOR",
                "NODE_TYPE_MODIFIER_GENERATOR",
                "NODE_TYPE_TRANSITION_EFFECT",
                "NODE_TYPE_BEHAVIOR_FILE_REFERENCE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "NodeType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<NodeType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
