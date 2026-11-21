use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpFixedConstraintDataAtoms`
/// - version: `0`
/// - signature: `0x269ef9c5`
/// - size: `  0`(x86)/`192`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpFixedConstraintDataAtoms<'a> {
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
    /// - name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`144`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transforms"))]
    #[cfg_attr(feature = "serde", serde(rename = "transforms"))]
    pub m_transforms: hkpSetLocalTransformsConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `setupStabilization`(ctype: `struct hkpSetupStabilizationAtom`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "setupStabilization"))]
    #[cfg_attr(feature = "serde", serde(rename = "setupStabilization"))]
    pub m_setupStabilization: hkpSetupStabilizationAtom<'a>,
    /// # C++ Info
    /// - name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ballSocket"))]
    #[cfg_attr(feature = "serde", serde(rename = "ballSocket"))]
    pub m_ballSocket: hkpBallSocketConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `ang`(ctype: `struct hkp3dAngConstraintAtom`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ang"))]
    #[cfg_attr(feature = "serde", serde(rename = "ang"))]
    pub m_ang: hkp3dAngConstraintAtom<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpFixedConstraintDataAtoms<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpFixedConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x269ef9c5)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_transforms.deps_indexes());
            v.extend(self.m_setupStabilization.deps_indexes());
            v.extend(self.m_ballSocket.deps_indexes());
            v.extend(self.m_ang.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkpFixedConstraintDataAtoms<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x269ef9c5)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpFixedConstraintDataAtoms",
                    class_meta,
                    (0u64, 192u64),
                )?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.serialize_field("ang", &self.m_ang)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpFixedConstraintDataAtoms<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transforms,
                m_setupStabilization,
                m_ballSocket,
                m_ang,
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
                        "transforms" => Ok(__Field::m_transforms),
                        "setupStabilization" => Ok(__Field::m_setupStabilization),
                        "ballSocket" => Ok(__Field::m_ballSocket),
                        "ang" => Ok(__Field::m_ang),
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
            struct __hkpFixedConstraintDataAtomsVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpFixedConstraintDataAtoms<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpFixedConstraintDataAtomsVisitor<'de> {
                type Value = hkpFixedConstraintDataAtoms<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpFixedConstraintDataAtoms",
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
                    let mut m_transforms: _serde::__private::Option<
                        hkpSetLocalTransformsConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_setupStabilization: _serde::__private::Option<
                        hkpSetupStabilizationAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_ballSocket: _serde::__private::Option<
                        hkpBallSocketConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_ang: _serde::__private::Option<
                        hkp3dAngConstraintAtom<'de>,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_transforms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transforms",
                                        ),
                                    );
                                }
                                m_transforms = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpSetLocalTransformsConstraintAtom<'de>,
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
                                    &m_setupStabilization,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "setupStabilization",
                                        ),
                                    );
                                }
                                m_setupStabilization = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpSetupStabilizationAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_ballSocket) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ballSocket",
                                        ),
                                    );
                                }
                                m_ballSocket = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpBallSocketConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_ang) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("ang"),
                                    );
                                }
                                m_ang = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkp3dAngConstraintAtom<'de>,
                                    >(&mut __map) {
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
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
                            );
                        }
                    };
                    let m_setupStabilization = match m_setupStabilization {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "setupStabilization",
                                ),
                            );
                        }
                    };
                    let m_ballSocket = match m_ballSocket {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballSocket",
                                ),
                            );
                        }
                    };
                    let m_ang = match m_ang {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("ang"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpFixedConstraintDataAtoms {
                        __ptr,
                        m_transforms,
                        m_setupStabilization,
                        m_ballSocket,
                        m_ang,
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
                    let mut m_transforms: _serde::__private::Option<
                        hkpSetLocalTransformsConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_setupStabilization: _serde::__private::Option<
                        hkpSetupStabilizationAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_ballSocket: _serde::__private::Option<
                        hkpBallSocketConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_ang: _serde::__private::Option<
                        hkp3dAngConstraintAtom<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_transforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transforms) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transforms",
                                        ),
                                    );
                                }
                                m_transforms = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpSetLocalTransformsConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_setupStabilization => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_setupStabilization,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "setupStabilization",
                                        ),
                                    );
                                }
                                m_setupStabilization = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpSetupStabilizationAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ballSocket => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ballSocket) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ballSocket",
                                        ),
                                    );
                                }
                                m_ballSocket = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpBallSocketConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ang => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ang) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("ang"),
                                    );
                                }
                                m_ang = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkp3dAngConstraintAtom<'de>,
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
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_setupStabilization = match m_setupStabilization {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "setupStabilization",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ballSocket = match m_ballSocket {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballSocket",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ang = match m_ang {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("ang"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpFixedConstraintDataAtoms {
                        __ptr: __ptr.clone(),
                        m_transforms,
                        m_setupStabilization,
                        m_ballSocket,
                        m_ang,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transforms",
                "setupStabilization",
                "ballSocket",
                "ang",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpFixedConstraintDataAtoms",
                FIELDS,
                __hkpFixedConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpFixedConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
