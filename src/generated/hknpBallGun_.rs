use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpBallGun`
/// - version: `0`
/// - signature: `0xfbbc39e7`
/// - size: `  0`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpBallGun<'a> {
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
    pub parent: hknpFirstPersonGun<'a>,
    /// # C++ Info
    /// - name: `bulletRadius`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bulletRadius"))]
    #[cfg_attr(feature = "serde", serde(rename = "bulletRadius"))]
    pub m_bulletRadius: f32,
    /// # C++ Info
    /// - name: `bulletVelocity`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 68`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bulletVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "bulletVelocity"))]
    pub m_bulletVelocity: f32,
    /// # C++ Info
    /// - name: `bulletMass`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bulletMass"))]
    #[cfg_attr(feature = "serde", serde(rename = "bulletMass"))]
    pub m_bulletMass: f32,
    /// # C++ Info
    /// - name: `damageMultiplier`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 76`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "damageMultiplier"))]
    #[cfg_attr(feature = "serde", serde(rename = "damageMultiplier"))]
    pub m_damageMultiplier: f32,
    /// # C++ Info
    /// - name: `maxBulletsInWorld`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxBulletsInWorld"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxBulletsInWorld"))]
    pub m_maxBulletsInWorld: I32<'a>,
    /// # C++ Info
    /// - name: `bulletOffsetFromCenter`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bulletOffsetFromCenter"))]
    #[cfg_attr(feature = "serde", serde(rename = "bulletOffsetFromCenter"))]
    pub m_bulletOffsetFromCenter: Vector4,
    /// # C++ Info
    /// - name: `addedBodyIds`(ctype: `void*`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "addedBodyIds"))]
    #[cfg_attr(feature = "serde", serde(rename = "addedBodyIds"))]
    pub m_addedBodyIds: Pointer<'a>,
    /// # C++ Info
    /// - name: `materialId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialId"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialId"))]
    pub m_materialId: U16<'a>,
    /// # C++ Info
    /// - name: `motionPropertiesId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`122`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionPropertiesId"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionPropertiesId"))]
    pub m_motionPropertiesId: U16<'a>,
    /// # C++ Info
    /// - name: `bulletShape`(ctype: `void*`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bulletShape"))]
    #[cfg_attr(feature = "serde", serde(rename = "bulletShape"))]
    pub m_bulletShape: Pointer<'a>,
    /// # C++ Info
    /// - name: `raiseContactEvents`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "raiseContactEvents"))]
    #[cfg_attr(feature = "serde", serde(rename = "raiseContactEvents"))]
    pub m_raiseContactEvents: U32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpBallGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpBallGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfbbc39e7)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.parent.m_listeners.iter());
            v.push(&self.m_addedBodyIds);
            v.push(&self.m_bulletShape);
            v
        }
    }
    impl<'a> _serde::Serialize for hknpBallGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xfbbc39e7)));
            let mut serializer = __serializer
                .serialize_struct("hknpBallGun", class_meta, (0u64, 144u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("name", &self.parent.m_name)?;
            serializer.serialize_field("keyboardKey", &self.parent.m_keyboardKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .skip_array_field(
                    "listeners",
                    &self.parent.m_listeners,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field("collisionFilterInfo", &self.parent.m_collisionFilterInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bulletRadius", &self.m_bulletRadius)?;
            serializer.serialize_field("bulletVelocity", &self.m_bulletVelocity)?;
            serializer.serialize_field("bulletMass", &self.m_bulletMass)?;
            serializer.serialize_field("damageMultiplier", &self.m_damageMultiplier)?;
            serializer.serialize_field("maxBulletsInWorld", &self.m_maxBulletsInWorld)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_field(
                    "bulletOffsetFromCenter",
                    &self.m_bulletOffsetFromCenter,
                )?;
            serializer.skip_field("addedBodyIds", &self.m_addedBodyIds)?;
            serializer.skip_field("materialId", &self.m_materialId)?;
            serializer.skip_field("motionPropertiesId", &self.m_motionPropertiesId)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("bulletShape", &self.m_bulletShape)?;
            serializer.skip_field("raiseContactEvents", &self.m_raiseContactEvents)?;
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
    impl<'de> _serde::Deserialize<'de> for hknpBallGun<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_keyboardKey,
                m_bulletRadius,
                m_bulletVelocity,
                m_bulletMass,
                m_damageMultiplier,
                m_maxBulletsInWorld,
                m_bulletOffsetFromCenter,
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
                        "bulletRadius" => Ok(__Field::m_bulletRadius),
                        "bulletVelocity" => Ok(__Field::m_bulletVelocity),
                        "bulletMass" => Ok(__Field::m_bulletMass),
                        "damageMultiplier" => Ok(__Field::m_damageMultiplier),
                        "maxBulletsInWorld" => Ok(__Field::m_maxBulletsInWorld),
                        "bulletOffsetFromCenter" => Ok(__Field::m_bulletOffsetFromCenter),
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
            struct __hknpBallGunVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpBallGun<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpBallGunVisitor<'de> {
                type Value = hknpBallGun<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpBallGun")
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
                    let mut m_bulletRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bulletVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bulletMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_damageMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxBulletsInWorld: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_bulletOffsetFromCenter: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_addedBodyIds: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_materialId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_motionPropertiesId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_bulletShape: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_raiseContactEvents: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_bulletRadius) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletRadius",
                                        ),
                                    );
                                }
                                m_bulletRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bulletVelocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletVelocity",
                                        ),
                                    );
                                }
                                m_bulletVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_bulletMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletMass",
                                        ),
                                    );
                                }
                                m_bulletMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_damageMultiplier) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "damageMultiplier",
                                        ),
                                    );
                                }
                                m_damageMultiplier = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxBulletsInWorld,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxBulletsInWorld",
                                        ),
                                    );
                                }
                                m_maxBulletsInWorld = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bulletOffsetFromCenter,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletOffsetFromCenter",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 12usize)?;
                                m_bulletOffsetFromCenter = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_addedBodyIds) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "addedBodyIds",
                                        ),
                                    );
                                }
                                m_addedBodyIds = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_materialId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialId",
                                        ),
                                    );
                                }
                                m_materialId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_motionPropertiesId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionPropertiesId",
                                        ),
                                    );
                                }
                                m_motionPropertiesId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_bulletShape) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletShape",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_bulletShape = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_raiseContactEvents,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raiseContactEvents",
                                        ),
                                    );
                                }
                                m_raiseContactEvents = _serde::__private::Some(
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
                    let m_bulletRadius = match m_bulletRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletRadius",
                                ),
                            );
                        }
                    };
                    let m_bulletVelocity = match m_bulletVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletVelocity",
                                ),
                            );
                        }
                    };
                    let m_bulletMass = match m_bulletMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletMass",
                                ),
                            );
                        }
                    };
                    let m_damageMultiplier = match m_damageMultiplier {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "damageMultiplier",
                                ),
                            );
                        }
                    };
                    let m_maxBulletsInWorld = match m_maxBulletsInWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxBulletsInWorld",
                                ),
                            );
                        }
                    };
                    let m_bulletOffsetFromCenter = match m_bulletOffsetFromCenter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletOffsetFromCenter",
                                ),
                            );
                        }
                    };
                    let m_addedBodyIds = match m_addedBodyIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "addedBodyIds",
                                ),
                            );
                        }
                    };
                    let m_materialId = match m_materialId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialId",
                                ),
                            );
                        }
                    };
                    let m_motionPropertiesId = match m_motionPropertiesId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionPropertiesId",
                                ),
                            );
                        }
                    };
                    let m_bulletShape = match m_bulletShape {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletShape",
                                ),
                            );
                        }
                    };
                    let m_raiseContactEvents = match m_raiseContactEvents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raiseContactEvents",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpBallGun {
                        __ptr,
                        parent,
                        m_bulletRadius,
                        m_bulletVelocity,
                        m_bulletMass,
                        m_damageMultiplier,
                        m_maxBulletsInWorld,
                        m_bulletOffsetFromCenter,
                        m_addedBodyIds,
                        m_materialId,
                        m_motionPropertiesId,
                        m_bulletShape,
                        m_raiseContactEvents,
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
                    let mut m_bulletRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bulletVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bulletMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_damageMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxBulletsInWorld: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_bulletOffsetFromCenter: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
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
                            __Field::m_bulletRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bulletRadius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletRadius",
                                        ),
                                    );
                                }
                                m_bulletRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bulletVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bulletVelocity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletVelocity",
                                        ),
                                    );
                                }
                                m_bulletVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bulletMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bulletMass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletMass",
                                        ),
                                    );
                                }
                                m_bulletMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_damageMultiplier => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_damageMultiplier) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "damageMultiplier",
                                        ),
                                    );
                                }
                                m_damageMultiplier = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxBulletsInWorld => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxBulletsInWorld,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxBulletsInWorld",
                                        ),
                                    );
                                }
                                m_maxBulletsInWorld = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bulletOffsetFromCenter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_bulletOffsetFromCenter,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bulletOffsetFromCenter",
                                        ),
                                    );
                                }
                                m_bulletOffsetFromCenter = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_bulletRadius = match m_bulletRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bulletVelocity = match m_bulletVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bulletMass = match m_bulletMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletMass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_damageMultiplier = match m_damageMultiplier {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "damageMultiplier",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxBulletsInWorld = match m_maxBulletsInWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxBulletsInWorld",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bulletOffsetFromCenter = match m_bulletOffsetFromCenter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bulletOffsetFromCenter",
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
                    let parent = hknpFirstPersonGun {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        m_keyboardKey,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpBallGun {
                        __ptr: __ptr.clone(),
                        parent,
                        m_bulletRadius,
                        m_bulletVelocity,
                        m_bulletMass,
                        m_damageMultiplier,
                        m_maxBulletsInWorld,
                        m_bulletOffsetFromCenter,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "bulletRadius",
                "bulletVelocity",
                "bulletMass",
                "damageMultiplier",
                "maxBulletsInWorld",
                "bulletOffsetFromCenter",
                "addedBodyIds",
                "materialId",
                "motionPropertiesId",
                "bulletShape",
                "raiseContactEvents",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpBallGun",
                FIELDS,
                __hknpBallGunVisitor {
                    marker: _serde::__private::PhantomData::<hknpBallGun>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
