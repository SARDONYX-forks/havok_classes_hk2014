use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSimClothDataOverridableSimulationInfo`
/// - version: `1`
/// - signature: `0x383b984f`
/// - size: `  0`(x86)/` 32`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSimClothDataOverridableSimulationInfo<'a> {
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
    /// - name: `gravity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gravity"))]
    #[cfg_attr(feature = "serde", serde(rename = "gravity"))]
    pub m_gravity: Vector4,
    /// # C++ Info
    /// - name: `globalDampingPerSecond`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "globalDampingPerSecond"))]
    #[cfg_attr(feature = "serde", serde(rename = "globalDampingPerSecond"))]
    pub m_globalDampingPerSecond: f32,
    /// # C++ Info
    /// - name: `collisionTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionTolerance"))]
    pub m_collisionTolerance: f32,
    /// # C++ Info
    /// - name: `subSteps`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "subSteps"))]
    #[cfg_attr(feature = "serde", serde(rename = "subSteps"))]
    pub m_subSteps: U32<'a>,
    /// # C++ Info
    /// - name: `pinchDetectionEnabled`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pinchDetectionEnabled"))]
    #[cfg_attr(feature = "serde", serde(rename = "pinchDetectionEnabled"))]
    pub m_pinchDetectionEnabled: bool,
    /// # C++ Info
    /// - name: `landscapeCollisionEnabled`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 29`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "landscapeCollisionEnabled"))]
    #[cfg_attr(feature = "serde", serde(rename = "landscapeCollisionEnabled"))]
    pub m_landscapeCollisionEnabled: bool,
    /// # C++ Info
    /// - name: `transferMotionEnabled`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 30`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transferMotionEnabled"))]
    #[cfg_attr(feature = "serde", serde(rename = "transferMotionEnabled"))]
    pub m_transferMotionEnabled: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSimClothDataOverridableSimulationInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSimClothDataOverridableSimulationInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x383b984f)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclSimClothDataOverridableSimulationInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x383b984f)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclSimClothDataOverridableSimulationInfo",
                    class_meta,
                    (0u64, 32u64),
                )?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer
                .serialize_field(
                    "globalDampingPerSecond",
                    &self.m_globalDampingPerSecond,
                )?;
            serializer
                .serialize_field("collisionTolerance", &self.m_collisionTolerance)?;
            serializer.serialize_field("subSteps", &self.m_subSteps)?;
            serializer
                .serialize_field(
                    "pinchDetectionEnabled",
                    &self.m_pinchDetectionEnabled,
                )?;
            serializer
                .serialize_field(
                    "landscapeCollisionEnabled",
                    &self.m_landscapeCollisionEnabled,
                )?;
            serializer
                .serialize_field(
                    "transferMotionEnabled",
                    &self.m_transferMotionEnabled,
                )?;
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
    impl<'de> _serde::Deserialize<'de>
    for hclSimClothDataOverridableSimulationInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_gravity,
                m_globalDampingPerSecond,
                m_collisionTolerance,
                m_subSteps,
                m_pinchDetectionEnabled,
                m_landscapeCollisionEnabled,
                m_transferMotionEnabled,
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
                        "gravity" => Ok(__Field::m_gravity),
                        "globalDampingPerSecond" => Ok(__Field::m_globalDampingPerSecond),
                        "collisionTolerance" => Ok(__Field::m_collisionTolerance),
                        "subSteps" => Ok(__Field::m_subSteps),
                        "pinchDetectionEnabled" => Ok(__Field::m_pinchDetectionEnabled),
                        "landscapeCollisionEnabled" => {
                            Ok(__Field::m_landscapeCollisionEnabled)
                        }
                        "transferMotionEnabled" => Ok(__Field::m_transferMotionEnabled),
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
            struct __hclSimClothDataOverridableSimulationInfoVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclSimClothDataOverridableSimulationInfo<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclSimClothDataOverridableSimulationInfoVisitor<'de> {
                type Value = hclSimClothDataOverridableSimulationInfo<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSimClothDataOverridableSimulationInfo",
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
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_globalDampingPerSecond: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_subSteps: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pinchDetectionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_landscapeCollisionEnabled: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_transferMotionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_gravity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravity",
                                        ),
                                    );
                                }
                                m_gravity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_globalDampingPerSecond,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "globalDampingPerSecond",
                                        ),
                                    );
                                }
                                m_globalDampingPerSecond = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionTolerance",
                                        ),
                                    );
                                }
                                m_collisionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_subSteps) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "subSteps",
                                        ),
                                    );
                                }
                                m_subSteps = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionEnabled,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionEnabled",
                                        ),
                                    );
                                }
                                m_pinchDetectionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionEnabled,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionEnabled",
                                        ),
                                    );
                                }
                                m_landscapeCollisionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionEnabled,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionEnabled",
                                        ),
                                    );
                                }
                                m_transferMotionEnabled = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 1usize)?;
                    let m_gravity = match m_gravity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gravity"),
                            );
                        }
                    };
                    let m_globalDampingPerSecond = match m_globalDampingPerSecond {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "globalDampingPerSecond",
                                ),
                            );
                        }
                    };
                    let m_collisionTolerance = match m_collisionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionTolerance",
                                ),
                            );
                        }
                    };
                    let m_subSteps = match m_subSteps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("subSteps"),
                            );
                        }
                    };
                    let m_pinchDetectionEnabled = match m_pinchDetectionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionEnabled",
                                ),
                            );
                        }
                    };
                    let m_landscapeCollisionEnabled = match m_landscapeCollisionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionEnabled",
                                ),
                            );
                        }
                    };
                    let m_transferMotionEnabled = match m_transferMotionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionEnabled",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSimClothDataOverridableSimulationInfo {
                        __ptr,
                        m_gravity,
                        m_globalDampingPerSecond,
                        m_collisionTolerance,
                        m_subSteps,
                        m_pinchDetectionEnabled,
                        m_landscapeCollisionEnabled,
                        m_transferMotionEnabled,
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
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_globalDampingPerSecond: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_subSteps: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pinchDetectionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_landscapeCollisionEnabled: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_transferMotionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_gravity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gravity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravity",
                                        ),
                                    );
                                }
                                m_gravity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_globalDampingPerSecond => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_globalDampingPerSecond,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "globalDampingPerSecond",
                                        ),
                                    );
                                }
                                m_globalDampingPerSecond = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionTolerance",
                                        ),
                                    );
                                }
                                m_collisionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_subSteps => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_subSteps) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "subSteps",
                                        ),
                                    );
                                }
                                m_subSteps = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pinchDetectionEnabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionEnabled,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionEnabled",
                                        ),
                                    );
                                }
                                m_pinchDetectionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_landscapeCollisionEnabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionEnabled,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionEnabled",
                                        ),
                                    );
                                }
                                m_landscapeCollisionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transferMotionEnabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionEnabled,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionEnabled",
                                        ),
                                    );
                                }
                                m_transferMotionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_gravity = match m_gravity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gravity"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_globalDampingPerSecond = match m_globalDampingPerSecond {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "globalDampingPerSecond",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionTolerance = match m_collisionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_subSteps = match m_subSteps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("subSteps"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pinchDetectionEnabled = match m_pinchDetectionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionEnabled",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_landscapeCollisionEnabled = match m_landscapeCollisionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionEnabled",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transferMotionEnabled = match m_transferMotionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionEnabled",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclSimClothDataOverridableSimulationInfo {
                        __ptr: __ptr.clone(),
                        m_gravity,
                        m_globalDampingPerSecond,
                        m_collisionTolerance,
                        m_subSteps,
                        m_pinchDetectionEnabled,
                        m_landscapeCollisionEnabled,
                        m_transferMotionEnabled,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "gravity",
                "globalDampingPerSecond",
                "collisionTolerance",
                "subSteps",
                "pinchDetectionEnabled",
                "landscapeCollisionEnabled",
                "transferMotionEnabled",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSimClothDataOverridableSimulationInfo",
                FIELDS,
                __hclSimClothDataOverridableSimulationInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclSimClothDataOverridableSimulationInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
