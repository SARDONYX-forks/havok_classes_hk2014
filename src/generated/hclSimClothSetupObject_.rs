use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSimClothSetupObject`
/// - version: `1`
/// - signature: `0x474aa25`
/// - size: `  0`(x86)/`352`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSimClothSetupObject<'a> {
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
    /// - name: `name`(ctype: `TYPE_STRINGPTR`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `simulationMesh`(ctype: `hclSimulationSetupMesh`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simulationMesh"))]
    #[cfg_attr(feature = "serde", serde(rename = "simulationMesh"))]
    pub m_simulationMesh: Pointer<'a>,
    /// # C++ Info
    /// - name: `collidableTransformSet`(ctype: `hclTransformSetSetupObject`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collidableTransformSet"))]
    #[cfg_attr(feature = "serde", serde(rename = "collidableTransformSet"))]
    pub m_collidableTransformSet: Pointer<'a>,
    /// # C++ Info
    /// - name: `gravity`(ctype: `TYPE_VECTOR4`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gravity"))]
    #[cfg_attr(feature = "serde", serde(rename = "gravity"))]
    pub m_gravity: Vector4,
    /// # C++ Info
    /// - name: `globalDampingPerSecond`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "globalDampingPerSecond"))]
    #[cfg_attr(feature = "serde", serde(rename = "globalDampingPerSecond"))]
    pub m_globalDampingPerSecond: f32,
    /// # C++ Info
    /// - name: `doNormals`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/` 68`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "doNormals"))]
    #[cfg_attr(feature = "serde", serde(rename = "doNormals"))]
    pub m_doNormals: bool,
    /// # C++ Info
    /// - name: `specifyDensity`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/` 69`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "specifyDensity"))]
    #[cfg_attr(feature = "serde", serde(rename = "specifyDensity"))]
    pub m_specifyDensity: bool,
    /// # C++ Info
    /// - name: `vertexDensity`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexDensity"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexDensity"))]
    pub m_vertexDensity: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `rescaleMass`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rescaleMass"))]
    #[cfg_attr(feature = "serde", serde(rename = "rescaleMass"))]
    pub m_rescaleMass: bool,
    /// # C++ Info
    /// - name: `totalMass`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "totalMass"))]
    #[cfg_attr(feature = "serde", serde(rename = "totalMass"))]
    pub m_totalMass: f32,
    /// # C++ Info
    /// - name: `particleMass`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleMass"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleMass"))]
    pub m_particleMass: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `particleRadius`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleRadius"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleRadius"))]
    pub m_particleRadius: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `particleFriction`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleFriction"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleFriction"))]
    pub m_particleFriction: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `fixedParticles`(ctype: `hclVertexSelectionInput`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fixedParticles"))]
    #[cfg_attr(feature = "serde", serde(rename = "fixedParticles"))]
    pub m_fixedParticles: hclVertexSelectionInput<'a>,
    /// # C++ Info
    /// - name: `enablePinchDetection`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "enablePinchDetection"))]
    #[cfg_attr(feature = "serde", serde(rename = "enablePinchDetection"))]
    pub m_enablePinchDetection: bool,
    /// # C++ Info
    /// - name: `pinchDetectionEnabledParticles`(ctype: `hclVertexSelectionInput`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "pinchDetectionEnabledParticles")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "pinchDetectionEnabledParticles"))]
    pub m_pinchDetectionEnabledParticles: hclVertexSelectionInput<'a>,
    /// # C++ Info
    /// - name: `toAnimPeriod`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toAnimPeriod"))]
    #[cfg_attr(feature = "serde", serde(rename = "toAnimPeriod"))]
    pub m_toAnimPeriod: f32,
    /// # C++ Info
    /// - name: `toSimPeriod`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/`188`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toSimPeriod"))]
    #[cfg_attr(feature = "serde", serde(rename = "toSimPeriod"))]
    pub m_toSimPeriod: f32,
    /// # C++ Info
    /// - name: `drivePinchedParticlesToReferenceMesh`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "drivePinchedParticlesToReferenceMesh")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "drivePinchedParticlesToReferenceMesh")
    )]
    pub m_drivePinchedParticlesToReferenceMesh: bool,
    /// # C++ Info
    /// - name: `pinchReferenceBufferSetup`(ctype: `hclBufferSetupObject`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pinchReferenceBufferSetup"))]
    #[cfg_attr(feature = "serde", serde(rename = "pinchReferenceBufferSetup"))]
    pub m_pinchReferenceBufferSetup: Pointer<'a>,
    /// # C++ Info
    /// - name: `collisionTolerance`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionTolerance"))]
    pub m_collisionTolerance: f32,
    /// # C++ Info
    /// - name: `landscapeCollisionParticleSelection`(ctype: `hclVertexSelectionInput`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "landscapeCollisionParticleSelection")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "landscapeCollisionParticleSelection"))]
    pub m_landscapeCollisionParticleSelection: hclVertexSelectionInput<'a>,
    /// # C++ Info
    /// - name: `landscapeCollisionParticleRadius`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "landscapeCollisionParticleRadius")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "landscapeCollisionParticleRadius"))]
    pub m_landscapeCollisionParticleRadius: f32,
    /// # C++ Info
    /// - name: `enableStuckParticleDetection`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`236`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "enableStuckParticleDetection")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "enableStuckParticleDetection"))]
    pub m_enableStuckParticleDetection: bool,
    /// # C++ Info
    /// - name: `stuckParticlesStretchFactor`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "stuckParticlesStretchFactor")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "stuckParticlesStretchFactor"))]
    pub m_stuckParticlesStretchFactor: f32,
    /// # C++ Info
    /// - name: `enableLandscapePinchDetection`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`244`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "enableLandscapePinchDetection")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "enableLandscapePinchDetection"))]
    pub m_enableLandscapePinchDetection: bool,
    /// # C++ Info
    /// - name: `landscapePinchDetectionPriority`(ctype: `TYPE_INT8`)
    /// - offset: `  0`(x86)/`245`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "landscapePinchDetectionPriority")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "landscapePinchDetectionPriority"))]
    pub m_landscapePinchDetectionPriority: I8<'a>,
    /// # C++ Info
    /// - name: `landscapePinchDetectionRadius`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/`248`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "landscapePinchDetectionRadius")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "landscapePinchDetectionRadius"))]
    pub m_landscapePinchDetectionRadius: f32,
    /// # C++ Info
    /// - name: `enableTransferMotion`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`252`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "enableTransferMotion"))]
    #[cfg_attr(feature = "serde", serde(rename = "enableTransferMotion"))]
    pub m_enableTransferMotion: bool,
    /// # C++ Info
    /// - name: `transferMotionSetupData`(ctype: `hclSimClothSetupObjectTransferMotionSetupData`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/` 56`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transferMotionSetupData"))]
    #[cfg_attr(feature = "serde", serde(rename = "transferMotionSetupData"))]
    pub m_transferMotionSetupData: hclSimClothSetupObjectTransferMotionSetupData<'a>,
    /// # C++ Info
    /// - name: `constraintSetSetups`(ctype: `hclConstraintSetSetupObject`)
    /// - offset: `  0`(x86)/`312`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "constraintSetSetups"))]
    #[cfg_attr(feature = "serde", serde(rename = "constraintSetSetups"))]
    pub m_constraintSetSetups: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `perInstanceCollidables`(ctype: `hclSimClothSetupObjectPerInstanceCollidable`)
    /// - offset: `  0`(x86)/`328`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "perInstanceCollidables"))]
    #[cfg_attr(feature = "serde", serde(rename = "perInstanceCollidables"))]
    pub m_perInstanceCollidables: Vec<hclSimClothSetupObjectPerInstanceCollidable<'a>>,
    /// # C++ Info
    /// - name: `orderMap`(ctype: `TYPE_POINTER`)
    /// - offset: `  0`(x86)/`344`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "orderMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "orderMap"))]
    pub m_orderMap: Pointer<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSimClothSetupObject<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSimClothSetupObject"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x474aa25)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_simulationMesh);
            v.push(&self.m_collidableTransformSet);
            v.extend(self.m_vertexDensity.deps_indexes());
            v.extend(self.m_particleMass.deps_indexes());
            v.extend(self.m_particleRadius.deps_indexes());
            v.extend(self.m_particleFriction.deps_indexes());
            v.extend(self.m_fixedParticles.deps_indexes());
            v.extend(self.m_pinchDetectionEnabledParticles.deps_indexes());
            v.push(&self.m_pinchReferenceBufferSetup);
            v.extend(self.m_landscapeCollisionParticleSelection.deps_indexes());
            v.extend(self.m_transferMotionSetupData.deps_indexes());
            v.extend(self.m_constraintSetSetups.iter());
            v.extend(
                self
                    .m_perInstanceCollidables
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.push(&self.m_orderMap);
            v
        }
    }
    impl<'a> _serde::Serialize for hclSimClothSetupObject<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x474aa25)));
            let mut serializer = __serializer
                .serialize_struct("hclSimClothSetupObject", class_meta, (0u64, 352u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("simulationMesh", &self.m_simulationMesh)?;
            serializer
                .serialize_field(
                    "collidableTransformSet",
                    &self.m_collidableTransformSet,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer
                .serialize_field(
                    "globalDampingPerSecond",
                    &self.m_globalDampingPerSecond,
                )?;
            serializer.serialize_field("doNormals", &self.m_doNormals)?;
            serializer.serialize_field("specifyDensity", &self.m_specifyDensity)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("vertexDensity", &self.m_vertexDensity)?;
            serializer.serialize_field("rescaleMass", &self.m_rescaleMass)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("totalMass", &self.m_totalMass)?;
            serializer.serialize_field("particleMass", &self.m_particleMass)?;
            serializer.serialize_field("particleRadius", &self.m_particleRadius)?;
            serializer.serialize_field("particleFriction", &self.m_particleFriction)?;
            serializer.serialize_field("fixedParticles", &self.m_fixedParticles)?;
            serializer
                .serialize_field("enablePinchDetection", &self.m_enablePinchDetection)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "pinchDetectionEnabledParticles",
                    &self.m_pinchDetectionEnabledParticles,
                )?;
            serializer.serialize_field("toAnimPeriod", &self.m_toAnimPeriod)?;
            serializer.serialize_field("toSimPeriod", &self.m_toSimPeriod)?;
            serializer
                .serialize_field(
                    "drivePinchedParticlesToReferenceMesh",
                    &self.m_drivePinchedParticlesToReferenceMesh,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "pinchReferenceBufferSetup",
                    &self.m_pinchReferenceBufferSetup,
                )?;
            serializer
                .serialize_field("collisionTolerance", &self.m_collisionTolerance)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "landscapeCollisionParticleSelection",
                    &self.m_landscapeCollisionParticleSelection,
                )?;
            serializer
                .serialize_field(
                    "landscapeCollisionParticleRadius",
                    &self.m_landscapeCollisionParticleRadius,
                )?;
            serializer
                .serialize_field(
                    "enableStuckParticleDetection",
                    &self.m_enableStuckParticleDetection,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "stuckParticlesStretchFactor",
                    &self.m_stuckParticlesStretchFactor,
                )?;
            serializer
                .serialize_field(
                    "enableLandscapePinchDetection",
                    &self.m_enableLandscapePinchDetection,
                )?;
            serializer
                .serialize_field(
                    "landscapePinchDetectionPriority",
                    &self.m_landscapePinchDetectionPriority,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "landscapePinchDetectionRadius",
                    &self.m_landscapePinchDetectionRadius,
                )?;
            serializer
                .serialize_field("enableTransferMotion", &self.m_enableTransferMotion)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "transferMotionSetupData",
                    &self.m_transferMotionSetupData,
                )?;
            serializer
                .serialize_array_field(
                    "constraintSetSetups",
                    &self.m_constraintSetSetups,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "perInstanceCollidables",
                    &self.m_perInstanceCollidables,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 40u64,
                    },
                )?;
            serializer.skip_field("orderMap", &self.m_orderMap)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclSimClothSetupObject<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_simulationMesh,
                m_collidableTransformSet,
                m_gravity,
                m_globalDampingPerSecond,
                m_doNormals,
                m_specifyDensity,
                m_vertexDensity,
                m_rescaleMass,
                m_totalMass,
                m_particleMass,
                m_particleRadius,
                m_particleFriction,
                m_fixedParticles,
                m_enablePinchDetection,
                m_pinchDetectionEnabledParticles,
                m_toAnimPeriod,
                m_toSimPeriod,
                m_drivePinchedParticlesToReferenceMesh,
                m_pinchReferenceBufferSetup,
                m_collisionTolerance,
                m_landscapeCollisionParticleSelection,
                m_landscapeCollisionParticleRadius,
                m_enableStuckParticleDetection,
                m_stuckParticlesStretchFactor,
                m_enableLandscapePinchDetection,
                m_landscapePinchDetectionPriority,
                m_landscapePinchDetectionRadius,
                m_enableTransferMotion,
                m_transferMotionSetupData,
                m_constraintSetSetups,
                m_perInstanceCollidables,
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
                        "simulationMesh" => Ok(__Field::m_simulationMesh),
                        "collidableTransformSet" => Ok(__Field::m_collidableTransformSet),
                        "gravity" => Ok(__Field::m_gravity),
                        "globalDampingPerSecond" => Ok(__Field::m_globalDampingPerSecond),
                        "doNormals" => Ok(__Field::m_doNormals),
                        "specifyDensity" => Ok(__Field::m_specifyDensity),
                        "vertexDensity" => Ok(__Field::m_vertexDensity),
                        "rescaleMass" => Ok(__Field::m_rescaleMass),
                        "totalMass" => Ok(__Field::m_totalMass),
                        "particleMass" => Ok(__Field::m_particleMass),
                        "particleRadius" => Ok(__Field::m_particleRadius),
                        "particleFriction" => Ok(__Field::m_particleFriction),
                        "fixedParticles" => Ok(__Field::m_fixedParticles),
                        "enablePinchDetection" => Ok(__Field::m_enablePinchDetection),
                        "pinchDetectionEnabledParticles" => {
                            Ok(__Field::m_pinchDetectionEnabledParticles)
                        }
                        "toAnimPeriod" => Ok(__Field::m_toAnimPeriod),
                        "toSimPeriod" => Ok(__Field::m_toSimPeriod),
                        "drivePinchedParticlesToReferenceMesh" => {
                            Ok(__Field::m_drivePinchedParticlesToReferenceMesh)
                        }
                        "pinchReferenceBufferSetup" => {
                            Ok(__Field::m_pinchReferenceBufferSetup)
                        }
                        "collisionTolerance" => Ok(__Field::m_collisionTolerance),
                        "landscapeCollisionParticleSelection" => {
                            Ok(__Field::m_landscapeCollisionParticleSelection)
                        }
                        "landscapeCollisionParticleRadius" => {
                            Ok(__Field::m_landscapeCollisionParticleRadius)
                        }
                        "enableStuckParticleDetection" => {
                            Ok(__Field::m_enableStuckParticleDetection)
                        }
                        "stuckParticlesStretchFactor" => {
                            Ok(__Field::m_stuckParticlesStretchFactor)
                        }
                        "enableLandscapePinchDetection" => {
                            Ok(__Field::m_enableLandscapePinchDetection)
                        }
                        "landscapePinchDetectionPriority" => {
                            Ok(__Field::m_landscapePinchDetectionPriority)
                        }
                        "landscapePinchDetectionRadius" => {
                            Ok(__Field::m_landscapePinchDetectionRadius)
                        }
                        "enableTransferMotion" => Ok(__Field::m_enableTransferMotion),
                        "transferMotionSetupData" => {
                            Ok(__Field::m_transferMotionSetupData)
                        }
                        "constraintSetSetups" => Ok(__Field::m_constraintSetSetups),
                        "perInstanceCollidables" => Ok(__Field::m_perInstanceCollidables),
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
            struct __hclSimClothSetupObjectVisitor<'de> {
                marker: _serde::__private::PhantomData<hclSimClothSetupObject<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclSimClothSetupObjectVisitor<'de> {
                type Value = hclSimClothSetupObject<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSimClothSetupObject",
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_simulationMesh: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_collidableTransformSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_globalDampingPerSecond: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_doNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_specifyDensity: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_vertexDensity: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_rescaleMass: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_totalMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_particleMass: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_particleRadius: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_particleFriction: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_fixedParticles: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_enablePinchDetection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pinchDetectionEnabledParticles: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_toAnimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toSimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_drivePinchedParticlesToReferenceMesh: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_pinchReferenceBufferSetup: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_landscapeCollisionParticleSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_landscapeCollisionParticleRadius: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableStuckParticleDetection: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_stuckParticlesStretchFactor: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableLandscapePinchDetection: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_landscapePinchDetectionPriority: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_landscapePinchDetectionRadius: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableTransferMotion: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_transferMotionSetupData: _serde::__private::Option<
                        hclSimClothSetupObjectTransferMotionSetupData<'de>,
                    > = _serde::__private::None;
                    let mut m_constraintSetSetups: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_perInstanceCollidables: _serde::__private::Option<
                        Vec<hclSimClothSetupObjectPerInstanceCollidable<'de>>,
                    > = _serde::__private::None;
                    let mut m_orderMap: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    for i in 0..33usize {
                        match i {
                            0usize => {
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
                            1usize => {
                                if _serde::__private::Option::is_some(&m_simulationMesh) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationMesh",
                                        ),
                                    );
                                }
                                m_simulationMesh = _serde::__private::Some(
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
                                    &m_collidableTransformSet,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidableTransformSet",
                                        ),
                                    );
                                }
                                m_collidableTransformSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
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
                            4usize => {
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
                            5usize => {
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
                            6usize => {
                                if _serde::__private::Option::is_some(&m_specifyDensity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "specifyDensity",
                                        ),
                                    );
                                }
                                m_specifyDensity = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_vertexDensity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexDensity",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_vertexDensity = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_rescaleMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rescaleMass",
                                        ),
                                    );
                                }
                                m_rescaleMass = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_totalMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "totalMass",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_totalMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_particleMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleMass",
                                        ),
                                    );
                                }
                                m_particleMass = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_particleRadius) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleRadius",
                                        ),
                                    );
                                }
                                m_particleRadius = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_particleFriction) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleFriction",
                                        ),
                                    );
                                }
                                m_particleFriction = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_fixedParticles) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedParticles",
                                        ),
                                    );
                                }
                                m_fixedParticles = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enablePinchDetection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enablePinchDetection",
                                        ),
                                    );
                                }
                                m_enablePinchDetection = _serde::__private::Some(
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
                                    &m_pinchDetectionEnabledParticles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionEnabledParticles",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_pinchDetectionEnabledParticles = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_toAnimPeriod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toAnimPeriod",
                                        ),
                                    );
                                }
                                m_toAnimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_toSimPeriod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toSimPeriod",
                                        ),
                                    );
                                }
                                m_toSimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_drivePinchedParticlesToReferenceMesh,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "drivePinchedParticlesToReferenceMesh",
                                        ),
                                    );
                                }
                                m_drivePinchedParticlesToReferenceMesh = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pinchReferenceBufferSetup,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchReferenceBufferSetup",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_pinchReferenceBufferSetup = _serde::__private::Some(
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
                                    &m_landscapeCollisionParticleSelection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionParticleSelection",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_landscapeCollisionParticleSelection = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionParticleRadius,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionParticleRadius",
                                        ),
                                    );
                                }
                                m_landscapeCollisionParticleRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableStuckParticleDetection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableStuckParticleDetection",
                                        ),
                                    );
                                }
                                m_enableStuckParticleDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stuckParticlesStretchFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stuckParticlesStretchFactor",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_stuckParticlesStretchFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableLandscapePinchDetection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableLandscapePinchDetection",
                                        ),
                                    );
                                }
                                m_enableLandscapePinchDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(
                                    &m_landscapePinchDetectionPriority,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapePinchDetectionPriority",
                                        ),
                                    );
                                }
                                m_landscapePinchDetectionPriority = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_landscapePinchDetectionRadius,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapePinchDetectionRadius",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_landscapePinchDetectionRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableTransferMotion,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableTransferMotion",
                                        ),
                                    );
                                }
                                m_enableTransferMotion = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionSetupData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionSetupData",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_transferMotionSetupData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothSetupObjectTransferMotionSetupData<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(
                                    &m_constraintSetSetups,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintSetSetups",
                                        ),
                                    );
                                }
                                m_constraintSetSetups = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
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
                                    match __A::next_value::<
                                        Vec<hclSimClothSetupObjectPerInstanceCollidable<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(&m_orderMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "orderMap",
                                        ),
                                    );
                                }
                                m_orderMap = _serde::__private::Some(
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_simulationMesh = match m_simulationMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationMesh",
                                ),
                            );
                        }
                    };
                    let m_collidableTransformSet = match m_collidableTransformSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidableTransformSet",
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
                    let m_specifyDensity = match m_specifyDensity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "specifyDensity",
                                ),
                            );
                        }
                    };
                    let m_vertexDensity = match m_vertexDensity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexDensity",
                                ),
                            );
                        }
                    };
                    let m_rescaleMass = match m_rescaleMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rescaleMass",
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
                    let m_particleMass = match m_particleMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleMass",
                                ),
                            );
                        }
                    };
                    let m_particleRadius = match m_particleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleRadius",
                                ),
                            );
                        }
                    };
                    let m_particleFriction = match m_particleFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleFriction",
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
                    let m_enablePinchDetection = match m_enablePinchDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enablePinchDetection",
                                ),
                            );
                        }
                    };
                    let m_pinchDetectionEnabledParticles = match m_pinchDetectionEnabledParticles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionEnabledParticles",
                                ),
                            );
                        }
                    };
                    let m_toAnimPeriod = match m_toAnimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toAnimPeriod",
                                ),
                            );
                        }
                    };
                    let m_toSimPeriod = match m_toSimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toSimPeriod",
                                ),
                            );
                        }
                    };
                    let m_drivePinchedParticlesToReferenceMesh = match m_drivePinchedParticlesToReferenceMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "drivePinchedParticlesToReferenceMesh",
                                ),
                            );
                        }
                    };
                    let m_pinchReferenceBufferSetup = match m_pinchReferenceBufferSetup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchReferenceBufferSetup",
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
                    let m_landscapeCollisionParticleSelection = match m_landscapeCollisionParticleSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionParticleSelection",
                                ),
                            );
                        }
                    };
                    let m_landscapeCollisionParticleRadius = match m_landscapeCollisionParticleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionParticleRadius",
                                ),
                            );
                        }
                    };
                    let m_enableStuckParticleDetection = match m_enableStuckParticleDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableStuckParticleDetection",
                                ),
                            );
                        }
                    };
                    let m_stuckParticlesStretchFactor = match m_stuckParticlesStretchFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stuckParticlesStretchFactor",
                                ),
                            );
                        }
                    };
                    let m_enableLandscapePinchDetection = match m_enableLandscapePinchDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableLandscapePinchDetection",
                                ),
                            );
                        }
                    };
                    let m_landscapePinchDetectionPriority = match m_landscapePinchDetectionPriority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapePinchDetectionPriority",
                                ),
                            );
                        }
                    };
                    let m_landscapePinchDetectionRadius = match m_landscapePinchDetectionRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapePinchDetectionRadius",
                                ),
                            );
                        }
                    };
                    let m_enableTransferMotion = match m_enableTransferMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableTransferMotion",
                                ),
                            );
                        }
                    };
                    let m_transferMotionSetupData = match m_transferMotionSetupData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionSetupData",
                                ),
                            );
                        }
                    };
                    let m_constraintSetSetups = match m_constraintSetSetups {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintSetSetups",
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
                    let m_orderMap = match m_orderMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("orderMap"),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSimClothSetupObject {
                        __ptr,
                        parent,
                        m_name,
                        m_simulationMesh,
                        m_collidableTransformSet,
                        m_gravity,
                        m_globalDampingPerSecond,
                        m_doNormals,
                        m_specifyDensity,
                        m_vertexDensity,
                        m_rescaleMass,
                        m_totalMass,
                        m_particleMass,
                        m_particleRadius,
                        m_particleFriction,
                        m_fixedParticles,
                        m_enablePinchDetection,
                        m_pinchDetectionEnabledParticles,
                        m_toAnimPeriod,
                        m_toSimPeriod,
                        m_drivePinchedParticlesToReferenceMesh,
                        m_pinchReferenceBufferSetup,
                        m_collisionTolerance,
                        m_landscapeCollisionParticleSelection,
                        m_landscapeCollisionParticleRadius,
                        m_enableStuckParticleDetection,
                        m_stuckParticlesStretchFactor,
                        m_enableLandscapePinchDetection,
                        m_landscapePinchDetectionPriority,
                        m_landscapePinchDetectionRadius,
                        m_enableTransferMotion,
                        m_transferMotionSetupData,
                        m_constraintSetSetups,
                        m_perInstanceCollidables,
                        m_orderMap,
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
                    let mut m_simulationMesh: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_collidableTransformSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_globalDampingPerSecond: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_doNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_specifyDensity: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_vertexDensity: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_rescaleMass: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_totalMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_particleMass: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_particleRadius: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_particleFriction: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_fixedParticles: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_enablePinchDetection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pinchDetectionEnabledParticles: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_toAnimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toSimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_drivePinchedParticlesToReferenceMesh: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_pinchReferenceBufferSetup: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_landscapeCollisionParticleSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_landscapeCollisionParticleRadius: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableStuckParticleDetection: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_stuckParticlesStretchFactor: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableLandscapePinchDetection: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_landscapePinchDetectionPriority: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_landscapePinchDetectionRadius: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableTransferMotion: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_transferMotionSetupData: _serde::__private::Option<
                        hclSimClothSetupObjectTransferMotionSetupData<'de>,
                    > = _serde::__private::None;
                    let mut m_constraintSetSetups: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_perInstanceCollidables: _serde::__private::Option<
                        Vec<hclSimClothSetupObjectPerInstanceCollidable<'de>>,
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
                            __Field::m_simulationMesh => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simulationMesh) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationMesh",
                                        ),
                                    );
                                }
                                m_simulationMesh = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collidableTransformSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collidableTransformSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidableTransformSet",
                                        ),
                                    );
                                }
                                m_collidableTransformSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                            __Field::m_specifyDensity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_specifyDensity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "specifyDensity",
                                        ),
                                    );
                                }
                                m_specifyDensity = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexDensity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexDensity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexDensity",
                                        ),
                                    );
                                }
                                m_vertexDensity = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rescaleMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rescaleMass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rescaleMass",
                                        ),
                                    );
                                }
                                m_rescaleMass = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                            __Field::m_particleMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleMass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleMass",
                                        ),
                                    );
                                }
                                m_particleMass = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particleRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleRadius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleRadius",
                                        ),
                                    );
                                }
                                m_particleRadius = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particleFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleFriction) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleFriction",
                                        ),
                                    );
                                }
                                m_particleFriction = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
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
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enablePinchDetection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enablePinchDetection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enablePinchDetection",
                                        ),
                                    );
                                }
                                m_enablePinchDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pinchDetectionEnabledParticles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionEnabledParticles,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionEnabledParticles",
                                        ),
                                    );
                                }
                                m_pinchDetectionEnabledParticles = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toAnimPeriod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toAnimPeriod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toAnimPeriod",
                                        ),
                                    );
                                }
                                m_toAnimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toSimPeriod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toSimPeriod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toSimPeriod",
                                        ),
                                    );
                                }
                                m_toSimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_drivePinchedParticlesToReferenceMesh => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_drivePinchedParticlesToReferenceMesh,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "drivePinchedParticlesToReferenceMesh",
                                        ),
                                    );
                                }
                                m_drivePinchedParticlesToReferenceMesh = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pinchReferenceBufferSetup => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pinchReferenceBufferSetup,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchReferenceBufferSetup",
                                        ),
                                    );
                                }
                                m_pinchReferenceBufferSetup = _serde::__private::Some(
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
                            __Field::m_landscapeCollisionParticleSelection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionParticleSelection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionParticleSelection",
                                        ),
                                    );
                                }
                                m_landscapeCollisionParticleSelection = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_landscapeCollisionParticleRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_landscapeCollisionParticleRadius,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeCollisionParticleRadius",
                                        ),
                                    );
                                }
                                m_landscapeCollisionParticleRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableStuckParticleDetection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableStuckParticleDetection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableStuckParticleDetection",
                                        ),
                                    );
                                }
                                m_enableStuckParticleDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stuckParticlesStretchFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_stuckParticlesStretchFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stuckParticlesStretchFactor",
                                        ),
                                    );
                                }
                                m_stuckParticlesStretchFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableLandscapePinchDetection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableLandscapePinchDetection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableLandscapePinchDetection",
                                        ),
                                    );
                                }
                                m_enableLandscapePinchDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_landscapePinchDetectionPriority => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_landscapePinchDetectionPriority,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapePinchDetectionPriority",
                                        ),
                                    );
                                }
                                m_landscapePinchDetectionPriority = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_landscapePinchDetectionRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_landscapePinchDetectionRadius,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapePinchDetectionRadius",
                                        ),
                                    );
                                }
                                m_landscapePinchDetectionRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableTransferMotion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableTransferMotion,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableTransferMotion",
                                        ),
                                    );
                                }
                                m_enableTransferMotion = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transferMotionSetupData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionSetupData,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionSetupData",
                                        ),
                                    );
                                }
                                m_transferMotionSetupData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimClothSetupObjectTransferMotionSetupData<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_constraintSetSetups => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_constraintSetSetups,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintSetSetups",
                                        ),
                                    );
                                }
                                m_constraintSetSetups = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                                    match __A::next_value::<
                                        Vec<hclSimClothSetupObjectPerInstanceCollidable<'de>>,
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
                    let m_simulationMesh = match m_simulationMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationMesh",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collidableTransformSet = match m_collidableTransformSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidableTransformSet",
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
                    let m_specifyDensity = match m_specifyDensity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "specifyDensity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexDensity = match m_vertexDensity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexDensity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rescaleMass = match m_rescaleMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rescaleMass",
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
                    let m_particleMass = match m_particleMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleMass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particleRadius = match m_particleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particleFriction = match m_particleFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleFriction",
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
                    let m_enablePinchDetection = match m_enablePinchDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enablePinchDetection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pinchDetectionEnabledParticles = match m_pinchDetectionEnabledParticles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionEnabledParticles",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toAnimPeriod = match m_toAnimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toAnimPeriod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toSimPeriod = match m_toSimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toSimPeriod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_drivePinchedParticlesToReferenceMesh = match m_drivePinchedParticlesToReferenceMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "drivePinchedParticlesToReferenceMesh",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pinchReferenceBufferSetup = match m_pinchReferenceBufferSetup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchReferenceBufferSetup",
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
                    let m_landscapeCollisionParticleSelection = match m_landscapeCollisionParticleSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionParticleSelection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_landscapeCollisionParticleRadius = match m_landscapeCollisionParticleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeCollisionParticleRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableStuckParticleDetection = match m_enableStuckParticleDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableStuckParticleDetection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stuckParticlesStretchFactor = match m_stuckParticlesStretchFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stuckParticlesStretchFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableLandscapePinchDetection = match m_enableLandscapePinchDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableLandscapePinchDetection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_landscapePinchDetectionPriority = match m_landscapePinchDetectionPriority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapePinchDetectionPriority",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_landscapePinchDetectionRadius = match m_landscapePinchDetectionRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapePinchDetectionRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableTransferMotion = match m_enableTransferMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableTransferMotion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transferMotionSetupData = match m_transferMotionSetupData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionSetupData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constraintSetSetups = match m_constraintSetSetups {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintSetSetups",
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
                    _serde::__private::Ok(hclSimClothSetupObject {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        m_simulationMesh,
                        m_collidableTransformSet,
                        m_gravity,
                        m_globalDampingPerSecond,
                        m_doNormals,
                        m_specifyDensity,
                        m_vertexDensity,
                        m_rescaleMass,
                        m_totalMass,
                        m_particleMass,
                        m_particleRadius,
                        m_particleFriction,
                        m_fixedParticles,
                        m_enablePinchDetection,
                        m_pinchDetectionEnabledParticles,
                        m_toAnimPeriod,
                        m_toSimPeriod,
                        m_drivePinchedParticlesToReferenceMesh,
                        m_pinchReferenceBufferSetup,
                        m_collisionTolerance,
                        m_landscapeCollisionParticleSelection,
                        m_landscapeCollisionParticleRadius,
                        m_enableStuckParticleDetection,
                        m_stuckParticlesStretchFactor,
                        m_enableLandscapePinchDetection,
                        m_landscapePinchDetectionPriority,
                        m_landscapePinchDetectionRadius,
                        m_enableTransferMotion,
                        m_transferMotionSetupData,
                        m_constraintSetSetups,
                        m_perInstanceCollidables,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "simulationMesh",
                "collidableTransformSet",
                "gravity",
                "globalDampingPerSecond",
                "doNormals",
                "specifyDensity",
                "vertexDensity",
                "rescaleMass",
                "totalMass",
                "particleMass",
                "particleRadius",
                "particleFriction",
                "fixedParticles",
                "enablePinchDetection",
                "pinchDetectionEnabledParticles",
                "toAnimPeriod",
                "toSimPeriod",
                "drivePinchedParticlesToReferenceMesh",
                "pinchReferenceBufferSetup",
                "collisionTolerance",
                "landscapeCollisionParticleSelection",
                "landscapeCollisionParticleRadius",
                "enableStuckParticleDetection",
                "stuckParticlesStretchFactor",
                "enableLandscapePinchDetection",
                "landscapePinchDetectionPriority",
                "landscapePinchDetectionRadius",
                "enableTransferMotion",
                "transferMotionSetupData",
                "constraintSetSetups",
                "perInstanceCollidables",
                "orderMap",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSimClothSetupObject",
                FIELDS,
                __hclSimClothSetupObjectVisitor {
                    marker: _serde::__private::PhantomData::<hclSimClothSetupObject>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
