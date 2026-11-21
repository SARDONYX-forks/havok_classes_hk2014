use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpLinearClearanceConstraintDataAtoms`
/// - version: `0`
/// - signature: `0xd0748d53`
/// - size: `  0`(x86)/`288`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinearClearanceConstraintDataAtoms<'a> {
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
    /// - name: `motor`(ctype: `struct hkpLinMotorConstraintAtom`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motor"))]
    #[cfg_attr(feature = "serde", serde(rename = "motor"))]
    pub m_motor: hkpLinMotorConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `friction0`(ctype: `struct hkpLinFrictionConstraintAtom`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "friction0"))]
    #[cfg_attr(feature = "serde", serde(rename = "friction0"))]
    pub m_friction0: hkpLinFrictionConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `friction1`(ctype: `struct hkpLinFrictionConstraintAtom`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "friction1"))]
    #[cfg_attr(feature = "serde", serde(rename = "friction1"))]
    pub m_friction1: hkpLinFrictionConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `friction2`(ctype: `struct hkpLinFrictionConstraintAtom`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "friction2"))]
    #[cfg_attr(feature = "serde", serde(rename = "friction2"))]
    pub m_friction2: hkpLinFrictionConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `ang`(ctype: `struct hkpAngConstraintAtom`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ang"))]
    #[cfg_attr(feature = "serde", serde(rename = "ang"))]
    pub m_ang: hkpAngConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `linLimit0`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linLimit0"))]
    #[cfg_attr(feature = "serde", serde(rename = "linLimit0"))]
    pub m_linLimit0: hkpLinLimitConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `linLimit1`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// - offset: `  0`(x86)/`248`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linLimit1"))]
    #[cfg_attr(feature = "serde", serde(rename = "linLimit1"))]
    pub m_linLimit1: hkpLinLimitConstraintAtom<'a>,
    /// # C++ Info
    /// - name: `linLimit2`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// - offset: `  0`(x86)/`264`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linLimit2"))]
    #[cfg_attr(feature = "serde", serde(rename = "linLimit2"))]
    pub m_linLimit2: hkpLinLimitConstraintAtom<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpLinearClearanceConstraintDataAtoms<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpLinearClearanceConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd0748d53)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_transforms.deps_indexes());
            v.extend(self.m_motor.deps_indexes());
            v.extend(self.m_friction0.deps_indexes());
            v.extend(self.m_friction1.deps_indexes());
            v.extend(self.m_friction2.deps_indexes());
            v.extend(self.m_ang.deps_indexes());
            v.extend(self.m_linLimit0.deps_indexes());
            v.extend(self.m_linLimit1.deps_indexes());
            v.extend(self.m_linLimit2.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkpLinearClearanceConstraintDataAtoms<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xd0748d53)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpLinearClearanceConstraintDataAtoms",
                    class_meta,
                    (0u64, 288u64),
                )?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer.serialize_field("motor", &self.m_motor)?;
            serializer.serialize_field("friction0", &self.m_friction0)?;
            serializer.serialize_field("friction1", &self.m_friction1)?;
            serializer.serialize_field("friction2", &self.m_friction2)?;
            serializer.serialize_field("ang", &self.m_ang)?;
            serializer.serialize_field("linLimit0", &self.m_linLimit0)?;
            serializer.serialize_field("linLimit1", &self.m_linLimit1)?;
            serializer.serialize_field("linLimit2", &self.m_linLimit2)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpLinearClearanceConstraintDataAtoms<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transforms,
                m_motor,
                m_friction0,
                m_friction1,
                m_friction2,
                m_ang,
                m_linLimit0,
                m_linLimit1,
                m_linLimit2,
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
                        "motor" => Ok(__Field::m_motor),
                        "friction0" => Ok(__Field::m_friction0),
                        "friction1" => Ok(__Field::m_friction1),
                        "friction2" => Ok(__Field::m_friction2),
                        "ang" => Ok(__Field::m_ang),
                        "linLimit0" => Ok(__Field::m_linLimit0),
                        "linLimit1" => Ok(__Field::m_linLimit1),
                        "linLimit2" => Ok(__Field::m_linLimit2),
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
            struct __hkpLinearClearanceConstraintDataAtomsVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpLinearClearanceConstraintDataAtoms<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpLinearClearanceConstraintDataAtomsVisitor<'de> {
                type Value = hkpLinearClearanceConstraintDataAtoms<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpLinearClearanceConstraintDataAtoms",
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
                    let mut m_motor: _serde::__private::Option<
                        hkpLinMotorConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_friction0: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_friction1: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_friction2: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_ang: _serde::__private::Option<
                        hkpAngConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_linLimit0: _serde::__private::Option<
                        hkpLinLimitConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_linLimit1: _serde::__private::Option<
                        hkpLinLimitConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_linLimit2: _serde::__private::Option<
                        hkpLinLimitConstraintAtom<'de>,
                    > = _serde::__private::None;
                    for i in 0..9usize {
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
                                if _serde::__private::Option::is_some(&m_motor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("motor"),
                                    );
                                }
                                m_motor = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinMotorConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_friction0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction0",
                                        ),
                                    );
                                }
                                m_friction0 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_friction1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction1",
                                        ),
                                    );
                                }
                                m_friction1 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_friction2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction2",
                                        ),
                                    );
                                }
                                m_friction2 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_ang) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("ang"),
                                    );
                                }
                                m_ang = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpAngConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_linLimit0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit0",
                                        ),
                                    );
                                }
                                m_linLimit0 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinLimitConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_linLimit1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit1",
                                        ),
                                    );
                                }
                                m_linLimit1 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinLimitConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_linLimit2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit2",
                                        ),
                                    );
                                }
                                m_linLimit2 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinLimitConstraintAtom<'de>,
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
                    __A::pad(&mut __map, 0usize, 8usize)?;
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
                    let m_motor = match m_motor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motor"),
                            );
                        }
                    };
                    let m_friction0 = match m_friction0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "friction0",
                                ),
                            );
                        }
                    };
                    let m_friction1 = match m_friction1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "friction1",
                                ),
                            );
                        }
                    };
                    let m_friction2 = match m_friction2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "friction2",
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
                    let m_linLimit0 = match m_linLimit0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linLimit0",
                                ),
                            );
                        }
                    };
                    let m_linLimit1 = match m_linLimit1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linLimit1",
                                ),
                            );
                        }
                    };
                    let m_linLimit2 = match m_linLimit2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linLimit2",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpLinearClearanceConstraintDataAtoms {
                        __ptr,
                        m_transforms,
                        m_motor,
                        m_friction0,
                        m_friction1,
                        m_friction2,
                        m_ang,
                        m_linLimit0,
                        m_linLimit1,
                        m_linLimit2,
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
                    let mut m_motor: _serde::__private::Option<
                        hkpLinMotorConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_friction0: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_friction1: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_friction2: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_ang: _serde::__private::Option<
                        hkpAngConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_linLimit0: _serde::__private::Option<
                        hkpLinLimitConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_linLimit1: _serde::__private::Option<
                        hkpLinLimitConstraintAtom<'de>,
                    > = _serde::__private::None;
                    let mut m_linLimit2: _serde::__private::Option<
                        hkpLinLimitConstraintAtom<'de>,
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
                            __Field::m_motor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("motor"),
                                    );
                                }
                                m_motor = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinMotorConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_friction0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_friction0) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction0",
                                        ),
                                    );
                                }
                                m_friction0 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_friction1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_friction1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction1",
                                        ),
                                    );
                                }
                                m_friction1 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_friction2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_friction2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction2",
                                        ),
                                    );
                                }
                                m_friction2 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom<'de>,
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
                                        hkpAngConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linLimit0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linLimit0) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit0",
                                        ),
                                    );
                                }
                                m_linLimit0 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinLimitConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linLimit1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linLimit1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit1",
                                        ),
                                    );
                                }
                                m_linLimit1 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinLimitConstraintAtom<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linLimit2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linLimit2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit2",
                                        ),
                                    );
                                }
                                m_linLimit2 = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinLimitConstraintAtom<'de>,
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
                    let m_motor = match m_motor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motor"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_friction0 = match m_friction0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "friction0",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_friction1 = match m_friction1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "friction1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_friction2 = match m_friction2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "friction2",
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
                    let m_linLimit0 = match m_linLimit0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linLimit0",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linLimit1 = match m_linLimit1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linLimit1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linLimit2 = match m_linLimit2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linLimit2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpLinearClearanceConstraintDataAtoms {
                        __ptr: __ptr.clone(),
                        m_transforms,
                        m_motor,
                        m_friction0,
                        m_friction1,
                        m_friction2,
                        m_ang,
                        m_linLimit0,
                        m_linLimit1,
                        m_linLimit2,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transforms",
                "motor",
                "friction0",
                "friction1",
                "friction2",
                "ang",
                "linLimit0",
                "linLimit1",
                "linLimit2",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpLinearClearanceConstraintDataAtoms",
                FIELDS,
                __hkpLinearClearanceConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpLinearClearanceConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
