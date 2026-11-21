use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConstraintData`
/// - version: `0`
/// - signature: `0x57339dd7`
/// - size: `  0`(x86)/` 24`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintData<'a> {
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
    /// - name: `userData`(ctype: `hkUlong`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: Ulong,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpConstraintData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x57339dd7)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpConstraintData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x57339dd7)));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintData", class_meta, (0u64, 24u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConstraintData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
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
                        "userData" => Ok(__Field::m_userData),
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
            struct __hkpConstraintDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpConstraintData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpConstraintDataVisitor<'de> {
                type Value = hkpConstraintData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpConstraintData",
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
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_userData) {
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
                            _ => {}
                        }
                    }
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpConstraintData {
                        __ptr,
                        parent,
                        m_userData,
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
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
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
                    _serde::__private::Ok(hkpConstraintData {
                        __ptr: __ptr.clone(),
                        parent,
                        m_userData,
                    })
                }
            }
            const FIELDS: &[&str] = &["userData"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConstraintData",
                FIELDS,
                __hkpConstraintDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpConstraintData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Enum as bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `ConstraintType`(ctype: `hkEnum<ConstraintType, hkUint8>`)"] #[doc = r""]
    #[doc = r" # Why this `enum` defined as `bitflags`?"] #[doc =
    r" Since Rust does not allow the definition of `enum` with duplicate discriminant values, they are defined as `bitflags`."]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    ConstraintType : u8 { #[doc = "0"] const CONSTRAINT_TYPE_BALLANDSOCKET = 0u8; #[doc =
    "1"] const CONSTRAINT_TYPE_HINGE = 1u8; #[doc = "2"] const
    CONSTRAINT_TYPE_LIMITEDHINGE = 2u8; #[doc = "3"] const CONSTRAINT_TYPE_POINTTOPATH =
    3u8; #[doc = "6"] const CONSTRAINT_TYPE_PRISMATIC = 6u8; #[doc = "7"] const
    CONSTRAINT_TYPE_RAGDOLL = 7u8; #[doc = "8"] const CONSTRAINT_TYPE_STIFFSPRING = 8u8;
    #[doc = "9"] const CONSTRAINT_TYPE_WHEEL = 9u8; #[doc = "10"] const
    CONSTRAINT_TYPE_GENERIC = 10u8; #[doc = "11"] const CONSTRAINT_TYPE_CONTACT = 11u8;
    #[doc = "12"] const CONSTRAINT_TYPE_BREAKABLE = 12u8; #[doc = "13"] const
    CONSTRAINT_TYPE_MALLEABLE = 13u8; #[doc = "14"] const CONSTRAINT_TYPE_POINTTOPLANE =
    14u8; #[doc = "15"] const CONSTRAINT_TYPE_PULLEY = 15u8; #[doc = "16"] const
    CONSTRAINT_TYPE_ROTATIONAL = 16u8; #[doc = "18"] const CONSTRAINT_TYPE_HINGE_LIMITS =
    18u8; #[doc = "19"] const CONSTRAINT_TYPE_RAGDOLL_LIMITS = 19u8; #[doc = "20"] const
    CONSTRAINT_TYPE_CUSTOM = 20u8; #[doc = "21"] const CONSTRAINT_TYPE_RACK_AND_PINION =
    21u8; #[doc = "22"] const CONSTRAINT_TYPE_COG_WHEEL = 22u8; #[doc = "23"] const
    CONSTRAINT_TYPE_FIXED = 23u8; #[doc = "24"] const CONSTRAINT_TYPE_DEFORMABLE_FIXED =
    24u8; #[doc = "25"] const CONSTRAINT_TYPE_LINEAR_SLACK = 25u8; #[doc = "100"] const
    BEGIN_CONSTRAINT_CHAIN_TYPES = 100u8; #[doc = "100"] const
    CONSTRAINT_TYPE_STIFF_SPRING_CHAIN = 100u8; #[doc = "101"] const
    CONSTRAINT_TYPE_BALL_SOCKET_CHAIN = 101u8; #[doc = "102"] const
    CONSTRAINT_TYPE_POWERED_CHAIN = 102u8; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for ConstraintType {
        fn schema_name() -> Cow<'static, str> {
            "ConstraintType".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "ConstraintType").into()
        }
        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            use schemars::_private::serde_json::{self, Value};
            let selection = &[
                "CONSTRAINT_TYPE_BALLANDSOCKET",
                "CONSTRAINT_TYPE_HINGE",
                "CONSTRAINT_TYPE_LIMITEDHINGE",
                "CONSTRAINT_TYPE_POINTTOPATH",
                "CONSTRAINT_TYPE_PRISMATIC",
                "CONSTRAINT_TYPE_RAGDOLL",
                "CONSTRAINT_TYPE_STIFFSPRING",
                "CONSTRAINT_TYPE_WHEEL",
                "CONSTRAINT_TYPE_GENERIC",
                "CONSTRAINT_TYPE_CONTACT",
                "CONSTRAINT_TYPE_BREAKABLE",
                "CONSTRAINT_TYPE_MALLEABLE",
                "CONSTRAINT_TYPE_POINTTOPLANE",
                "CONSTRAINT_TYPE_PULLEY",
                "CONSTRAINT_TYPE_ROTATIONAL",
                "CONSTRAINT_TYPE_HINGE_LIMITS",
                "CONSTRAINT_TYPE_RAGDOLL_LIMITS",
                "CONSTRAINT_TYPE_CUSTOM",
                "CONSTRAINT_TYPE_RACK_AND_PINION",
                "CONSTRAINT_TYPE_COG_WHEEL",
                "CONSTRAINT_TYPE_FIXED",
                "CONSTRAINT_TYPE_DEFORMABLE_FIXED",
                "CONSTRAINT_TYPE_LINEAR_SLACK",
                "BEGIN_CONSTRAINT_CHAIN_TYPES",
                "CONSTRAINT_TYPE_STIFF_SPRING_CHAIN",
                "CONSTRAINT_TYPE_BALL_SOCKET_CHAIN",
                "CONSTRAINT_TYPE_POWERED_CHAIN",
            ];
            let selection = selection
                .iter()
                .map(|s| Value::String(s.to_string()))
                .collect();
            let mut schema = Value::json_schema(generator);
            let mut map = schema.ensure_object();
            map.insert("type".to_string(), Value::String("string".to_string()));
            map.insert("enum".to_string(), Value::Array(selection));
            schema
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ConstraintType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match *self {
                Self::CONSTRAINT_TYPE_BALLANDSOCKET => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_BALLANDSOCKET", &0u64)
                }
                Self::CONSTRAINT_TYPE_HINGE => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_HINGE", &1u64)
                }
                Self::CONSTRAINT_TYPE_LIMITEDHINGE => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_LIMITEDHINGE", &2u64)
                }
                Self::CONSTRAINT_TYPE_POINTTOPATH => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_POINTTOPATH", &3u64)
                }
                Self::CONSTRAINT_TYPE_PRISMATIC => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_PRISMATIC", &6u64)
                }
                Self::CONSTRAINT_TYPE_RAGDOLL => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_RAGDOLL", &7u64)
                }
                Self::CONSTRAINT_TYPE_STIFFSPRING => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_STIFFSPRING", &8u64)
                }
                Self::CONSTRAINT_TYPE_WHEEL => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_WHEEL", &9u64)
                }
                Self::CONSTRAINT_TYPE_GENERIC => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_GENERIC", &10u64)
                }
                Self::CONSTRAINT_TYPE_CONTACT => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_CONTACT", &11u64)
                }
                Self::CONSTRAINT_TYPE_BREAKABLE => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_BREAKABLE", &12u64)
                }
                Self::CONSTRAINT_TYPE_MALLEABLE => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_MALLEABLE", &13u64)
                }
                Self::CONSTRAINT_TYPE_POINTTOPLANE => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_POINTTOPLANE", &14u64)
                }
                Self::CONSTRAINT_TYPE_PULLEY => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_PULLEY", &15u64)
                }
                Self::CONSTRAINT_TYPE_ROTATIONAL => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_ROTATIONAL", &16u64)
                }
                Self::CONSTRAINT_TYPE_HINGE_LIMITS => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_HINGE_LIMITS", &18u64)
                }
                Self::CONSTRAINT_TYPE_RAGDOLL_LIMITS => {
                    __serializer
                        .serialize_field("CONSTRAINT_TYPE_RAGDOLL_LIMITS", &19u64)
                }
                Self::CONSTRAINT_TYPE_CUSTOM => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_CUSTOM", &20u64)
                }
                Self::CONSTRAINT_TYPE_RACK_AND_PINION => {
                    __serializer
                        .serialize_field("CONSTRAINT_TYPE_RACK_AND_PINION", &21u64)
                }
                Self::CONSTRAINT_TYPE_COG_WHEEL => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_COG_WHEEL", &22u64)
                }
                Self::CONSTRAINT_TYPE_FIXED => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_FIXED", &23u64)
                }
                Self::CONSTRAINT_TYPE_DEFORMABLE_FIXED => {
                    __serializer
                        .serialize_field("CONSTRAINT_TYPE_DEFORMABLE_FIXED", &24u64)
                }
                Self::CONSTRAINT_TYPE_LINEAR_SLACK => {
                    __serializer.serialize_field("CONSTRAINT_TYPE_LINEAR_SLACK", &25u64)
                }
                Self::BEGIN_CONSTRAINT_CHAIN_TYPES => {
                    __serializer.serialize_field("BEGIN_CONSTRAINT_CHAIN_TYPES", &100u64)
                }
                Self::CONSTRAINT_TYPE_STIFF_SPRING_CHAIN => {
                    __serializer
                        .serialize_field("CONSTRAINT_TYPE_STIFF_SPRING_CHAIN", &100u64)
                }
                Self::CONSTRAINT_TYPE_BALL_SOCKET_CHAIN => {
                    __serializer
                        .serialize_field("CONSTRAINT_TYPE_BALL_SOCKET_CHAIN", &101u64)
                }
                Self::CONSTRAINT_TYPE_POWERED_CHAIN => {
                    __serializer
                        .serialize_field("CONSTRAINT_TYPE_POWERED_CHAIN", &102u64)
                }
                unknown => {
                    return Err(
                        S::Error::custom(
                            format!(
                                "The enum ConstraintType contains an unknown value ({}).",
                                unknown.bits()
                            ),
                        ),
                    );
                }
            }?;
            __serializer.serialize_bits(&self.bits())?;
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
    impl<'de> _serde::Deserialize<'de> for ConstraintType {
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
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
                __field23,
                __field24,
                __field25,
                __field26,
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
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        U8::Number(3u8) => _serde::__private::Ok(__Field::__field3),
                        U8::Number(6u8) => _serde::__private::Ok(__Field::__field4),
                        U8::Number(7u8) => _serde::__private::Ok(__Field::__field5),
                        U8::Number(8u8) => _serde::__private::Ok(__Field::__field6),
                        U8::Number(9u8) => _serde::__private::Ok(__Field::__field7),
                        U8::Number(10u8) => _serde::__private::Ok(__Field::__field8),
                        U8::Number(11u8) => _serde::__private::Ok(__Field::__field9),
                        U8::Number(12u8) => _serde::__private::Ok(__Field::__field10),
                        U8::Number(13u8) => _serde::__private::Ok(__Field::__field11),
                        U8::Number(14u8) => _serde::__private::Ok(__Field::__field12),
                        U8::Number(15u8) => _serde::__private::Ok(__Field::__field13),
                        U8::Number(16u8) => _serde::__private::Ok(__Field::__field14),
                        U8::Number(18u8) => _serde::__private::Ok(__Field::__field15),
                        U8::Number(19u8) => _serde::__private::Ok(__Field::__field16),
                        U8::Number(20u8) => _serde::__private::Ok(__Field::__field17),
                        U8::Number(21u8) => _serde::__private::Ok(__Field::__field18),
                        U8::Number(22u8) => _serde::__private::Ok(__Field::__field19),
                        U8::Number(23u8) => _serde::__private::Ok(__Field::__field20),
                        U8::Number(24u8) => _serde::__private::Ok(__Field::__field21),
                        U8::Number(25u8) => _serde::__private::Ok(__Field::__field22),
                        U8::Number(100u8) => _serde::__private::Ok(__Field::__field23),
                        U8::Number(100u8) => _serde::__private::Ok(__Field::__field24),
                        U8::Number(101u8) => _serde::__private::Ok(__Field::__field25),
                        U8::Number(102u8) => _serde::__private::Ok(__Field::__field26),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25, 100, 100, 101, 102",
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
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_BALLANDSOCKET") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_HINGE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_LIMITEDHINGE") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_POINTTOPATH") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_PRISMATIC") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_RAGDOLL") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "8"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_STIFFSPRING") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "9"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_WHEEL") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_GENERIC") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "11"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_CONTACT") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "12"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_BREAKABLE") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "13"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_MALLEABLE") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "14"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_POINTTOPLANE") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "15"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_PULLEY") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "16"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_ROTATIONAL") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "18"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_HINGE_LIMITS") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "19"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_RAGDOLL_LIMITS") => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            v if v == "20"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_CUSTOM") => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            v if v == "21"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_RACK_AND_PINION") => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            v if v == "22"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_COG_WHEEL") => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            v if v == "23"
                                || v.eq_ignore_ascii_case("CONSTRAINT_TYPE_FIXED") => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            v if v == "24"
                                || v
                                    .eq_ignore_ascii_case(
                                        "CONSTRAINT_TYPE_DEFORMABLE_FIXED",
                                    ) => _serde::__private::Ok(__Field::__field21),
                            v if v == "25"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_LINEAR_SLACK") => {
                                _serde::__private::Ok(__Field::__field22)
                            }
                            v if v == "100"
                                || v
                                    .eq_ignore_ascii_case("BEGIN_CONSTRAINT_CHAIN_TYPES") => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            v if v == "100"
                                || v
                                    .eq_ignore_ascii_case(
                                        "CONSTRAINT_TYPE_STIFF_SPRING_CHAIN",
                                    ) => _serde::__private::Ok(__Field::__field24),
                            v if v == "101"
                                || v
                                    .eq_ignore_ascii_case(
                                        "CONSTRAINT_TYPE_BALL_SOCKET_CHAIN",
                                    ) => _serde::__private::Ok(__Field::__field25),
                            v if v == "102"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAINT_TYPE_POWERED_CHAIN") => {
                                _serde::__private::Ok(__Field::__field26)
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
                marker: _serde::__private::PhantomData<ConstraintType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ConstraintType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ConstraintType",
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
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_BALLANDSOCKET,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintType::CONSTRAINT_TYPE_HINGE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_LIMITEDHINGE,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_POINTTOPATH,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_PRISMATIC,
                            )
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_RAGDOLL,
                            )
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_STIFFSPRING,
                            )
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintType::CONSTRAINT_TYPE_WHEEL)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_GENERIC,
                            )
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_CONTACT,
                            )
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_BREAKABLE,
                            )
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_MALLEABLE,
                            )
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_POINTTOPLANE,
                            )
                        }
                        (__Field::__field13, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintType::CONSTRAINT_TYPE_PULLEY)
                        }
                        (__Field::__field14, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_ROTATIONAL,
                            )
                        }
                        (__Field::__field15, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_HINGE_LIMITS,
                            )
                        }
                        (__Field::__field16, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_RAGDOLL_LIMITS,
                            )
                        }
                        (__Field::__field17, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintType::CONSTRAINT_TYPE_CUSTOM)
                        }
                        (__Field::__field18, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_RACK_AND_PINION,
                            )
                        }
                        (__Field::__field19, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_COG_WHEEL,
                            )
                        }
                        (__Field::__field20, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintType::CONSTRAINT_TYPE_FIXED)
                        }
                        (__Field::__field21, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_DEFORMABLE_FIXED,
                            )
                        }
                        (__Field::__field22, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_LINEAR_SLACK,
                            )
                        }
                        (__Field::__field23, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::BEGIN_CONSTRAINT_CHAIN_TYPES,
                            )
                        }
                        (__Field::__field24, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_STIFF_SPRING_CHAIN,
                            )
                        }
                        (__Field::__field25, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_BALL_SOCKET_CHAIN,
                            )
                        }
                        (__Field::__field26, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintType::CONSTRAINT_TYPE_POWERED_CHAIN,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "CONSTRAINT_TYPE_BALLANDSOCKET",
                "CONSTRAINT_TYPE_HINGE",
                "CONSTRAINT_TYPE_LIMITEDHINGE",
                "CONSTRAINT_TYPE_POINTTOPATH",
                "CONSTRAINT_TYPE_PRISMATIC",
                "CONSTRAINT_TYPE_RAGDOLL",
                "CONSTRAINT_TYPE_STIFFSPRING",
                "CONSTRAINT_TYPE_WHEEL",
                "CONSTRAINT_TYPE_GENERIC",
                "CONSTRAINT_TYPE_CONTACT",
                "CONSTRAINT_TYPE_BREAKABLE",
                "CONSTRAINT_TYPE_MALLEABLE",
                "CONSTRAINT_TYPE_POINTTOPLANE",
                "CONSTRAINT_TYPE_PULLEY",
                "CONSTRAINT_TYPE_ROTATIONAL",
                "CONSTRAINT_TYPE_HINGE_LIMITS",
                "CONSTRAINT_TYPE_RAGDOLL_LIMITS",
                "CONSTRAINT_TYPE_CUSTOM",
                "CONSTRAINT_TYPE_RACK_AND_PINION",
                "CONSTRAINT_TYPE_COG_WHEEL",
                "CONSTRAINT_TYPE_FIXED",
                "CONSTRAINT_TYPE_DEFORMABLE_FIXED",
                "CONSTRAINT_TYPE_LINEAR_SLACK",
                "BEGIN_CONSTRAINT_CHAIN_TYPES",
                "CONSTRAINT_TYPE_STIFF_SPRING_CHAIN",
                "CONSTRAINT_TYPE_BALL_SOCKET_CHAIN",
                "CONSTRAINT_TYPE_POWERED_CHAIN",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ConstraintType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ConstraintType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
