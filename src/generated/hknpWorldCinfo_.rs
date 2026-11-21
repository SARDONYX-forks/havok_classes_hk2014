use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpWorldCinfo`
/// - version: `5`
/// - signature: `0x6300cf42`
/// - size: `  0`(x86)/`256`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpWorldCinfo<'a> {
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
    /// - name: `bodyBufferCapacity`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bodyBufferCapacity"))]
    #[cfg_attr(feature = "serde", serde(rename = "bodyBufferCapacity"))]
    pub m_bodyBufferCapacity: I32<'a>,
    /// # C++ Info
    /// - name: `userBodyBuffer`(ctype: `void*`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userBodyBuffer"))]
    #[cfg_attr(feature = "serde", serde(rename = "userBodyBuffer"))]
    pub m_userBodyBuffer: Pointer<'a>,
    /// # C++ Info
    /// - name: `motionBufferCapacity`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionBufferCapacity"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionBufferCapacity"))]
    pub m_motionBufferCapacity: I32<'a>,
    /// # C++ Info
    /// - name: `userMotionBuffer`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userMotionBuffer"))]
    #[cfg_attr(feature = "serde", serde(rename = "userMotionBuffer"))]
    pub m_userMotionBuffer: Pointer<'a>,
    /// # C++ Info
    /// - name: `constraintBufferCapacity`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "constraintBufferCapacity"))]
    #[cfg_attr(feature = "serde", serde(rename = "constraintBufferCapacity"))]
    pub m_constraintBufferCapacity: I32<'a>,
    /// # C++ Info
    /// - name: `userConstraintBuffer`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userConstraintBuffer"))]
    #[cfg_attr(feature = "serde", serde(rename = "userConstraintBuffer"))]
    pub m_userConstraintBuffer: Pointer<'a>,
    /// # C++ Info
    /// - name: `persistentStreamAllocator`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "persistentStreamAllocator"))]
    #[cfg_attr(feature = "serde", serde(rename = "persistentStreamAllocator"))]
    pub m_persistentStreamAllocator: Pointer<'a>,
    /// # C++ Info
    /// - name: `materialLibrary`(ctype: `struct hknpMaterialLibrary*`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialLibrary"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialLibrary"))]
    pub m_materialLibrary: Pointer<'a>,
    /// # C++ Info
    /// - name: `motionPropertiesLibrary`(ctype: `struct hknpMotionPropertiesLibrary*`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionPropertiesLibrary"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionPropertiesLibrary"))]
    pub m_motionPropertiesLibrary: Pointer<'a>,
    /// # C++ Info
    /// - name: `qualityLibrary`(ctype: `struct hknpBodyQualityLibrary*`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "qualityLibrary"))]
    #[cfg_attr(feature = "serde", serde(rename = "qualityLibrary"))]
    pub m_qualityLibrary: Pointer<'a>,
    /// # C++ Info
    /// - name: `simulationType`(ctype: `enum SimulationType`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simulationType"))]
    #[cfg_attr(feature = "serde", serde(rename = "simulationType"))]
    pub m_simulationType: SimulationType,
    /// # C++ Info
    /// - name: `numSplitterCells`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 84`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numSplitterCells"))]
    #[cfg_attr(feature = "serde", serde(rename = "numSplitterCells"))]
    pub m_numSplitterCells: I32<'a>,
    /// # C++ Info
    /// - name: `gravity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gravity"))]
    #[cfg_attr(feature = "serde", serde(rename = "gravity"))]
    pub m_gravity: Vector4,
    /// # C++ Info
    /// - name: `enableContactCaching`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "enableContactCaching"))]
    #[cfg_attr(feature = "serde", serde(rename = "enableContactCaching"))]
    pub m_enableContactCaching: bool,
    /// # C++ Info
    /// - name: `mergeEventsBeforeDispatch`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`113`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "mergeEventsBeforeDispatch"))]
    #[cfg_attr(feature = "serde", serde(rename = "mergeEventsBeforeDispatch"))]
    pub m_mergeEventsBeforeDispatch: bool,
    /// # C++ Info
    /// - name: `leavingBroadPhaseBehavior`(ctype: `enum LeavingBroadPhaseBehavior`)
    /// - offset: `  0`(x86)/`114`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "leavingBroadPhaseBehavior"))]
    #[cfg_attr(feature = "serde", serde(rename = "leavingBroadPhaseBehavior"))]
    pub m_leavingBroadPhaseBehavior: LeavingBroadPhaseBehavior,
    /// # C++ Info
    /// - name: `broadPhaseAabb`(ctype: `struct hkAabb`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "broadPhaseAabb"))]
    #[cfg_attr(feature = "serde", serde(rename = "broadPhaseAabb"))]
    pub m_broadPhaseAabb: hkAabb<'a>,
    /// # C++ Info
    /// - name: `broadPhaseConfig`(ctype: `struct hknpBroadPhaseConfig*`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "broadPhaseConfig"))]
    #[cfg_attr(feature = "serde", serde(rename = "broadPhaseConfig"))]
    pub m_broadPhaseConfig: Pointer<'a>,
    /// # C++ Info
    /// - name: `collisionFilter`(ctype: `struct hknpCollisionFilter*`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionFilter"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionFilter"))]
    pub m_collisionFilter: Pointer<'a>,
    /// # C++ Info
    /// - name: `shapeTagCodec`(ctype: `struct hknpShapeTagCodec*`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "shapeTagCodec"))]
    #[cfg_attr(feature = "serde", serde(rename = "shapeTagCodec"))]
    pub m_shapeTagCodec: Pointer<'a>,
    /// # C++ Info
    /// - name: `collisionTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionTolerance"))]
    pub m_collisionTolerance: f32,
    /// # C++ Info
    /// - name: `relativeCollisionAccuracy`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`188`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "relativeCollisionAccuracy"))]
    #[cfg_attr(feature = "serde", serde(rename = "relativeCollisionAccuracy"))]
    pub m_relativeCollisionAccuracy: f32,
    /// # C++ Info
    /// - name: `enableWeldingForDefaultObjects`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "enableWeldingForDefaultObjects")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "enableWeldingForDefaultObjects"))]
    pub m_enableWeldingForDefaultObjects: bool,
    /// # C++ Info
    /// - name: `enableWeldingForCriticalObjects`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`193`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "enableWeldingForCriticalObjects")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "enableWeldingForCriticalObjects"))]
    pub m_enableWeldingForCriticalObjects: bool,
    /// # C++ Info
    /// - name: `solverTau`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`196`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "solverTau"))]
    #[cfg_attr(feature = "serde", serde(rename = "solverTau"))]
    pub m_solverTau: f32,
    /// # C++ Info
    /// - name: `solverDamp`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "solverDamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "solverDamp"))]
    pub m_solverDamp: f32,
    /// # C++ Info
    /// - name: `solverIterations`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`204`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "solverIterations"))]
    #[cfg_attr(feature = "serde", serde(rename = "solverIterations"))]
    pub m_solverIterations: I32<'a>,
    /// # C++ Info
    /// - name: `solverMicrosteps`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "solverMicrosteps"))]
    #[cfg_attr(feature = "serde", serde(rename = "solverMicrosteps"))]
    pub m_solverMicrosteps: I32<'a>,
    /// # C++ Info
    /// - name: `defaultSolverTimestep`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`212`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultSolverTimestep"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultSolverTimestep"))]
    pub m_defaultSolverTimestep: f32,
    /// # C++ Info
    /// - name: `maxApproachSpeedForHighQualitySolver`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "maxApproachSpeedForHighQualitySolver")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "maxApproachSpeedForHighQualitySolver")
    )]
    pub m_maxApproachSpeedForHighQualitySolver: f32,
    /// # C++ Info
    /// - name: `enableDeactivation`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`220`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "enableDeactivation"))]
    #[cfg_attr(feature = "serde", serde(rename = "enableDeactivation"))]
    pub m_enableDeactivation: bool,
    /// # C++ Info
    /// - name: `deleteCachesOnDeactivation`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`221`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "deleteCachesOnDeactivation"))]
    #[cfg_attr(feature = "serde", serde(rename = "deleteCachesOnDeactivation"))]
    pub m_deleteCachesOnDeactivation: bool,
    /// # C++ Info
    /// - name: `largeIslandSize`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "largeIslandSize"))]
    #[cfg_attr(feature = "serde", serde(rename = "largeIslandSize"))]
    pub m_largeIslandSize: I32<'a>,
    /// # C++ Info
    /// - name: `enableSolverDynamicScheduling`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`228`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "enableSolverDynamicScheduling")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "enableSolverDynamicScheduling"))]
    pub m_enableSolverDynamicScheduling: bool,
    /// # C++ Info
    /// - name: `contactSolverType`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "contactSolverType"))]
    #[cfg_attr(feature = "serde", serde(rename = "contactSolverType"))]
    pub m_contactSolverType: I32<'a>,
    /// # C++ Info
    /// - name: `unitScale`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`236`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "unitScale"))]
    #[cfg_attr(feature = "serde", serde(rename = "unitScale"))]
    pub m_unitScale: f32,
    /// # C++ Info
    /// - name: `applyUnitScaleToStaticConstants`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "applyUnitScaleToStaticConstants")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "applyUnitScaleToStaticConstants"))]
    pub m_applyUnitScaleToStaticConstants: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpWorldCinfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpWorldCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6300cf42)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_userBodyBuffer);
            v.push(&self.m_userMotionBuffer);
            v.push(&self.m_userConstraintBuffer);
            v.push(&self.m_persistentStreamAllocator);
            v.push(&self.m_materialLibrary);
            v.push(&self.m_motionPropertiesLibrary);
            v.push(&self.m_qualityLibrary);
            v.extend(self.m_broadPhaseAabb.deps_indexes());
            v.push(&self.m_broadPhaseConfig);
            v.push(&self.m_collisionFilter);
            v.push(&self.m_shapeTagCodec);
            v
        }
    }
    impl<'a> _serde::Serialize for hknpWorldCinfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x6300cf42)));
            let mut serializer = __serializer
                .serialize_struct("hknpWorldCinfo", class_meta, (0u64, 256u64))?;
            serializer
                .serialize_field("bodyBufferCapacity", &self.m_bodyBufferCapacity)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("userBodyBuffer", &self.m_userBodyBuffer)?;
            serializer
                .serialize_field("motionBufferCapacity", &self.m_motionBufferCapacity)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("userMotionBuffer", &self.m_userMotionBuffer)?;
            serializer
                .serialize_field(
                    "constraintBufferCapacity",
                    &self.m_constraintBufferCapacity,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("userConstraintBuffer", &self.m_userConstraintBuffer)?;
            serializer
                .skip_field(
                    "persistentStreamAllocator",
                    &self.m_persistentStreamAllocator,
                )?;
            serializer.serialize_field("materialLibrary", &self.m_materialLibrary)?;
            serializer
                .serialize_field(
                    "motionPropertiesLibrary",
                    &self.m_motionPropertiesLibrary,
                )?;
            serializer.serialize_field("qualityLibrary", &self.m_qualityLibrary)?;
            serializer.serialize_field("simulationType", &self.m_simulationType)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("numSplitterCells", &self.m_numSplitterCells)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer
                .serialize_field("enableContactCaching", &self.m_enableContactCaching)?;
            serializer
                .serialize_field(
                    "mergeEventsBeforeDispatch",
                    &self.m_mergeEventsBeforeDispatch,
                )?;
            serializer
                .serialize_field(
                    "leavingBroadPhaseBehavior",
                    &self.m_leavingBroadPhaseBehavior,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 13usize].as_slice())?;
            serializer.serialize_field("broadPhaseAabb", &self.m_broadPhaseAabb)?;
            serializer.serialize_field("broadPhaseConfig", &self.m_broadPhaseConfig)?;
            serializer.serialize_field("collisionFilter", &self.m_collisionFilter)?;
            serializer.serialize_field("shapeTagCodec", &self.m_shapeTagCodec)?;
            serializer
                .serialize_field("collisionTolerance", &self.m_collisionTolerance)?;
            serializer
                .serialize_field(
                    "relativeCollisionAccuracy",
                    &self.m_relativeCollisionAccuracy,
                )?;
            serializer
                .serialize_field(
                    "enableWeldingForDefaultObjects",
                    &self.m_enableWeldingForDefaultObjects,
                )?;
            serializer
                .serialize_field(
                    "enableWeldingForCriticalObjects",
                    &self.m_enableWeldingForCriticalObjects,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("solverTau", &self.m_solverTau)?;
            serializer.serialize_field("solverDamp", &self.m_solverDamp)?;
            serializer.serialize_field("solverIterations", &self.m_solverIterations)?;
            serializer.serialize_field("solverMicrosteps", &self.m_solverMicrosteps)?;
            serializer
                .serialize_field(
                    "defaultSolverTimestep",
                    &self.m_defaultSolverTimestep,
                )?;
            serializer
                .serialize_field(
                    "maxApproachSpeedForHighQualitySolver",
                    &self.m_maxApproachSpeedForHighQualitySolver,
                )?;
            serializer
                .serialize_field("enableDeactivation", &self.m_enableDeactivation)?;
            serializer
                .serialize_field(
                    "deleteCachesOnDeactivation",
                    &self.m_deleteCachesOnDeactivation,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("largeIslandSize", &self.m_largeIslandSize)?;
            serializer
                .serialize_field(
                    "enableSolverDynamicScheduling",
                    &self.m_enableSolverDynamicScheduling,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("contactSolverType", &self.m_contactSolverType)?;
            serializer.serialize_field("unitScale", &self.m_unitScale)?;
            serializer
                .serialize_field(
                    "applyUnitScaleToStaticConstants",
                    &self.m_applyUnitScaleToStaticConstants,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpWorldCinfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_bodyBufferCapacity,
                m_motionBufferCapacity,
                m_constraintBufferCapacity,
                m_materialLibrary,
                m_motionPropertiesLibrary,
                m_qualityLibrary,
                m_simulationType,
                m_numSplitterCells,
                m_gravity,
                m_enableContactCaching,
                m_mergeEventsBeforeDispatch,
                m_leavingBroadPhaseBehavior,
                m_broadPhaseAabb,
                m_broadPhaseConfig,
                m_collisionFilter,
                m_shapeTagCodec,
                m_collisionTolerance,
                m_relativeCollisionAccuracy,
                m_enableWeldingForDefaultObjects,
                m_enableWeldingForCriticalObjects,
                m_solverTau,
                m_solverDamp,
                m_solverIterations,
                m_solverMicrosteps,
                m_defaultSolverTimestep,
                m_maxApproachSpeedForHighQualitySolver,
                m_enableDeactivation,
                m_deleteCachesOnDeactivation,
                m_largeIslandSize,
                m_enableSolverDynamicScheduling,
                m_contactSolverType,
                m_unitScale,
                m_applyUnitScaleToStaticConstants,
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
                        "bodyBufferCapacity" => Ok(__Field::m_bodyBufferCapacity),
                        "motionBufferCapacity" => Ok(__Field::m_motionBufferCapacity),
                        "constraintBufferCapacity" => {
                            Ok(__Field::m_constraintBufferCapacity)
                        }
                        "materialLibrary" => Ok(__Field::m_materialLibrary),
                        "motionPropertiesLibrary" => {
                            Ok(__Field::m_motionPropertiesLibrary)
                        }
                        "qualityLibrary" => Ok(__Field::m_qualityLibrary),
                        "simulationType" => Ok(__Field::m_simulationType),
                        "numSplitterCells" => Ok(__Field::m_numSplitterCells),
                        "gravity" => Ok(__Field::m_gravity),
                        "enableContactCaching" => Ok(__Field::m_enableContactCaching),
                        "mergeEventsBeforeDispatch" => {
                            Ok(__Field::m_mergeEventsBeforeDispatch)
                        }
                        "leavingBroadPhaseBehavior" => {
                            Ok(__Field::m_leavingBroadPhaseBehavior)
                        }
                        "broadPhaseAabb" => Ok(__Field::m_broadPhaseAabb),
                        "broadPhaseConfig" => Ok(__Field::m_broadPhaseConfig),
                        "collisionFilter" => Ok(__Field::m_collisionFilter),
                        "shapeTagCodec" => Ok(__Field::m_shapeTagCodec),
                        "collisionTolerance" => Ok(__Field::m_collisionTolerance),
                        "relativeCollisionAccuracy" => {
                            Ok(__Field::m_relativeCollisionAccuracy)
                        }
                        "enableWeldingForDefaultObjects" => {
                            Ok(__Field::m_enableWeldingForDefaultObjects)
                        }
                        "enableWeldingForCriticalObjects" => {
                            Ok(__Field::m_enableWeldingForCriticalObjects)
                        }
                        "solverTau" => Ok(__Field::m_solverTau),
                        "solverDamp" => Ok(__Field::m_solverDamp),
                        "solverIterations" => Ok(__Field::m_solverIterations),
                        "solverMicrosteps" => Ok(__Field::m_solverMicrosteps),
                        "defaultSolverTimestep" => Ok(__Field::m_defaultSolverTimestep),
                        "maxApproachSpeedForHighQualitySolver" => {
                            Ok(__Field::m_maxApproachSpeedForHighQualitySolver)
                        }
                        "enableDeactivation" => Ok(__Field::m_enableDeactivation),
                        "deleteCachesOnDeactivation" => {
                            Ok(__Field::m_deleteCachesOnDeactivation)
                        }
                        "largeIslandSize" => Ok(__Field::m_largeIslandSize),
                        "enableSolverDynamicScheduling" => {
                            Ok(__Field::m_enableSolverDynamicScheduling)
                        }
                        "contactSolverType" => Ok(__Field::m_contactSolverType),
                        "unitScale" => Ok(__Field::m_unitScale),
                        "applyUnitScaleToStaticConstants" => {
                            Ok(__Field::m_applyUnitScaleToStaticConstants)
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
            struct __hknpWorldCinfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpWorldCinfo<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpWorldCinfoVisitor<'de> {
                type Value = hknpWorldCinfo<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpWorldCinfo")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_bodyBufferCapacity: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_userBodyBuffer: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_motionBufferCapacity: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_userMotionBuffer: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_constraintBufferCapacity: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_userConstraintBuffer: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_persistentStreamAllocator: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_materialLibrary: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_motionPropertiesLibrary: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_qualityLibrary: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_simulationType: _serde::__private::Option<
                        SimulationType,
                    > = _serde::__private::None;
                    let mut m_numSplitterCells: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_enableContactCaching: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_mergeEventsBeforeDispatch: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_leavingBroadPhaseBehavior: _serde::__private::Option<
                        LeavingBroadPhaseBehavior,
                    > = _serde::__private::None;
                    let mut m_broadPhaseAabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_broadPhaseConfig: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_collisionFilter: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_shapeTagCodec: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_relativeCollisionAccuracy: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableWeldingForDefaultObjects: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_enableWeldingForCriticalObjects: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_solverTau: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverDamp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverIterations: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_solverMicrosteps: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_defaultSolverTimestep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxApproachSpeedForHighQualitySolver: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_deleteCachesOnDeactivation: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_largeIslandSize: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_enableSolverDynamicScheduling: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_contactSolverType: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_unitScale: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_applyUnitScaleToStaticConstants: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..37usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bodyBufferCapacity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyBufferCapacity",
                                        ),
                                    );
                                }
                                m_bodyBufferCapacity = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_userBodyBuffer) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userBodyBuffer",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_userBodyBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_motionBufferCapacity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionBufferCapacity",
                                        ),
                                    );
                                }
                                m_motionBufferCapacity = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_userMotionBuffer) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userMotionBuffer",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_userMotionBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_constraintBufferCapacity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintBufferCapacity",
                                        ),
                                    );
                                }
                                m_constraintBufferCapacity = _serde::__private::Some(
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
                                    &m_userConstraintBuffer,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userConstraintBuffer",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_userConstraintBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_persistentStreamAllocator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "persistentStreamAllocator",
                                        ),
                                    );
                                }
                                m_persistentStreamAllocator = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_materialLibrary) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialLibrary",
                                        ),
                                    );
                                }
                                m_materialLibrary = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_motionPropertiesLibrary,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionPropertiesLibrary",
                                        ),
                                    );
                                }
                                m_motionPropertiesLibrary = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_qualityLibrary) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "qualityLibrary",
                                        ),
                                    );
                                }
                                m_qualityLibrary = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_simulationType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationType",
                                        ),
                                    );
                                }
                                m_simulationType = _serde::__private::Some(
                                    match __A::next_value::<SimulationType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_numSplitterCells) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSplitterCells",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_numSplitterCells = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_gravity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravity",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_gravity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableContactCaching,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableContactCaching",
                                        ),
                                    );
                                }
                                m_enableContactCaching = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mergeEventsBeforeDispatch,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mergeEventsBeforeDispatch",
                                        ),
                                    );
                                }
                                m_mergeEventsBeforeDispatch = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_leavingBroadPhaseBehavior,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leavingBroadPhaseBehavior",
                                        ),
                                    );
                                }
                                m_leavingBroadPhaseBehavior = _serde::__private::Some(
                                    match __A::next_value::<
                                        LeavingBroadPhaseBehavior,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_broadPhaseAabb) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseAabb",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 13usize)?;
                                m_broadPhaseAabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_broadPhaseConfig) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseConfig",
                                        ),
                                    );
                                }
                                m_broadPhaseConfig = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_collisionFilter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilter",
                                        ),
                                    );
                                }
                                m_collisionFilter = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_shapeTagCodec) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeTagCodec",
                                        ),
                                    );
                                }
                                m_shapeTagCodec = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
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
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_relativeCollisionAccuracy,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativeCollisionAccuracy",
                                        ),
                                    );
                                }
                                m_relativeCollisionAccuracy = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableWeldingForDefaultObjects,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableWeldingForDefaultObjects",
                                        ),
                                    );
                                }
                                m_enableWeldingForDefaultObjects = _serde::__private::Some(
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
                                    &m_enableWeldingForCriticalObjects,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableWeldingForCriticalObjects",
                                        ),
                                    );
                                }
                                m_enableWeldingForCriticalObjects = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(&m_solverTau) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverTau",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_solverTau = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(&m_solverDamp) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverDamp",
                                        ),
                                    );
                                }
                                m_solverDamp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(&m_solverIterations) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverIterations",
                                        ),
                                    );
                                }
                                m_solverIterations = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(&m_solverMicrosteps) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverMicrosteps",
                                        ),
                                    );
                                }
                                m_solverMicrosteps = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_defaultSolverTimestep,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultSolverTimestep",
                                        ),
                                    );
                                }
                                m_defaultSolverTimestep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxApproachSpeedForHighQualitySolver,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxApproachSpeedForHighQualitySolver",
                                        ),
                                    );
                                }
                                m_maxApproachSpeedForHighQualitySolver = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableDeactivation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeactivation",
                                        ),
                                    );
                                }
                                m_enableDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deleteCachesOnDeactivation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deleteCachesOnDeactivation",
                                        ),
                                    );
                                }
                                m_deleteCachesOnDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(&m_largeIslandSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "largeIslandSize",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_largeIslandSize = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableSolverDynamicScheduling,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableSolverDynamicScheduling",
                                        ),
                                    );
                                }
                                m_enableSolverDynamicScheduling = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            34usize => {
                                if _serde::__private::Option::is_some(
                                    &m_contactSolverType,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactSolverType",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_contactSolverType = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(&m_unitScale) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "unitScale",
                                        ),
                                    );
                                }
                                m_unitScale = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            36usize => {
                                if _serde::__private::Option::is_some(
                                    &m_applyUnitScaleToStaticConstants,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applyUnitScaleToStaticConstants",
                                        ),
                                    );
                                }
                                m_applyUnitScaleToStaticConstants = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 15usize)?;
                    let m_bodyBufferCapacity = match m_bodyBufferCapacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bodyBufferCapacity",
                                ),
                            );
                        }
                    };
                    let m_userBodyBuffer = match m_userBodyBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userBodyBuffer",
                                ),
                            );
                        }
                    };
                    let m_motionBufferCapacity = match m_motionBufferCapacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionBufferCapacity",
                                ),
                            );
                        }
                    };
                    let m_userMotionBuffer = match m_userMotionBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userMotionBuffer",
                                ),
                            );
                        }
                    };
                    let m_constraintBufferCapacity = match m_constraintBufferCapacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintBufferCapacity",
                                ),
                            );
                        }
                    };
                    let m_userConstraintBuffer = match m_userConstraintBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userConstraintBuffer",
                                ),
                            );
                        }
                    };
                    let m_persistentStreamAllocator = match m_persistentStreamAllocator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "persistentStreamAllocator",
                                ),
                            );
                        }
                    };
                    let m_materialLibrary = match m_materialLibrary {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialLibrary",
                                ),
                            );
                        }
                    };
                    let m_motionPropertiesLibrary = match m_motionPropertiesLibrary {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionPropertiesLibrary",
                                ),
                            );
                        }
                    };
                    let m_qualityLibrary = match m_qualityLibrary {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "qualityLibrary",
                                ),
                            );
                        }
                    };
                    let m_simulationType = match m_simulationType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationType",
                                ),
                            );
                        }
                    };
                    let m_numSplitterCells = match m_numSplitterCells {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSplitterCells",
                                ),
                            );
                        }
                    };
                    let m_gravity = match m_gravity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gravity"),
                            );
                        }
                    };
                    let m_enableContactCaching = match m_enableContactCaching {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableContactCaching",
                                ),
                            );
                        }
                    };
                    let m_mergeEventsBeforeDispatch = match m_mergeEventsBeforeDispatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mergeEventsBeforeDispatch",
                                ),
                            );
                        }
                    };
                    let m_leavingBroadPhaseBehavior = match m_leavingBroadPhaseBehavior {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leavingBroadPhaseBehavior",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseAabb = match m_broadPhaseAabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseAabb",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseConfig = match m_broadPhaseConfig {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseConfig",
                                ),
                            );
                        }
                    };
                    let m_collisionFilter = match m_collisionFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilter",
                                ),
                            );
                        }
                    };
                    let m_shapeTagCodec = match m_shapeTagCodec {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapeTagCodec",
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
                    let m_relativeCollisionAccuracy = match m_relativeCollisionAccuracy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativeCollisionAccuracy",
                                ),
                            );
                        }
                    };
                    let m_enableWeldingForDefaultObjects = match m_enableWeldingForDefaultObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableWeldingForDefaultObjects",
                                ),
                            );
                        }
                    };
                    let m_enableWeldingForCriticalObjects = match m_enableWeldingForCriticalObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableWeldingForCriticalObjects",
                                ),
                            );
                        }
                    };
                    let m_solverTau = match m_solverTau {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverTau",
                                ),
                            );
                        }
                    };
                    let m_solverDamp = match m_solverDamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverDamp",
                                ),
                            );
                        }
                    };
                    let m_solverIterations = match m_solverIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverIterations",
                                ),
                            );
                        }
                    };
                    let m_solverMicrosteps = match m_solverMicrosteps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverMicrosteps",
                                ),
                            );
                        }
                    };
                    let m_defaultSolverTimestep = match m_defaultSolverTimestep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultSolverTimestep",
                                ),
                            );
                        }
                    };
                    let m_maxApproachSpeedForHighQualitySolver = match m_maxApproachSpeedForHighQualitySolver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxApproachSpeedForHighQualitySolver",
                                ),
                            );
                        }
                    };
                    let m_enableDeactivation = match m_enableDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeactivation",
                                ),
                            );
                        }
                    };
                    let m_deleteCachesOnDeactivation = match m_deleteCachesOnDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deleteCachesOnDeactivation",
                                ),
                            );
                        }
                    };
                    let m_largeIslandSize = match m_largeIslandSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "largeIslandSize",
                                ),
                            );
                        }
                    };
                    let m_enableSolverDynamicScheduling = match m_enableSolverDynamicScheduling {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableSolverDynamicScheduling",
                                ),
                            );
                        }
                    };
                    let m_contactSolverType = match m_contactSolverType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactSolverType",
                                ),
                            );
                        }
                    };
                    let m_unitScale = match m_unitScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "unitScale",
                                ),
                            );
                        }
                    };
                    let m_applyUnitScaleToStaticConstants = match m_applyUnitScaleToStaticConstants {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applyUnitScaleToStaticConstants",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpWorldCinfo {
                        __ptr,
                        m_bodyBufferCapacity,
                        m_userBodyBuffer,
                        m_motionBufferCapacity,
                        m_userMotionBuffer,
                        m_constraintBufferCapacity,
                        m_userConstraintBuffer,
                        m_persistentStreamAllocator,
                        m_materialLibrary,
                        m_motionPropertiesLibrary,
                        m_qualityLibrary,
                        m_simulationType,
                        m_numSplitterCells,
                        m_gravity,
                        m_enableContactCaching,
                        m_mergeEventsBeforeDispatch,
                        m_leavingBroadPhaseBehavior,
                        m_broadPhaseAabb,
                        m_broadPhaseConfig,
                        m_collisionFilter,
                        m_shapeTagCodec,
                        m_collisionTolerance,
                        m_relativeCollisionAccuracy,
                        m_enableWeldingForDefaultObjects,
                        m_enableWeldingForCriticalObjects,
                        m_solverTau,
                        m_solverDamp,
                        m_solverIterations,
                        m_solverMicrosteps,
                        m_defaultSolverTimestep,
                        m_maxApproachSpeedForHighQualitySolver,
                        m_enableDeactivation,
                        m_deleteCachesOnDeactivation,
                        m_largeIslandSize,
                        m_enableSolverDynamicScheduling,
                        m_contactSolverType,
                        m_unitScale,
                        m_applyUnitScaleToStaticConstants,
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
                    let mut m_bodyBufferCapacity: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_motionBufferCapacity: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_constraintBufferCapacity: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_materialLibrary: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_motionPropertiesLibrary: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_qualityLibrary: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_simulationType: _serde::__private::Option<
                        SimulationType,
                    > = _serde::__private::None;
                    let mut m_numSplitterCells: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_enableContactCaching: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_mergeEventsBeforeDispatch: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_leavingBroadPhaseBehavior: _serde::__private::Option<
                        LeavingBroadPhaseBehavior,
                    > = _serde::__private::None;
                    let mut m_broadPhaseAabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_broadPhaseConfig: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_collisionFilter: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_shapeTagCodec: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_relativeCollisionAccuracy: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableWeldingForDefaultObjects: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_enableWeldingForCriticalObjects: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_solverTau: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverDamp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverIterations: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_solverMicrosteps: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_defaultSolverTimestep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxApproachSpeedForHighQualitySolver: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_deleteCachesOnDeactivation: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_largeIslandSize: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_enableSolverDynamicScheduling: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_contactSolverType: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_unitScale: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_applyUnitScaleToStaticConstants: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_bodyBufferCapacity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_bodyBufferCapacity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyBufferCapacity",
                                        ),
                                    );
                                }
                                m_bodyBufferCapacity = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionBufferCapacity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_motionBufferCapacity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionBufferCapacity",
                                        ),
                                    );
                                }
                                m_motionBufferCapacity = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_constraintBufferCapacity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_constraintBufferCapacity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintBufferCapacity",
                                        ),
                                    );
                                }
                                m_constraintBufferCapacity = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materialLibrary => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materialLibrary) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialLibrary",
                                        ),
                                    );
                                }
                                m_materialLibrary = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionPropertiesLibrary => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_motionPropertiesLibrary,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionPropertiesLibrary",
                                        ),
                                    );
                                }
                                m_motionPropertiesLibrary = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_qualityLibrary => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_qualityLibrary) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "qualityLibrary",
                                        ),
                                    );
                                }
                                m_qualityLibrary = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simulationType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simulationType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationType",
                                        ),
                                    );
                                }
                                m_simulationType = _serde::__private::Some(
                                    match __A::next_value::<SimulationType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numSplitterCells => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numSplitterCells) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSplitterCells",
                                        ),
                                    );
                                }
                                m_numSplitterCells = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
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
                            __Field::m_enableContactCaching => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableContactCaching,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableContactCaching",
                                        ),
                                    );
                                }
                                m_enableContactCaching = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mergeEventsBeforeDispatch => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_mergeEventsBeforeDispatch,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mergeEventsBeforeDispatch",
                                        ),
                                    );
                                }
                                m_mergeEventsBeforeDispatch = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_leavingBroadPhaseBehavior => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_leavingBroadPhaseBehavior,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leavingBroadPhaseBehavior",
                                        ),
                                    );
                                }
                                m_leavingBroadPhaseBehavior = _serde::__private::Some(
                                    match __A::next_value::<
                                        LeavingBroadPhaseBehavior,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseAabb => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_broadPhaseAabb) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseAabb",
                                        ),
                                    );
                                }
                                m_broadPhaseAabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseConfig => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_broadPhaseConfig) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseConfig",
                                        ),
                                    );
                                }
                                m_broadPhaseConfig = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionFilter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_collisionFilter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilter",
                                        ),
                                    );
                                }
                                m_collisionFilter = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapeTagCodec => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapeTagCodec) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeTagCodec",
                                        ),
                                    );
                                }
                                m_shapeTagCodec = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                            __Field::m_relativeCollisionAccuracy => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_relativeCollisionAccuracy,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativeCollisionAccuracy",
                                        ),
                                    );
                                }
                                m_relativeCollisionAccuracy = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableWeldingForDefaultObjects => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableWeldingForDefaultObjects,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableWeldingForDefaultObjects",
                                        ),
                                    );
                                }
                                m_enableWeldingForDefaultObjects = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableWeldingForCriticalObjects => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableWeldingForCriticalObjects,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableWeldingForCriticalObjects",
                                        ),
                                    );
                                }
                                m_enableWeldingForCriticalObjects = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_solverTau => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverTau) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverTau",
                                        ),
                                    );
                                }
                                m_solverTau = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_solverDamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverDamp) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverDamp",
                                        ),
                                    );
                                }
                                m_solverDamp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_solverIterations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverIterations) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverIterations",
                                        ),
                                    );
                                }
                                m_solverIterations = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_solverMicrosteps => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverMicrosteps) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverMicrosteps",
                                        ),
                                    );
                                }
                                m_solverMicrosteps = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_defaultSolverTimestep => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_defaultSolverTimestep,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultSolverTimestep",
                                        ),
                                    );
                                }
                                m_defaultSolverTimestep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxApproachSpeedForHighQualitySolver => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxApproachSpeedForHighQualitySolver,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxApproachSpeedForHighQualitySolver",
                                        ),
                                    );
                                }
                                m_maxApproachSpeedForHighQualitySolver = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableDeactivation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableDeactivation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeactivation",
                                        ),
                                    );
                                }
                                m_enableDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_deleteCachesOnDeactivation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deleteCachesOnDeactivation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deleteCachesOnDeactivation",
                                        ),
                                    );
                                }
                                m_deleteCachesOnDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_largeIslandSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_largeIslandSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "largeIslandSize",
                                        ),
                                    );
                                }
                                m_largeIslandSize = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableSolverDynamicScheduling => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableSolverDynamicScheduling,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableSolverDynamicScheduling",
                                        ),
                                    );
                                }
                                m_enableSolverDynamicScheduling = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_contactSolverType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_contactSolverType,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactSolverType",
                                        ),
                                    );
                                }
                                m_contactSolverType = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_unitScale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_unitScale) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "unitScale",
                                        ),
                                    );
                                }
                                m_unitScale = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_applyUnitScaleToStaticConstants => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_applyUnitScaleToStaticConstants,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applyUnitScaleToStaticConstants",
                                        ),
                                    );
                                }
                                m_applyUnitScaleToStaticConstants = _serde::__private::Some(
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
                    let m_bodyBufferCapacity = match m_bodyBufferCapacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bodyBufferCapacity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionBufferCapacity = match m_motionBufferCapacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionBufferCapacity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constraintBufferCapacity = match m_constraintBufferCapacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintBufferCapacity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materialLibrary = match m_materialLibrary {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialLibrary",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionPropertiesLibrary = match m_motionPropertiesLibrary {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionPropertiesLibrary",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_qualityLibrary = match m_qualityLibrary {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "qualityLibrary",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simulationType = match m_simulationType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numSplitterCells = match m_numSplitterCells {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSplitterCells",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let m_enableContactCaching = match m_enableContactCaching {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableContactCaching",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mergeEventsBeforeDispatch = match m_mergeEventsBeforeDispatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mergeEventsBeforeDispatch",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_leavingBroadPhaseBehavior = match m_leavingBroadPhaseBehavior {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leavingBroadPhaseBehavior",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseAabb = match m_broadPhaseAabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseAabb",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseConfig = match m_broadPhaseConfig {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseConfig",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionFilter = match m_collisionFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapeTagCodec = match m_shapeTagCodec {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapeTagCodec",
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
                    let m_relativeCollisionAccuracy = match m_relativeCollisionAccuracy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativeCollisionAccuracy",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableWeldingForDefaultObjects = match m_enableWeldingForDefaultObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableWeldingForDefaultObjects",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableWeldingForCriticalObjects = match m_enableWeldingForCriticalObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableWeldingForCriticalObjects",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverTau = match m_solverTau {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverTau",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverDamp = match m_solverDamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverDamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverIterations = match m_solverIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverIterations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverMicrosteps = match m_solverMicrosteps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverMicrosteps",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_defaultSolverTimestep = match m_defaultSolverTimestep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultSolverTimestep",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxApproachSpeedForHighQualitySolver = match m_maxApproachSpeedForHighQualitySolver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxApproachSpeedForHighQualitySolver",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableDeactivation = match m_enableDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeactivation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deleteCachesOnDeactivation = match m_deleteCachesOnDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deleteCachesOnDeactivation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_largeIslandSize = match m_largeIslandSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "largeIslandSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableSolverDynamicScheduling = match m_enableSolverDynamicScheduling {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableSolverDynamicScheduling",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_contactSolverType = match m_contactSolverType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactSolverType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_unitScale = match m_unitScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "unitScale",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_applyUnitScaleToStaticConstants = match m_applyUnitScaleToStaticConstants {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applyUnitScaleToStaticConstants",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpWorldCinfo {
                        __ptr: __ptr.clone(),
                        m_bodyBufferCapacity,
                        m_motionBufferCapacity,
                        m_constraintBufferCapacity,
                        m_materialLibrary,
                        m_motionPropertiesLibrary,
                        m_qualityLibrary,
                        m_simulationType,
                        m_numSplitterCells,
                        m_gravity,
                        m_enableContactCaching,
                        m_mergeEventsBeforeDispatch,
                        m_leavingBroadPhaseBehavior,
                        m_broadPhaseAabb,
                        m_broadPhaseConfig,
                        m_collisionFilter,
                        m_shapeTagCodec,
                        m_collisionTolerance,
                        m_relativeCollisionAccuracy,
                        m_enableWeldingForDefaultObjects,
                        m_enableWeldingForCriticalObjects,
                        m_solverTau,
                        m_solverDamp,
                        m_solverIterations,
                        m_solverMicrosteps,
                        m_defaultSolverTimestep,
                        m_maxApproachSpeedForHighQualitySolver,
                        m_enableDeactivation,
                        m_deleteCachesOnDeactivation,
                        m_largeIslandSize,
                        m_enableSolverDynamicScheduling,
                        m_contactSolverType,
                        m_unitScale,
                        m_applyUnitScaleToStaticConstants,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "bodyBufferCapacity",
                "userBodyBuffer",
                "motionBufferCapacity",
                "userMotionBuffer",
                "constraintBufferCapacity",
                "userConstraintBuffer",
                "persistentStreamAllocator",
                "materialLibrary",
                "motionPropertiesLibrary",
                "qualityLibrary",
                "simulationType",
                "numSplitterCells",
                "gravity",
                "enableContactCaching",
                "mergeEventsBeforeDispatch",
                "leavingBroadPhaseBehavior",
                "broadPhaseAabb",
                "broadPhaseConfig",
                "collisionFilter",
                "shapeTagCodec",
                "collisionTolerance",
                "relativeCollisionAccuracy",
                "enableWeldingForDefaultObjects",
                "enableWeldingForCriticalObjects",
                "solverTau",
                "solverDamp",
                "solverIterations",
                "solverMicrosteps",
                "defaultSolverTimestep",
                "maxApproachSpeedForHighQualitySolver",
                "enableDeactivation",
                "deleteCachesOnDeactivation",
                "largeIslandSize",
                "enableSolverDynamicScheduling",
                "contactSolverType",
                "unitScale",
                "applyUnitScaleToStaticConstants",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpWorldCinfo",
                FIELDS,
                __hknpWorldCinfoVisitor {
                    marker: _serde::__private::PhantomData::<hknpWorldCinfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `SimulationType`(ctype: `hkEnum<SimulationType, hkUint8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum SimulationType {
    #[default]
    SIMULATION_TYPE_SINGLE_THREADED = 0isize,
    SIMULATION_TYPE_MULTI_THREADED = 1isize,
}
/// # C++ Info
/// - name: `LeavingBroadPhaseBehavior`(ctype: `hkEnum<LeavingBroadPhaseBehavior, hkUint8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum LeavingBroadPhaseBehavior {
    #[default]
    ON_LEAVING_BROAD_PHASE_DO_NOTHING = 0isize,
    ON_LEAVING_BROAD_PHASE_REMOVE_BODY = 1isize,
    ON_LEAVING_BROAD_PHASE_FREEZE_BODY = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SimulationType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SIMULATION_TYPE_SINGLE_THREADED => {
                    __serializer
                        .serialize_field("SIMULATION_TYPE_SINGLE_THREADED", &0u64)
                }
                Self::SIMULATION_TYPE_MULTI_THREADED => {
                    __serializer.serialize_field("SIMULATION_TYPE_MULTI_THREADED", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum SimulationType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for LeavingBroadPhaseBehavior {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ON_LEAVING_BROAD_PHASE_DO_NOTHING => {
                    __serializer
                        .serialize_field("ON_LEAVING_BROAD_PHASE_DO_NOTHING", &0u64)
                }
                Self::ON_LEAVING_BROAD_PHASE_REMOVE_BODY => {
                    __serializer
                        .serialize_field("ON_LEAVING_BROAD_PHASE_REMOVE_BODY", &1u64)
                }
                Self::ON_LEAVING_BROAD_PHASE_FREEZE_BODY => {
                    __serializer
                        .serialize_field("ON_LEAVING_BROAD_PHASE_FREEZE_BODY", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum LeavingBroadPhaseBehavior to_u8"))?;
            __serializer.serialize_bits(&num)?;
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
    impl<'de> _serde::Deserialize<'de> for SimulationType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1",
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
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v
                                    .eq_ignore_ascii_case("SIMULATION_TYPE_SINGLE_THREADED") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("SIMULATION_TYPE_MULTI_THREADED") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SimulationType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimulationType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SimulationType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SimulationType::SIMULATION_TYPE_SINGLE_THREADED,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SimulationType::SIMULATION_TYPE_MULTI_THREADED,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SIMULATION_TYPE_SINGLE_THREADED",
                "SIMULATION_TYPE_MULTI_THREADED",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SimulationType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimulationType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for LeavingBroadPhaseBehavior {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2",
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
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v
                                    .eq_ignore_ascii_case(
                                        "ON_LEAVING_BROAD_PHASE_DO_NOTHING",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "ON_LEAVING_BROAD_PHASE_REMOVE_BODY",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "ON_LEAVING_BROAD_PHASE_FREEZE_BODY",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<LeavingBroadPhaseBehavior>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = LeavingBroadPhaseBehavior;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum LeavingBroadPhaseBehavior",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                LeavingBroadPhaseBehavior::ON_LEAVING_BROAD_PHASE_DO_NOTHING,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                LeavingBroadPhaseBehavior::ON_LEAVING_BROAD_PHASE_REMOVE_BODY,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                LeavingBroadPhaseBehavior::ON_LEAVING_BROAD_PHASE_FREEZE_BODY,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ON_LEAVING_BROAD_PHASE_DO_NOTHING",
                "ON_LEAVING_BROAD_PHASE_REMOVE_BODY",
                "ON_LEAVING_BROAD_PHASE_FREEZE_BODY",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "LeavingBroadPhaseBehavior",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<LeavingBroadPhaseBehavior>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
