use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpVehicleLinearCastWheelCollide`
/// - version: `0`
/// - signature: `0xe6330f00`
/// - size: `  0`(x86)/` 56`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpVehicleLinearCastWheelCollide<'a> {
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
    pub parent: hknpVehicleWheelCollide<'a>,
    /// # C++ Info
    /// - name: `wheelStates`(ctype: `hkArray<struct hknpVehicleLinearCastWheelCollideWheelState>`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "wheelStates"))]
    #[cfg_attr(feature = "serde", serde(rename = "wheelStates"))]
    pub m_wheelStates: Vec<hknpVehicleLinearCastWheelCollideWheelState<'a>>,
    /// # C++ Info
    /// - name: `maxExtraPenetration`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxExtraPenetration"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxExtraPenetration"))]
    pub m_maxExtraPenetration: f32,
    /// # C++ Info
    /// - name: `startPointTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 44`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "startPointTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "startPointTolerance"))]
    pub m_startPointTolerance: f32,
    /// # C++ Info
    /// - name: `chassisBody`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "chassisBody"))]
    #[cfg_attr(feature = "serde", serde(rename = "chassisBody"))]
    pub m_chassisBody: U32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpVehicleLinearCastWheelCollide<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpVehicleLinearCastWheelCollide"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe6330f00)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_wheelStates
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hknpVehicleLinearCastWheelCollide<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xe6330f00)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hknpVehicleLinearCastWheelCollide",
                    class_meta,
                    (0u64, 56u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("alreadyUsed", &self.parent.m_alreadyUsed)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .skip_field(
                    "wheelCollisionFilterInfo",
                    &self.parent.m_wheelCollisionFilterInfo,
                )?;
            serializer
                .serialize_array_field(
                    "wheelStates",
                    &self.m_wheelStates,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 128u64,
                    },
                )?;
            serializer
                .serialize_field("maxExtraPenetration", &self.m_maxExtraPenetration)?;
            serializer
                .serialize_field("startPointTolerance", &self.m_startPointTolerance)?;
            serializer.serialize_field("chassisBody", &self.m_chassisBody)?;
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
    impl<'de> _serde::Deserialize<'de> for hknpVehicleLinearCastWheelCollide<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_alreadyUsed,
                m_wheelStates,
                m_maxExtraPenetration,
                m_startPointTolerance,
                m_chassisBody,
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
                        "alreadyUsed" => Ok(__Field::m_alreadyUsed),
                        "wheelStates" => Ok(__Field::m_wheelStates),
                        "maxExtraPenetration" => Ok(__Field::m_maxExtraPenetration),
                        "startPointTolerance" => Ok(__Field::m_startPointTolerance),
                        "chassisBody" => Ok(__Field::m_chassisBody),
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
            struct __hknpVehicleLinearCastWheelCollideVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hknpVehicleLinearCastWheelCollide<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hknpVehicleLinearCastWheelCollideVisitor<'de> {
                type Value = hknpVehicleLinearCastWheelCollide<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpVehicleLinearCastWheelCollide",
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
                    let mut m_wheelStates: _serde::__private::Option<
                        Vec<hknpVehicleLinearCastWheelCollideWheelState<'de>>,
                    > = _serde::__private::None;
                    let mut m_maxExtraPenetration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_startPointTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_chassisBody: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_wheelStates) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelStates",
                                        ),
                                    );
                                }
                                m_wheelStates = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpVehicleLinearCastWheelCollideWheelState<'de>>,
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
                                    &m_maxExtraPenetration,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxExtraPenetration",
                                        ),
                                    );
                                }
                                m_maxExtraPenetration = _serde::__private::Some(
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
                                    &m_startPointTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startPointTolerance",
                                        ),
                                    );
                                }
                                m_startPointTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_chassisBody) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chassisBody",
                                        ),
                                    );
                                }
                                m_chassisBody = _serde::__private::Some(
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
                    let m_wheelStates = match m_wheelStates {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelStates",
                                ),
                            );
                        }
                    };
                    let m_maxExtraPenetration = match m_maxExtraPenetration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxExtraPenetration",
                                ),
                            );
                        }
                    };
                    let m_startPointTolerance = match m_startPointTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startPointTolerance",
                                ),
                            );
                        }
                    };
                    let m_chassisBody = match m_chassisBody {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "chassisBody",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpVehicleLinearCastWheelCollide {
                        __ptr,
                        parent,
                        m_wheelStates,
                        m_maxExtraPenetration,
                        m_startPointTolerance,
                        m_chassisBody,
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
                    let mut m_alreadyUsed: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_wheelStates: _serde::__private::Option<
                        Vec<hknpVehicleLinearCastWheelCollideWheelState<'de>>,
                    > = _serde::__private::None;
                    let mut m_maxExtraPenetration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_startPointTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_chassisBody: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_alreadyUsed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_alreadyUsed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alreadyUsed",
                                        ),
                                    );
                                }
                                m_alreadyUsed = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wheelStates => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wheelStates) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelStates",
                                        ),
                                    );
                                }
                                m_wheelStates = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpVehicleLinearCastWheelCollideWheelState<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxExtraPenetration => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxExtraPenetration,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxExtraPenetration",
                                        ),
                                    );
                                }
                                m_maxExtraPenetration = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startPointTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_startPointTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startPointTolerance",
                                        ),
                                    );
                                }
                                m_startPointTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_chassisBody => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_chassisBody) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chassisBody",
                                        ),
                                    );
                                }
                                m_chassisBody = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                    let m_alreadyUsed = match m_alreadyUsed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alreadyUsed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelStates = match m_wheelStates {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelStates",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxExtraPenetration = match m_maxExtraPenetration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxExtraPenetration",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startPointTolerance = match m_startPointTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startPointTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_chassisBody = match m_chassisBody {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "chassisBody",
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
                    let parent = hknpVehicleWheelCollide {
                        __ptr: __ptr.clone(),
                        parent,
                        m_alreadyUsed,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpVehicleLinearCastWheelCollide {
                        __ptr: __ptr.clone(),
                        parent,
                        m_wheelStates,
                        m_maxExtraPenetration,
                        m_startPointTolerance,
                        m_chassisBody,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "wheelStates",
                "maxExtraPenetration",
                "startPointTolerance",
                "chassisBody",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpVehicleLinearCastWheelCollide",
                FIELDS,
                __hknpVehicleLinearCastWheelCollideVisitor {
                    marker: _serde::__private::PhantomData::<
                        hknpVehicleLinearCastWheelCollide,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
