use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpBodyCinfo`
/// - version: `2`
/// - signature: `0x6896f7c9`
/// - size: `  0`(x86)/` 96`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpBodyCinfo<'a> {
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
    /// - name: `shape`(ctype: `struct hknpShape*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `NOT_OWNED`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "shape"))]
    #[cfg_attr(feature = "serde", serde(rename = "shape"))]
    pub m_shape: Pointer<'a>,
    /// # C++ Info
    /// - name: `reservedBodyId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "reservedBodyId"))]
    #[cfg_attr(feature = "serde", serde(rename = "reservedBodyId"))]
    pub m_reservedBodyId: U32<'a>,
    /// # C++ Info
    /// - name: `motionId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionId"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionId"))]
    pub m_motionId: U32<'a>,
    /// # C++ Info
    /// - name: `qualityId`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "qualityId"))]
    #[cfg_attr(feature = "serde", serde(rename = "qualityId"))]
    pub m_qualityId: U8<'a>,
    /// # C++ Info
    /// - name: `materialId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 18`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialId"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialId"))]
    pub m_materialId: U16<'a>,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionFilterInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionFilterInfo"))]
    pub m_collisionFilterInfo: U32<'a>,
    /// # C++ Info
    /// - name: `flags`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: I32<'a>,
    /// # C++ Info
    /// - name: `collisionLookAheadDistance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionLookAheadDistance"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionLookAheadDistance"))]
    pub m_collisionLookAheadDistance: f32,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `userData`(ctype: `hkUint64`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: U64<'a>,
    /// # C++ Info
    /// - name: `position`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "position"))]
    #[cfg_attr(feature = "serde", serde(rename = "position"))]
    pub m_position: Vector4,
    /// # C++ Info
    /// - name: `orientation`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "orientation"))]
    #[cfg_attr(feature = "serde", serde(rename = "orientation"))]
    pub m_orientation: Quaternion,
    /// # C++ Info
    /// - name: `spuFlags`(ctype: `flags SpuFlagsEnum`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "spuFlags"))]
    #[cfg_attr(feature = "serde", serde(rename = "spuFlags"))]
    pub m_spuFlags: SpuFlagsEnum,
    /// # C++ Info
    /// - name: `localFrame`(ctype: `struct hkLocalFrame*`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localFrame"))]
    #[cfg_attr(feature = "serde", serde(rename = "localFrame"))]
    pub m_localFrame: Pointer<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpBodyCinfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpBodyCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6896f7c9)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_shape);
            v.push(&self.m_localFrame);
            v
        }
    }
    impl<'a> _serde::Serialize for hknpBodyCinfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x6896f7c9)));
            let mut serializer = __serializer
                .serialize_struct("hknpBodyCinfo", class_meta, (0u64, 96u64))?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer.serialize_field("reservedBodyId", &self.m_reservedBodyId)?;
            serializer.serialize_field("motionId", &self.m_motionId)?;
            serializer.serialize_field("qualityId", &self.m_qualityId)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("materialId", &self.m_materialId)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer
                .serialize_field(
                    "collisionLookAheadDistance",
                    &self.m_collisionLookAheadDistance,
                )?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("orientation", &self.m_orientation)?;
            serializer.serialize_field("spuFlags", &self.m_spuFlags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("localFrame", &self.m_localFrame)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpBodyCinfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_shape,
                m_reservedBodyId,
                m_motionId,
                m_qualityId,
                m_materialId,
                m_collisionFilterInfo,
                m_flags,
                m_collisionLookAheadDistance,
                m_name,
                m_userData,
                m_position,
                m_orientation,
                m_spuFlags,
                m_localFrame,
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
                        "shape" => Ok(__Field::m_shape),
                        "reservedBodyId" => Ok(__Field::m_reservedBodyId),
                        "motionId" => Ok(__Field::m_motionId),
                        "qualityId" => Ok(__Field::m_qualityId),
                        "materialId" => Ok(__Field::m_materialId),
                        "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
                        "flags" => Ok(__Field::m_flags),
                        "collisionLookAheadDistance" => {
                            Ok(__Field::m_collisionLookAheadDistance)
                        }
                        "name" => Ok(__Field::m_name),
                        "userData" => Ok(__Field::m_userData),
                        "position" => Ok(__Field::m_position),
                        "orientation" => Ok(__Field::m_orientation),
                        "spuFlags" => Ok(__Field::m_spuFlags),
                        "localFrame" => Ok(__Field::m_localFrame),
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
            struct __hknpBodyCinfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpBodyCinfo<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpBodyCinfoVisitor<'de> {
                type Value = hknpBodyCinfo<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpBodyCinfo")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_shape: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_reservedBodyId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_motionId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_qualityId: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_materialId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_collisionLookAheadDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    let mut m_position: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_orientation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_spuFlags: _serde::__private::Option<SpuFlagsEnum> = _serde::__private::None;
                    let mut m_localFrame: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    for i in 0..14usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_shape) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                                    );
                                }
                                m_shape = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_reservedBodyId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reservedBodyId",
                                        ),
                                    );
                                }
                                m_reservedBodyId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_motionId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionId",
                                        ),
                                    );
                                }
                                m_motionId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_qualityId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "qualityId",
                                        ),
                                    );
                                }
                                m_qualityId = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_materialId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialId",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_materialId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionLookAheadDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionLookAheadDistance",
                                        ),
                                    );
                                }
                                m_collisionLookAheadDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
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
                            9usize => {
                                if _serde::__private::Option::is_some(&m_userData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<U64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_position) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "position",
                                        ),
                                    );
                                }
                                m_position = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_orientation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "orientation",
                                        ),
                                    );
                                }
                                m_orientation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_spuFlags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spuFlags",
                                        ),
                                    );
                                }
                                m_spuFlags = _serde::__private::Some(
                                    match __A::next_value::<SpuFlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_localFrame) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localFrame",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_localFrame = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                    let m_shape = match m_shape {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shape"),
                            );
                        }
                    };
                    let m_reservedBodyId = match m_reservedBodyId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reservedBodyId",
                                ),
                            );
                        }
                    };
                    let m_motionId = match m_motionId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motionId"),
                            );
                        }
                    };
                    let m_qualityId = match m_qualityId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "qualityId",
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
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    let m_collisionLookAheadDistance = match m_collisionLookAheadDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionLookAheadDistance",
                                ),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                        }
                    };
                    let m_position = match m_position {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("position"),
                            );
                        }
                    };
                    let m_orientation = match m_orientation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "orientation",
                                ),
                            );
                        }
                    };
                    let m_spuFlags = match m_spuFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("spuFlags"),
                            );
                        }
                    };
                    let m_localFrame = match m_localFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localFrame",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpBodyCinfo {
                        __ptr,
                        m_shape,
                        m_reservedBodyId,
                        m_motionId,
                        m_qualityId,
                        m_materialId,
                        m_collisionFilterInfo,
                        m_flags,
                        m_collisionLookAheadDistance,
                        m_name,
                        m_userData,
                        m_position,
                        m_orientation,
                        m_spuFlags,
                        m_localFrame,
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
                    let mut m_shape: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_reservedBodyId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_motionId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_qualityId: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_materialId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_collisionLookAheadDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    let mut m_position: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_orientation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_spuFlags: _serde::__private::Option<SpuFlagsEnum> = _serde::__private::None;
                    let mut m_localFrame: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_shape => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shape) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                                    );
                                }
                                m_shape = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_reservedBodyId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_reservedBodyId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reservedBodyId",
                                        ),
                                    );
                                }
                                m_reservedBodyId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motionId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionId",
                                        ),
                                    );
                                }
                                m_motionId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_qualityId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_qualityId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "qualityId",
                                        ),
                                    );
                                }
                                m_qualityId = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materialId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materialId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_collisionFilterInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_flags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_flags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionLookAheadDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionLookAheadDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionLookAheadDistance",
                                        ),
                                    );
                                }
                                m_collisionLookAheadDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
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
                                    match __A::next_value::<U64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_position => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_position) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "position",
                                        ),
                                    );
                                }
                                m_position = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_orientation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_orientation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "orientation",
                                        ),
                                    );
                                }
                                m_orientation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_spuFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_spuFlags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spuFlags",
                                        ),
                                    );
                                }
                                m_spuFlags = _serde::__private::Some(
                                    match __A::next_value::<SpuFlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localFrame => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localFrame) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localFrame",
                                        ),
                                    );
                                }
                                m_localFrame = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                    let m_shape = match m_shape {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shape"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_reservedBodyId = match m_reservedBodyId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reservedBodyId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionId = match m_motionId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motionId"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_qualityId = match m_qualityId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "qualityId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materialId = match m_materialId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionLookAheadDistance = match m_collisionLookAheadDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionLookAheadDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let m_position = match m_position {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("position"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_orientation = match m_orientation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "orientation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_spuFlags = match m_spuFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("spuFlags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_localFrame = match m_localFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localFrame",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpBodyCinfo {
                        __ptr: __ptr.clone(),
                        m_shape,
                        m_reservedBodyId,
                        m_motionId,
                        m_qualityId,
                        m_materialId,
                        m_collisionFilterInfo,
                        m_flags,
                        m_collisionLookAheadDistance,
                        m_name,
                        m_userData,
                        m_position,
                        m_orientation,
                        m_spuFlags,
                        m_localFrame,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "shape",
                "reservedBodyId",
                "motionId",
                "qualityId",
                "materialId",
                "collisionFilterInfo",
                "flags",
                "collisionLookAheadDistance",
                "name",
                "userData",
                "position",
                "orientation",
                "spuFlags",
                "localFrame",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpBodyCinfo",
                FIELDS,
                __hknpBodyCinfoVisitor {
                    marker: _serde::__private::PhantomData::<hknpBodyCinfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
