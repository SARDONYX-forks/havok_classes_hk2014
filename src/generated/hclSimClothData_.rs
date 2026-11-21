use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSimClothData`
/// - version: `11`
/// - signature: `0xe6105187`
/// - size: `  0`(x86)/`384`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSimClothData<'a> {
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
    /// - name: `simulationInfo`(ctype: `struct hclSimClothDataOverridableSimulationInfo`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simulationInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "simulationInfo"))]
    pub m_simulationInfo: hclSimClothDataOverridableSimulationInfo<'a>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `particleDatas`(ctype: `hkArray<struct hclSimClothDataParticleData>`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleDatas"))]
    pub m_particleDatas: Vec<hclSimClothDataParticleData<'a>>,
    /// # C++ Info
    /// - name: `fixedParticles`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fixedParticles"))]
    #[cfg_attr(feature = "serde", serde(rename = "fixedParticles"))]
    pub m_fixedParticles: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `triangleIndices`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "triangleIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "triangleIndices"))]
    pub m_triangleIndices: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `triangleFlips`(ctype: `hkArray<hkUint8>`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "triangleFlips"))]
    #[cfg_attr(feature = "serde", serde(rename = "triangleFlips"))]
    pub m_triangleFlips: Vec<U8<'a>>,
    /// # C++ Info
    /// - name: `totalMass`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "totalMass"))]
    #[cfg_attr(feature = "serde", serde(rename = "totalMass"))]
    pub m_totalMass: f32,
    /// # C++ Info
    /// - name: `collidableTransformMap`(ctype: `struct hclSimClothDataCollidableTransformMap`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 40`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collidableTransformMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "collidableTransformMap"))]
    pub m_collidableTransformMap: hclSimClothDataCollidableTransformMap<'a>,
    /// # C++ Info
    /// - name: `perInstanceCollidables`(ctype: `hkArray<hclCollidable*>`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "perInstanceCollidables"))]
    #[cfg_attr(feature = "serde", serde(rename = "perInstanceCollidables"))]
    pub m_perInstanceCollidables: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `staticConstraintSets`(ctype: `hkArray<hclConstraintSet*>`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticConstraintSets"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticConstraintSets"))]
    pub m_staticConstraintSets: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `antiPinchConstraintSets`(ctype: `hkArray<hclConstraintSet*>`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "antiPinchConstraintSets"))]
    #[cfg_attr(feature = "serde", serde(rename = "antiPinchConstraintSets"))]
    pub m_antiPinchConstraintSets: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `simClothPoses`(ctype: `hkArray<hclSimClothPose*>`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simClothPoses"))]
    #[cfg_attr(feature = "serde", serde(rename = "simClothPoses"))]
    pub m_simClothPoses: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `actions`(ctype: `hkArray<hclAction*>`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "actions"))]
    #[cfg_attr(feature = "serde", serde(rename = "actions"))]
    pub m_actions: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `staticCollisionMasks`(ctype: `hkArray<hkUint32>`)
    /// - offset: `  0`(x86)/`248`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticCollisionMasks"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticCollisionMasks"))]
    pub m_staticCollisionMasks: Vec<U32<'a>>,
    /// # C++ Info
    /// - name: `perParticlePinchDetectionEnabledFlags`(ctype: `hkArray<hkBool>`)
    /// - offset: `  0`(x86)/`264`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "perParticlePinchDetectionEnabledFlags")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "perParticlePinchDetectionEnabledFlags")
    )]
    pub m_perParticlePinchDetectionEnabledFlags: Vec<bool>,
    /// # C++ Info
    /// - name: `collidablePinchingDatas`(ctype: `hkArray<struct hclSimClothDataCollidablePinchingData>`)
    /// - offset: `  0`(x86)/`280`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collidablePinchingDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "collidablePinchingDatas"))]
    pub m_collidablePinchingDatas: Vec<hclSimClothDataCollidablePinchingData<'a>>,
    /// # C++ Info
    /// - name: `minPinchedParticleIndex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`296`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minPinchedParticleIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "minPinchedParticleIndex"))]
    pub m_minPinchedParticleIndex: U16<'a>,
    /// # C++ Info
    /// - name: `maxPinchedParticleIndex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`298`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxPinchedParticleIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxPinchedParticleIndex"))]
    pub m_maxPinchedParticleIndex: U16<'a>,
    /// # C++ Info
    /// - name: `maxCollisionPairs`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`300`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxCollisionPairs"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxCollisionPairs"))]
    pub m_maxCollisionPairs: U32<'a>,
    /// # C++ Info
    /// - name: `maxParticleRadius`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`304`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxParticleRadius"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxParticleRadius"))]
    pub m_maxParticleRadius: f32,
    /// # C++ Info
    /// - name: `landscapeCollisionData`(ctype: `struct hclSimClothDataLandscapeCollisionData`)
    /// - offset: `  0`(x86)/`308`(x86_64)
    /// - type_size: `  0`(x86)/` 20`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "landscapeCollisionData"))]
    #[cfg_attr(feature = "serde", serde(rename = "landscapeCollisionData"))]
    pub m_landscapeCollisionData: hclSimClothDataLandscapeCollisionData<'a>,
    /// # C++ Info
    /// - name: `numLandscapeCollidableParticles`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`328`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "numLandscapeCollidableParticles")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "numLandscapeCollidableParticles"))]
    pub m_numLandscapeCollidableParticles: U32<'a>,
    /// # C++ Info
    /// - name: `doNormals`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`332`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "doNormals"))]
    #[cfg_attr(feature = "serde", serde(rename = "doNormals"))]
    pub m_doNormals: bool,
    /// # C++ Info
    /// - name: `transferMotionData`(ctype: `struct hclSimClothDataTransferMotionData`)
    /// - offset: `  0`(x86)/`336`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transferMotionData"))]
    #[cfg_attr(feature = "serde", serde(rename = "transferMotionData"))]
    pub m_transferMotionData: hclSimClothDataTransferMotionData<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSimClothData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSimClothData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe6105187)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_simulationInfo.deps_indexes());
            v.extend(
                self
                    .m_particleDatas
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.m_collidableTransformMap.deps_indexes());
            v.extend(self.m_perInstanceCollidables.iter());
            v.extend(self.m_staticConstraintSets.iter());
            v.extend(self.m_antiPinchConstraintSets.iter());
            v.extend(self.m_simClothPoses.iter());
            v.extend(self.m_actions.iter());
            v.extend(
                self
                    .m_collidablePinchingDatas
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.m_landscapeCollisionData.deps_indexes());
            v.extend(self.m_transferMotionData.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hclSimClothData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xe6105187)));
            let mut serializer = __serializer
                .serialize_struct("hclSimClothData", class_meta, (0u64, 384u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("simulationInfo", &self.m_simulationInfo)?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer
                .serialize_array_field(
                    "particleDatas",
                    &self.m_particleDatas,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "fixedParticles",
                    &self.m_fixedParticles,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "triangleIndices",
                    &self.m_triangleIndices,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "triangleFlips",
                    &self.m_triangleFlips,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("totalMass", &self.m_totalMass)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "collidableTransformMap",
                    &self.m_collidableTransformMap,
                )?;
            serializer
                .serialize_array_field(
                    "perInstanceCollidables",
                    &self.m_perInstanceCollidables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "staticConstraintSets",
                    &self.m_staticConstraintSets,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "antiPinchConstraintSets",
                    &self.m_antiPinchConstraintSets,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "simClothPoses",
                    &self.m_simClothPoses,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field("actions", &self.m_actions, TypeSize::NonPtr)?;
            serializer
                .serialize_array_field(
                    "staticCollisionMasks",
                    &self.m_staticCollisionMasks,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "perParticlePinchDetectionEnabledFlags",
                    &self.m_perParticlePinchDetectionEnabledFlags,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "collidablePinchingDatas",
                    &self.m_collidablePinchingDatas,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 8u64,
                    },
                )?;
            serializer
                .serialize_field(
                    "minPinchedParticleIndex",
                    &self.m_minPinchedParticleIndex,
                )?;
            serializer
                .serialize_field(
                    "maxPinchedParticleIndex",
                    &self.m_maxPinchedParticleIndex,
                )?;
            serializer.serialize_field("maxCollisionPairs", &self.m_maxCollisionPairs)?;
            serializer.serialize_field("maxParticleRadius", &self.m_maxParticleRadius)?;
            serializer
                .serialize_field(
                    "landscapeCollisionData",
                    &self.m_landscapeCollisionData,
                )?;
            serializer
                .serialize_field(
                    "numLandscapeCollidableParticles",
                    &self.m_numLandscapeCollidableParticles,
                )?;
            serializer.serialize_field("doNormals", &self.m_doNormals)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("transferMotionData", &self.m_transferMotionData)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclSimClothData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_simulationInfo,
                m_name,
                m_particleDatas,
                m_fixedParticles,
                m_triangleIndices,
                m_triangleFlips,
                m_totalMass,
                m_collidableTransformMap,
                m_perInstanceCollidables,
                m_staticConstraintSets,
                m_antiPinchConstraintSets,
                m_simClothPoses,
                m_actions,
                m_staticCollisionMasks,
                m_perParticlePinchDetectionEnabledFlags,
                m_collidablePinchingDatas,
                m_minPinchedParticleIndex,
                m_maxPinchedParticleIndex,
                m_maxCollisionPairs,
                m_maxParticleRadius,
                m_landscapeCollisionData,
                m_numLandscapeCollidableParticles,
                m_doNormals,
                m_transferMotionData,
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
                        "simulationInfo" => Ok(__Field::m_simulationInfo),
                        "name" => Ok(__Field::m_name),
                        "particleDatas" => Ok(__Field::m_particleDatas),
                        "fixedParticles" => Ok(__Field::m_fixedParticles),
                        "triangleIndices" => Ok(__Field::m_triangleIndices),
                        "triangleFlips" => Ok(__Field::m_triangleFlips),
                        "totalMass" => Ok(__Field::m_totalMass),
                        "collidableTransformMap" => Ok(__Field::m_collidableTransformMap),
                        "perInstanceCollidables" => Ok(__Field::m_perInstanceCollidables),
                        "staticConstraintSets" => Ok(__Field::m_staticConstraintSets),
                        "antiPinchConstraintSets" => {
                            Ok(__Field::m_antiPinchConstraintSets)
                        }
                        "simClothPoses" => Ok(__Field::m_simClothPoses),
                        "actions" => Ok(__Field::m_actions),
                        "staticCollisionMasks" => Ok(__Field::m_staticCollisionMasks),
                        "perParticlePinchDetectionEnabledFlags" => {
                            Ok(__Field::m_perParticlePinchDetectionEnabledFlags)
                        }
                        "collidablePinchingDatas" => {
                            Ok(__Field::m_collidablePinchingDatas)
                        }
                        "minPinchedParticleIndex" => {
                            Ok(__Field::m_minPinchedParticleIndex)
                        }
                        "maxPinchedParticleIndex" => {
                            Ok(__Field::m_maxPinchedParticleIndex)
                        }
                        "maxCollisionPairs" => Ok(__Field::m_maxCollisionPairs),
                        "maxParticleRadius" => Ok(__Field::m_maxParticleRadius),
                        "landscapeCollisionData" => Ok(__Field::m_landscapeCollisionData),
                        "numLandscapeCollidableParticles" => {
                            Ok(__Field::m_numLandscapeCollidableParticles)
                        }
                        "doNormals" => Ok(__Field::m_doNormals),
                        "transferMotionData" => Ok(__Field::m_transferMotionData),
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
            struct __hclSimClothDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hclSimClothData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclSimClothDataVisitor<'de> {
                type Value = hclSimClothData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSimClothData",
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
                    let mut m_simulationInfo: _serde::__private::Option<
                        hclSimClothDataOverridableSimulationInfo<'de>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_particleDatas: _serde::__private::Option<
                        Vec<hclSimClothDataParticleData>,
                    > = _serde::__private::None;
                    let mut m_fixedParticles: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_triangleIndices: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_triangleFlips: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_totalMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collidableTransformMap: _serde::__private::Option<
                        hclSimClothDataCollidableTransformMap<'de>,
                    > = _serde::__private::None;
                    let mut m_perInstanceCollidables: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_staticConstraintSets: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_antiPinchConstraintSets: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_simClothPoses: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_actions: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_staticCollisionMasks: _serde::__private::Option<
                        Vec<U32<'de>>,
                    > = _serde::__private::None;
                    let mut m_perParticlePinchDetectionEnabledFlags: _serde::__private::Option<
                        Vec<bool>,
                    > = _serde::__private::None;
                    let mut m_collidablePinchingDatas: _serde::__private::Option<
                        Vec<hclSimClothDataCollidablePinchingData<'de>>,
                    > = _serde::__private::None;
                    let mut m_minPinchedParticleIndex: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_maxPinchedParticleIndex: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_maxCollisionPairs: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_maxParticleRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_landscapeCollisionData: _serde::__private::Option<
                        hclSimClothDataLandscapeCollisionData<'de>,
                    > = _serde::__private::None;
                    let mut m_numLandscapeCollidableParticles: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_doNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_transferMotionData: _serde::__private::Option<
                        hclSimClothDataTransferMotionData<'de>,
                    > = _serde::__private::None;
                    for i in 0..24usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_simulationInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationInfo",
                                        ),
                                    );
                                }
                                m_simulationInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataOverridableSimulationInfo<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
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
                            2usize => {
                                if _serde::__private::Option::is_some(&m_particleDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleDatas",
                                        ),
                                    );
                                }
                                m_particleDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclSimClothDataParticleData>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_fixedParticles) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedParticles",
                                        ),
                                    );
                                }
                                m_fixedParticles = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_triangleIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleIndices",
                                        ),
                                    );
                                }
                                m_triangleIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_triangleFlips) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleFlips",
                                        ),
                                    );
                                }
                                m_triangleFlips = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_totalMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "totalMass",
                                        ),
                                    );
                                }
                                m_totalMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collidableTransformMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidableTransformMap",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_collidableTransformMap = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataCollidableTransformMap<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_perInstanceCollidables,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "perInstanceCollidables",
                                        ),
                                    );
                                }
                                m_perInstanceCollidables = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_staticConstraintSets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticConstraintSets",
                                        ),
                                    );
                                }
                                m_staticConstraintSets = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_antiPinchConstraintSets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "antiPinchConstraintSets",
                                        ),
                                    );
                                }
                                m_antiPinchConstraintSets = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_simClothPoses) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simClothPoses",
                                        ),
                                    );
                                }
                                m_simClothPoses = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_actions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "actions",
                                        ),
                                    );
                                }
                                m_actions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_staticCollisionMasks,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticCollisionMasks",
                                        ),
                                    );
                                }
                                m_staticCollisionMasks = _serde::__private::Some(
                                    match __A::next_value::<Vec<U32<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_perParticlePinchDetectionEnabledFlags,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "perParticlePinchDetectionEnabledFlags",
                                        ),
                                    );
                                }
                                m_perParticlePinchDetectionEnabledFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<bool>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collidablePinchingDatas,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidablePinchingDatas",
                                        ),
                                    );
                                }
                                m_collidablePinchingDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclSimClothDataCollidablePinchingData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minPinchedParticleIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minPinchedParticleIndex",
                                        ),
                                    );
                                }
                                m_minPinchedParticleIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxPinchedParticleIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxPinchedParticleIndex",
                                        ),
                                    );
                                }
                                m_maxPinchedParticleIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxCollisionPairs,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCollisionPairs",
                                        ),
                                    );
                                }
                                m_maxCollisionPairs = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxParticleRadius,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxParticleRadius",
                                        ),
                                    );
                                }
                                m_maxParticleRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionData",
                                        ),
                                    );
                                }
                                m_landscapeCollisionData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataLandscapeCollisionData<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numLandscapeCollidableParticles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numLandscapeCollidableParticles",
                                        ),
                                    );
                                }
                                m_numLandscapeCollidableParticles = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_doNormals) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "doNormals",
                                        ),
                                    );
                                }
                                m_doNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionData",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_transferMotionData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataTransferMotionData<'de>,
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
                    let m_simulationInfo = match m_simulationInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationInfo",
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
                    let m_particleDatas = match m_particleDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleDatas",
                                ),
                            );
                        }
                    };
                    let m_fixedParticles = match m_fixedParticles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedParticles",
                                ),
                            );
                        }
                    };
                    let m_triangleIndices = match m_triangleIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleIndices",
                                ),
                            );
                        }
                    };
                    let m_triangleFlips = match m_triangleFlips {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleFlips",
                                ),
                            );
                        }
                    };
                    let m_totalMass = match m_totalMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "totalMass",
                                ),
                            );
                        }
                    };
                    let m_collidableTransformMap = match m_collidableTransformMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidableTransformMap",
                                ),
                            );
                        }
                    };
                    let m_perInstanceCollidables = match m_perInstanceCollidables {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "perInstanceCollidables",
                                ),
                            );
                        }
                    };
                    let m_staticConstraintSets = match m_staticConstraintSets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticConstraintSets",
                                ),
                            );
                        }
                    };
                    let m_antiPinchConstraintSets = match m_antiPinchConstraintSets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "antiPinchConstraintSets",
                                ),
                            );
                        }
                    };
                    let m_simClothPoses = match m_simClothPoses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simClothPoses",
                                ),
                            );
                        }
                    };
                    let m_actions = match m_actions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("actions"),
                            );
                        }
                    };
                    let m_staticCollisionMasks = match m_staticCollisionMasks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticCollisionMasks",
                                ),
                            );
                        }
                    };
                    let m_perParticlePinchDetectionEnabledFlags = match m_perParticlePinchDetectionEnabledFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "perParticlePinchDetectionEnabledFlags",
                                ),
                            );
                        }
                    };
                    let m_collidablePinchingDatas = match m_collidablePinchingDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidablePinchingDatas",
                                ),
                            );
                        }
                    };
                    let m_minPinchedParticleIndex = match m_minPinchedParticleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minPinchedParticleIndex",
                                ),
                            );
                        }
                    };
                    let m_maxPinchedParticleIndex = match m_maxPinchedParticleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxPinchedParticleIndex",
                                ),
                            );
                        }
                    };
                    let m_maxCollisionPairs = match m_maxCollisionPairs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCollisionPairs",
                                ),
                            );
                        }
                    };
                    let m_maxParticleRadius = match m_maxParticleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxParticleRadius",
                                ),
                            );
                        }
                    };
                    let m_landscapeCollisionData = match m_landscapeCollisionData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionData",
                                ),
                            );
                        }
                    };
                    let m_numLandscapeCollidableParticles = match m_numLandscapeCollidableParticles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numLandscapeCollidableParticles",
                                ),
                            );
                        }
                    };
                    let m_doNormals = match m_doNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "doNormals",
                                ),
                            );
                        }
                    };
                    let m_transferMotionData = match m_transferMotionData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionData",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSimClothData {
                        __ptr,
                        parent,
                        m_simulationInfo,
                        m_name,
                        m_particleDatas,
                        m_fixedParticles,
                        m_triangleIndices,
                        m_triangleFlips,
                        m_totalMass,
                        m_collidableTransformMap,
                        m_perInstanceCollidables,
                        m_staticConstraintSets,
                        m_antiPinchConstraintSets,
                        m_simClothPoses,
                        m_actions,
                        m_staticCollisionMasks,
                        m_perParticlePinchDetectionEnabledFlags,
                        m_collidablePinchingDatas,
                        m_minPinchedParticleIndex,
                        m_maxPinchedParticleIndex,
                        m_maxCollisionPairs,
                        m_maxParticleRadius,
                        m_landscapeCollisionData,
                        m_numLandscapeCollidableParticles,
                        m_doNormals,
                        m_transferMotionData,
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
                    let mut m_simulationInfo: _serde::__private::Option<
                        hclSimClothDataOverridableSimulationInfo<'de>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_particleDatas: _serde::__private::Option<
                        Vec<hclSimClothDataParticleData>,
                    > = _serde::__private::None;
                    let mut m_fixedParticles: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_triangleIndices: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_triangleFlips: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_totalMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collidableTransformMap: _serde::__private::Option<
                        hclSimClothDataCollidableTransformMap<'de>,
                    > = _serde::__private::None;
                    let mut m_perInstanceCollidables: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_staticConstraintSets: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_antiPinchConstraintSets: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_simClothPoses: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_actions: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_staticCollisionMasks: _serde::__private::Option<
                        Vec<U32<'de>>,
                    > = _serde::__private::None;
                    let mut m_perParticlePinchDetectionEnabledFlags: _serde::__private::Option<
                        Vec<bool>,
                    > = _serde::__private::None;
                    let mut m_collidablePinchingDatas: _serde::__private::Option<
                        Vec<hclSimClothDataCollidablePinchingData<'de>>,
                    > = _serde::__private::None;
                    let mut m_minPinchedParticleIndex: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_maxPinchedParticleIndex: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_maxCollisionPairs: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_maxParticleRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_landscapeCollisionData: _serde::__private::Option<
                        hclSimClothDataLandscapeCollisionData<'de>,
                    > = _serde::__private::None;
                    let mut m_numLandscapeCollidableParticles: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_doNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_transferMotionData: _serde::__private::Option<
                        hclSimClothDataTransferMotionData<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_simulationInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simulationInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationInfo",
                                        ),
                                    );
                                }
                                m_simulationInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataOverridableSimulationInfo<'de>,
                                    >(&mut __map) {
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
                            __Field::m_particleDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleDatas",
                                        ),
                                    );
                                }
                                m_particleDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclSimClothDataParticleData>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fixedParticles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fixedParticles) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedParticles",
                                        ),
                                    );
                                }
                                m_fixedParticles = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triangleIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_triangleIndices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleIndices",
                                        ),
                                    );
                                }
                                m_triangleIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triangleFlips => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_triangleFlips) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleFlips",
                                        ),
                                    );
                                }
                                m_triangleFlips = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_totalMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_totalMass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "totalMass",
                                        ),
                                    );
                                }
                                m_totalMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collidableTransformMap => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collidableTransformMap,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidableTransformMap",
                                        ),
                                    );
                                }
                                m_collidableTransformMap = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataCollidableTransformMap<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_perInstanceCollidables => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_perInstanceCollidables,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "perInstanceCollidables",
                                        ),
                                    );
                                }
                                m_perInstanceCollidables = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticConstraintSets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticConstraintSets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticConstraintSets",
                                        ),
                                    );
                                }
                                m_staticConstraintSets = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_antiPinchConstraintSets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_antiPinchConstraintSets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "antiPinchConstraintSets",
                                        ),
                                    );
                                }
                                m_antiPinchConstraintSets = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simClothPoses => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simClothPoses) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simClothPoses",
                                        ),
                                    );
                                }
                                m_simClothPoses = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_actions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_actions) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "actions",
                                        ),
                                    );
                                }
                                m_actions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticCollisionMasks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticCollisionMasks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticCollisionMasks",
                                        ),
                                    );
                                }
                                m_staticCollisionMasks = _serde::__private::Some(
                                    match __A::next_value::<Vec<U32<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_perParticlePinchDetectionEnabledFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_perParticlePinchDetectionEnabledFlags,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "perParticlePinchDetectionEnabledFlags",
                                        ),
                                    );
                                }
                                m_perParticlePinchDetectionEnabledFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<bool>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collidablePinchingDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collidablePinchingDatas,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidablePinchingDatas",
                                        ),
                                    );
                                }
                                m_collidablePinchingDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclSimClothDataCollidablePinchingData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minPinchedParticleIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minPinchedParticleIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minPinchedParticleIndex",
                                        ),
                                    );
                                }
                                m_minPinchedParticleIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxPinchedParticleIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxPinchedParticleIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxPinchedParticleIndex",
                                        ),
                                    );
                                }
                                m_maxPinchedParticleIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxCollisionPairs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxCollisionPairs,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCollisionPairs",
                                        ),
                                    );
                                }
                                m_maxCollisionPairs = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxParticleRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxParticleRadius,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxParticleRadius",
                                        ),
                                    );
                                }
                                m_maxParticleRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_landscapeCollisionData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionData,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionData",
                                        ),
                                    );
                                }
                                m_landscapeCollisionData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataLandscapeCollisionData<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numLandscapeCollidableParticles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numLandscapeCollidableParticles,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numLandscapeCollidableParticles",
                                        ),
                                    );
                                }
                                m_numLandscapeCollidableParticles = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_doNormals => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_doNormals) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "doNormals",
                                        ),
                                    );
                                }
                                m_doNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transferMotionData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionData,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionData",
                                        ),
                                    );
                                }
                                m_transferMotionData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothDataTransferMotionData<'de>,
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
                    let m_simulationInfo = match m_simulationInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationInfo",
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
                    let m_particleDatas = match m_particleDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fixedParticles = match m_fixedParticles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedParticles",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_triangleIndices = match m_triangleIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_triangleFlips = match m_triangleFlips {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleFlips",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_totalMass = match m_totalMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "totalMass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collidableTransformMap = match m_collidableTransformMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidableTransformMap",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_perInstanceCollidables = match m_perInstanceCollidables {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "perInstanceCollidables",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticConstraintSets = match m_staticConstraintSets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticConstraintSets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_antiPinchConstraintSets = match m_antiPinchConstraintSets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "antiPinchConstraintSets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simClothPoses = match m_simClothPoses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simClothPoses",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_actions = match m_actions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("actions"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticCollisionMasks = match m_staticCollisionMasks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticCollisionMasks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_perParticlePinchDetectionEnabledFlags = match m_perParticlePinchDetectionEnabledFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "perParticlePinchDetectionEnabledFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collidablePinchingDatas = match m_collidablePinchingDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidablePinchingDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minPinchedParticleIndex = match m_minPinchedParticleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minPinchedParticleIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxPinchedParticleIndex = match m_maxPinchedParticleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxPinchedParticleIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxCollisionPairs = match m_maxCollisionPairs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCollisionPairs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxParticleRadius = match m_maxParticleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxParticleRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_landscapeCollisionData = match m_landscapeCollisionData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numLandscapeCollidableParticles = match m_numLandscapeCollidableParticles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numLandscapeCollidableParticles",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_doNormals = match m_doNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "doNormals",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transferMotionData = match m_transferMotionData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionData",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclSimClothData {
                        __ptr: __ptr.clone(),
                        parent,
                        m_simulationInfo,
                        m_name,
                        m_particleDatas,
                        m_fixedParticles,
                        m_triangleIndices,
                        m_triangleFlips,
                        m_totalMass,
                        m_collidableTransformMap,
                        m_perInstanceCollidables,
                        m_staticConstraintSets,
                        m_antiPinchConstraintSets,
                        m_simClothPoses,
                        m_actions,
                        m_staticCollisionMasks,
                        m_perParticlePinchDetectionEnabledFlags,
                        m_collidablePinchingDatas,
                        m_minPinchedParticleIndex,
                        m_maxPinchedParticleIndex,
                        m_maxCollisionPairs,
                        m_maxParticleRadius,
                        m_landscapeCollisionData,
                        m_numLandscapeCollidableParticles,
                        m_doNormals,
                        m_transferMotionData,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "simulationInfo",
                "name",
                "particleDatas",
                "fixedParticles",
                "triangleIndices",
                "triangleFlips",
                "totalMass",
                "collidableTransformMap",
                "perInstanceCollidables",
                "staticConstraintSets",
                "antiPinchConstraintSets",
                "simClothPoses",
                "actions",
                "staticCollisionMasks",
                "perParticlePinchDetectionEnabledFlags",
                "collidablePinchingDatas",
                "minPinchedParticleIndex",
                "maxPinchedParticleIndex",
                "maxCollisionPairs",
                "maxParticleRadius",
                "landscapeCollisionData",
                "numLandscapeCollidableParticles",
                "doNormals",
                "transferMotionData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSimClothData",
                FIELDS,
                __hclSimClothDataVisitor {
                    marker: _serde::__private::PhantomData::<hclSimClothData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
