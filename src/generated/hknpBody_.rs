use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpBody`
/// - version: `1`
/// - signature: `0x28533d8`
/// - size: `  0`(x86)/`144`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpBody<'a> {
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
    /// - name: `transform`(ctype: `hkTransform`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transform"))]
    #[cfg_attr(feature = "serde", serde(rename = "transform"))]
    pub m_transform: Transform,
    /// # C++ Info
    /// - name: `flags`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: I32<'a>,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 68`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionFilterInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionFilterInfo"))]
    pub m_collisionFilterInfo: U32<'a>,
    /// # C++ Info
    /// - name: `shape`(ctype: `struct hknpShape*`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "shape"))]
    #[cfg_attr(feature = "serde", serde(rename = "shape"))]
    pub m_shape: Pointer<'a>,
    /// # C++ Info
    /// - name: `aabb`(ctype: `struct hkAabb16`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "aabb"))]
    #[cfg_attr(feature = "serde", serde(rename = "aabb"))]
    pub m_aabb: hkAabb16<'a>,
    /// # C++ Info
    /// - name: `id`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "id"))]
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    pub m_id: U32<'a>,
    /// # C++ Info
    /// - name: `nextAttachedBodyId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`100`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "nextAttachedBodyId"))]
    #[cfg_attr(feature = "serde", serde(rename = "nextAttachedBodyId"))]
    pub m_nextAttachedBodyId: U32<'a>,
    /// # C++ Info
    /// - name: `motionId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionId"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionId"))]
    pub m_motionId: U32<'a>,
    /// # C++ Info
    /// - name: `broadPhaseId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`108`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "broadPhaseId"))]
    #[cfg_attr(feature = "serde", serde(rename = "broadPhaseId"))]
    pub m_broadPhaseId: U32<'a>,
    /// # C++ Info
    /// - name: `materialId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialId"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialId"))]
    pub m_materialId: U16<'a>,
    /// # C++ Info
    /// - name: `qualityId`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`114`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "qualityId"))]
    #[cfg_attr(feature = "serde", serde(rename = "qualityId"))]
    pub m_qualityId: U8<'a>,
    /// # C++ Info
    /// - name: `timAngle`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`115`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timAngle"))]
    #[cfg_attr(feature = "serde", serde(rename = "timAngle"))]
    pub m_timAngle: U8<'a>,
    /// # C++ Info
    /// - name: `maxTimDistance`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`116`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxTimDistance"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxTimDistance"))]
    pub m_maxTimDistance: U16<'a>,
    /// # C++ Info
    /// - name: `maxContactDistance`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`118`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxContactDistance"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxContactDistance"))]
    pub m_maxContactDistance: U16<'a>,
    /// # C++ Info
    /// - name: `indexIntoActiveListOrDeactivatedIslandId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "indexIntoActiveListOrDeactivatedIslandId")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "indexIntoActiveListOrDeactivatedIslandId")
    )]
    pub m_indexIntoActiveListOrDeactivatedIslandId: U32<'a>,
    /// # C++ Info
    /// - name: `radiusOfComCenteredBoundingSphere`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/`124`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "radiusOfComCenteredBoundingSphere")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "radiusOfComCenteredBoundingSphere"))]
    pub m_radiusOfComCenteredBoundingSphere: f16,
    /// # C++ Info
    /// - name: `spuFlags`(ctype: `flags SpuFlagsEnum`)
    /// - offset: `  0`(x86)/`126`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "spuFlags"))]
    #[cfg_attr(feature = "serde", serde(rename = "spuFlags"))]
    pub m_spuFlags: SpuFlagsEnum,
    /// # C++ Info
    /// - name: `shapeSizeDiv16`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`127`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "shapeSizeDiv16"))]
    #[cfg_attr(feature = "serde", serde(rename = "shapeSizeDiv16"))]
    pub m_shapeSizeDiv16: U8<'a>,
    /// # C++ Info
    /// - name: `motionToBodyRotation`(ctype: `hkInt16[4]`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionToBodyRotation"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionToBodyRotation"))]
    pub m_motionToBodyRotation: [I16<'a>; 4usize],
    /// # C++ Info
    /// - name: `userData`(ctype: `hkUint64`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: U64<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpBody<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpBody"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x28533d8)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_shape);
            v.extend(self.m_aabb.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hknpBody<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x28533d8)));
            let mut serializer = __serializer
                .serialize_struct("hknpBody", class_meta, (0u64, 144u64))?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer.serialize_field("aabb", &self.m_aabb)?;
            serializer.serialize_field("id", &self.m_id)?;
            serializer
                .serialize_field("nextAttachedBodyId", &self.m_nextAttachedBodyId)?;
            serializer.serialize_field("motionId", &self.m_motionId)?;
            serializer.serialize_field("broadPhaseId", &self.m_broadPhaseId)?;
            serializer.serialize_field("materialId", &self.m_materialId)?;
            serializer.serialize_field("qualityId", &self.m_qualityId)?;
            serializer.serialize_field("timAngle", &self.m_timAngle)?;
            serializer.serialize_field("maxTimDistance", &self.m_maxTimDistance)?;
            serializer
                .serialize_field("maxContactDistance", &self.m_maxContactDistance)?;
            serializer
                .serialize_field(
                    "indexIntoActiveListOrDeactivatedIslandId",
                    &self.m_indexIntoActiveListOrDeactivatedIslandId,
                )?;
            serializer
                .serialize_field(
                    "radiusOfComCenteredBoundingSphere",
                    &self.m_radiusOfComCenteredBoundingSphere,
                )?;
            serializer.serialize_field("spuFlags", &self.m_spuFlags)?;
            serializer.serialize_field("shapeSizeDiv16", &self.m_shapeSizeDiv16)?;
            serializer
                .serialize_fixed_array_field(
                    "motionToBodyRotation",
                    self.m_motionToBodyRotation.as_slice(),
                    TypeSize::NonPtr,
                )?;
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
    impl<'de> _serde::Deserialize<'de> for hknpBody<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transform,
                m_flags,
                m_collisionFilterInfo,
                m_shape,
                m_aabb,
                m_id,
                m_nextAttachedBodyId,
                m_motionId,
                m_broadPhaseId,
                m_materialId,
                m_qualityId,
                m_timAngle,
                m_maxTimDistance,
                m_maxContactDistance,
                m_indexIntoActiveListOrDeactivatedIslandId,
                m_radiusOfComCenteredBoundingSphere,
                m_spuFlags,
                m_shapeSizeDiv16,
                m_motionToBodyRotation,
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
                        "transform" => Ok(__Field::m_transform),
                        "flags" => Ok(__Field::m_flags),
                        "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
                        "shape" => Ok(__Field::m_shape),
                        "aabb" => Ok(__Field::m_aabb),
                        "id" => Ok(__Field::m_id),
                        "nextAttachedBodyId" => Ok(__Field::m_nextAttachedBodyId),
                        "motionId" => Ok(__Field::m_motionId),
                        "broadPhaseId" => Ok(__Field::m_broadPhaseId),
                        "materialId" => Ok(__Field::m_materialId),
                        "qualityId" => Ok(__Field::m_qualityId),
                        "timAngle" => Ok(__Field::m_timAngle),
                        "maxTimDistance" => Ok(__Field::m_maxTimDistance),
                        "maxContactDistance" => Ok(__Field::m_maxContactDistance),
                        "indexIntoActiveListOrDeactivatedIslandId" => {
                            Ok(__Field::m_indexIntoActiveListOrDeactivatedIslandId)
                        }
                        "radiusOfComCenteredBoundingSphere" => {
                            Ok(__Field::m_radiusOfComCenteredBoundingSphere)
                        }
                        "spuFlags" => Ok(__Field::m_spuFlags),
                        "shapeSizeDiv16" => Ok(__Field::m_shapeSizeDiv16),
                        "motionToBodyRotation" => Ok(__Field::m_motionToBodyRotation),
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
            struct __hknpBodyVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpBody<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpBodyVisitor<'de> {
                type Value = hknpBody<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpBody")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_transform: _serde::__private::Option<Transform> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_shape: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_aabb: _serde::__private::Option<hkAabb16<'de>> = _serde::__private::None;
                    let mut m_id: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_nextAttachedBodyId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_motionId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_broadPhaseId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_materialId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_qualityId: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_timAngle: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_maxTimDistance: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_maxContactDistance: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_indexIntoActiveListOrDeactivatedIslandId: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_radiusOfComCenteredBoundingSphere: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_spuFlags: _serde::__private::Option<SpuFlagsEnum> = _serde::__private::None;
                    let mut m_shapeSizeDiv16: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_motionToBodyRotation: _serde::__private::Option<
                        [I16<'de>; 4usize],
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    for i in 0..20usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_transform) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transform",
                                        ),
                                    );
                                }
                                m_transform = _serde::__private::Some(
                                    match __A::next_value::<Transform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
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
                            2usize => {
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
                            3usize => {
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
                            4usize => {
                                if _serde::__private::Option::is_some(&m_aabb) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("aabb"),
                                    );
                                }
                                m_aabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_id) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                m_id = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_nextAttachedBodyId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextAttachedBodyId",
                                        ),
                                    );
                                }
                                m_nextAttachedBodyId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
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
                            8usize => {
                                if _serde::__private::Option::is_some(&m_broadPhaseId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseId",
                                        ),
                                    );
                                }
                                m_broadPhaseId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
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
                            10usize => {
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
                            11usize => {
                                if _serde::__private::Option::is_some(&m_timAngle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timAngle",
                                        ),
                                    );
                                }
                                m_timAngle = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_maxTimDistance) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTimDistance",
                                        ),
                                    );
                                }
                                m_maxTimDistance = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxContactDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxContactDistance",
                                        ),
                                    );
                                }
                                m_maxContactDistance = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_indexIntoActiveListOrDeactivatedIslandId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexIntoActiveListOrDeactivatedIslandId",
                                        ),
                                    );
                                }
                                m_indexIntoActiveListOrDeactivatedIslandId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_radiusOfComCenteredBoundingSphere,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "radiusOfComCenteredBoundingSphere",
                                        ),
                                    );
                                }
                                m_radiusOfComCenteredBoundingSphere = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
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
                            17usize => {
                                if _serde::__private::Option::is_some(&m_shapeSizeDiv16) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeSizeDiv16",
                                        ),
                                    );
                                }
                                m_shapeSizeDiv16 = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_motionToBodyRotation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionToBodyRotation",
                                        ),
                                    );
                                }
                                m_motionToBodyRotation = _serde::__private::Some(
                                    match __A::next_value::<[I16<'de>; 4usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
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
                            _ => {}
                        }
                    }
                    let m_transform = match m_transform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transform",
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
                    let m_shape = match m_shape {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shape"),
                            );
                        }
                    };
                    let m_aabb = match m_aabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("aabb"),
                            );
                        }
                    };
                    let m_id = match m_id {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("id"),
                            );
                        }
                    };
                    let m_nextAttachedBodyId = match m_nextAttachedBodyId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextAttachedBodyId",
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
                    let m_broadPhaseId = match m_broadPhaseId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseId",
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
                    let m_timAngle = match m_timAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("timAngle"),
                            );
                        }
                    };
                    let m_maxTimDistance = match m_maxTimDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTimDistance",
                                ),
                            );
                        }
                    };
                    let m_maxContactDistance = match m_maxContactDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxContactDistance",
                                ),
                            );
                        }
                    };
                    let m_indexIntoActiveListOrDeactivatedIslandId = match m_indexIntoActiveListOrDeactivatedIslandId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexIntoActiveListOrDeactivatedIslandId",
                                ),
                            );
                        }
                    };
                    let m_radiusOfComCenteredBoundingSphere = match m_radiusOfComCenteredBoundingSphere {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "radiusOfComCenteredBoundingSphere",
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
                    let m_shapeSizeDiv16 = match m_shapeSizeDiv16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapeSizeDiv16",
                                ),
                            );
                        }
                    };
                    let m_motionToBodyRotation = match m_motionToBodyRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionToBodyRotation",
                                ),
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
                    _serde::__private::Ok(hknpBody {
                        __ptr,
                        m_transform,
                        m_flags,
                        m_collisionFilterInfo,
                        m_shape,
                        m_aabb,
                        m_id,
                        m_nextAttachedBodyId,
                        m_motionId,
                        m_broadPhaseId,
                        m_materialId,
                        m_qualityId,
                        m_timAngle,
                        m_maxTimDistance,
                        m_maxContactDistance,
                        m_indexIntoActiveListOrDeactivatedIslandId,
                        m_radiusOfComCenteredBoundingSphere,
                        m_spuFlags,
                        m_shapeSizeDiv16,
                        m_motionToBodyRotation,
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
                    let mut m_transform: _serde::__private::Option<Transform> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_shape: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_aabb: _serde::__private::Option<hkAabb16<'de>> = _serde::__private::None;
                    let mut m_id: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_nextAttachedBodyId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_motionId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_broadPhaseId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_materialId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_qualityId: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_timAngle: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_maxTimDistance: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_maxContactDistance: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_indexIntoActiveListOrDeactivatedIslandId: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_radiusOfComCenteredBoundingSphere: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_spuFlags: _serde::__private::Option<SpuFlagsEnum> = _serde::__private::None;
                    let mut m_shapeSizeDiv16: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_motionToBodyRotation: _serde::__private::Option<
                        [I16<'de>; 4usize],
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_transform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transform) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transform",
                                        ),
                                    );
                                }
                                m_transform = _serde::__private::Some(
                                    match __A::next_value::<Transform>(&mut __map) {
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
                            __Field::m_aabb => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_aabb) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("aabb"),
                                    );
                                }
                                m_aabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_id => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_id) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                m_id = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nextAttachedBodyId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nextAttachedBodyId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextAttachedBodyId",
                                        ),
                                    );
                                }
                                m_nextAttachedBodyId = _serde::__private::Some(
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
                            __Field::m_broadPhaseId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_broadPhaseId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseId",
                                        ),
                                    );
                                }
                                m_broadPhaseId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                            __Field::m_timAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timAngle) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timAngle",
                                        ),
                                    );
                                }
                                m_timAngle = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxTimDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxTimDistance) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTimDistance",
                                        ),
                                    );
                                }
                                m_maxTimDistance = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxContactDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxContactDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxContactDistance",
                                        ),
                                    );
                                }
                                m_maxContactDistance = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexIntoActiveListOrDeactivatedIslandId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_indexIntoActiveListOrDeactivatedIslandId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexIntoActiveListOrDeactivatedIslandId",
                                        ),
                                    );
                                }
                                m_indexIntoActiveListOrDeactivatedIslandId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_radiusOfComCenteredBoundingSphere => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_radiusOfComCenteredBoundingSphere,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "radiusOfComCenteredBoundingSphere",
                                        ),
                                    );
                                }
                                m_radiusOfComCenteredBoundingSphere = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
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
                            __Field::m_shapeSizeDiv16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapeSizeDiv16) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeSizeDiv16",
                                        ),
                                    );
                                }
                                m_shapeSizeDiv16 = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionToBodyRotation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_motionToBodyRotation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionToBodyRotation",
                                        ),
                                    );
                                }
                                m_motionToBodyRotation = _serde::__private::Some(
                                    match __A::next_value::<[I16<'de>; 4usize]>(&mut __map) {
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_transform = match m_transform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transform",
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
                    let m_aabb = match m_aabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("aabb"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_id = match m_id {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("id"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nextAttachedBodyId = match m_nextAttachedBodyId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextAttachedBodyId",
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
                    let m_broadPhaseId = match m_broadPhaseId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseId",
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
                    let m_timAngle = match m_timAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("timAngle"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxTimDistance = match m_maxTimDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTimDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxContactDistance = match m_maxContactDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxContactDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexIntoActiveListOrDeactivatedIslandId = match m_indexIntoActiveListOrDeactivatedIslandId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexIntoActiveListOrDeactivatedIslandId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_radiusOfComCenteredBoundingSphere = match m_radiusOfComCenteredBoundingSphere {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "radiusOfComCenteredBoundingSphere",
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
                    let m_shapeSizeDiv16 = match m_shapeSizeDiv16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapeSizeDiv16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionToBodyRotation = match m_motionToBodyRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionToBodyRotation",
                                ),
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpBody {
                        __ptr: __ptr.clone(),
                        m_transform,
                        m_flags,
                        m_collisionFilterInfo,
                        m_shape,
                        m_aabb,
                        m_id,
                        m_nextAttachedBodyId,
                        m_motionId,
                        m_broadPhaseId,
                        m_materialId,
                        m_qualityId,
                        m_timAngle,
                        m_maxTimDistance,
                        m_maxContactDistance,
                        m_indexIntoActiveListOrDeactivatedIslandId,
                        m_radiusOfComCenteredBoundingSphere,
                        m_spuFlags,
                        m_shapeSizeDiv16,
                        m_motionToBodyRotation,
                        m_userData,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transform",
                "flags",
                "collisionFilterInfo",
                "shape",
                "aabb",
                "id",
                "nextAttachedBodyId",
                "motionId",
                "broadPhaseId",
                "materialId",
                "qualityId",
                "timAngle",
                "maxTimDistance",
                "maxContactDistance",
                "indexIntoActiveListOrDeactivatedIslandId",
                "radiusOfComCenteredBoundingSphere",
                "spuFlags",
                "shapeSizeDiv16",
                "motionToBodyRotation",
                "userData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpBody",
                FIELDS,
                __hknpBodyVisitor {
                    marker: _serde::__private::PhantomData::<hknpBody>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `SpuFlagsEnum`(ctype: `hkFlags<SpuFlagsEnum, hkUint8>`)"]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    SpuFlagsEnum : u8 { #[doc = "1"] const FORCE_NARROW_PHASE_PPU = 1u8; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for SpuFlagsEnum {
        fn schema_name() -> Cow<'static, str> {
            "SpuFlagsEnum".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "SpuFlagsEnum").into()
        }
        fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
            json_schema!(
                { "description" :
                "Bitflags field. Specific flags: FORCE_NARROW_PHASE_PPU: 1. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
                "type" : "string", }
            )
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SpuFlagsEnum {
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
                    Self::FORCE_NARROW_PHASE_PPU => {
                        __serializer
                            .serialize_field(
                                "FORCE_NARROW_PHASE_PPU",
                                &Self::FORCE_NARROW_PHASE_PPU,
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
    impl<'de> _serde::Deserialize<'de> for SpuFlagsEnum {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SpuFlagsEnum>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SpuFlagsEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SpuFlagsEnum(flags)",
                    )
                }
                #[inline]
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(__value) => {
                            Ok(SpuFlagsEnum::from_bits_retain(__value as _))
                        }
                        _ => {
                            Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value as _),
                                    &"SpuFlagsEnum(U8) Number",
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
                    match <SpuFlagsEnum as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SpuFlagsEnum>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
