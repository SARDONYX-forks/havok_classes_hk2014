use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpBodyQuality`
/// - version: `1`
/// - signature: `0x4d0fbd5b`
/// - size: `  0`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpBodyQuality<'a> {
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
    /// - name: `priority`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "priority"))]
    #[cfg_attr(feature = "serde", serde(rename = "priority"))]
    pub m_priority: I32<'a>,
    /// # C++ Info
    /// - name: `supportedFlags`(ctype: `flags FlagsEnum`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "supportedFlags"))]
    #[cfg_attr(feature = "serde", serde(rename = "supportedFlags"))]
    pub m_supportedFlags: FlagsEnum,
    /// # C++ Info
    /// - name: `requestedFlags`(ctype: `flags FlagsEnum`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "requestedFlags"))]
    #[cfg_attr(feature = "serde", serde(rename = "requestedFlags"))]
    pub m_requestedFlags: FlagsEnum,
    /// # C++ Info
    /// - name: `contactCachingRelativeMovementThreshold`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "contactCachingRelativeMovementThreshold")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "contactCachingRelativeMovementThreshold")
    )]
    pub m_contactCachingRelativeMovementThreshold: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpBodyQuality<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpBodyQuality"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4d0fbd5b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hknpBodyQuality<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x4d0fbd5b)));
            let mut serializer = __serializer
                .serialize_struct("hknpBodyQuality", class_meta, (0u64, 16u64))?;
            serializer.serialize_field("priority", &self.m_priority)?;
            serializer.serialize_field("supportedFlags", &self.m_supportedFlags)?;
            serializer.serialize_field("requestedFlags", &self.m_requestedFlags)?;
            serializer
                .serialize_field(
                    "contactCachingRelativeMovementThreshold",
                    &self.m_contactCachingRelativeMovementThreshold,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpBodyQuality<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_priority,
                m_supportedFlags,
                m_requestedFlags,
                m_contactCachingRelativeMovementThreshold,
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
                        "priority" => Ok(__Field::m_priority),
                        "supportedFlags" => Ok(__Field::m_supportedFlags),
                        "requestedFlags" => Ok(__Field::m_requestedFlags),
                        "contactCachingRelativeMovementThreshold" => {
                            Ok(__Field::m_contactCachingRelativeMovementThreshold)
                        }
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
            struct __hknpBodyQualityVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpBodyQuality<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpBodyQualityVisitor<'de> {
                type Value = hknpBodyQuality<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpBodyQuality",
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
                    let mut m_priority: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_supportedFlags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_requestedFlags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_contactCachingRelativeMovementThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_priority) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "priority",
                                        ),
                                    );
                                }
                                m_priority = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_supportedFlags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "supportedFlags",
                                        ),
                                    );
                                }
                                m_supportedFlags = _serde::__private::Some(
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_requestedFlags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "requestedFlags",
                                        ),
                                    );
                                }
                                m_requestedFlags = _serde::__private::Some(
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_contactCachingRelativeMovementThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactCachingRelativeMovementThreshold",
                                        ),
                                    );
                                }
                                m_contactCachingRelativeMovementThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_priority = match m_priority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("priority"),
                            );
                        }
                    };
                    let m_supportedFlags = match m_supportedFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "supportedFlags",
                                ),
                            );
                        }
                    };
                    let m_requestedFlags = match m_requestedFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "requestedFlags",
                                ),
                            );
                        }
                    };
                    let m_contactCachingRelativeMovementThreshold = match m_contactCachingRelativeMovementThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactCachingRelativeMovementThreshold",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpBodyQuality {
                        __ptr,
                        m_priority,
                        m_supportedFlags,
                        m_requestedFlags,
                        m_contactCachingRelativeMovementThreshold,
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
                    let mut m_priority: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_supportedFlags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_requestedFlags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_contactCachingRelativeMovementThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_priority => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_priority) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "priority",
                                        ),
                                    );
                                }
                                m_priority = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_supportedFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_supportedFlags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "supportedFlags",
                                        ),
                                    );
                                }
                                m_supportedFlags = _serde::__private::Some(
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_requestedFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_requestedFlags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "requestedFlags",
                                        ),
                                    );
                                }
                                m_requestedFlags = _serde::__private::Some(
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_contactCachingRelativeMovementThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_contactCachingRelativeMovementThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactCachingRelativeMovementThreshold",
                                        ),
                                    );
                                }
                                m_contactCachingRelativeMovementThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_priority = match m_priority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("priority"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_supportedFlags = match m_supportedFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "supportedFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_requestedFlags = match m_requestedFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "requestedFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_contactCachingRelativeMovementThreshold = match m_contactCachingRelativeMovementThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactCachingRelativeMovementThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpBodyQuality {
                        __ptr: __ptr.clone(),
                        m_priority,
                        m_supportedFlags,
                        m_requestedFlags,
                        m_contactCachingRelativeMovementThreshold,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "priority",
                "supportedFlags",
                "requestedFlags",
                "contactCachingRelativeMovementThreshold",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpBodyQuality",
                FIELDS,
                __hknpBodyQualityVisitor {
                    marker: _serde::__private::PhantomData::<hknpBodyQuality>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `FlagsEnum`(ctype: `hkFlags<FlagsEnum, hkUint32>`)"]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagsEnum : u32 { #[doc = "1"] const DROP_MANIFOLD_CACHE = 1u32; #[doc = "2"] const
    FORCE_GSK_EXECUTION = 2u32; #[doc = "4"] const FORCE_GSK_SINGLE_POINT_MANIFOLD =
    4u32; #[doc = "8"] const ALLOW_INTERIOR_TRIANGLE_COLLISIONS = 8u32; #[doc = "16"]
    const ENABLE_SILHOUETTE_MANIFOLDS = 16u32; #[doc = "64"] const
    USE_HIGHER_QUALITY_CONTACT_SOLVING = 64u32; #[doc = "128"] const
    ENABLE_NEIGHBOR_WELDING = 128u32; #[doc = "256"] const ENABLE_MOTION_WELDING =
    256u32; #[doc = "512"] const ENABLE_TRIANGLE_WELDING = 512u32; #[doc = "896"] const
    ANY_WELDING = 896u32; #[doc = "1024"] const ENABLE_CONTACT_CACHING = 1024u32; #[doc =
    "2048"] const REQUEST_LINEAR_ONLY_COLLISION_LOOK_AHEAD = 2048u32; #[doc = "4096"]
    const FORCE_LINEAR_ONLY_COLLISION_LOOK_AHEAD = 4096u32; #[doc = "8192"] const
    ENABLE_FRICTION_ESTIMATION = 8192u32; #[doc = "65536"] const FIRST_NON_CACHABLE_FLAG
    = 65536u32; #[doc = "65536"] const CLIP_ANGULAR_VELOCITY = 65536u32; #[doc =
    "131072"] const USE_DISCRETE_AABB_EXPANSION = 131072u32; #[doc = "262144"] const
    MERGE_FRICTION_JACOBIANS = 262144u32; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for FlagsEnum {
        fn schema_name() -> Cow<'static, str> {
            "FlagsEnum".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "FlagsEnum").into()
        }
        fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
            json_schema!(
                { "description" :
                "Bitflags field. Specific flags: DROP_MANIFOLD_CACHE: 1, FORCE_GSK_EXECUTION: 2, FORCE_GSK_SINGLE_POINT_MANIFOLD: 4, ALLOW_INTERIOR_TRIANGLE_COLLISIONS: 8, ENABLE_SILHOUETTE_MANIFOLDS: 16, USE_HIGHER_QUALITY_CONTACT_SOLVING: 64, ENABLE_NEIGHBOR_WELDING: 128, ENABLE_MOTION_WELDING: 256, ENABLE_TRIANGLE_WELDING: 512, ANY_WELDING: 896, ENABLE_CONTACT_CACHING: 1024, REQUEST_LINEAR_ONLY_COLLISION_LOOK_AHEAD: 2048, FORCE_LINEAR_ONLY_COLLISION_LOOK_AHEAD: 4096, ENABLE_FRICTION_ESTIMATION: 8192, FIRST_NON_CACHABLE_FLAG: 65536, CLIP_ANGULAR_VELOCITY: 65536, USE_DISCRETE_AABB_EXPANSION: 131072, MERGE_FRICTION_JACOBIANS: 262144. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
                "type" : "string", }
            )
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagsEnum {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_bits(&self.bits())?;
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::DROP_MANIFOLD_CACHE => {
                        __serializer
                            .serialize_field(
                                "DROP_MANIFOLD_CACHE",
                                &Self::DROP_MANIFOLD_CACHE,
                            )
                    }
                    Self::FORCE_GSK_EXECUTION => {
                        __serializer
                            .serialize_field(
                                "FORCE_GSK_EXECUTION",
                                &Self::FORCE_GSK_EXECUTION,
                            )
                    }
                    Self::FORCE_GSK_SINGLE_POINT_MANIFOLD => {
                        __serializer
                            .serialize_field(
                                "FORCE_GSK_SINGLE_POINT_MANIFOLD",
                                &Self::FORCE_GSK_SINGLE_POINT_MANIFOLD,
                            )
                    }
                    Self::ALLOW_INTERIOR_TRIANGLE_COLLISIONS => {
                        __serializer
                            .serialize_field(
                                "ALLOW_INTERIOR_TRIANGLE_COLLISIONS",
                                &Self::ALLOW_INTERIOR_TRIANGLE_COLLISIONS,
                            )
                    }
                    Self::ENABLE_SILHOUETTE_MANIFOLDS => {
                        __serializer
                            .serialize_field(
                                "ENABLE_SILHOUETTE_MANIFOLDS",
                                &Self::ENABLE_SILHOUETTE_MANIFOLDS,
                            )
                    }
                    Self::USE_HIGHER_QUALITY_CONTACT_SOLVING => {
                        __serializer
                            .serialize_field(
                                "USE_HIGHER_QUALITY_CONTACT_SOLVING",
                                &Self::USE_HIGHER_QUALITY_CONTACT_SOLVING,
                            )
                    }
                    Self::ENABLE_NEIGHBOR_WELDING => {
                        __serializer
                            .serialize_field(
                                "ENABLE_NEIGHBOR_WELDING",
                                &Self::ENABLE_NEIGHBOR_WELDING,
                            )
                    }
                    Self::ENABLE_MOTION_WELDING => {
                        __serializer
                            .serialize_field(
                                "ENABLE_MOTION_WELDING",
                                &Self::ENABLE_MOTION_WELDING,
                            )
                    }
                    Self::ENABLE_TRIANGLE_WELDING => {
                        __serializer
                            .serialize_field(
                                "ENABLE_TRIANGLE_WELDING",
                                &Self::ENABLE_TRIANGLE_WELDING,
                            )
                    }
                    Self::ANY_WELDING => {
                        __serializer.serialize_field("ANY_WELDING", &Self::ANY_WELDING)
                    }
                    Self::ENABLE_CONTACT_CACHING => {
                        __serializer
                            .serialize_field(
                                "ENABLE_CONTACT_CACHING",
                                &Self::ENABLE_CONTACT_CACHING,
                            )
                    }
                    Self::REQUEST_LINEAR_ONLY_COLLISION_LOOK_AHEAD => {
                        __serializer
                            .serialize_field(
                                "REQUEST_LINEAR_ONLY_COLLISION_LOOK_AHEAD",
                                &Self::REQUEST_LINEAR_ONLY_COLLISION_LOOK_AHEAD,
                            )
                    }
                    Self::FORCE_LINEAR_ONLY_COLLISION_LOOK_AHEAD => {
                        __serializer
                            .serialize_field(
                                "FORCE_LINEAR_ONLY_COLLISION_LOOK_AHEAD",
                                &Self::FORCE_LINEAR_ONLY_COLLISION_LOOK_AHEAD,
                            )
                    }
                    Self::ENABLE_FRICTION_ESTIMATION => {
                        __serializer
                            .serialize_field(
                                "ENABLE_FRICTION_ESTIMATION",
                                &Self::ENABLE_FRICTION_ESTIMATION,
                            )
                    }
                    Self::FIRST_NON_CACHABLE_FLAG => {
                        __serializer
                            .serialize_field(
                                "FIRST_NON_CACHABLE_FLAG",
                                &Self::FIRST_NON_CACHABLE_FLAG,
                            )
                    }
                    Self::CLIP_ANGULAR_VELOCITY => {
                        __serializer
                            .serialize_field(
                                "CLIP_ANGULAR_VELOCITY",
                                &Self::CLIP_ANGULAR_VELOCITY,
                            )
                    }
                    Self::USE_DISCRETE_AABB_EXPANSION => {
                        __serializer
                            .serialize_field(
                                "USE_DISCRETE_AABB_EXPANSION",
                                &Self::USE_DISCRETE_AABB_EXPANSION,
                            )
                    }
                    Self::MERGE_FRICTION_JACOBIANS => {
                        __serializer
                            .serialize_field(
                                "MERGE_FRICTION_JACOBIANS",
                                &Self::MERGE_FRICTION_JACOBIANS,
                            )
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
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
    impl<'de> _serde::Deserialize<'de> for FlagsEnum {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagsEnum>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagsEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlagsEnum(flags)",
                    )
                }
                #[inline]
                fn visit_uint32<__E>(
                    self,
                    __value: U32<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U32::Number(__value) => {
                            Ok(FlagsEnum::from_bits_retain(__value as _))
                        }
                        _ => {
                            Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value as _),
                                    &"FlagsEnum(U32) Number",
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
                    match <FlagsEnum as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagsEnum>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
