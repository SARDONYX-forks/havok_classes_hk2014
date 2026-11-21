use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbJigglerGroup`
/// - version: `1`
/// - signature: `0x99b14961`
/// - size: `  0`(x86)/` 88`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbJigglerGroup<'a> {
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
    pub parent: hkbBindable<'a>,
    /// # C++ Info
    /// - name: `boneIndices`(ctype: `struct hkbBoneIndexArray*`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneIndices"))]
    pub m_boneIndices: Pointer<'a>,
    /// # C++ Info
    /// - name: `mass`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "mass"))]
    #[cfg_attr(feature = "serde", serde(rename = "mass"))]
    pub m_mass: f32,
    /// # C++ Info
    /// - name: `stiffness`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 60`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stiffness"))]
    #[cfg_attr(feature = "serde", serde(rename = "stiffness"))]
    pub m_stiffness: f32,
    /// # C++ Info
    /// - name: `damping`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "damping"))]
    #[cfg_attr(feature = "serde", serde(rename = "damping"))]
    pub m_damping: f32,
    /// # C++ Info
    /// - name: `maxElongation`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 68`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxElongation"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxElongation"))]
    pub m_maxElongation: f32,
    /// # C++ Info
    /// - name: `maxCompression`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxCompression"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxCompression"))]
    pub m_maxCompression: f32,
    /// # C++ Info
    /// - name: `propagateToChildren`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 76`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "propagateToChildren"))]
    #[cfg_attr(feature = "serde", serde(rename = "propagateToChildren"))]
    pub m_propagateToChildren: bool,
    /// # C++ Info
    /// - name: `affectSiblings`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 77`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "affectSiblings"))]
    #[cfg_attr(feature = "serde", serde(rename = "affectSiblings"))]
    pub m_affectSiblings: bool,
    /// # C++ Info
    /// - name: `rotateBonesForSkinning`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 78`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rotateBonesForSkinning"))]
    #[cfg_attr(feature = "serde", serde(rename = "rotateBonesForSkinning"))]
    pub m_rotateBonesForSkinning: bool,
    /// # C++ Info
    /// - name: `pad`(ctype: `hkBool[5]`)
    /// - offset: `  0`(x86)/` 79`(x86_64)
    /// - type_size: `  0`(x86)/`  5`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pad"))]
    #[cfg_attr(feature = "serde", serde(rename = "pad"))]
    pub m_pad: [bool; 5usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbJigglerGroup<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbJigglerGroup"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x99b14961)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.m_variableBindingSet);
            v.extend(self.parent.m_cachedBindables.iter());
            v.push(&self.m_boneIndices);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbJigglerGroup<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x99b14961)));
            let mut serializer = __serializer
                .serialize_struct("hkbJigglerGroup", class_meta, (0u64, 88u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field("areBindablesCached", &self.parent.m_areBindablesCached)?;
            serializer.skip_field("hasEnableChanged", &self.parent.m_hasEnableChanged)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("boneIndices", &self.m_boneIndices)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.serialize_field("stiffness", &self.m_stiffness)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer.serialize_field("maxElongation", &self.m_maxElongation)?;
            serializer.serialize_field("maxCompression", &self.m_maxCompression)?;
            serializer
                .serialize_field("propagateToChildren", &self.m_propagateToChildren)?;
            serializer.serialize_field("affectSiblings", &self.m_affectSiblings)?;
            serializer
                .serialize_field(
                    "rotateBonesForSkinning",
                    &self.m_rotateBonesForSkinning,
                )?;
            serializer
                .skip_fixed_array_field("pad", self.m_pad.as_slice(), TypeSize::NonPtr)?;
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
    impl<'de> _serde::Deserialize<'de> for hkbJigglerGroup<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_boneIndices,
                m_mass,
                m_stiffness,
                m_damping,
                m_maxElongation,
                m_maxCompression,
                m_propagateToChildren,
                m_affectSiblings,
                m_rotateBonesForSkinning,
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
                        "boneIndices" => Ok(__Field::m_boneIndices),
                        "mass" => Ok(__Field::m_mass),
                        "stiffness" => Ok(__Field::m_stiffness),
                        "damping" => Ok(__Field::m_damping),
                        "maxElongation" => Ok(__Field::m_maxElongation),
                        "maxCompression" => Ok(__Field::m_maxCompression),
                        "propagateToChildren" => Ok(__Field::m_propagateToChildren),
                        "affectSiblings" => Ok(__Field::m_affectSiblings),
                        "rotateBonesForSkinning" => Ok(__Field::m_rotateBonesForSkinning),
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
            struct __hkbJigglerGroupVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbJigglerGroup<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbJigglerGroupVisitor<'de> {
                type Value = hkbJigglerGroup<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbJigglerGroup",
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
                    let mut m_boneIndices: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_stiffness: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxElongation: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxCompression: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_propagateToChildren: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_affectSiblings: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_rotateBonesForSkinning: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pad: _serde::__private::Option<[bool; 5usize]> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_boneIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndices",
                                        ),
                                    );
                                }
                                m_boneIndices = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_mass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mass"),
                                    );
                                }
                                m_mass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_stiffness) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffness",
                                        ),
                                    );
                                }
                                m_stiffness = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_damping) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "damping",
                                        ),
                                    );
                                }
                                m_damping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_maxElongation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxElongation",
                                        ),
                                    );
                                }
                                m_maxElongation = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_maxCompression) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCompression",
                                        ),
                                    );
                                }
                                m_maxCompression = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_propagateToChildren,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "propagateToChildren",
                                        ),
                                    );
                                }
                                m_propagateToChildren = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_affectSiblings) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "affectSiblings",
                                        ),
                                    );
                                }
                                m_affectSiblings = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_rotateBonesForSkinning,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotateBonesForSkinning",
                                        ),
                                    );
                                }
                                m_rotateBonesForSkinning = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_pad) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad"),
                                    );
                                }
                                m_pad = _serde::__private::Some(
                                    match __A::next_value::<[bool; 5usize]>(&mut __map) {
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
                    let m_boneIndices = match m_boneIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndices",
                                ),
                            );
                        }
                    };
                    let m_mass = match m_mass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("mass"),
                            );
                        }
                    };
                    let m_stiffness = match m_stiffness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffness",
                                ),
                            );
                        }
                    };
                    let m_damping = match m_damping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("damping"),
                            );
                        }
                    };
                    let m_maxElongation = match m_maxElongation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxElongation",
                                ),
                            );
                        }
                    };
                    let m_maxCompression = match m_maxCompression {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCompression",
                                ),
                            );
                        }
                    };
                    let m_propagateToChildren = match m_propagateToChildren {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "propagateToChildren",
                                ),
                            );
                        }
                    };
                    let m_affectSiblings = match m_affectSiblings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "affectSiblings",
                                ),
                            );
                        }
                    };
                    let m_rotateBonesForSkinning = match m_rotateBonesForSkinning {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotateBonesForSkinning",
                                ),
                            );
                        }
                    };
                    let m_pad = match m_pad {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbJigglerGroup {
                        __ptr,
                        parent,
                        m_boneIndices,
                        m_mass,
                        m_stiffness,
                        m_damping,
                        m_maxElongation,
                        m_maxCompression,
                        m_propagateToChildren,
                        m_affectSiblings,
                        m_rotateBonesForSkinning,
                        m_pad,
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
                    let mut m_boneIndices: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_stiffness: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxElongation: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxCompression: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_propagateToChildren: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_affectSiblings: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_rotateBonesForSkinning: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_boneIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneIndices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndices",
                                        ),
                                    );
                                }
                                m_boneIndices = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_mass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mass"),
                                    );
                                }
                                m_mass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stiffness => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stiffness) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffness",
                                        ),
                                    );
                                }
                                m_stiffness = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_damping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_damping) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "damping",
                                        ),
                                    );
                                }
                                m_damping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxElongation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxElongation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxElongation",
                                        ),
                                    );
                                }
                                m_maxElongation = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxCompression => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxCompression) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCompression",
                                        ),
                                    );
                                }
                                m_maxCompression = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_propagateToChildren => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_propagateToChildren,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "propagateToChildren",
                                        ),
                                    );
                                }
                                m_propagateToChildren = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_affectSiblings => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_affectSiblings) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "affectSiblings",
                                        ),
                                    );
                                }
                                m_affectSiblings = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rotateBonesForSkinning => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_rotateBonesForSkinning,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotateBonesForSkinning",
                                        ),
                                    );
                                }
                                m_rotateBonesForSkinning = _serde::__private::Some(
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
                    let m_boneIndices = match m_boneIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mass = match m_mass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("mass"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stiffness = match m_stiffness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffness",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_damping = match m_damping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("damping"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxElongation = match m_maxElongation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxElongation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxCompression = match m_maxCompression {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCompression",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_propagateToChildren = match m_propagateToChildren {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "propagateToChildren",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_affectSiblings = match m_affectSiblings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "affectSiblings",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rotateBonesForSkinning = match m_rotateBonesForSkinning {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotateBonesForSkinning",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbJigglerGroup {
                        __ptr: __ptr.clone(),
                        parent,
                        m_boneIndices,
                        m_mass,
                        m_stiffness,
                        m_damping,
                        m_maxElongation,
                        m_maxCompression,
                        m_propagateToChildren,
                        m_affectSiblings,
                        m_rotateBonesForSkinning,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "boneIndices",
                "mass",
                "stiffness",
                "damping",
                "maxElongation",
                "maxCompression",
                "propagateToChildren",
                "affectSiblings",
                "rotateBonesForSkinning",
                "pad",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbJigglerGroup",
                FIELDS,
                __hkbJigglerGroupVisitor {
                    marker: _serde::__private::PhantomData::<hkbJigglerGroup>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
