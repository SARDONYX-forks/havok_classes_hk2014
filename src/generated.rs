/// Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
mod class_requires {
    pub use havok_serde::{
        ser::{Error as _, SerializeFlags, SerializeStruct, Serializer, TypeSize},
        de::{self, Error as _, Deserializer},
        HavokClass,
    };
    pub use havok_types::*;
    #[cfg(feature = "json_schema")]
    pub fn make_large_int_array_schema(
        generator: &mut schemars::SchemaGenerator,
    ) -> schemars::Schema {
        let mut schema = <Vec<i32> as schemars::JsonSchema>::json_schema(generator);
        let mut map = schema.ensure_object();
        schema
    }
}
pub mod BGSGamebryoSequenceGenerator_;
pub use BGSGamebryoSequenceGenerator_::*;
pub mod BSAlignBoneModifier_;
pub use BSAlignBoneModifier_::*;
pub mod BSAssignVariablesModifier_;
pub use BSAssignVariablesModifier_::*;
pub mod BSBehaviorGraphSwapGenerator_;
pub use BSBehaviorGraphSwapGenerator_::*;
pub mod BSBoneSwitchGenerator_;
pub use BSBoneSwitchGenerator_::*;
pub mod BSBoneSwitchGeneratorBoneData_;
pub use BSBoneSwitchGeneratorBoneData_::*;
pub mod BSComputeAddBoneAnimModifier_;
pub use BSComputeAddBoneAnimModifier_::*;
pub mod BSCyclicBlendTransitionGenerator_;
pub use BSCyclicBlendTransitionGenerator_::*;
pub mod BSDecomposeVectorModifier_;
pub use BSDecomposeVectorModifier_::*;
pub mod BSDirectAtCapturePoseModifier_;
pub use BSDirectAtCapturePoseModifier_::*;
pub mod BSDirectAtModifier_;
pub use BSDirectAtModifier_::*;
pub mod BSDistTriggerModifier_;
pub use BSDistTriggerModifier_::*;
pub mod BSEventEveryNEventsModifier_;
pub use BSEventEveryNEventsModifier_::*;
pub mod BSEventOnDeactivateModifier_;
pub use BSEventOnDeactivateModifier_::*;
pub mod BSEventOnFalseToTrueModifier_;
pub use BSEventOnFalseToTrueModifier_::*;
pub mod BSGetTimeStepModifier_;
pub use BSGetTimeStepModifier_::*;
pub mod BSInterpValueModifier_;
pub use BSInterpValueModifier_::*;
pub mod BSIsActiveModifier_;
pub use BSIsActiveModifier_::*;
pub mod BSIStateManagerModifier_;
pub use BSIStateManagerModifier_::*;
pub mod BSIStateManagerModifierBSiStateData_;
pub use BSIStateManagerModifierBSiStateData_::*;
pub mod BSIStateManagerModifierBSIStateManagerStateListener_;
pub use BSIStateManagerModifierBSIStateManagerStateListener_::*;
pub mod BSiStateTaggingGenerator_;
pub use BSiStateTaggingGenerator_::*;
pub mod BSiStateTaggingStateListener_;
pub use BSiStateTaggingStateListener_::*;
pub mod BSLimbCycleModifier_;
pub use BSLimbCycleModifier_::*;
pub mod BSLimbIKModifier_;
pub use BSLimbIKModifier_::*;
pub mod BSLookAtCapturePoseModifier_;
pub use BSLookAtCapturePoseModifier_::*;
pub mod BSLookAtModifier_;
pub use BSLookAtModifier_::*;
pub mod BSLookAtModifierBoneData_;
pub use BSLookAtModifierBoneData_::*;
pub mod BSModifyOnceModifier_;
pub use BSModifyOnceModifier_::*;
pub mod BSOffsetAnimationGenerator_;
pub use BSOffsetAnimationGenerator_::*;
pub mod BSPassByTargetTriggerModifier_;
pub use BSPassByTargetTriggerModifier_::*;
pub mod BSRagdollContactListenerModifier_;
pub use BSRagdollContactListenerModifier_::*;
pub mod BSRandomAlarmModifier_;
pub use BSRandomAlarmModifier_::*;
pub mod BSRetargetBodyBlend_;
pub use BSRetargetBodyBlend_::*;
pub mod BSReverseSpineTwistModifier_;
pub use BSReverseSpineTwistModifier_::*;
pub mod BSRootTwistModifier_;
pub use BSRootTwistModifier_::*;
pub mod BSSpeedSamplerModifier_;
pub use BSSpeedSamplerModifier_::*;
pub mod BSSpringState_;
pub use BSSpringState_::*;
pub mod BSTimerModifier_;
pub use BSTimerModifier_::*;
pub mod BSTweenerModifier_;
pub use BSTweenerModifier_::*;
pub mod DynamicAnimationTaggingGenerator_;
pub use DynamicAnimationTaggingGenerator_::*;
pub mod hclAction_;
pub use hclAction_::*;
pub mod hclAntiPinchConstraintSet_;
pub use hclAntiPinchConstraintSet_::*;
pub mod hclAntiPinchConstraintSetPerParticle_;
pub use hclAntiPinchConstraintSetPerParticle_::*;
pub mod hclBendLinkConstraintSet_;
pub use hclBendLinkConstraintSet_::*;
pub mod hclBendLinkConstraintSetLink_;
pub use hclBendLinkConstraintSetLink_::*;
pub mod hclBendLinkConstraintSetMx_;
pub use hclBendLinkConstraintSetMx_::*;
pub mod hclBendLinkConstraintSetMxBatch_;
pub use hclBendLinkConstraintSetMxBatch_::*;
pub mod hclBendLinkConstraintSetMxSingle_;
pub use hclBendLinkConstraintSetMxSingle_::*;
pub mod hclBendStiffnessConstraintSet_;
pub use hclBendStiffnessConstraintSet_::*;
pub mod hclBendStiffnessConstraintSetLink_;
pub use hclBendStiffnessConstraintSetLink_::*;
pub mod hclBendStiffnessConstraintSetMx_;
pub use hclBendStiffnessConstraintSetMx_::*;
pub mod hclBendStiffnessConstraintSetMxBatch_;
pub use hclBendStiffnessConstraintSetMxBatch_::*;
pub mod hclBendStiffnessConstraintSetMxSingle_;
pub use hclBendStiffnessConstraintSetMxSingle_::*;
pub mod hclBendStiffnessSetupObject_;
pub use hclBendStiffnessSetupObject_::*;
pub mod hclBlendSomeVerticesOperator_;
pub use hclBlendSomeVerticesOperator_::*;
pub mod hclBlendSomeVerticesOperatorBlendEntry_;
pub use hclBlendSomeVerticesOperatorBlendEntry_::*;
pub mod hclBonePlanesConstraintSet_;
pub use hclBonePlanesConstraintSet_::*;
pub mod hclBonePlanesConstraintSetBonePlane_;
pub use hclBonePlanesConstraintSetBonePlane_::*;
pub mod hclBoneSpaceDeformer_;
pub use hclBoneSpaceDeformer_::*;
pub mod hclBoneSpaceDeformerFourBlendEntryBlock_;
pub use hclBoneSpaceDeformerFourBlendEntryBlock_::*;
pub mod hclBoneSpaceDeformerLocalBlockP_;
pub use hclBoneSpaceDeformerLocalBlockP_::*;
pub mod hclBoneSpaceDeformerLocalBlockPN_;
pub use hclBoneSpaceDeformerLocalBlockPN_::*;
pub mod hclBoneSpaceDeformerLocalBlockPNT_;
pub use hclBoneSpaceDeformerLocalBlockPNT_::*;
pub mod hclBoneSpaceDeformerLocalBlockPNTB_;
pub use hclBoneSpaceDeformerLocalBlockPNTB_::*;
pub mod hclBoneSpaceDeformerLocalBlockUnpackedP_;
pub use hclBoneSpaceDeformerLocalBlockUnpackedP_::*;
pub mod hclBoneSpaceDeformerLocalBlockUnpackedPN_;
pub use hclBoneSpaceDeformerLocalBlockUnpackedPN_::*;
pub mod hclBoneSpaceDeformerLocalBlockUnpackedPNT_;
pub use hclBoneSpaceDeformerLocalBlockUnpackedPNT_::*;
pub mod hclBoneSpaceDeformerLocalBlockUnpackedPNTB_;
pub use hclBoneSpaceDeformerLocalBlockUnpackedPNTB_::*;
pub mod hclBoneSpaceDeformerOneBlendEntryBlock_;
pub use hclBoneSpaceDeformerOneBlendEntryBlock_::*;
pub mod hclBoneSpaceDeformerThreeBlendEntryBlock_;
pub use hclBoneSpaceDeformerThreeBlendEntryBlock_::*;
pub mod hclBoneSpaceDeformerTwoBlendEntryBlock_;
pub use hclBoneSpaceDeformerTwoBlendEntryBlock_::*;
pub mod hclBoneSpaceMeshMeshDeformOperator_;
pub use hclBoneSpaceMeshMeshDeformOperator_::*;
pub mod hclBoneSpaceMeshMeshDeformPNOperator_;
pub use hclBoneSpaceMeshMeshDeformPNOperator_::*;
pub mod hclBoneSpaceMeshMeshDeformPNTBOperator_;
pub use hclBoneSpaceMeshMeshDeformPNTBOperator_::*;
pub mod hclBoneSpaceMeshMeshDeformPNTOperator_;
pub use hclBoneSpaceMeshMeshDeformPNTOperator_::*;
pub mod hclBoneSpaceMeshMeshDeformPOperator_;
pub use hclBoneSpaceMeshMeshDeformPOperator_::*;
pub mod hclBoneSpaceSkinOperator_;
pub use hclBoneSpaceSkinOperator_::*;
pub mod hclBoneSpaceSkinPNOperator_;
pub use hclBoneSpaceSkinPNOperator_::*;
pub mod hclBoneSpaceSkinPNTBOperator_;
pub use hclBoneSpaceSkinPNTBOperator_::*;
pub mod hclBoneSpaceSkinPNTOperator_;
pub use hclBoneSpaceSkinPNTOperator_::*;
pub mod hclBoneSpaceSkinPOperator_;
pub use hclBoneSpaceSkinPOperator_::*;
pub mod hclBufferDefinition_;
pub use hclBufferDefinition_::*;
pub mod hclBufferLayout_;
pub use hclBufferLayout_::*;
pub mod hclBufferLayoutBufferElement_;
pub use hclBufferLayoutBufferElement_::*;
pub mod hclBufferLayoutSlot_;
pub use hclBufferLayoutSlot_::*;
pub mod hclBufferSetupObject_;
pub use hclBufferSetupObject_::*;
pub mod hclBufferUsage_;
pub use hclBufferUsage_::*;
pub mod hclCapsuleShape_;
pub use hclCapsuleShape_::*;
pub mod hclClothContainer_;
pub use hclClothContainer_::*;
pub mod hclClothData_;
pub use hclClothData_::*;
pub mod hclClothSetupContainer_;
pub use hclClothSetupContainer_::*;
pub mod hclClothSetupObject_;
pub use hclClothSetupObject_::*;
pub mod hclClothState_;
pub use hclClothState_::*;
pub mod hclClothStateBufferAccess_;
pub use hclClothStateBufferAccess_::*;
pub mod hclClothStateSetupObject_;
pub use hclClothStateSetupObject_::*;
pub mod hclClothStateTransformSetAccess_;
pub use hclClothStateTransformSetAccess_::*;
pub mod hclCollidable_;
pub use hclCollidable_::*;
pub mod hclCompressibleLinkConstraintSet_;
pub use hclCompressibleLinkConstraintSet_::*;
pub mod hclCompressibleLinkConstraintSetLink_;
pub use hclCompressibleLinkConstraintSetLink_::*;
pub mod hclCompressibleLinkConstraintSetMx_;
pub use hclCompressibleLinkConstraintSetMx_::*;
pub mod hclCompressibleLinkConstraintSetMxBatch_;
pub use hclCompressibleLinkConstraintSetMxBatch_::*;
pub mod hclCompressibleLinkConstraintSetMxSingle_;
pub use hclCompressibleLinkConstraintSetMxSingle_::*;
pub mod hclConstraintSet_;
pub use hclConstraintSet_::*;
pub mod hclConstraintSetSetupObject_;
pub use hclConstraintSetSetupObject_::*;
pub mod hclConvexGeometryShape_;
pub use hclConvexGeometryShape_::*;
pub mod hclConvexHeightFieldShape_;
pub use hclConvexHeightFieldShape_::*;
pub mod hclConvexPlanesShape_;
pub use hclConvexPlanesShape_::*;
pub mod hclCopyVerticesOperator_;
pub use hclCopyVerticesOperator_::*;
pub mod hclEdgeSelectionInput_;
pub use hclEdgeSelectionInput_::*;
pub mod hclGatherAllVerticesOperator_;
pub use hclGatherAllVerticesOperator_::*;
pub mod hclGatherSomeVerticesOperator_;
pub use hclGatherSomeVerticesOperator_::*;
pub mod hclGatherSomeVerticesOperatorVertexPair_;
pub use hclGatherSomeVerticesOperatorVertexPair_::*;
pub mod hclInputConvertOperator_;
pub use hclInputConvertOperator_::*;
pub mod hclLocalRangeConstraintSet_;
pub use hclLocalRangeConstraintSet_::*;
pub mod hclLocalRangeConstraintSetLocalConstraint_;
pub use hclLocalRangeConstraintSetLocalConstraint_::*;
pub mod hclLocalRangeSetupObject_;
pub use hclLocalRangeSetupObject_::*;
pub mod hclMeshBoneDeformOperator_;
pub use hclMeshBoneDeformOperator_::*;
pub mod hclMeshBoneDeformOperatorTriangleBonePair_;
pub use hclMeshBoneDeformOperatorTriangleBonePair_::*;
pub mod hclMeshBoneDeformSetupObject_;
pub use hclMeshBoneDeformSetupObject_::*;
pub mod hclMeshMeshDeformOperator_;
pub use hclMeshMeshDeformOperator_::*;
pub mod hclMeshMeshDeformOperatorTriangleVertexPair_;
pub use hclMeshMeshDeformOperatorTriangleVertexPair_::*;
pub mod hclMoveFixedParticlesSetupObject_;
pub use hclMoveFixedParticlesSetupObject_::*;
pub mod hclMoveParticlesOperator_;
pub use hclMoveParticlesOperator_::*;
pub mod hclMoveParticlesOperatorVertexParticlePair_;
pub use hclMoveParticlesOperatorVertexParticlePair_::*;
pub mod hclNamedSetupMesh_;
pub use hclNamedSetupMesh_::*;
pub mod hclNamedTransformSetSetupObject_;
pub use hclNamedTransformSetSetupObject_::*;
pub mod hclObjectSpaceDeformer_;
pub use hclObjectSpaceDeformer_::*;
pub mod hclObjectSpaceDeformerFourBlendEntryBlock_;
pub use hclObjectSpaceDeformerFourBlendEntryBlock_::*;
pub mod hclObjectSpaceDeformerLocalBlockP_;
pub use hclObjectSpaceDeformerLocalBlockP_::*;
pub mod hclObjectSpaceDeformerLocalBlockPN_;
pub use hclObjectSpaceDeformerLocalBlockPN_::*;
pub mod hclObjectSpaceDeformerLocalBlockPNT_;
pub use hclObjectSpaceDeformerLocalBlockPNT_::*;
pub mod hclObjectSpaceDeformerLocalBlockPNTB_;
pub use hclObjectSpaceDeformerLocalBlockPNTB_::*;
pub mod hclObjectSpaceDeformerLocalBlockUnpackedP_;
pub use hclObjectSpaceDeformerLocalBlockUnpackedP_::*;
pub mod hclObjectSpaceDeformerLocalBlockUnpackedPN_;
pub use hclObjectSpaceDeformerLocalBlockUnpackedPN_::*;
pub mod hclObjectSpaceDeformerLocalBlockUnpackedPNT_;
pub use hclObjectSpaceDeformerLocalBlockUnpackedPNT_::*;
pub mod hclObjectSpaceDeformerLocalBlockUnpackedPNTB_;
pub use hclObjectSpaceDeformerLocalBlockUnpackedPNTB_::*;
pub mod hclObjectSpaceDeformerOneBlendEntryBlock_;
pub use hclObjectSpaceDeformerOneBlendEntryBlock_::*;
pub mod hclObjectSpaceDeformerThreeBlendEntryBlock_;
pub use hclObjectSpaceDeformerThreeBlendEntryBlock_::*;
pub mod hclObjectSpaceDeformerTwoBlendEntryBlock_;
pub use hclObjectSpaceDeformerTwoBlendEntryBlock_::*;
pub mod hclObjectSpaceMeshMeshDeformOperator_;
pub use hclObjectSpaceMeshMeshDeformOperator_::*;
pub mod hclObjectSpaceMeshMeshDeformPNOperator_;
pub use hclObjectSpaceMeshMeshDeformPNOperator_::*;
pub mod hclObjectSpaceMeshMeshDeformPNTBOperator_;
pub use hclObjectSpaceMeshMeshDeformPNTBOperator_::*;
pub mod hclObjectSpaceMeshMeshDeformPNTOperator_;
pub use hclObjectSpaceMeshMeshDeformPNTOperator_::*;
pub mod hclObjectSpaceMeshMeshDeformPOperator_;
pub use hclObjectSpaceMeshMeshDeformPOperator_::*;
pub mod hclObjectSpaceSkinOperator_;
pub use hclObjectSpaceSkinOperator_::*;
pub mod hclObjectSpaceSkinPNOperator_;
pub use hclObjectSpaceSkinPNOperator_::*;
pub mod hclObjectSpaceSkinPNTBOperator_;
pub use hclObjectSpaceSkinPNTBOperator_::*;
pub mod hclObjectSpaceSkinPNTOperator_;
pub use hclObjectSpaceSkinPNTOperator_::*;
pub mod hclObjectSpaceSkinPOperator_;
pub use hclObjectSpaceSkinPOperator_::*;
pub mod hclOperator_;
pub use hclOperator_::*;
pub mod hclOperatorSetupObject_;
pub use hclOperatorSetupObject_::*;
pub mod hclOutputConvertOperator_;
pub use hclOutputConvertOperator_::*;
pub mod hclPlaneShape_;
pub use hclPlaneShape_::*;
pub mod hclRuntimeConversionInfo_;
pub use hclRuntimeConversionInfo_::*;
pub mod hclRuntimeConversionInfoElementConversion_;
pub use hclRuntimeConversionInfoElementConversion_::*;
pub mod hclRuntimeConversionInfoSlotConversion_;
pub use hclRuntimeConversionInfoSlotConversion_::*;
pub mod hclScratchBufferDefinition_;
pub use hclScratchBufferDefinition_::*;
pub mod hclScratchBufferSetupObject_;
pub use hclScratchBufferSetupObject_::*;
pub mod hclSetupMesh_;
pub use hclSetupMesh_::*;
pub mod hclShadowBufferDefinition_;
pub use hclShadowBufferDefinition_::*;
pub mod hclShape_;
pub use hclShape_::*;
pub mod hclSimClothBufferSetupObject_;
pub use hclSimClothBufferSetupObject_::*;
pub mod hclSimClothData_;
pub use hclSimClothData_::*;
pub mod hclSimClothDataCollidablePinchingData_;
pub use hclSimClothDataCollidablePinchingData_::*;
pub mod hclSimClothDataCollidableTransformMap_;
pub use hclSimClothDataCollidableTransformMap_::*;
pub mod hclSimClothDataLandscapeCollisionData_;
pub use hclSimClothDataLandscapeCollisionData_::*;
pub mod hclSimClothDataOverridableSimulationInfo_;
pub use hclSimClothDataOverridableSimulationInfo_::*;
pub mod hclSimClothDataParticleData_;
pub use hclSimClothDataParticleData_::*;
pub mod hclSimClothDataTransferMotionData_;
pub use hclSimClothDataTransferMotionData_::*;
pub mod hclSimClothPose_;
pub use hclSimClothPose_::*;
pub mod hclSimClothSetupObject_;
pub use hclSimClothSetupObject_::*;
pub mod hclSimClothSetupObjectPerInstanceCollidable_;
pub use hclSimClothSetupObjectPerInstanceCollidable_::*;
pub mod hclSimClothSetupObjectTransferMotionSetupData_;
pub use hclSimClothSetupObjectTransferMotionSetupData_::*;
pub mod hclSimpleMeshBoneDeformOperator_;
pub use hclSimpleMeshBoneDeformOperator_::*;
pub mod hclSimpleMeshBoneDeformOperatorTriangleBonePair_;
pub use hclSimpleMeshBoneDeformOperatorTriangleBonePair_::*;
pub mod hclSimpleWindAction_;
pub use hclSimpleWindAction_::*;
pub mod hclSimulateOperator_;
pub use hclSimulateOperator_::*;
pub mod hclSimulateSetupObject_;
pub use hclSimulateSetupObject_::*;
pub mod hclSimulationSetupMesh_;
pub use hclSimulationSetupMesh_::*;
pub mod hclSimulationSetupMeshMapOptions_;
pub use hclSimulationSetupMeshMapOptions_::*;
pub mod hclSkinOperator_;
pub use hclSkinOperator_::*;
pub mod hclSkinOperatorBoneInfluence_;
pub use hclSkinOperatorBoneInfluence_::*;
pub mod hclSkinSetupObject_;
pub use hclSkinSetupObject_::*;
pub mod hclSphereShape_;
pub use hclSphereShape_::*;
pub mod hclStandardLinkConstraintSet_;
pub use hclStandardLinkConstraintSet_::*;
pub mod hclStandardLinkConstraintSetLink_;
pub use hclStandardLinkConstraintSetLink_::*;
pub mod hclStandardLinkConstraintSetMx_;
pub use hclStandardLinkConstraintSetMx_::*;
pub mod hclStandardLinkConstraintSetMxBatch_;
pub use hclStandardLinkConstraintSetMxBatch_::*;
pub mod hclStandardLinkConstraintSetMxSingle_;
pub use hclStandardLinkConstraintSetMxSingle_::*;
pub mod hclStandardLinkSetupObject_;
pub use hclStandardLinkSetupObject_::*;
pub mod hclStaticDisplayBufferSetupObject_;
pub use hclStaticDisplayBufferSetupObject_::*;
pub mod hclStaticShadowBufferDefinition_;
pub use hclStaticShadowBufferDefinition_::*;
pub mod hclStretchLinkConstraintSet_;
pub use hclStretchLinkConstraintSet_::*;
pub mod hclStretchLinkConstraintSetLink_;
pub use hclStretchLinkConstraintSetLink_::*;
pub mod hclStretchLinkConstraintSetMx_;
pub use hclStretchLinkConstraintSetMx_::*;
pub mod hclStretchLinkConstraintSetMxBatch_;
pub use hclStretchLinkConstraintSetMxBatch_::*;
pub mod hclStretchLinkConstraintSetMxSingle_;
pub use hclStretchLinkConstraintSetMxSingle_::*;
pub mod hclStretchLinkSetupObject_;
pub use hclStretchLinkSetupObject_::*;
pub mod hclTaperedCapsuleShape_;
pub use hclTaperedCapsuleShape_::*;
pub mod hclTransformSetDefinition_;
pub use hclTransformSetDefinition_::*;
pub mod hclTransformSetSetupObject_;
pub use hclTransformSetSetupObject_::*;
pub mod hclTransformSetUsage_;
pub use hclTransformSetUsage_::*;
pub mod hclTransformSetUsageTransformTracker_;
pub use hclTransformSetUsageTransformTracker_::*;
pub mod hclTransitionConstraintSet_;
pub use hclTransitionConstraintSet_::*;
pub mod hclTransitionConstraintSetPerParticle_;
pub use hclTransitionConstraintSetPerParticle_::*;
pub mod hclTriangleSelectionInput_;
pub use hclTriangleSelectionInput_::*;
pub mod hclUpdateAllVertexFramesOperator_;
pub use hclUpdateAllVertexFramesOperator_::*;
pub mod hclUpdateSomeVertexFramesOperator_;
pub use hclUpdateSomeVertexFramesOperator_::*;
pub mod hclUpdateSomeVertexFramesOperatorTriangle_;
pub use hclUpdateSomeVertexFramesOperatorTriangle_::*;
pub mod hclVertexFloatInput_;
pub use hclVertexFloatInput_::*;
pub mod hclVertexGatherSetupObject_;
pub use hclVertexGatherSetupObject_::*;
pub mod hclVertexSelectionInput_;
pub use hclVertexSelectionInput_::*;
pub mod hclVolumeConstraint_;
pub use hclVolumeConstraint_::*;
pub mod hclVolumeConstraintApplyData_;
pub use hclVolumeConstraintApplyData_::*;
pub mod hclVolumeConstraintFrameData_;
pub use hclVolumeConstraintFrameData_::*;
pub mod hclVolumeConstraintMx_;
pub use hclVolumeConstraintMx_::*;
pub mod hclVolumeConstraintMxApplyBatchData_;
pub use hclVolumeConstraintMxApplyBatchData_::*;
pub mod hclVolumeConstraintMxApplySingleData_;
pub use hclVolumeConstraintMxApplySingleData_::*;
pub mod hclVolumeConstraintMxFrameBatchData_;
pub use hclVolumeConstraintMxFrameBatchData_::*;
pub mod hclVolumeConstraintMxFrameSingleData_;
pub use hclVolumeConstraintMxFrameSingleData_::*;
pub mod hclVolumeConstraintSetupObject_;
pub use hclVolumeConstraintSetupObject_::*;
pub mod hkAabb_;
pub use hkAabb_::*;
pub mod hkAabb16_;
pub use hkAabb16_::*;
pub mod hkAabbHalf_;
pub use hkAabbHalf_::*;
pub mod hkAabbUint32_;
pub use hkAabbUint32_::*;
pub mod hkaAngularReferenceFrame_;
pub use hkaAngularReferenceFrame_::*;
pub mod hkaAnimatedReferenceFrame_;
pub use hkaAnimatedReferenceFrame_::*;
pub mod hkaAnimation_;
pub use hkaAnimation_::*;
pub mod hkaAnimationBinding_;
pub use hkaAnimationBinding_::*;
pub mod hkaAnimationContainer_;
pub use hkaAnimationContainer_::*;
pub mod hkaAnimationPreviewColorContainer_;
pub use hkaAnimationPreviewColorContainer_::*;
pub mod hkaAnnotationTrack_;
pub use hkaAnnotationTrack_::*;
pub mod hkaAnnotationTrackAnnotation_;
pub use hkaAnnotationTrackAnnotation_::*;
pub mod hkaBone_;
pub use hkaBone_::*;
pub mod hkaBoneAttachment_;
pub use hkaBoneAttachment_::*;
pub mod hkaDefaultAnimatedReferenceFrame_;
pub use hkaDefaultAnimatedReferenceFrame_::*;
pub mod hkaDirectionalReferenceFrame_;
pub use hkaDirectionalReferenceFrame_::*;
pub mod hkaFootstepAnalysisInfo_;
pub use hkaFootstepAnalysisInfo_::*;
pub mod hkaFootstepAnalysisInfoContainer_;
pub use hkaFootstepAnalysisInfoContainer_::*;
pub mod hkaInterleavedUncompressedAnimation_;
pub use hkaInterleavedUncompressedAnimation_::*;
pub mod hkaLosslessCompressedAnimation_;
pub use hkaLosslessCompressedAnimation_::*;
pub mod hkaMeshBinding_;
pub use hkaMeshBinding_::*;
pub mod hkaMeshBindingMapping_;
pub use hkaMeshBindingMapping_::*;
pub mod hkaParameterizedAnimationReferenceFrame_;
pub use hkaParameterizedAnimationReferenceFrame_::*;
pub mod hkaParameterizedReferenceFrame_;
pub use hkaParameterizedReferenceFrame_::*;
pub mod hkaPredictiveCompressedAnimation_;
pub use hkaPredictiveCompressedAnimation_::*;
pub mod hkaPredictiveCompressedAnimationTrackCompressionParams_;
pub use hkaPredictiveCompressedAnimationTrackCompressionParams_::*;
pub mod hkaQuantizedAnimation_;
pub use hkaQuantizedAnimation_::*;
pub mod hkaQuantizedAnimationTrackCompressionParams_;
pub use hkaQuantizedAnimationTrackCompressionParams_::*;
pub mod hkaReferencePoseAnimation_;
pub use hkaReferencePoseAnimation_::*;
pub mod hkArrayTypeAttribute_;
pub use hkArrayTypeAttribute_::*;
pub mod hkaSkeleton_;
pub use hkaSkeleton_::*;
pub mod hkaSkeletonLocalFrameOnBone_;
pub use hkaSkeletonLocalFrameOnBone_::*;
pub mod hkaSkeletonMapper_;
pub use hkaSkeletonMapper_::*;
pub mod hkaSkeletonMapperData_;
pub use hkaSkeletonMapperData_::*;
pub mod hkaSkeletonMapperDataChainMapping_;
pub use hkaSkeletonMapperDataChainMapping_::*;
pub mod hkaSkeletonMapperDataPartitionMappingRange_;
pub use hkaSkeletonMapperDataPartitionMappingRange_::*;
pub mod hkaSkeletonMapperDataSimpleMapping_;
pub use hkaSkeletonMapperDataSimpleMapping_::*;
pub mod hkaSkeletonPartition_;
pub use hkaSkeletonPartition_::*;
pub mod hkaSplineCompressedAnimation_;
pub use hkaSplineCompressedAnimation_::*;
pub mod hkaSplineCompressedAnimationAnimationCompressionParams_;
pub use hkaSplineCompressedAnimationAnimationCompressionParams_::*;
pub mod hkaSplineCompressedAnimationTrackCompressionParams_;
pub use hkaSplineCompressedAnimationTrackCompressionParams_::*;
pub mod hkbAiControlCancelPathCommand_;
pub use hkbAiControlCancelPathCommand_::*;
pub mod hkbAiControlControlData_;
pub use hkbAiControlControlData_::*;
pub mod hkbAiControlControlDataBlendable_;
pub use hkbAiControlControlDataBlendable_::*;
pub mod hkbAiControlControlDataNonBlendable_;
pub use hkbAiControlControlDataNonBlendable_::*;
pub mod hkbAiControlPathToCommand_;
pub use hkbAiControlPathToCommand_::*;
pub mod hkbAlignBoneModifier_;
pub use hkbAlignBoneModifier_::*;
pub mod hkbAnimatedSkeletonGenerator_;
pub use hkbAnimatedSkeletonGenerator_::*;
pub mod hkBaseObject_;
pub use hkBaseObject_::*;
pub mod hkbAssetBundle_;
pub use hkbAssetBundle_::*;
pub mod hkbAssetBundleStringData_;
pub use hkbAssetBundleStringData_::*;
pub mod hkbAttachmentModifier_;
pub use hkbAttachmentModifier_::*;
pub mod hkbAttachmentSetup_;
pub use hkbAttachmentSetup_::*;
pub mod hkbAttributeModifier_;
pub use hkbAttributeModifier_::*;
pub mod hkbAttributeModifierAssignment_;
pub use hkbAttributeModifierAssignment_::*;
pub mod hkbAuxiliaryNodeInfo_;
pub use hkbAuxiliaryNodeInfo_::*;
pub mod hkbBehaviorEventsInfo_;
pub use hkbBehaviorEventsInfo_::*;
pub mod hkbBehaviorGraph_;
pub use hkbBehaviorGraph_::*;
pub mod hkbBehaviorGraphData_;
pub use hkbBehaviorGraphData_::*;
pub mod hkbBehaviorGraphInternalState_;
pub use hkbBehaviorGraphInternalState_::*;
pub mod hkbBehaviorGraphInternalStateInfo_;
pub use hkbBehaviorGraphInternalStateInfo_::*;
pub mod hkbBehaviorGraphStringData_;
pub use hkbBehaviorGraphStringData_::*;
pub mod hkbBehaviorInfo_;
pub use hkbBehaviorInfo_::*;
pub mod hkbBehaviorInfoIdToNamePair_;
pub use hkbBehaviorInfoIdToNamePair_::*;
pub mod hkbBehaviorReferenceGenerator_;
pub use hkbBehaviorReferenceGenerator_::*;
pub mod hkbBindable_;
pub use hkbBindable_::*;
pub mod hkbBlendCurveUtils_;
pub use hkbBlendCurveUtils_::*;
pub mod hkbBlenderGenerator_;
pub use hkbBlenderGenerator_::*;
pub mod hkbBlenderGeneratorChild_;
pub use hkbBlenderGeneratorChild_::*;
pub mod hkbBlenderGeneratorChildInternalState_;
pub use hkbBlenderGeneratorChildInternalState_::*;
pub mod hkbBlenderGeneratorInternalState_;
pub use hkbBlenderGeneratorInternalState_::*;
pub mod hkbBlendingTransitionEffect_;
pub use hkbBlendingTransitionEffect_::*;
pub mod hkbBlendingTransitionEffectInternalState_;
pub use hkbBlendingTransitionEffectInternalState_::*;
pub mod hkbBoneIndexArray_;
pub use hkbBoneIndexArray_::*;
pub mod hkbBoneWeightArray_;
pub use hkbBoneWeightArray_::*;
pub mod hkbBoolVariableSequencedData_;
pub use hkbBoolVariableSequencedData_::*;
pub mod hkbBoolVariableSequencedDataSample_;
pub use hkbBoolVariableSequencedDataSample_::*;
pub mod hkbCameraShakeEventPayload_;
pub use hkbCameraShakeEventPayload_::*;
pub mod hkbCameraVariablesChangedCommand_;
pub use hkbCameraVariablesChangedCommand_::*;
pub mod hkbCharacter_;
pub use hkbCharacter_::*;
pub mod hkbCharacterAddedInfo_;
pub use hkbCharacterAddedInfo_::*;
pub mod hkbCharacterControlCommand_;
pub use hkbCharacterControlCommand_::*;
pub mod hkbCharacterControllerModifier_;
pub use hkbCharacterControllerModifier_::*;
pub mod hkbCharacterControllerModifierControlData_;
pub use hkbCharacterControllerModifierControlData_::*;
pub mod hkbCharacterControllerModifierInternalState_;
pub use hkbCharacterControllerModifierInternalState_::*;
pub mod hkbCharacterControllerSetup_;
pub use hkbCharacterControllerSetup_::*;
pub mod hkbCharacterData_;
pub use hkbCharacterData_::*;
pub mod hkbCharacterInfo_;
pub use hkbCharacterInfo_::*;
pub mod hkbCharacterSelectedInfo_;
pub use hkbCharacterSelectedInfo_::*;
pub mod hkbCharacterSetup_;
pub use hkbCharacterSetup_::*;
pub mod hkbCharacterSkinInfo_;
pub use hkbCharacterSkinInfo_::*;
pub mod hkbCharacterSteppedInfo_;
pub use hkbCharacterSteppedInfo_::*;
pub mod hkbCharacterStringData_;
pub use hkbCharacterStringData_::*;
pub mod hkbCharacterStringDataFileNameMeshNamePair_;
pub use hkbCharacterStringDataFileNameMeshNamePair_::*;
pub mod hkbClientCharacterState_;
pub use hkbClientCharacterState_::*;
pub mod hkbClipGenerator_;
pub use hkbClipGenerator_::*;
pub mod hkbClipGeneratorEcho_;
pub use hkbClipGeneratorEcho_::*;
pub mod hkbClipGeneratorInternalState_;
pub use hkbClipGeneratorInternalState_::*;
pub mod hkbClipTrigger_;
pub use hkbClipTrigger_::*;
pub mod hkbClipTriggerArray_;
pub use hkbClipTriggerArray_::*;
pub mod hkbCombineTransformsModifier_;
pub use hkbCombineTransformsModifier_::*;
pub mod hkbCombineTransformsModifierInternalState_;
pub use hkbCombineTransformsModifierInternalState_::*;
pub mod hkbCompiledExpressionSet_;
pub use hkbCompiledExpressionSet_::*;
pub mod hkbCompiledExpressionSetToken_;
pub use hkbCompiledExpressionSetToken_::*;
pub mod hkbComputeDirectionModifier_;
pub use hkbComputeDirectionModifier_::*;
pub mod hkbComputeDirectionModifierInternalState_;
pub use hkbComputeDirectionModifierInternalState_::*;
pub mod hkbComputeRotationFromAxisAngleModifier_;
pub use hkbComputeRotationFromAxisAngleModifier_::*;
pub mod hkbComputeRotationFromAxisAngleModifierInternalState_;
pub use hkbComputeRotationFromAxisAngleModifierInternalState_::*;
pub mod hkbComputeRotationToTargetModifier_;
pub use hkbComputeRotationToTargetModifier_::*;
pub mod hkbComputeRotationToTargetModifierInternalState_;
pub use hkbComputeRotationToTargetModifierInternalState_::*;
pub mod hkbComputeWorldFromModelModifier_;
pub use hkbComputeWorldFromModelModifier_::*;
pub mod hkbCondition_;
pub use hkbCondition_::*;
pub mod hkbConstraintSetup_;
pub use hkbConstraintSetup_::*;
pub mod hkbContext_;
pub use hkbContext_::*;
pub mod hkbCustomIdSelector_;
pub use hkbCustomIdSelector_::*;
pub mod hkbCustomTestGenerator_;
pub use hkbCustomTestGenerator_::*;
pub mod hkbCustomTestGeneratorAnnotatedTypes_;
pub use hkbCustomTestGeneratorAnnotatedTypes_::*;
pub mod hkbCustomTestGeneratorBoneTypes_;
pub use hkbCustomTestGeneratorBoneTypes_::*;
pub mod hkbCustomTestGeneratorComplexTypes_;
pub use hkbCustomTestGeneratorComplexTypes_::*;
pub mod hkbCustomTestGeneratorHiddenTypes_;
pub use hkbCustomTestGeneratorHiddenTypes_::*;
pub mod hkbCustomTestGeneratorNestedTypes_;
pub use hkbCustomTestGeneratorNestedTypes_::*;
pub mod hkbCustomTestGeneratorNestedTypesBase_;
pub use hkbCustomTestGeneratorNestedTypesBase_::*;
pub mod hkbCustomTestGeneratorSimpleTypes_;
pub use hkbCustomTestGeneratorSimpleTypes_::*;
pub mod hkbDampingModifier_;
pub use hkbDampingModifier_::*;
pub mod hkbDampingModifierInternalState_;
pub use hkbDampingModifierInternalState_::*;
pub mod hkbDelayedModifier_;
pub use hkbDelayedModifier_::*;
pub mod hkbDelayedModifierInternalState_;
pub use hkbDelayedModifierInternalState_::*;
pub mod hkbDetectCloseToGroundModifier_;
pub use hkbDetectCloseToGroundModifier_::*;
pub mod hkbDetectCloseToGroundModifierInternalState_;
pub use hkbDetectCloseToGroundModifierInternalState_::*;
pub mod hkbDockingGenerator_;
pub use hkbDockingGenerator_::*;
pub mod hkbDockingGeneratorInternalState_;
pub use hkbDockingGeneratorInternalState_::*;
pub mod hkbEvaluateExpressionModifier_;
pub use hkbEvaluateExpressionModifier_::*;
pub mod hkbEvaluateExpressionModifierInternalExpressionData_;
pub use hkbEvaluateExpressionModifierInternalExpressionData_::*;
pub mod hkbEvaluateExpressionModifierInternalState_;
pub use hkbEvaluateExpressionModifierInternalState_::*;
pub mod hkbEvaluateHandleModifier_;
pub use hkbEvaluateHandleModifier_::*;
pub mod hkbEvent_;
pub use hkbEvent_::*;
pub mod hkbEventBase_;
pub use hkbEventBase_::*;
pub mod hkbEventDrivenModifier_;
pub use hkbEventDrivenModifier_::*;
pub mod hkbEventDrivenModifierInternalState_;
pub use hkbEventDrivenModifierInternalState_::*;
pub mod hkbEventInfo_;
pub use hkbEventInfo_::*;
pub mod hkbEventPayload_;
pub use hkbEventPayload_::*;
pub mod hkbEventPayloadList_;
pub use hkbEventPayloadList_::*;
pub mod hkbEventProperty_;
pub use hkbEventProperty_::*;
pub mod hkbEventRaisedInfo_;
pub use hkbEventRaisedInfo_::*;
pub mod hkbEventRangeData_;
pub use hkbEventRangeData_::*;
pub mod hkbEventRangeDataArray_;
pub use hkbEventRangeDataArray_::*;
pub mod hkbEventSequencedData_;
pub use hkbEventSequencedData_::*;
pub mod hkbEventSequencedDataSequencedEvent_;
pub use hkbEventSequencedDataSequencedEvent_::*;
pub mod hkbEventsFromRangeModifier_;
pub use hkbEventsFromRangeModifier_::*;
pub mod hkbEventsFromRangeModifierInternalState_;
pub use hkbEventsFromRangeModifierInternalState_::*;
pub mod hkbExpressionCondition_;
pub use hkbExpressionCondition_::*;
pub mod hkbExpressionData_;
pub use hkbExpressionData_::*;
pub mod hkbExpressionDataArray_;
pub use hkbExpressionDataArray_::*;
pub mod hkbExtractRagdollPoseModifier_;
pub use hkbExtractRagdollPoseModifier_::*;
pub mod hkbExtrapolatingTransitionEffect_;
pub use hkbExtrapolatingTransitionEffect_::*;
pub mod hkbExtrapolatingTransitionEffectInternalState_;
pub use hkbExtrapolatingTransitionEffectInternalState_::*;
pub mod hkbFootIkControlData_;
pub use hkbFootIkControlData_::*;
pub mod hkbFootIkControlsModifier_;
pub use hkbFootIkControlsModifier_::*;
pub mod hkbFootIkControlsModifierLeg_;
pub use hkbFootIkControlsModifierLeg_::*;
pub mod hkbFootIkDriverInfo_;
pub use hkbFootIkDriverInfo_::*;
pub mod hkbFootIkDriverInfoLeg_;
pub use hkbFootIkDriverInfoLeg_::*;
pub mod hkbFootIkGains_;
pub use hkbFootIkGains_::*;
pub mod hkbFootIkModifier_;
pub use hkbFootIkModifier_::*;
pub mod hkbFootIkModifierInternalLegData_;
pub use hkbFootIkModifierInternalLegData_::*;
pub mod hkbFootIkModifierLeg_;
pub use hkbFootIkModifierLeg_::*;
pub mod hkbGenerator_;
pub use hkbGenerator_::*;
pub mod hkbGeneratorOutputListener_;
pub use hkbGeneratorOutputListener_::*;
pub mod hkbGeneratorPartitionInfo_;
pub use hkbGeneratorPartitionInfo_::*;
pub mod hkbGeneratorSyncInfo_;
pub use hkbGeneratorSyncInfo_::*;
pub mod hkbGeneratorSyncInfoActiveInterval_;
pub use hkbGeneratorSyncInfoActiveInterval_::*;
pub mod hkbGeneratorSyncInfoSyncPoint_;
pub use hkbGeneratorSyncInfoSyncPoint_::*;
pub mod hkbGeneratorTransitionEffect_;
pub use hkbGeneratorTransitionEffect_::*;
pub mod hkbGeneratorTransitionEffectInternalState_;
pub use hkbGeneratorTransitionEffectInternalState_::*;
pub mod hkbGetHandleOnBoneModifier_;
pub use hkbGetHandleOnBoneModifier_::*;
pub mod hkbGetUpModifier_;
pub use hkbGetUpModifier_::*;
pub mod hkbGetUpModifierInternalState_;
pub use hkbGetUpModifierInternalState_::*;
pub mod hkbGetWorldFromModelModifier_;
pub use hkbGetWorldFromModelModifier_::*;
pub mod hkbGetWorldFromModelModifierInternalState_;
pub use hkbGetWorldFromModelModifierInternalState_::*;
pub mod hkbGravityModifier_;
pub use hkbGravityModifier_::*;
pub mod hkbHandIkControlData_;
pub use hkbHandIkControlData_::*;
pub mod hkbHandIkControlsModifier_;
pub use hkbHandIkControlsModifier_::*;
pub mod hkbHandIkControlsModifierHand_;
pub use hkbHandIkControlsModifierHand_::*;
pub mod hkbHandIkDriverInfo_;
pub use hkbHandIkDriverInfo_::*;
pub mod hkbHandIkDriverInfoHand_;
pub use hkbHandIkDriverInfoHand_::*;
pub mod hkbHandIkModifier_;
pub use hkbHandIkModifier_::*;
pub mod hkbHandIkModifierHand_;
pub use hkbHandIkModifierHand_::*;
pub mod hkbHandle_;
pub use hkbHandle_::*;
pub mod hkbHoldFromBlendingTransitionEffect_;
pub use hkbHoldFromBlendingTransitionEffect_::*;
pub mod hkbIntEventPayload_;
pub use hkbIntEventPayload_::*;
pub mod hkbIntVariableSequencedData_;
pub use hkbIntVariableSequencedData_::*;
pub mod hkbIntVariableSequencedDataSample_;
pub use hkbIntVariableSequencedDataSample_::*;
pub mod hkBitField_;
pub use hkBitField_::*;
pub mod hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_;
pub use hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_::*;
pub mod hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_;
pub use hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_::*;
pub mod hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_;
pub use hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_::*;
pub mod hkbJigglerGroup_;
pub use hkbJigglerGroup_::*;
pub mod hkbJigglerModifier_;
pub use hkbJigglerModifier_::*;
pub mod hkbJigglerModifierInternalState_;
pub use hkbJigglerModifierInternalState_::*;
pub mod hkbKeyframeBonesModifier_;
pub use hkbKeyframeBonesModifier_::*;
pub mod hkbKeyframeBonesModifierKeyframeInfo_;
pub use hkbKeyframeBonesModifierKeyframeInfo_::*;
pub mod hkbKeyFrameControlData_;
pub use hkbKeyFrameControlData_::*;
pub mod hkbLayer_;
pub use hkbLayer_::*;
pub mod hkbLayerGenerator_;
pub use hkbLayerGenerator_::*;
pub mod hkbLayerGeneratorInternalState_;
pub use hkbLayerGeneratorInternalState_::*;
pub mod hkbLayerGeneratorLayerInternalState_;
pub use hkbLayerGeneratorLayerInternalState_::*;
pub mod hkbLinkedSymbolInfo_;
pub use hkbLinkedSymbolInfo_::*;
pub mod hkbLookAtModifier_;
pub use hkbLookAtModifier_::*;
pub mod hkbLookAtModifierInternalState_;
pub use hkbLookAtModifierInternalState_::*;
pub mod hkbManualSelectorGenerator_;
pub use hkbManualSelectorGenerator_::*;
pub mod hkbManualSelectorGeneratorInternalState_;
pub use hkbManualSelectorGeneratorInternalState_::*;
pub mod hkbManualSelectorTransitionEffect_;
pub use hkbManualSelectorTransitionEffect_::*;
pub mod hkbManualSelectorTransitionEffectInternalState_;
pub use hkbManualSelectorTransitionEffectInternalState_::*;
pub mod hkbMessageLog_;
pub use hkbMessageLog_::*;
pub mod hkbMirroredSkeletonInfo_;
pub use hkbMirroredSkeletonInfo_::*;
pub mod hkbMirrorModifier_;
pub use hkbMirrorModifier_::*;
pub mod hkbModifier_;
pub use hkbModifier_::*;
pub mod hkbModifierGenerator_;
pub use hkbModifierGenerator_::*;
pub mod hkbModifierList_;
pub use hkbModifierList_::*;
pub mod hkbModifierWrapper_;
pub use hkbModifierWrapper_::*;
pub mod hkbMoveBoneAttachmentModifier_;
pub use hkbMoveBoneAttachmentModifier_::*;
pub mod hkbMoveCharacterModifier_;
pub use hkbMoveCharacterModifier_::*;
pub mod hkbMoveCharacterModifierInternalState_;
pub use hkbMoveCharacterModifierInternalState_::*;
pub mod hkbNamedEventPayload_;
pub use hkbNamedEventPayload_::*;
pub mod hkbNamedIntEventPayload_;
pub use hkbNamedIntEventPayload_::*;
pub mod hkbNamedRealEventPayload_;
pub use hkbNamedRealEventPayload_::*;
pub mod hkbNamedStringEventPayload_;
pub use hkbNamedStringEventPayload_::*;
pub mod hkbNode_;
pub use hkbNode_::*;
pub mod hkbNodeInternalStateInfo_;
pub use hkbNodeInternalStateInfo_::*;
pub mod hkbnpPhysicsInterface_;
pub use hkbnpPhysicsInterface_::*;
pub mod hkbnpRagdollInterface_;
pub use hkbnpRagdollInterface_::*;
pub mod hkbNullPhysicsInterface_;
pub use hkbNullPhysicsInterface_::*;
pub mod hkbParametricMotionGenerator_;
pub use hkbParametricMotionGenerator_::*;
pub mod hkbParticleSystemEventPayload_;
pub use hkbParticleSystemEventPayload_::*;
pub mod hkbPhysicsInterface_;
pub use hkbPhysicsInterface_::*;
pub mod hkbPinBonesGenerator_;
pub use hkbPinBonesGenerator_::*;
pub mod hkbPoseMatchingGenerator_;
pub use hkbPoseMatchingGenerator_::*;
pub mod hkbPoseMatchingGeneratorInternalState_;
pub use hkbPoseMatchingGeneratorInternalState_::*;
pub mod hkbPoseStoringGeneratorOutputListener_;
pub use hkbPoseStoringGeneratorOutputListener_::*;
pub mod hkbPoseStoringGeneratorOutputListenerStoredPose_;
pub use hkbPoseStoringGeneratorOutputListenerStoredPose_::*;
pub mod hkbPoweredRagdollControlData_;
pub use hkbPoweredRagdollControlData_::*;
pub mod hkbPoweredRagdollControlsModifier_;
pub use hkbPoweredRagdollControlsModifier_::*;
pub mod hkbProceduralBlenderGenerator_;
pub use hkbProceduralBlenderGenerator_::*;
pub mod hkbProjectData_;
pub use hkbProjectData_::*;
pub mod hkbProjectStringData_;
pub use hkbProjectStringData_::*;
pub mod hkbRadialSelectorGenerator_;
pub use hkbRadialSelectorGenerator_::*;
pub mod hkbRadialSelectorGeneratorGeneratorInfo_;
pub use hkbRadialSelectorGeneratorGeneratorInfo_::*;
pub mod hkbRadialSelectorGeneratorGeneratorPair_;
pub use hkbRadialSelectorGeneratorGeneratorPair_::*;
pub mod hkbRagdollControllerSetup_;
pub use hkbRagdollControllerSetup_::*;
pub mod hkbRagdollInterface_;
pub use hkbRagdollInterface_::*;
pub mod hkbRaiseEventCommand_;
pub use hkbRaiseEventCommand_::*;
pub mod hkbRealEventPayload_;
pub use hkbRealEventPayload_::*;
pub mod hkbRealVariableSequencedData_;
pub use hkbRealVariableSequencedData_::*;
pub mod hkbRealVariableSequencedDataSample_;
pub use hkbRealVariableSequencedDataSample_::*;
pub mod hkbRebuildScriptsCommand_;
pub use hkbRebuildScriptsCommand_::*;
pub mod hkbReferencedGeneratorSyncInfo_;
pub use hkbReferencedGeneratorSyncInfo_::*;
pub mod hkbReferencePoseGenerator_;
pub use hkbReferencePoseGenerator_::*;
pub mod hkbRigidBodyRagdollControlData_;
pub use hkbRigidBodyRagdollControlData_::*;
pub mod hkbRigidBodyRagdollControlsModifier_;
pub use hkbRigidBodyRagdollControlsModifier_::*;
pub mod hkbRigidBodySetup_;
pub use hkbRigidBodySetup_::*;
pub mod hkbRoleAttribute_;
pub use hkbRoleAttribute_::*;
pub mod hkbRotateCharacterModifier_;
pub use hkbRotateCharacterModifier_::*;
pub mod hkbRotateCharacterModifierInternalState_;
pub use hkbRotateCharacterModifierInternalState_::*;
pub mod hkbScriptCondition_;
pub use hkbScriptCondition_::*;
pub mod hkbScriptGenerator_;
pub use hkbScriptGenerator_::*;
pub mod hkbSenseHandleModifier_;
pub use hkbSenseHandleModifier_::*;
pub mod hkbSenseHandleModifierRange_;
pub use hkbSenseHandleModifierRange_::*;
pub mod hkbSequence_;
pub use hkbSequence_::*;
pub mod hkbSequencedData_;
pub use hkbSequencedData_::*;
pub mod hkbSequenceInternalState_;
pub use hkbSequenceInternalState_::*;
pub mod hkbSequenceStringData_;
pub use hkbSequenceStringData_::*;
pub mod hkbSetBehaviorCommand_;
pub use hkbSetBehaviorCommand_::*;
pub mod hkbSetLocalTimeOfClipGeneratorCommand_;
pub use hkbSetLocalTimeOfClipGeneratorCommand_::*;
pub mod hkbSetNodePropertyCommand_;
pub use hkbSetNodePropertyCommand_::*;
pub mod hkbSetSelectedCharacterCommand_;
pub use hkbSetSelectedCharacterCommand_::*;
pub mod hkbSetWordVariableCommand_;
pub use hkbSetWordVariableCommand_::*;
pub mod hkbSetWorldFromModelModifier_;
pub use hkbSetWorldFromModelModifier_::*;
pub mod hkbShapeSetup_;
pub use hkbShapeSetup_::*;
pub mod hkbSimulationControlCommand_;
pub use hkbSimulationControlCommand_::*;
pub mod hkbSimulationStateInfo_;
pub use hkbSimulationStateInfo_::*;
pub mod hkbStateChooser_;
pub use hkbStateChooser_::*;
pub mod hkbStateChooserWrapper_;
pub use hkbStateChooserWrapper_::*;
pub mod hkbStateDependentModifier_;
pub use hkbStateDependentModifier_::*;
pub mod hkbStateListener_;
pub use hkbStateListener_::*;
pub mod hkbStateMachine_;
pub use hkbStateMachine_::*;
pub mod hkbStateMachineActiveTransitionInfo_;
pub use hkbStateMachineActiveTransitionInfo_::*;
pub mod hkbStateMachineDelayedTransitionInfo_;
pub use hkbStateMachineDelayedTransitionInfo_::*;
pub mod hkbStateMachineEventPropertyArray_;
pub use hkbStateMachineEventPropertyArray_::*;
pub mod hkbStateMachineInternalState_;
pub use hkbStateMachineInternalState_::*;
pub mod hkbStateMachineNestedStateMachineData_;
pub use hkbStateMachineNestedStateMachineData_::*;
pub mod hkbStateMachineProspectiveTransitionInfo_;
pub use hkbStateMachineProspectiveTransitionInfo_::*;
pub mod hkbStateMachineStateInfo_;
pub use hkbStateMachineStateInfo_::*;
pub mod hkbStateMachineTimeInterval_;
pub use hkbStateMachineTimeInterval_::*;
pub mod hkbStateMachineTransitionInfo_;
pub use hkbStateMachineTransitionInfo_::*;
pub mod hkbStateMachineTransitionInfoArray_;
pub use hkbStateMachineTransitionInfoArray_::*;
pub mod hkbStateMachineTransitionInfoReference_;
pub use hkbStateMachineTransitionInfoReference_::*;
pub mod hkbStringCondition_;
pub use hkbStringCondition_::*;
pub mod hkbStringEventPayload_;
pub use hkbStringEventPayload_::*;
pub mod hkbTestIdSelector_;
pub use hkbTestIdSelector_::*;
pub mod hkbTimerModifier_;
pub use hkbTimerModifier_::*;
pub mod hkbTimerModifierInternalState_;
pub use hkbTimerModifierInternalState_::*;
pub mod hkbToolNodeType_;
pub use hkbToolNodeType_::*;
pub mod hkbTransformVectorModifier_;
pub use hkbTransformVectorModifier_::*;
pub mod hkbTransformVectorModifierInternalState_;
pub use hkbTransformVectorModifierInternalState_::*;
pub mod hkbTransitionEffect_;
pub use hkbTransitionEffect_::*;
pub mod hkbTwistModifier_;
pub use hkbTwistModifier_::*;
pub mod hkbVariableBindingSet_;
pub use hkbVariableBindingSet_::*;
pub mod hkbVariableBindingSetBinding_;
pub use hkbVariableBindingSetBinding_::*;
pub mod hkbVariableBounds_;
pub use hkbVariableBounds_::*;
pub mod hkbVariableInfo_;
pub use hkbVariableInfo_::*;
pub mod hkbVariableValue_;
pub use hkbVariableValue_::*;
pub mod hkbVariableValueSet_;
pub use hkbVariableValueSet_::*;
pub mod hkbWorldEnums_;
pub use hkbWorldEnums_::*;
pub mod hkbWorldFromModelModeData_;
pub use hkbWorldFromModelModeData_::*;
pub mod hkcdAdf_;
pub use hkcdAdf_::*;
pub mod hkcdDynamicAabbTree_;
pub use hkcdDynamicAabbTree_::*;
pub mod hkcdDynamicTreeAnisotropicMetric_;
pub use hkcdDynamicTreeAnisotropicMetric_::*;
pub mod hkcdDynamicTreeCodec18_;
pub use hkcdDynamicTreeCodec18_::*;
pub mod hkcdDynamicTreeCodec32_;
pub use hkcdDynamicTreeCodec32_::*;
pub mod hkcdDynamicTreeCodecInt16_;
pub use hkcdDynamicTreeCodecInt16_::*;
pub mod hkcdDynamicTreeCodecInt16IntAabb_;
pub use hkcdDynamicTreeCodecInt16IntAabb_::*;
pub mod hkcdDynamicTreeCodecRawUint_;
pub use hkcdDynamicTreeCodecRawUint_::*;
pub mod hkcdDynamicTreeCodecRawUlong_;
pub use hkcdDynamicTreeCodecRawUlong_::*;
pub mod hkcdDynamicTreeCodecRawunsignedint_;
pub use hkcdDynamicTreeCodecRawunsignedint_::*;
pub mod hkcdDynamicTreeCodecRawunsignedlong_;
pub use hkcdDynamicTreeCodecRawunsignedlong_::*;
pub mod hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32_;
pub use hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32_::*;
pub mod hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16_;
pub use hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16_::*;
pub mod hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint_;
pub use hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint_::*;
pub mod hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong_;
pub use hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong_::*;
pub mod hkcdDynamicTreeDefaultTree32Storage_;
pub use hkcdDynamicTreeDefaultTree32Storage_::*;
pub mod hkcdDynamicTreeDefaultTree48Storage_;
pub use hkcdDynamicTreeDefaultTree48Storage_::*;
pub mod hkcdDynamicTreeDefaultTreeInt16Storage_;
pub use hkcdDynamicTreeDefaultTreeInt16Storage_::*;
pub mod hkcdDynamicTreeDefaultTreePtrStorage_;
pub use hkcdDynamicTreeDefaultTreePtrStorage_::*;
pub mod hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32_;
pub use hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32_::*;
pub mod hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16_;
pub use hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16_::*;
pub mod hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint_;
pub use hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint_::*;
pub mod hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong_;
pub use hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong_::*;
pub mod hkcdDynamicTreeDynamicStorage16_;
pub use hkcdDynamicTreeDynamicStorage16_::*;
pub mod hkcdDynamicTreeDynamicStorage32_;
pub use hkcdDynamicTreeDynamicStorage32_::*;
pub mod hkcdDynamicTreeDynamicStorageInt16_;
pub use hkcdDynamicTreeDynamicStorageInt16_::*;
pub mod hkcdDynamicTreeDynamicStoragePtr_;
pub use hkcdDynamicTreeDynamicStoragePtr_::*;
pub mod hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16_;
pub use hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16_::*;
pub mod hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32_;
pub use hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32_::*;
pub mod hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16_;
pub use hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16_::*;
pub mod hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr_;
pub use hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr_::*;
pub mod hkcdFourAabb_;
pub use hkcdFourAabb_::*;
pub mod hkcdPlanarCsgOperand_;
pub use hkcdPlanarCsgOperand_::*;
pub mod hkcdPlanarCsgOperandGeomSource_;
pub use hkcdPlanarCsgOperandGeomSource_::*;
pub mod hkcdPlanarEntity_;
pub use hkcdPlanarEntity_::*;
pub mod hkcdPlanarGeometry_;
pub use hkcdPlanarGeometry_::*;
pub mod hkcdPlanarGeometryPlanesCollection_;
pub use hkcdPlanarGeometryPlanesCollection_::*;
pub mod hkcdPlanarGeometryPolygonCollection_;
pub use hkcdPlanarGeometryPolygonCollection_::*;
pub mod hkcdPlanarGeometryPrimitivesCollection28_;
pub use hkcdPlanarGeometryPrimitivesCollection28_::*;
pub mod hkcdPlanarGeometryPrimitivesPlane_;
pub use hkcdPlanarGeometryPrimitivesPlane_::*;
pub mod hkcdPlanarSolid_;
pub use hkcdPlanarSolid_::*;
pub mod hkcdPlanarSolidNode_;
pub use hkcdPlanarSolidNode_::*;
pub mod hkcdPlanarSolidNodeStorage_;
pub use hkcdPlanarSolidNodeStorage_::*;
pub mod hkcdShape_;
pub use hkcdShape_::*;
pub mod hkcdShapeDispatchType_;
pub use hkcdShapeDispatchType_::*;
pub mod hkcdShapeInfoCodecType_;
pub use hkcdShapeInfoCodecType_::*;
pub mod hkcdShapeType_;
pub use hkcdShapeType_::*;
pub mod hkcdSimdTree_;
pub use hkcdSimdTree_::*;
pub mod hkcdSimdTreeNode_;
pub use hkcdSimdTreeNode_::*;
pub mod hkcdStaticAabbTree_;
pub use hkcdStaticAabbTree_::*;
pub mod hkcdStaticMeshTreeBase_;
pub use hkcdStaticMeshTreeBase_::*;
pub mod hkcdStaticMeshTreeBasePrimitive_;
pub use hkcdStaticMeshTreeBasePrimitive_::*;
pub mod hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData_;
pub use hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData_::*;
pub mod hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort_;
pub use hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort_::*;
pub mod hkcdStaticMeshTreeBaseSection_;
pub use hkcdStaticMeshTreeBaseSection_::*;
pub mod hkcdStaticMeshTreeBaseSectionDataRuns_;
pub use hkcdStaticMeshTreeBaseSectionDataRuns_::*;
pub mod hkcdStaticMeshTreeBaseSectionPrimitives_;
pub use hkcdStaticMeshTreeBaseSectionPrimitives_::*;
pub mod hkcdStaticMeshTreeBaseSectionSharedVertices_;
pub use hkcdStaticMeshTreeBaseSectionSharedVertices_::*;
pub mod hkcdStaticMeshTreeDefaultDataRun_;
pub use hkcdStaticMeshTreeDefaultDataRun_::*;
pub mod hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun_;
pub use hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun_::*;
pub mod hkcdStaticPvs_;
pub use hkcdStaticPvs_::*;
pub mod hkcdStaticPvsBlockHeader_;
pub use hkcdStaticPvsBlockHeader_::*;
pub mod hkcdStaticTreeCodec3Axis_;
pub use hkcdStaticTreeCodec3Axis_::*;
pub mod hkcdStaticTreeCodec3Axis4_;
pub use hkcdStaticTreeCodec3Axis4_::*;
pub mod hkcdStaticTreeCodec3Axis5_;
pub use hkcdStaticTreeCodec3Axis5_::*;
pub mod hkcdStaticTreeCodec3Axis6_;
pub use hkcdStaticTreeCodec3Axis6_::*;
pub mod hkcdStaticTreeCodecRaw_;
pub use hkcdStaticTreeCodecRaw_::*;
pub mod hkcdStaticTreeDefaultTreeStorage32_;
pub use hkcdStaticTreeDefaultTreeStorage32_::*;
pub mod hkcdStaticTreeDefaultTreeStorage4_;
pub use hkcdStaticTreeDefaultTreeStorage4_::*;
pub mod hkcdStaticTreeDefaultTreeStorage5_;
pub use hkcdStaticTreeDefaultTreeStorage5_::*;
pub mod hkcdStaticTreeDefaultTreeStorage6_;
pub use hkcdStaticTreeDefaultTreeStorage6_::*;
pub mod hkcdStaticTreeDynamicStorage32_;
pub use hkcdStaticTreeDynamicStorage32_::*;
pub mod hkcdStaticTreeDynamicStorage4_;
pub use hkcdStaticTreeDynamicStorage4_::*;
pub mod hkcdStaticTreeDynamicStorage5_;
pub use hkcdStaticTreeDynamicStorage5_::*;
pub mod hkcdStaticTreeDynamicStorage6_;
pub use hkcdStaticTreeDynamicStorage6_::*;
pub mod hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4_;
pub use hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4_::*;
pub mod hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5_;
pub use hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5_::*;
pub mod hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6_;
pub use hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6_::*;
pub mod hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw_;
pub use hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw_::*;
pub mod hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32_;
pub use hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32_::*;
pub mod hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4_;
pub use hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4_::*;
pub mod hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5_;
pub use hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5_::*;
pub mod hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6_;
pub use hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6_::*;
pub mod hkClass_;
pub use hkClass_::*;
pub mod hkClassEnum_;
pub use hkClassEnum_::*;
pub mod hkClassEnumItem_;
pub use hkClassEnumItem_::*;
pub mod hkClassMember_;
pub use hkClassMember_::*;
pub mod hkCompressedMassProperties_;
pub use hkCompressedMassProperties_::*;
pub mod hkContactPoint_;
pub use hkContactPoint_::*;
pub mod hkContactPointMaterial_;
pub use hkContactPointMaterial_::*;
pub mod hkCustomAttributes_;
pub use hkCustomAttributes_::*;
pub mod hkCustomAttributesAttribute_;
pub use hkCustomAttributesAttribute_::*;
pub mod hkDataObjectTypeAttribute_;
pub use hkDataObjectTypeAttribute_::*;
pub mod hkDefaultMemoryTracker_;
pub use hkDefaultMemoryTracker_::*;
pub mod hkDescriptionAttribute_;
pub use hkDescriptionAttribute_::*;
pub mod hkDocumentationAttribute_;
pub use hkDocumentationAttribute_::*;
pub mod hkFloat16_;
pub use hkFloat16_::*;
pub mod hkFloat16Transform_;
pub use hkFloat16Transform_::*;
pub mod hkFourTransposedPointsf_;
pub use hkFourTransposedPointsf_::*;
pub mod hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations_;
pub use hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations_::*;
pub mod hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations_;
pub use hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations_::*;
pub mod hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance_;
pub use hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance_::*;
pub mod hkGeometry_;
pub use hkGeometry_::*;
pub mod hkGeometryTriangle_;
pub use hkGeometryTriangle_::*;
pub mod hkGizmoAttribute_;
pub use hkGizmoAttribute_::*;
pub mod hkIndexedTransformSet_;
pub use hkIndexedTransformSet_::*;
pub mod hkIntRealPair_;
pub use hkIntRealPair_::*;
pub mod hkLinkAttribute_;
pub use hkLinkAttribute_::*;
pub mod hkLocalFrame_;
pub use hkLocalFrame_::*;
pub mod hkLocalFrameGroup_;
pub use hkLocalFrameGroup_::*;
pub mod hkMassProperties_;
pub use hkMassProperties_::*;
pub mod hkMemoryMeshBody_;
pub use hkMemoryMeshBody_::*;
pub mod hkMemoryMeshMaterial_;
pub use hkMemoryMeshMaterial_::*;
pub mod hkMemoryMeshShape_;
pub use hkMemoryMeshShape_::*;
pub mod hkMemoryMeshShapeSection_;
pub use hkMemoryMeshShapeSection_::*;
pub mod hkMemoryMeshTexture_;
pub use hkMemoryMeshTexture_::*;
pub mod hkMemoryMeshVertexBuffer_;
pub use hkMemoryMeshVertexBuffer_::*;
pub mod hkMemoryResourceContainer_;
pub use hkMemoryResourceContainer_::*;
pub mod hkMemoryResourceHandle_;
pub use hkMemoryResourceHandle_::*;
pub mod hkMemoryResourceHandleExternalLink_;
pub use hkMemoryResourceHandleExternalLink_::*;
pub mod hkMeshBody_;
pub use hkMeshBody_::*;
pub mod hkMeshBoneIndexMapping_;
pub use hkMeshBoneIndexMapping_::*;
pub mod hkMeshMaterial_;
pub use hkMeshMaterial_::*;
pub mod hkMeshSection_;
pub use hkMeshSection_::*;
pub mod hkMeshSectionCinfo_;
pub use hkMeshSectionCinfo_::*;
pub mod hkMeshShape_;
pub use hkMeshShape_::*;
pub mod hkMeshTexture_;
pub use hkMeshTexture_::*;
pub mod hkMeshTextureRawBufferDescriptor_;
pub use hkMeshTextureRawBufferDescriptor_::*;
pub mod hkMeshVertexBuffer_;
pub use hkMeshVertexBuffer_::*;
pub mod hkModelerNodeTypeAttribute_;
pub use hkModelerNodeTypeAttribute_::*;
pub mod hkMonitorStreamColorTable_;
pub use hkMonitorStreamColorTable_::*;
pub mod hkMonitorStreamColorTableColorPair_;
pub use hkMonitorStreamColorTableColorPair_::*;
pub mod hkMonitorStreamFrameInfo_;
pub use hkMonitorStreamFrameInfo_::*;
pub mod hkMonitorStreamStringMap_;
pub use hkMonitorStreamStringMap_::*;
pub mod hkMonitorStreamStringMapStringMap_;
pub use hkMonitorStreamStringMapStringMap_::*;
pub mod hkMotionState_;
pub use hkMotionState_::*;
pub mod hkMultipleVertexBuffer_;
pub use hkMultipleVertexBuffer_::*;
pub mod hkMultipleVertexBufferElementInfo_;
pub use hkMultipleVertexBufferElementInfo_::*;
pub mod hkMultipleVertexBufferLockedElement_;
pub use hkMultipleVertexBufferLockedElement_::*;
pub mod hkMultipleVertexBufferVertexBufferInfo_;
pub use hkMultipleVertexBufferVertexBufferInfo_::*;
pub mod hkMultiThreadCheck_;
pub use hkMultiThreadCheck_::*;
pub mod hknpAction_;
pub use hknpAction_::*;
pub mod hknpArrayAction_;
pub use hknpArrayAction_::*;
pub mod hknpBallGun_;
pub use hknpBallGun_::*;
pub mod hknpBinaryAction_;
pub use hknpBinaryAction_::*;
pub mod hknpBody_;
pub use hknpBody_::*;
pub mod hknpBodyCinfo_;
pub use hknpBodyCinfo_::*;
pub mod hknpBodyQuality_;
pub use hknpBodyQuality_::*;
pub mod hknpBodyQualityLibrary_;
pub use hknpBodyQualityLibrary_::*;
pub mod hknpBodyReference_;
pub use hknpBodyReference_::*;
pub mod hknpBreakableConstraintData_;
pub use hknpBreakableConstraintData_::*;
pub mod hknpBridgeConstraintAtom_;
pub use hknpBridgeConstraintAtom_::*;
pub mod hknpBroadPhaseConfig_;
pub use hknpBroadPhaseConfig_::*;
pub mod hknpBroadPhaseConfigLayer_;
pub use hknpBroadPhaseConfigLayer_::*;
pub mod hknpBSMaterial_;
pub use hknpBSMaterial_::*;
pub mod hknpBSMaterialProperties_;
pub use hknpBSMaterialProperties_::*;
pub mod hknpCapsuleShape_;
pub use hknpCapsuleShape_::*;
pub mod hknpCharacterProxyCinfo_;
pub use hknpCharacterProxyCinfo_::*;
pub mod hknpCharacterRigidBodyCinfo_;
pub use hknpCharacterRigidBodyCinfo_::*;
pub mod hknpCircularSurfaceVelocity_;
pub use hknpCircularSurfaceVelocity_::*;
pub mod hknpCollisionDispatchType_;
pub use hknpCollisionDispatchType_::*;
pub mod hknpCollisionFilter_;
pub use hknpCollisionFilter_::*;
pub mod hknpCompositeShape_;
pub use hknpCompositeShape_::*;
pub mod hknpCompoundShape_;
pub use hknpCompoundShape_::*;
pub mod hknpCompoundShapeInternalsKeyMask_;
pub use hknpCompoundShapeInternalsKeyMask_::*;
pub mod hknpCompoundShapeKeyMask_;
pub use hknpCompoundShapeKeyMask_::*;
pub mod hknpCompressedHeightFieldShape_;
pub use hknpCompressedHeightFieldShape_::*;
pub mod hknpCompressedMeshShape_;
pub use hknpCompressedMeshShape_::*;
pub mod hknpCompressedMeshShapeData_;
pub use hknpCompressedMeshShapeData_::*;
pub mod hknpCompressedMeshShapeInternalsKeyMask_;
pub use hknpCompressedMeshShapeInternalsKeyMask_::*;
pub mod hknpCompressedMeshShapeTree_;
pub use hknpCompressedMeshShapeTree_::*;
pub mod hknpCompressedMeshShapeTreeDataRun_;
pub use hknpCompressedMeshShapeTreeDataRun_::*;
pub mod hknpCompressedMeshShapeTreeDataRunData_;
pub use hknpCompressedMeshShapeTreeDataRunData_::*;
pub mod hknpConstraint_;
pub use hknpConstraint_::*;
pub mod hknpConstraintCinfo_;
pub use hknpConstraintCinfo_::*;
pub mod hknpConstraintCollisionFilter_;
pub use hknpConstraintCollisionFilter_::*;
pub mod hknpConvexPolytopeShape_;
pub use hknpConvexPolytopeShape_::*;
pub mod hknpConvexPolytopeShapeFace_;
pub use hknpConvexPolytopeShapeFace_::*;
pub mod hknpConvexShape_;
pub use hknpConvexShape_::*;
pub mod hknpDecoratorShape_;
pub use hknpDecoratorShape_::*;
pub mod hknpDefaultBroadPhaseConfig_;
pub use hknpDefaultBroadPhaseConfig_::*;
pub mod hknpDefaultExternMeshShapeGeometry_;
pub use hknpDefaultExternMeshShapeGeometry_::*;
pub mod hknpDestructionShapeProperties_;
pub use hknpDestructionShapeProperties_::*;
pub mod hknpDisableCollisionFilter_;
pub use hknpDisableCollisionFilter_::*;
pub mod hknpDummyShape_;
pub use hknpDummyShape_::*;
pub mod hknpDynamicCompoundShape_;
pub use hknpDynamicCompoundShape_::*;
pub mod hknpDynamicCompoundShapeData_;
pub use hknpDynamicCompoundShapeData_::*;
pub mod hknpDynamicCompoundShapeTree_;
pub use hknpDynamicCompoundShapeTree_::*;
pub mod hknpExternMeshShape_;
pub use hknpExternMeshShape_::*;
pub mod hknpExternMeshShapeData_;
pub use hknpExternMeshShapeData_::*;
pub mod hknpExternMeshShapeGeometry_;
pub use hknpExternMeshShapeGeometry_::*;
pub mod hknpFirstPersonGun_;
pub use hknpFirstPersonGun_::*;
pub mod hknpGroupCollisionFilter_;
pub use hknpGroupCollisionFilter_::*;
pub mod hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516_;
pub use hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516_::*;
pub mod hknpHeightFieldShape_;
pub use hknpHeightFieldShape_::*;
pub mod hknpLinearSurfaceVelocity_;
pub use hknpLinearSurfaceVelocity_::*;
pub mod hknpLodShape_;
pub use hknpLodShape_::*;
pub mod hknpLodShapeLevelOfDetailInfo_;
pub use hknpLodShapeLevelOfDetailInfo_::*;
pub mod hknpMalleableConstraintData_;
pub use hknpMalleableConstraintData_::*;
pub mod hknpMaskedShape_;
pub use hknpMaskedShape_::*;
pub mod hknpMaskedShapeMaskWrapper_;
pub use hknpMaskedShapeMaskWrapper_::*;
pub mod hknpMaterial_;
pub use hknpMaterial_::*;
pub mod hknpMaterialDescriptor_;
pub use hknpMaterialDescriptor_::*;
pub mod hknpMaterialLibrary_;
pub use hknpMaterialLibrary_::*;
pub mod hknpMaterialPalette_;
pub use hknpMaterialPalette_::*;
pub mod hknpMinMaxQuadTree_;
pub use hknpMinMaxQuadTree_::*;
pub mod hknpMinMaxQuadTreeMinMaxLevel_;
pub use hknpMinMaxQuadTreeMinMaxLevel_::*;
pub mod hknpMotion_;
pub use hknpMotion_::*;
pub mod hknpMotionCinfo_;
pub use hknpMotionCinfo_::*;
pub mod hknpMotionProperties_;
pub use hknpMotionProperties_::*;
pub mod hknpMotionPropertiesLibrary_;
pub use hknpMotionPropertiesLibrary_::*;
pub mod hknpPairCollisionFilter_;
pub use hknpPairCollisionFilter_::*;
pub mod hknpPairCollisionFilterMapPairFilterKeyOverrideType_;
pub use hknpPairCollisionFilterMapPairFilterKeyOverrideType_::*;
pub mod hknpPhysicsSceneData_;
pub use hknpPhysicsSceneData_::*;
pub mod hknpPhysicsSystemData_;
pub use hknpPhysicsSystemData_::*;
pub mod hknpProjectileGun_;
pub use hknpProjectileGun_::*;
pub mod hknpRagdollData_;
pub use hknpRagdollData_::*;
pub mod hknpRagdollKeyFrameHierarchyUtilityControlData_;
pub use hknpRagdollKeyFrameHierarchyUtilityControlData_::*;
pub mod hknpRefMaterial_;
pub use hknpRefMaterial_::*;
pub mod hknpRefWorldCinfo_;
pub use hknpRefWorldCinfo_::*;
pub mod hknpScaledConvexShape_;
pub use hknpScaledConvexShape_::*;
pub mod hknpScaledConvexShapeBase_;
pub use hknpScaledConvexShapeBase_::*;
pub mod hknpShape_;
pub use hknpShape_::*;
pub mod hknpShapeInstance_;
pub use hknpShapeInstance_::*;
pub mod hknpShapeKeyMask_;
pub use hknpShapeKeyMask_::*;
pub mod hknpShapeMassProperties_;
pub use hknpShapeMassProperties_::*;
pub mod hknpShapeSignals_;
pub use hknpShapeSignals_::*;
pub mod hknpShapeTagCodec_;
pub use hknpShapeTagCodec_::*;
pub mod hknpShapeType_;
pub use hknpShapeType_::*;
pub mod hknpSparseCompactMapunsignedshort_;
pub use hknpSparseCompactMapunsignedshort_::*;
pub mod hknpSphereShape_;
pub use hknpSphereShape_::*;
pub mod hknpStaticCompoundShape_;
pub use hknpStaticCompoundShape_::*;
pub mod hknpStaticCompoundShapeData_;
pub use hknpStaticCompoundShapeData_::*;
pub mod hknpStaticCompoundShapeInternals_;
pub use hknpStaticCompoundShapeInternals_::*;
pub mod hknpStaticCompoundShapeKeyMask_;
pub use hknpStaticCompoundShapeKeyMask_::*;
pub mod hknpStaticCompoundShapeTree_;
pub use hknpStaticCompoundShapeTree_::*;
pub mod hknpSurfaceVelocity_;
pub use hknpSurfaceVelocity_::*;
pub mod hknpTriangleShape_;
pub use hknpTriangleShape_::*;
pub mod hknpTyremarkPoint_;
pub use hknpTyremarkPoint_::*;
pub mod hknpTyremarksInfo_;
pub use hknpTyremarksInfo_::*;
pub mod hknpTyremarksWheel_;
pub use hknpTyremarksWheel_::*;
pub mod hknpUnaryAction_;
pub use hknpUnaryAction_::*;
pub mod hknpVehicleAerodynamics_;
pub use hknpVehicleAerodynamics_::*;
pub mod hknpVehicleBrake_;
pub use hknpVehicleBrake_::*;
pub mod hknpVehicleData_;
pub use hknpVehicleData_::*;
pub mod hknpVehicleDataWheelComponentParams_;
pub use hknpVehicleDataWheelComponentParams_::*;
pub mod hknpVehicleDefaultAerodynamics_;
pub use hknpVehicleDefaultAerodynamics_::*;
pub mod hknpVehicleDefaultAnalogDriverInput_;
pub use hknpVehicleDefaultAnalogDriverInput_::*;
pub mod hknpVehicleDefaultBrake_;
pub use hknpVehicleDefaultBrake_::*;
pub mod hknpVehicleDefaultBrakeWheelBrakingProperties_;
pub use hknpVehicleDefaultBrakeWheelBrakingProperties_::*;
pub mod hknpVehicleDefaultEngine_;
pub use hknpVehicleDefaultEngine_::*;
pub mod hknpVehicleDefaultSteering_;
pub use hknpVehicleDefaultSteering_::*;
pub mod hknpVehicleDefaultSuspension_;
pub use hknpVehicleDefaultSuspension_::*;
pub mod hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters_;
pub use hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters_::*;
pub mod hknpVehicleDefaultTransmission_;
pub use hknpVehicleDefaultTransmission_::*;
pub mod hknpVehicleDefaultVelocityDamper_;
pub use hknpVehicleDefaultVelocityDamper_::*;
pub mod hknpVehicleDriverInput_;
pub use hknpVehicleDriverInput_::*;
pub mod hknpVehicleDriverInputAnalogStatus_;
pub use hknpVehicleDriverInputAnalogStatus_::*;
pub mod hknpVehicleDriverInputStatus_;
pub use hknpVehicleDriverInputStatus_::*;
pub mod hknpVehicleEngine_;
pub use hknpVehicleEngine_::*;
pub mod hknpVehicleInstance_;
pub use hknpVehicleInstance_::*;
pub mod hknpVehicleInstanceWheelInfo_;
pub use hknpVehicleInstanceWheelInfo_::*;
pub mod hknpVehicleLinearCastWheelCollide_;
pub use hknpVehicleLinearCastWheelCollide_::*;
pub mod hknpVehicleLinearCastWheelCollideWheelState_;
pub use hknpVehicleLinearCastWheelCollideWheelState_::*;
pub mod hknpVehicleRayCastWheelCollide_;
pub use hknpVehicleRayCastWheelCollide_::*;
pub mod hknpVehicleSteering_;
pub use hknpVehicleSteering_::*;
pub mod hknpVehicleSuspension_;
pub use hknpVehicleSuspension_::*;
pub mod hknpVehicleSuspensionSuspensionWheelParameters_;
pub use hknpVehicleSuspensionSuspensionWheelParameters_::*;
pub mod hknpVehicleTransmission_;
pub use hknpVehicleTransmission_::*;
pub mod hknpVehicleVelocityDamper_;
pub use hknpVehicleVelocityDamper_::*;
pub mod hknpVehicleWheelCollide_;
pub use hknpVehicleWheelCollide_::*;
pub mod hknpWorldCinfo_;
pub use hknpWorldCinfo_::*;
pub mod hknpWorldSnapshot_;
pub use hknpWorldSnapshot_::*;
pub mod hkOffsetBitField_;
pub use hkOffsetBitField_::*;
pub mod hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_;
pub use hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_::*;
pub mod hkp2dAngConstraintAtom_;
pub use hkp2dAngConstraintAtom_::*;
pub mod hkp3dAngConstraintAtom_;
pub use hkp3dAngConstraintAtom_::*;
pub mod hkPackedVector3_;
pub use hkPackedVector3_::*;
pub mod hkPackedVector8_3_;
pub use hkPackedVector8_3_::*;
pub mod hkPackfileHeader_;
pub use hkPackfileHeader_::*;
pub mod hkPackfileSectionHeader_;
pub use hkPackfileSectionHeader_::*;
pub mod hkpAngConstraintAtom_;
pub use hkpAngConstraintAtom_::*;
pub mod hkpAngFrictionConstraintAtom_;
pub use hkpAngFrictionConstraintAtom_::*;
pub mod hkpAngLimitConstraintAtom_;
pub use hkpAngLimitConstraintAtom_::*;
pub mod hkpAngMotorConstraintAtom_;
pub use hkpAngMotorConstraintAtom_::*;
pub mod hkpBallAndSocketConstraintData_;
pub use hkpBallAndSocketConstraintData_::*;
pub mod hkpBallAndSocketConstraintDataAtoms_;
pub use hkpBallAndSocketConstraintDataAtoms_::*;
pub mod hkpBallSocketConstraintAtom_;
pub use hkpBallSocketConstraintAtom_::*;
pub mod hkpBridgeAtoms_;
pub use hkpBridgeAtoms_::*;
pub mod hkpBridgeConstraintAtom_;
pub use hkpBridgeConstraintAtom_::*;
pub mod hkpCallbackConstraintMotor_;
pub use hkpCallbackConstraintMotor_::*;
pub mod hkpCogWheelConstraintAtom_;
pub use hkpCogWheelConstraintAtom_::*;
pub mod hkpCogWheelConstraintData_;
pub use hkpCogWheelConstraintData_::*;
pub mod hkpCogWheelConstraintDataAtoms_;
pub use hkpCogWheelConstraintDataAtoms_::*;
pub mod hkpConeLimitConstraintAtom_;
pub use hkpConeLimitConstraintAtom_::*;
pub mod hkpConstraintAtom_;
pub use hkpConstraintAtom_::*;
pub mod hkpConstraintData_;
pub use hkpConstraintData_::*;
pub mod hkpConstraintMotor_;
pub use hkpConstraintMotor_::*;
pub mod hkpDeformableAngConstraintAtom_;
pub use hkpDeformableAngConstraintAtom_::*;
pub mod hkpDeformableFixedConstraintData_;
pub use hkpDeformableFixedConstraintData_::*;
pub mod hkpDeformableFixedConstraintDataAtoms_;
pub use hkpDeformableFixedConstraintDataAtoms_::*;
pub mod hkpDeformableLinConstraintAtom_;
pub use hkpDeformableLinConstraintAtom_::*;
pub mod hkpFixedConstraintData_;
pub use hkpFixedConstraintData_::*;
pub mod hkpFixedConstraintDataAtoms_;
pub use hkpFixedConstraintDataAtoms_::*;
pub mod hkpHingeConstraintData_;
pub use hkpHingeConstraintData_::*;
pub mod hkpHingeConstraintDataAtoms_;
pub use hkpHingeConstraintDataAtoms_::*;
pub mod hkpHingeLimitsData_;
pub use hkpHingeLimitsData_::*;
pub mod hkpHingeLimitsDataAtoms_;
pub use hkpHingeLimitsDataAtoms_::*;
pub mod hkpLimitedForceConstraintMotor_;
pub use hkpLimitedForceConstraintMotor_::*;
pub mod hkpLimitedHingeConstraintData_;
pub use hkpLimitedHingeConstraintData_::*;
pub mod hkpLimitedHingeConstraintDataAtoms_;
pub use hkpLimitedHingeConstraintDataAtoms_::*;
pub mod hkpLinConstraintAtom_;
pub use hkpLinConstraintAtom_::*;
pub mod hkpLinearClearanceConstraintData_;
pub use hkpLinearClearanceConstraintData_::*;
pub mod hkpLinearClearanceConstraintDataAtoms_;
pub use hkpLinearClearanceConstraintDataAtoms_::*;
pub mod hkpLinearParametricCurve_;
pub use hkpLinearParametricCurve_::*;
pub mod hkpLinFrictionConstraintAtom_;
pub use hkpLinFrictionConstraintAtom_::*;
pub mod hkpLinLimitConstraintAtom_;
pub use hkpLinLimitConstraintAtom_::*;
pub mod hkpLinMotorConstraintAtom_;
pub use hkpLinMotorConstraintAtom_::*;
pub mod hkpLinSoftConstraintAtom_;
pub use hkpLinSoftConstraintAtom_::*;
pub mod hkPostFinishAttribute_;
pub use hkPostFinishAttribute_::*;
pub mod hkpOverwritePivotConstraintAtom_;
pub use hkpOverwritePivotConstraintAtom_::*;
pub mod hkpParametricCurve_;
pub use hkpParametricCurve_::*;
pub mod hkpPointToPathConstraintData_;
pub use hkpPointToPathConstraintData_::*;
pub mod hkpPointToPlaneConstraintData_;
pub use hkpPointToPlaneConstraintData_::*;
pub mod hkpPointToPlaneConstraintDataAtoms_;
pub use hkpPointToPlaneConstraintDataAtoms_::*;
pub mod hkpPositionConstraintMotor_;
pub use hkpPositionConstraintMotor_::*;
pub mod hkpPrismaticConstraintData_;
pub use hkpPrismaticConstraintData_::*;
pub mod hkpPrismaticConstraintDataAtoms_;
pub use hkpPrismaticConstraintDataAtoms_::*;
pub mod hkpPulleyConstraintAtom_;
pub use hkpPulleyConstraintAtom_::*;
pub mod hkpPulleyConstraintData_;
pub use hkpPulleyConstraintData_::*;
pub mod hkpPulleyConstraintDataAtoms_;
pub use hkpPulleyConstraintDataAtoms_::*;
pub mod hkpRackAndPinionConstraintAtom_;
pub use hkpRackAndPinionConstraintAtom_::*;
pub mod hkpRackAndPinionConstraintData_;
pub use hkpRackAndPinionConstraintData_::*;
pub mod hkpRackAndPinionConstraintDataAtoms_;
pub use hkpRackAndPinionConstraintDataAtoms_::*;
pub mod hkpRagdollConstraintData_;
pub use hkpRagdollConstraintData_::*;
pub mod hkpRagdollConstraintDataAtoms_;
pub use hkpRagdollConstraintDataAtoms_::*;
pub mod hkpRagdollLimitsData_;
pub use hkpRagdollLimitsData_::*;
pub mod hkpRagdollLimitsDataAtoms_;
pub use hkpRagdollLimitsDataAtoms_::*;
pub mod hkpRagdollMotorConstraintAtom_;
pub use hkpRagdollMotorConstraintAtom_::*;
pub mod hkpRotationalConstraintData_;
pub use hkpRotationalConstraintData_::*;
pub mod hkpRotationalConstraintDataAtoms_;
pub use hkpRotationalConstraintDataAtoms_::*;
pub mod hkpSetLocalRotationsConstraintAtom_;
pub use hkpSetLocalRotationsConstraintAtom_::*;
pub mod hkpSetLocalTransformsConstraintAtom_;
pub use hkpSetLocalTransformsConstraintAtom_::*;
pub mod hkpSetLocalTranslationsConstraintAtom_;
pub use hkpSetLocalTranslationsConstraintAtom_::*;
pub mod hkpSetupStabilizationAtom_;
pub use hkpSetupStabilizationAtom_::*;
pub mod hkpSpringDamperConstraintMotor_;
pub use hkpSpringDamperConstraintMotor_::*;
pub mod hkpStiffSpringConstraintAtom_;
pub use hkpStiffSpringConstraintAtom_::*;
pub mod hkpStiffSpringConstraintData_;
pub use hkpStiffSpringConstraintData_::*;
pub mod hkpStiffSpringConstraintDataAtoms_;
pub use hkpStiffSpringConstraintDataAtoms_::*;
pub mod hkpTwistLimitConstraintAtom_;
pub use hkpTwistLimitConstraintAtom_::*;
pub mod hkpVehicleFrictionDescription_;
pub use hkpVehicleFrictionDescription_::*;
pub mod hkpVehicleFrictionDescriptionAxisDescription_;
pub use hkpVehicleFrictionDescriptionAxisDescription_::*;
pub mod hkpVehicleFrictionStatus_;
pub use hkpVehicleFrictionStatus_::*;
pub mod hkpVehicleFrictionStatusAxisStatus_;
pub use hkpVehicleFrictionStatusAxisStatus_::*;
pub mod hkpVelocityConstraintMotor_;
pub use hkpVelocityConstraintMotor_::*;
pub mod hkpWheelConstraintData_;
pub use hkpWheelConstraintData_::*;
pub mod hkpWheelConstraintDataAtoms_;
pub use hkpWheelConstraintDataAtoms_::*;
pub mod hkpWheelFrictionConstraintAtom_;
pub use hkpWheelFrictionConstraintAtom_::*;
pub mod hkpWheelFrictionConstraintAtomAxle_;
pub use hkpWheelFrictionConstraintAtomAxle_::*;
pub mod hkpWheelFrictionConstraintData_;
pub use hkpWheelFrictionConstraintData_::*;
pub mod hkpWheelFrictionConstraintDataAtoms_;
pub use hkpWheelFrictionConstraintDataAtoms_::*;
pub mod hkpWrappedConstraintData_;
pub use hkpWrappedConstraintData_::*;
pub mod hkQTransform_;
pub use hkQTransform_::*;
pub mod hkQTransformf_;
pub use hkQTransformf_::*;
pub mod hkRangeInt32Attribute_;
pub use hkRangeInt32Attribute_::*;
pub mod hkRangeRealAttribute_;
pub use hkRangeRealAttribute_::*;
pub mod hkRefCountedProperties_;
pub use hkRefCountedProperties_::*;
pub mod hkRefCountedPropertiesEntry_;
pub use hkRefCountedPropertiesEntry_::*;
pub mod hkReferencedObject_;
pub use hkReferencedObject_::*;
pub mod hkResourceBase_;
pub use hkResourceBase_::*;
pub mod hkResourceContainer_;
pub use hkResourceContainer_::*;
pub mod hkResourceHandle_;
pub use hkResourceHandle_::*;
pub mod hkRootLevelContainer_;
pub use hkRootLevelContainer_::*;
pub mod hkRootLevelContainerNamedVariant_;
pub use hkRootLevelContainerNamedVariant_::*;
pub mod hkSemanticsAttribute_;
pub use hkSemanticsAttribute_::*;
pub mod hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair_;
pub use hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair_::*;
pub mod hkSetIntFloatPair_;
pub use hkSetIntFloatPair_::*;
pub mod hkSetUint32_;
pub use hkSetUint32_::*;
pub mod hkSetUint64_;
pub use hkSetUint64_::*;
pub mod hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint_;
pub use hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint_::*;
pub mod hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong_;
pub use hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong_::*;
pub mod hkSimpleLocalFrame_;
pub use hkSimpleLocalFrame_::*;
pub mod hkSimpleProperty_;
pub use hkSimpleProperty_::*;
pub mod hkSimplePropertyValue_;
pub use hkSimplePropertyValue_::*;
pub mod hkSkinBinding_;
pub use hkSkinBinding_::*;
pub mod hkSkinnedMeshShape_;
pub use hkSkinnedMeshShape_::*;
pub mod hkSkinnedMeshShapeBoneSection_;
pub use hkSkinnedMeshShapeBoneSection_::*;
pub mod hkSkinnedMeshShapeBoneSet_;
pub use hkSkinnedMeshShapeBoneSet_::*;
pub mod hkSkinnedMeshShapePart_;
pub use hkSkinnedMeshShapePart_::*;
pub mod hkSkinnedRefMeshShape_;
pub use hkSkinnedRefMeshShape_::*;
pub mod hkSphere_;
pub use hkSphere_::*;
pub mod hkStorageSkinnedMeshShape_;
pub use hkStorageSkinnedMeshShape_::*;
pub mod hkStringObject_;
pub use hkStringObject_::*;
pub mod hkSweptTransformf_;
pub use hkSweptTransformf_::*;
pub mod hkTraceStreamTitle_;
pub use hkTraceStreamTitle_::*;
pub mod hkUFloat8_;
pub use hkUFloat8_::*;
pub mod hkUiAttribute_;
pub use hkUiAttribute_::*;
pub mod hkUuid_;
pub use hkUuid_::*;
pub mod hkVariableTweakingHelper_;
pub use hkVariableTweakingHelper_::*;
pub mod hkVariableTweakingHelperBoolVariableInfo_;
pub use hkVariableTweakingHelperBoolVariableInfo_::*;
pub mod hkVariableTweakingHelperIntVariableInfo_;
pub use hkVariableTweakingHelperIntVariableInfo_::*;
pub mod hkVariableTweakingHelperRealVariableInfo_;
pub use hkVariableTweakingHelperRealVariableInfo_::*;
pub mod hkVariableTweakingHelperVector4VariableInfo_;
pub use hkVariableTweakingHelperVector4VariableInfo_::*;
pub mod hkVertexFormat_;
pub use hkVertexFormat_::*;
pub mod hkVertexFormatElement_;
pub use hkVertexFormatElement_::*;
pub mod hkxAnimatedFloat_;
pub use hkxAnimatedFloat_::*;
pub mod hkxAnimatedMatrix_;
pub use hkxAnimatedMatrix_::*;
pub mod hkxAnimatedQuaternion_;
pub use hkxAnimatedQuaternion_::*;
pub mod hkxAnimatedVector_;
pub use hkxAnimatedVector_::*;
pub mod hkxAttribute_;
pub use hkxAttribute_::*;
pub mod hkxAttributeGroup_;
pub use hkxAttributeGroup_::*;
pub mod hkxAttributeHolder_;
pub use hkxAttributeHolder_::*;
pub mod hkxEnum_;
pub use hkxEnum_::*;
pub mod hkxEnumItem_;
pub use hkxEnumItem_::*;
pub mod hkxIndexBuffer_;
pub use hkxIndexBuffer_::*;
pub mod hkxMaterial_;
pub use hkxMaterial_::*;
pub mod hkxMaterialProperty_;
pub use hkxMaterialProperty_::*;
pub mod hkxMaterialTextureStage_;
pub use hkxMaterialTextureStage_::*;
pub mod hkxMesh_;
pub use hkxMesh_::*;
pub mod hkxMeshSection_;
pub use hkxMeshSection_::*;
pub mod hkxMeshUserChannelInfo_;
pub use hkxMeshUserChannelInfo_::*;
pub mod hkxSparselyAnimatedBool_;
pub use hkxSparselyAnimatedBool_::*;
pub mod hkxSparselyAnimatedEnum_;
pub use hkxSparselyAnimatedEnum_::*;
pub mod hkxSparselyAnimatedInt_;
pub use hkxSparselyAnimatedInt_::*;
pub mod hkxSparselyAnimatedString_;
pub use hkxSparselyAnimatedString_::*;
pub mod hkxVertexAnimation_;
pub use hkxVertexAnimation_::*;
pub mod hkxVertexAnimationUsageMap_;
pub use hkxVertexAnimationUsageMap_::*;
pub mod hkxVertexBuffer_;
pub use hkxVertexBuffer_::*;
pub mod hkxVertexBufferVertexData_;
pub use hkxVertexBufferVertexData_::*;
pub mod hkxVertexDescription_;
pub use hkxVertexDescription_::*;
pub mod hkxVertexDescriptionElementDecl_;
pub use hkxVertexDescriptionElementDecl_::*;
use havok_serde as _serde;
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Classes<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    BGSGamebryoSequenceGenerator(Box<BGSGamebryoSequenceGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSAlignBoneModifier(Box<BSAlignBoneModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSAssignVariablesModifier(Box<BSAssignVariablesModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSBehaviorGraphSwapGenerator(Box<BSBehaviorGraphSwapGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSBoneSwitchGenerator(Box<BSBoneSwitchGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSBoneSwitchGeneratorBoneData(Box<BSBoneSwitchGeneratorBoneData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSComputeAddBoneAnimModifier(Box<BSComputeAddBoneAnimModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSCyclicBlendTransitionGenerator(Box<BSCyclicBlendTransitionGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSDecomposeVectorModifier(Box<BSDecomposeVectorModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSDirectAtCapturePoseModifier(Box<BSDirectAtCapturePoseModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSDirectAtModifier(Box<BSDirectAtModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSDistTriggerModifier(Box<BSDistTriggerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSEventEveryNEventsModifier(Box<BSEventEveryNEventsModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSEventOnDeactivateModifier(Box<BSEventOnDeactivateModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSEventOnFalseToTrueModifier(Box<BSEventOnFalseToTrueModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSGetTimeStepModifier(Box<BSGetTimeStepModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSInterpValueModifier(Box<BSInterpValueModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSIsActiveModifier(Box<BSIsActiveModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSIStateManagerModifier(Box<BSIStateManagerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSIStateManagerModifierBSiStateData(Box<BSIStateManagerModifierBSiStateData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSIStateManagerModifierBSIStateManagerStateListener(
        Box<BSIStateManagerModifierBSIStateManagerStateListener<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSiStateTaggingGenerator(Box<BSiStateTaggingGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSiStateTaggingStateListener(Box<BSiStateTaggingStateListener<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSLimbCycleModifier(Box<BSLimbCycleModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSLimbIKModifier(Box<BSLimbIKModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSLookAtCapturePoseModifier(Box<BSLookAtCapturePoseModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSLookAtModifier(Box<BSLookAtModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSLookAtModifierBoneData(Box<BSLookAtModifierBoneData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSModifyOnceModifier(Box<BSModifyOnceModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSOffsetAnimationGenerator(Box<BSOffsetAnimationGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSPassByTargetTriggerModifier(Box<BSPassByTargetTriggerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSRagdollContactListenerModifier(Box<BSRagdollContactListenerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSRandomAlarmModifier(Box<BSRandomAlarmModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSRetargetBodyBlend(Box<BSRetargetBodyBlend<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSReverseSpineTwistModifier(Box<BSReverseSpineTwistModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSRootTwistModifier(Box<BSRootTwistModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSSpeedSamplerModifier(Box<BSSpeedSamplerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSSpringState(Box<BSSpringState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSTimerModifier(Box<BSTimerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    BSTweenerModifier(Box<BSTweenerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    DynamicAnimationTaggingGenerator(Box<DynamicAnimationTaggingGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclAction(Box<hclAction<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclAntiPinchConstraintSet(Box<hclAntiPinchConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclAntiPinchConstraintSetPerParticle(Box<hclAntiPinchConstraintSetPerParticle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendLinkConstraintSet(Box<hclBendLinkConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendLinkConstraintSetLink(Box<hclBendLinkConstraintSetLink<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendLinkConstraintSetMx(Box<hclBendLinkConstraintSetMx<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendLinkConstraintSetMxBatch(Box<hclBendLinkConstraintSetMxBatch<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendLinkConstraintSetMxSingle(Box<hclBendLinkConstraintSetMxSingle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendStiffnessConstraintSet(Box<hclBendStiffnessConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendStiffnessConstraintSetLink(Box<hclBendStiffnessConstraintSetLink<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendStiffnessConstraintSetMx(Box<hclBendStiffnessConstraintSetMx<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendStiffnessConstraintSetMxBatch(Box<hclBendStiffnessConstraintSetMxBatch<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendStiffnessConstraintSetMxSingle(
        Box<hclBendStiffnessConstraintSetMxSingle<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBendStiffnessSetupObject(Box<hclBendStiffnessSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBlendSomeVerticesOperator(Box<hclBlendSomeVerticesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBlendSomeVerticesOperatorBlendEntry(
        Box<hclBlendSomeVerticesOperatorBlendEntry<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBonePlanesConstraintSet(Box<hclBonePlanesConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBonePlanesConstraintSetBonePlane(Box<hclBonePlanesConstraintSetBonePlane<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformer(Box<hclBoneSpaceDeformer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerFourBlendEntryBlock(
        Box<hclBoneSpaceDeformerFourBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockP(Box<hclBoneSpaceDeformerLocalBlockP<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockPN(Box<hclBoneSpaceDeformerLocalBlockPN<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockPNT(Box<hclBoneSpaceDeformerLocalBlockPNT<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockPNTB(Box<hclBoneSpaceDeformerLocalBlockPNTB<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockUnpackedP(
        Box<hclBoneSpaceDeformerLocalBlockUnpackedP<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockUnpackedPN(
        Box<hclBoneSpaceDeformerLocalBlockUnpackedPN<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockUnpackedPNT(
        Box<hclBoneSpaceDeformerLocalBlockUnpackedPNT<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerLocalBlockUnpackedPNTB(
        Box<hclBoneSpaceDeformerLocalBlockUnpackedPNTB<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerOneBlendEntryBlock(
        Box<hclBoneSpaceDeformerOneBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerThreeBlendEntryBlock(
        Box<hclBoneSpaceDeformerThreeBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceDeformerTwoBlendEntryBlock(
        Box<hclBoneSpaceDeformerTwoBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceMeshMeshDeformOperator(Box<hclBoneSpaceMeshMeshDeformOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceMeshMeshDeformPNOperator(Box<hclBoneSpaceMeshMeshDeformPNOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceMeshMeshDeformPNTBOperator(
        Box<hclBoneSpaceMeshMeshDeformPNTBOperator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceMeshMeshDeformPNTOperator(
        Box<hclBoneSpaceMeshMeshDeformPNTOperator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceMeshMeshDeformPOperator(Box<hclBoneSpaceMeshMeshDeformPOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceSkinOperator(Box<hclBoneSpaceSkinOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceSkinPNOperator(Box<hclBoneSpaceSkinPNOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceSkinPNTBOperator(Box<hclBoneSpaceSkinPNTBOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceSkinPNTOperator(Box<hclBoneSpaceSkinPNTOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBoneSpaceSkinPOperator(Box<hclBoneSpaceSkinPOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBufferDefinition(Box<hclBufferDefinition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBufferLayout(Box<hclBufferLayout<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBufferLayoutBufferElement(Box<hclBufferLayoutBufferElement<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBufferLayoutSlot(Box<hclBufferLayoutSlot<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBufferSetupObject(Box<hclBufferSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclBufferUsage(Box<hclBufferUsage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCapsuleShape(Box<hclCapsuleShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothContainer(Box<hclClothContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothData(Box<hclClothData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothSetupContainer(Box<hclClothSetupContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothSetupObject(Box<hclClothSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothState(Box<hclClothState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothStateBufferAccess(Box<hclClothStateBufferAccess<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothStateSetupObject(Box<hclClothStateSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclClothStateTransformSetAccess(Box<hclClothStateTransformSetAccess<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCollidable(Box<hclCollidable<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCompressibleLinkConstraintSet(Box<hclCompressibleLinkConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCompressibleLinkConstraintSetLink(Box<hclCompressibleLinkConstraintSetLink<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCompressibleLinkConstraintSetMx(Box<hclCompressibleLinkConstraintSetMx<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCompressibleLinkConstraintSetMxBatch(
        Box<hclCompressibleLinkConstraintSetMxBatch<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCompressibleLinkConstraintSetMxSingle(
        Box<hclCompressibleLinkConstraintSetMxSingle<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclConstraintSet(Box<hclConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclConstraintSetSetupObject(Box<hclConstraintSetSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclConvexGeometryShape(Box<hclConvexGeometryShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclConvexHeightFieldShape(Box<hclConvexHeightFieldShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclConvexPlanesShape(Box<hclConvexPlanesShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclCopyVerticesOperator(Box<hclCopyVerticesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclEdgeSelectionInput(Box<hclEdgeSelectionInput<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclGatherAllVerticesOperator(Box<hclGatherAllVerticesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclGatherSomeVerticesOperator(Box<hclGatherSomeVerticesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclGatherSomeVerticesOperatorVertexPair(
        Box<hclGatherSomeVerticesOperatorVertexPair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclInputConvertOperator(Box<hclInputConvertOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclLocalRangeConstraintSet(Box<hclLocalRangeConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclLocalRangeConstraintSetLocalConstraint(
        Box<hclLocalRangeConstraintSetLocalConstraint<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclLocalRangeSetupObject(Box<hclLocalRangeSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMeshBoneDeformOperator(Box<hclMeshBoneDeformOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMeshBoneDeformOperatorTriangleBonePair(
        Box<hclMeshBoneDeformOperatorTriangleBonePair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMeshBoneDeformSetupObject(Box<hclMeshBoneDeformSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMeshMeshDeformOperator(Box<hclMeshMeshDeformOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMeshMeshDeformOperatorTriangleVertexPair(
        Box<hclMeshMeshDeformOperatorTriangleVertexPair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMoveFixedParticlesSetupObject(Box<hclMoveFixedParticlesSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMoveParticlesOperator(Box<hclMoveParticlesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclMoveParticlesOperatorVertexParticlePair(
        Box<hclMoveParticlesOperatorVertexParticlePair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclNamedSetupMesh(Box<hclNamedSetupMesh<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclNamedTransformSetSetupObject(Box<hclNamedTransformSetSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformer(Box<hclObjectSpaceDeformer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerFourBlendEntryBlock(
        Box<hclObjectSpaceDeformerFourBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockP(Box<hclObjectSpaceDeformerLocalBlockP<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockPN(Box<hclObjectSpaceDeformerLocalBlockPN<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockPNT(Box<hclObjectSpaceDeformerLocalBlockPNT<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockPNTB(Box<hclObjectSpaceDeformerLocalBlockPNTB<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockUnpackedP(
        Box<hclObjectSpaceDeformerLocalBlockUnpackedP<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockUnpackedPN(
        Box<hclObjectSpaceDeformerLocalBlockUnpackedPN<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockUnpackedPNT(
        Box<hclObjectSpaceDeformerLocalBlockUnpackedPNT<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerLocalBlockUnpackedPNTB(
        Box<hclObjectSpaceDeformerLocalBlockUnpackedPNTB<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerOneBlendEntryBlock(
        Box<hclObjectSpaceDeformerOneBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerThreeBlendEntryBlock(
        Box<hclObjectSpaceDeformerThreeBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceDeformerTwoBlendEntryBlock(
        Box<hclObjectSpaceDeformerTwoBlendEntryBlock<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceMeshMeshDeformOperator(Box<hclObjectSpaceMeshMeshDeformOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceMeshMeshDeformPNOperator(
        Box<hclObjectSpaceMeshMeshDeformPNOperator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceMeshMeshDeformPNTBOperator(
        Box<hclObjectSpaceMeshMeshDeformPNTBOperator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceMeshMeshDeformPNTOperator(
        Box<hclObjectSpaceMeshMeshDeformPNTOperator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceMeshMeshDeformPOperator(
        Box<hclObjectSpaceMeshMeshDeformPOperator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceSkinOperator(Box<hclObjectSpaceSkinOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceSkinPNOperator(Box<hclObjectSpaceSkinPNOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceSkinPNTBOperator(Box<hclObjectSpaceSkinPNTBOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceSkinPNTOperator(Box<hclObjectSpaceSkinPNTOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclObjectSpaceSkinPOperator(Box<hclObjectSpaceSkinPOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclOperator(Box<hclOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclOperatorSetupObject(Box<hclOperatorSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclOutputConvertOperator(Box<hclOutputConvertOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclPlaneShape(Box<hclPlaneShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclRuntimeConversionInfo(Box<hclRuntimeConversionInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclRuntimeConversionInfoElementConversion(
        Box<hclRuntimeConversionInfoElementConversion<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclRuntimeConversionInfoSlotConversion(
        Box<hclRuntimeConversionInfoSlotConversion<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclScratchBufferDefinition(Box<hclScratchBufferDefinition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclScratchBufferSetupObject(Box<hclScratchBufferSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSetupMesh(Box<hclSetupMesh<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclShadowBufferDefinition(Box<hclShadowBufferDefinition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclShape(Box<hclShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothBufferSetupObject(Box<hclSimClothBufferSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothData(Box<hclSimClothData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothDataCollidablePinchingData(
        Box<hclSimClothDataCollidablePinchingData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothDataCollidableTransformMap(
        Box<hclSimClothDataCollidableTransformMap<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothDataLandscapeCollisionData(
        Box<hclSimClothDataLandscapeCollisionData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothDataOverridableSimulationInfo(
        Box<hclSimClothDataOverridableSimulationInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothDataParticleData(Box<hclSimClothDataParticleData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothDataTransferMotionData(Box<hclSimClothDataTransferMotionData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothPose(Box<hclSimClothPose<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothSetupObject(Box<hclSimClothSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothSetupObjectPerInstanceCollidable(
        Box<hclSimClothSetupObjectPerInstanceCollidable<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimClothSetupObjectTransferMotionSetupData(
        Box<hclSimClothSetupObjectTransferMotionSetupData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimpleMeshBoneDeformOperator(Box<hclSimpleMeshBoneDeformOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimpleMeshBoneDeformOperatorTriangleBonePair(
        Box<hclSimpleMeshBoneDeformOperatorTriangleBonePair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimpleWindAction(Box<hclSimpleWindAction<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimulateOperator(Box<hclSimulateOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimulateSetupObject(Box<hclSimulateSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimulationSetupMesh(Box<hclSimulationSetupMesh<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSimulationSetupMeshMapOptions(Box<hclSimulationSetupMeshMapOptions<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSkinOperator(Box<hclSkinOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSkinOperatorBoneInfluence(Box<hclSkinOperatorBoneInfluence<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSkinSetupObject(Box<hclSkinSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclSphereShape(Box<hclSphereShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStandardLinkConstraintSet(Box<hclStandardLinkConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStandardLinkConstraintSetLink(Box<hclStandardLinkConstraintSetLink<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStandardLinkConstraintSetMx(Box<hclStandardLinkConstraintSetMx<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStandardLinkConstraintSetMxBatch(Box<hclStandardLinkConstraintSetMxBatch<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStandardLinkConstraintSetMxSingle(Box<hclStandardLinkConstraintSetMxSingle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStandardLinkSetupObject(Box<hclStandardLinkSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStaticDisplayBufferSetupObject(Box<hclStaticDisplayBufferSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStaticShadowBufferDefinition(Box<hclStaticShadowBufferDefinition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStretchLinkConstraintSet(Box<hclStretchLinkConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStretchLinkConstraintSetLink(Box<hclStretchLinkConstraintSetLink<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStretchLinkConstraintSetMx(Box<hclStretchLinkConstraintSetMx<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStretchLinkConstraintSetMxBatch(Box<hclStretchLinkConstraintSetMxBatch<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStretchLinkConstraintSetMxSingle(Box<hclStretchLinkConstraintSetMxSingle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclStretchLinkSetupObject(Box<hclStretchLinkSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTaperedCapsuleShape(Box<hclTaperedCapsuleShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTransformSetDefinition(Box<hclTransformSetDefinition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTransformSetSetupObject(Box<hclTransformSetSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTransformSetUsage(Box<hclTransformSetUsage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTransformSetUsageTransformTracker(Box<hclTransformSetUsageTransformTracker<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTransitionConstraintSet(Box<hclTransitionConstraintSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTransitionConstraintSetPerParticle(
        Box<hclTransitionConstraintSetPerParticle<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclTriangleSelectionInput(Box<hclTriangleSelectionInput<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclUpdateAllVertexFramesOperator(Box<hclUpdateAllVertexFramesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclUpdateSomeVertexFramesOperator(Box<hclUpdateSomeVertexFramesOperator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclUpdateSomeVertexFramesOperatorTriangle(
        Box<hclUpdateSomeVertexFramesOperatorTriangle<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVertexFloatInput(Box<hclVertexFloatInput<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVertexGatherSetupObject(Box<hclVertexGatherSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVertexSelectionInput(Box<hclVertexSelectionInput<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraint(Box<hclVolumeConstraint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintApplyData(Box<hclVolumeConstraintApplyData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintFrameData(Box<hclVolumeConstraintFrameData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintMx(Box<hclVolumeConstraintMx<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintMxApplyBatchData(Box<hclVolumeConstraintMxApplyBatchData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintMxApplySingleData(Box<hclVolumeConstraintMxApplySingleData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintMxFrameBatchData(Box<hclVolumeConstraintMxFrameBatchData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintMxFrameSingleData(Box<hclVolumeConstraintMxFrameSingleData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hclVolumeConstraintSetupObject(Box<hclVolumeConstraintSetupObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkAabb(Box<hkAabb<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkAabb16(Box<hkAabb16<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkAabbHalf(Box<hkAabbHalf<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkAabbUint32(Box<hkAabbUint32<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAngularReferenceFrame(Box<hkaAngularReferenceFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnimatedReferenceFrame(Box<hkaAnimatedReferenceFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnimation(Box<hkaAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnimationBinding(Box<hkaAnimationBinding<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnimationContainer(Box<hkaAnimationContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnimationPreviewColorContainer(Box<hkaAnimationPreviewColorContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnnotationTrack(Box<hkaAnnotationTrack<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaAnnotationTrackAnnotation(Box<hkaAnnotationTrackAnnotation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaBone(Box<hkaBone<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaBoneAttachment(Box<hkaBoneAttachment<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaDefaultAnimatedReferenceFrame(Box<hkaDefaultAnimatedReferenceFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaDirectionalReferenceFrame(Box<hkaDirectionalReferenceFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaFootstepAnalysisInfo(Box<hkaFootstepAnalysisInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaFootstepAnalysisInfoContainer(Box<hkaFootstepAnalysisInfoContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaInterleavedUncompressedAnimation(Box<hkaInterleavedUncompressedAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaLosslessCompressedAnimation(Box<hkaLosslessCompressedAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaMeshBinding(Box<hkaMeshBinding<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaMeshBindingMapping(Box<hkaMeshBindingMapping<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaParameterizedAnimationReferenceFrame(
        Box<hkaParameterizedAnimationReferenceFrame<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaParameterizedReferenceFrame(Box<hkaParameterizedReferenceFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaPredictiveCompressedAnimation(Box<hkaPredictiveCompressedAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaPredictiveCompressedAnimationTrackCompressionParams(
        Box<hkaPredictiveCompressedAnimationTrackCompressionParams<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaQuantizedAnimation(Box<hkaQuantizedAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaQuantizedAnimationTrackCompressionParams(
        Box<hkaQuantizedAnimationTrackCompressionParams<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaReferencePoseAnimation(Box<hkaReferencePoseAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkArrayTypeAttribute(Box<hkArrayTypeAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeleton(Box<hkaSkeleton<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonLocalFrameOnBone(Box<hkaSkeletonLocalFrameOnBone<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonMapper(Box<hkaSkeletonMapper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonMapperData(Box<hkaSkeletonMapperData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonMapperDataChainMapping(Box<hkaSkeletonMapperDataChainMapping<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonMapperDataPartitionMappingRange(
        Box<hkaSkeletonMapperDataPartitionMappingRange<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonMapperDataSimpleMapping(Box<hkaSkeletonMapperDataSimpleMapping<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSkeletonPartition(Box<hkaSkeletonPartition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSplineCompressedAnimation(Box<hkaSplineCompressedAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSplineCompressedAnimationAnimationCompressionParams(
        Box<hkaSplineCompressedAnimationAnimationCompressionParams<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkaSplineCompressedAnimationTrackCompressionParams(
        Box<hkaSplineCompressedAnimationTrackCompressionParams<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAiControlCancelPathCommand(Box<hkbAiControlCancelPathCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAiControlControlData(Box<hkbAiControlControlData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAiControlControlDataBlendable(Box<hkbAiControlControlDataBlendable<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAiControlControlDataNonBlendable(Box<hkbAiControlControlDataNonBlendable<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAiControlPathToCommand(Box<hkbAiControlPathToCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAlignBoneModifier(Box<hkbAlignBoneModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAnimatedSkeletonGenerator(Box<hkbAnimatedSkeletonGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkBaseObject(Box<hkBaseObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAssetBundle(Box<hkbAssetBundle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAssetBundleStringData(Box<hkbAssetBundleStringData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAttachmentModifier(Box<hkbAttachmentModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAttachmentSetup(Box<hkbAttachmentSetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAttributeModifier(Box<hkbAttributeModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAttributeModifierAssignment(Box<hkbAttributeModifierAssignment<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbAuxiliaryNodeInfo(Box<hkbAuxiliaryNodeInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorEventsInfo(Box<hkbBehaviorEventsInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorGraph(Box<hkbBehaviorGraph<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorGraphData(Box<hkbBehaviorGraphData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorGraphInternalState(Box<hkbBehaviorGraphInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorGraphInternalStateInfo(Box<hkbBehaviorGraphInternalStateInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorGraphStringData(Box<hkbBehaviorGraphStringData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorInfo(Box<hkbBehaviorInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorInfoIdToNamePair(Box<hkbBehaviorInfoIdToNamePair<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBehaviorReferenceGenerator(Box<hkbBehaviorReferenceGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBindable(Box<hkbBindable<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlendCurveUtils(Box<hkbBlendCurveUtils<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlenderGenerator(Box<hkbBlenderGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlenderGeneratorChild(Box<hkbBlenderGeneratorChild<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlenderGeneratorChildInternalState(
        Box<hkbBlenderGeneratorChildInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlenderGeneratorInternalState(Box<hkbBlenderGeneratorInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlendingTransitionEffect(Box<hkbBlendingTransitionEffect<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBlendingTransitionEffectInternalState(
        Box<hkbBlendingTransitionEffectInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBoneIndexArray(Box<hkbBoneIndexArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBoneWeightArray(Box<hkbBoneWeightArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBoolVariableSequencedData(Box<hkbBoolVariableSequencedData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbBoolVariableSequencedDataSample(Box<hkbBoolVariableSequencedDataSample<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCameraShakeEventPayload(Box<hkbCameraShakeEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCameraVariablesChangedCommand(Box<hkbCameraVariablesChangedCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacter(Box<hkbCharacter<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterAddedInfo(Box<hkbCharacterAddedInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterControlCommand(Box<hkbCharacterControlCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterControllerModifier(Box<hkbCharacterControllerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterControllerModifierControlData(
        Box<hkbCharacterControllerModifierControlData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterControllerModifierInternalState(
        Box<hkbCharacterControllerModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterControllerSetup(Box<hkbCharacterControllerSetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterData(Box<hkbCharacterData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterInfo(Box<hkbCharacterInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterSelectedInfo(Box<hkbCharacterSelectedInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterSetup(Box<hkbCharacterSetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterSkinInfo(Box<hkbCharacterSkinInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterSteppedInfo(Box<hkbCharacterSteppedInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterStringData(Box<hkbCharacterStringData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCharacterStringDataFileNameMeshNamePair(
        Box<hkbCharacterStringDataFileNameMeshNamePair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbClientCharacterState(Box<hkbClientCharacterState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbClipGenerator(Box<hkbClipGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbClipGeneratorEcho(Box<hkbClipGeneratorEcho<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbClipGeneratorInternalState(Box<hkbClipGeneratorInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbClipTrigger(Box<hkbClipTrigger<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbClipTriggerArray(Box<hkbClipTriggerArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCombineTransformsModifier(Box<hkbCombineTransformsModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCombineTransformsModifierInternalState(
        Box<hkbCombineTransformsModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCompiledExpressionSet(Box<hkbCompiledExpressionSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCompiledExpressionSetToken(Box<hkbCompiledExpressionSetToken<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeDirectionModifier(Box<hkbComputeDirectionModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeDirectionModifierInternalState(
        Box<hkbComputeDirectionModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeRotationFromAxisAngleModifier(
        Box<hkbComputeRotationFromAxisAngleModifier<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeRotationFromAxisAngleModifierInternalState(
        Box<hkbComputeRotationFromAxisAngleModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeRotationToTargetModifier(Box<hkbComputeRotationToTargetModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeRotationToTargetModifierInternalState(
        Box<hkbComputeRotationToTargetModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbComputeWorldFromModelModifier(Box<hkbComputeWorldFromModelModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCondition(Box<hkbCondition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbConstraintSetup(Box<hkbConstraintSetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbContext(Box<hkbContext<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomIdSelector(Box<hkbCustomIdSelector<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGenerator(Box<hkbCustomTestGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorAnnotatedTypes(Box<hkbCustomTestGeneratorAnnotatedTypes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorBoneTypes(Box<hkbCustomTestGeneratorBoneTypes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorComplexTypes(Box<hkbCustomTestGeneratorComplexTypes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorHiddenTypes(Box<hkbCustomTestGeneratorHiddenTypes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorNestedTypes(Box<hkbCustomTestGeneratorNestedTypes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorNestedTypesBase(
        Box<hkbCustomTestGeneratorNestedTypesBase<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbCustomTestGeneratorSimpleTypes(Box<hkbCustomTestGeneratorSimpleTypes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDampingModifier(Box<hkbDampingModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDampingModifierInternalState(Box<hkbDampingModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDelayedModifier(Box<hkbDelayedModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDelayedModifierInternalState(Box<hkbDelayedModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDetectCloseToGroundModifier(Box<hkbDetectCloseToGroundModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDetectCloseToGroundModifierInternalState(
        Box<hkbDetectCloseToGroundModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDockingGenerator(Box<hkbDockingGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbDockingGeneratorInternalState(Box<hkbDockingGeneratorInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEvaluateExpressionModifier(Box<hkbEvaluateExpressionModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEvaluateExpressionModifierInternalExpressionData(
        Box<hkbEvaluateExpressionModifierInternalExpressionData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEvaluateExpressionModifierInternalState(
        Box<hkbEvaluateExpressionModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEvaluateHandleModifier(Box<hkbEvaluateHandleModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEvent(Box<hkbEvent<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventBase(Box<hkbEventBase<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventDrivenModifier(Box<hkbEventDrivenModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventDrivenModifierInternalState(Box<hkbEventDrivenModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventInfo(Box<hkbEventInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventPayload(Box<hkbEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventPayloadList(Box<hkbEventPayloadList<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventProperty(Box<hkbEventProperty<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventRaisedInfo(Box<hkbEventRaisedInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventRangeData(Box<hkbEventRangeData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventRangeDataArray(Box<hkbEventRangeDataArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventSequencedData(Box<hkbEventSequencedData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventSequencedDataSequencedEvent(Box<hkbEventSequencedDataSequencedEvent<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventsFromRangeModifier(Box<hkbEventsFromRangeModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbEventsFromRangeModifierInternalState(
        Box<hkbEventsFromRangeModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbExpressionCondition(Box<hkbExpressionCondition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbExpressionData(Box<hkbExpressionData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbExpressionDataArray(Box<hkbExpressionDataArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbExtractRagdollPoseModifier(Box<hkbExtractRagdollPoseModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbExtrapolatingTransitionEffect(Box<hkbExtrapolatingTransitionEffect<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbExtrapolatingTransitionEffectInternalState(
        Box<hkbExtrapolatingTransitionEffectInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkControlData(Box<hkbFootIkControlData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkControlsModifier(Box<hkbFootIkControlsModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkControlsModifierLeg(Box<hkbFootIkControlsModifierLeg<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkDriverInfo(Box<hkbFootIkDriverInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkDriverInfoLeg(Box<hkbFootIkDriverInfoLeg<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkGains(Box<hkbFootIkGains<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkModifier(Box<hkbFootIkModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkModifierInternalLegData(Box<hkbFootIkModifierInternalLegData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbFootIkModifierLeg(Box<hkbFootIkModifierLeg<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGenerator(Box<hkbGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorOutputListener(Box<hkbGeneratorOutputListener<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorPartitionInfo(Box<hkbGeneratorPartitionInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorSyncInfo(Box<hkbGeneratorSyncInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorSyncInfoActiveInterval(Box<hkbGeneratorSyncInfoActiveInterval<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorSyncInfoSyncPoint(Box<hkbGeneratorSyncInfoSyncPoint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorTransitionEffect(Box<hkbGeneratorTransitionEffect<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGeneratorTransitionEffectInternalState(
        Box<hkbGeneratorTransitionEffectInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGetHandleOnBoneModifier(Box<hkbGetHandleOnBoneModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGetUpModifier(Box<hkbGetUpModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGetUpModifierInternalState(Box<hkbGetUpModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGetWorldFromModelModifier(Box<hkbGetWorldFromModelModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGetWorldFromModelModifierInternalState(
        Box<hkbGetWorldFromModelModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbGravityModifier(Box<hkbGravityModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkControlData(Box<hkbHandIkControlData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkControlsModifier(Box<hkbHandIkControlsModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkControlsModifierHand(Box<hkbHandIkControlsModifierHand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkDriverInfo(Box<hkbHandIkDriverInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkDriverInfoHand(Box<hkbHandIkDriverInfoHand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkModifier(Box<hkbHandIkModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandIkModifierHand(Box<hkbHandIkModifierHand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHandle(Box<hkbHandle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbHoldFromBlendingTransitionEffect(Box<hkbHoldFromBlendingTransitionEffect<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbIntEventPayload(Box<hkbIntEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbIntVariableSequencedData(Box<hkbIntVariableSequencedData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbIntVariableSequencedDataSample(Box<hkbIntVariableSequencedDataSample<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkBitField(Box<hkBitField<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
        Box<
            hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator<'a>,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
        Box<
            hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
        Box<hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbJigglerGroup(Box<hkbJigglerGroup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbJigglerModifier(Box<hkbJigglerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbJigglerModifierInternalState(Box<hkbJigglerModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbKeyframeBonesModifier(Box<hkbKeyframeBonesModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbKeyframeBonesModifierKeyframeInfo(Box<hkbKeyframeBonesModifierKeyframeInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbKeyFrameControlData(Box<hkbKeyFrameControlData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLayer(Box<hkbLayer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLayerGenerator(Box<hkbLayerGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLayerGeneratorInternalState(Box<hkbLayerGeneratorInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLayerGeneratorLayerInternalState(Box<hkbLayerGeneratorLayerInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLinkedSymbolInfo(Box<hkbLinkedSymbolInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLookAtModifier(Box<hkbLookAtModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbLookAtModifierInternalState(Box<hkbLookAtModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbManualSelectorGenerator(Box<hkbManualSelectorGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbManualSelectorGeneratorInternalState(
        Box<hkbManualSelectorGeneratorInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbManualSelectorTransitionEffect(Box<hkbManualSelectorTransitionEffect<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbManualSelectorTransitionEffectInternalState(
        Box<hkbManualSelectorTransitionEffectInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbMessageLog(Box<hkbMessageLog<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbMirroredSkeletonInfo(Box<hkbMirroredSkeletonInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbMirrorModifier(Box<hkbMirrorModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbModifier(Box<hkbModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbModifierGenerator(Box<hkbModifierGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbModifierList(Box<hkbModifierList<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbModifierWrapper(Box<hkbModifierWrapper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbMoveBoneAttachmentModifier(Box<hkbMoveBoneAttachmentModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbMoveCharacterModifier(Box<hkbMoveCharacterModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbMoveCharacterModifierInternalState(
        Box<hkbMoveCharacterModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNamedEventPayload(Box<hkbNamedEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNamedIntEventPayload(Box<hkbNamedIntEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNamedRealEventPayload(Box<hkbNamedRealEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNamedStringEventPayload(Box<hkbNamedStringEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNode(Box<hkbNode<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNodeInternalStateInfo(Box<hkbNodeInternalStateInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbnpPhysicsInterface(Box<hkbnpPhysicsInterface<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbnpRagdollInterface(Box<hkbnpRagdollInterface<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbNullPhysicsInterface(Box<hkbNullPhysicsInterface<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbParametricMotionGenerator(Box<hkbParametricMotionGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbParticleSystemEventPayload(Box<hkbParticleSystemEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPhysicsInterface(Box<hkbPhysicsInterface<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPinBonesGenerator(Box<hkbPinBonesGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPoseMatchingGenerator(Box<hkbPoseMatchingGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPoseMatchingGeneratorInternalState(
        Box<hkbPoseMatchingGeneratorInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPoseStoringGeneratorOutputListener(
        Box<hkbPoseStoringGeneratorOutputListener<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPoseStoringGeneratorOutputListenerStoredPose(
        Box<hkbPoseStoringGeneratorOutputListenerStoredPose<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPoweredRagdollControlData(Box<hkbPoweredRagdollControlData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbPoweredRagdollControlsModifier(Box<hkbPoweredRagdollControlsModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbProceduralBlenderGenerator(Box<hkbProceduralBlenderGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbProjectData(Box<hkbProjectData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbProjectStringData(Box<hkbProjectStringData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRadialSelectorGenerator(Box<hkbRadialSelectorGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRadialSelectorGeneratorGeneratorInfo(
        Box<hkbRadialSelectorGeneratorGeneratorInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRadialSelectorGeneratorGeneratorPair(
        Box<hkbRadialSelectorGeneratorGeneratorPair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRagdollControllerSetup(Box<hkbRagdollControllerSetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRagdollInterface(Box<hkbRagdollInterface<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRaiseEventCommand(Box<hkbRaiseEventCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRealEventPayload(Box<hkbRealEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRealVariableSequencedData(Box<hkbRealVariableSequencedData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRealVariableSequencedDataSample(Box<hkbRealVariableSequencedDataSample<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRebuildScriptsCommand(Box<hkbRebuildScriptsCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbReferencedGeneratorSyncInfo(Box<hkbReferencedGeneratorSyncInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbReferencePoseGenerator(Box<hkbReferencePoseGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRigidBodyRagdollControlData(Box<hkbRigidBodyRagdollControlData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRigidBodyRagdollControlsModifier(Box<hkbRigidBodyRagdollControlsModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRigidBodySetup(Box<hkbRigidBodySetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRoleAttribute(Box<hkbRoleAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRotateCharacterModifier(Box<hkbRotateCharacterModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbRotateCharacterModifierInternalState(
        Box<hkbRotateCharacterModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbScriptCondition(Box<hkbScriptCondition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbScriptGenerator(Box<hkbScriptGenerator<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSenseHandleModifier(Box<hkbSenseHandleModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSenseHandleModifierRange(Box<hkbSenseHandleModifierRange<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSequence(Box<hkbSequence<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSequencedData(Box<hkbSequencedData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSequenceInternalState(Box<hkbSequenceInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSequenceStringData(Box<hkbSequenceStringData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSetBehaviorCommand(Box<hkbSetBehaviorCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSetLocalTimeOfClipGeneratorCommand(
        Box<hkbSetLocalTimeOfClipGeneratorCommand<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSetNodePropertyCommand(Box<hkbSetNodePropertyCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSetSelectedCharacterCommand(Box<hkbSetSelectedCharacterCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSetWordVariableCommand(Box<hkbSetWordVariableCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSetWorldFromModelModifier(Box<hkbSetWorldFromModelModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbShapeSetup(Box<hkbShapeSetup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSimulationControlCommand(Box<hkbSimulationControlCommand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbSimulationStateInfo(Box<hkbSimulationStateInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateChooser(Box<hkbStateChooser<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateChooserWrapper(Box<hkbStateChooserWrapper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateDependentModifier(Box<hkbStateDependentModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateListener(Box<hkbStateListener<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachine(Box<hkbStateMachine<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineActiveTransitionInfo(Box<hkbStateMachineActiveTransitionInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineDelayedTransitionInfo(Box<hkbStateMachineDelayedTransitionInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineEventPropertyArray(Box<hkbStateMachineEventPropertyArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineInternalState(Box<hkbStateMachineInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineNestedStateMachineData(
        Box<hkbStateMachineNestedStateMachineData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineProspectiveTransitionInfo(
        Box<hkbStateMachineProspectiveTransitionInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineStateInfo(Box<hkbStateMachineStateInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineTimeInterval(Box<hkbStateMachineTimeInterval<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineTransitionInfo(Box<hkbStateMachineTransitionInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineTransitionInfoArray(Box<hkbStateMachineTransitionInfoArray<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStateMachineTransitionInfoReference(
        Box<hkbStateMachineTransitionInfoReference<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStringCondition(Box<hkbStringCondition<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbStringEventPayload(Box<hkbStringEventPayload<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTestIdSelector(Box<hkbTestIdSelector<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTimerModifier(Box<hkbTimerModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTimerModifierInternalState(Box<hkbTimerModifierInternalState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbToolNodeType(Box<hkbToolNodeType<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTransformVectorModifier(Box<hkbTransformVectorModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTransformVectorModifierInternalState(
        Box<hkbTransformVectorModifierInternalState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTransitionEffect(Box<hkbTransitionEffect<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbTwistModifier(Box<hkbTwistModifier<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbVariableBindingSet(Box<hkbVariableBindingSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbVariableBindingSetBinding(Box<hkbVariableBindingSetBinding<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbVariableBounds(Box<hkbVariableBounds<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbVariableInfo(Box<hkbVariableInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbVariableValue(Box<hkbVariableValue<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbVariableValueSet(Box<hkbVariableValueSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbWorldEnums(Box<hkbWorldEnums<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkbWorldFromModelModeData(Box<hkbWorldFromModelModeData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdAdf(Box<hkcdAdf<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicAabbTree(Box<hkcdDynamicAabbTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeAnisotropicMetric(Box<hkcdDynamicTreeAnisotropicMetric<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodec18(Box<hkcdDynamicTreeCodec18<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodec32(Box<hkcdDynamicTreeCodec32<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodecInt16(Box<hkcdDynamicTreeCodecInt16<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodecInt16IntAabb(Box<hkcdDynamicTreeCodecInt16IntAabb<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodecRawUint(Box<hkcdDynamicTreeCodecRawUint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodecRawUlong(Box<hkcdDynamicTreeCodecRawUlong<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodecRawunsignedint(Box<hkcdDynamicTreeCodecRawunsignedint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeCodecRawunsignedlong(Box<hkcdDynamicTreeCodecRawunsignedlong<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32(
        Box<hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16(
        Box<hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint(
        Box<hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong(
        Box<hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultTree32Storage(Box<hkcdDynamicTreeDefaultTree32Storage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultTree48Storage(Box<hkcdDynamicTreeDefaultTree48Storage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultTreeInt16Storage(
        Box<hkcdDynamicTreeDefaultTreeInt16Storage<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDefaultTreePtrStorage(Box<hkcdDynamicTreeDefaultTreePtrStorage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32(
        Box<
            hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16(
        Box<
            hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint(
        Box<
            hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong(
        Box<
            hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorage16(Box<hkcdDynamicTreeDynamicStorage16<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorage32(Box<hkcdDynamicTreeDynamicStorage32<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStorageInt16(Box<hkcdDynamicTreeDynamicStorageInt16<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeDynamicStoragePtr(Box<hkcdDynamicTreeDynamicStoragePtr<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16(
        Box<hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32(
        Box<hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16(
        Box<hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr(
        Box<hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdFourAabb(Box<hkcdFourAabb<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarCsgOperand(Box<hkcdPlanarCsgOperand<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarCsgOperandGeomSource(Box<hkcdPlanarCsgOperandGeomSource<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarEntity(Box<hkcdPlanarEntity<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarGeometry(Box<hkcdPlanarGeometry<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarGeometryPlanesCollection(Box<hkcdPlanarGeometryPlanesCollection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarGeometryPolygonCollection(Box<hkcdPlanarGeometryPolygonCollection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarGeometryPrimitivesCollection28(
        Box<hkcdPlanarGeometryPrimitivesCollection28<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarGeometryPrimitivesPlane(Box<hkcdPlanarGeometryPrimitivesPlane<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarSolid(Box<hkcdPlanarSolid<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarSolidNode(Box<hkcdPlanarSolidNode<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdPlanarSolidNodeStorage(Box<hkcdPlanarSolidNodeStorage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdShape(Box<hkcdShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdShapeDispatchType(Box<hkcdShapeDispatchType<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdShapeInfoCodecType(Box<hkcdShapeInfoCodecType<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdShapeType(Box<hkcdShapeType<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdSimdTree(Box<hkcdSimdTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdSimdTreeNode(Box<hkcdSimdTreeNode<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticAabbTree(Box<hkcdStaticAabbTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBase(Box<hkcdStaticMeshTreeBase<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBasePrimitive(Box<hkcdStaticMeshTreeBasePrimitive<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData(
        Box<
            hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort(
        Box<hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBaseSection(Box<hkcdStaticMeshTreeBaseSection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBaseSectionDataRuns(
        Box<hkcdStaticMeshTreeBaseSectionDataRuns<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBaseSectionPrimitives(
        Box<hkcdStaticMeshTreeBaseSectionPrimitives<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeBaseSectionSharedVertices(
        Box<hkcdStaticMeshTreeBaseSectionSharedVertices<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreeDefaultDataRun(Box<hkcdStaticMeshTreeDefaultDataRun<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun(
        Box<
            hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticPvs(Box<hkcdStaticPvs<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticPvsBlockHeader(Box<hkcdStaticPvsBlockHeader<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeCodec3Axis(Box<hkcdStaticTreeCodec3Axis<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeCodec3Axis4(Box<hkcdStaticTreeCodec3Axis4<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeCodec3Axis5(Box<hkcdStaticTreeCodec3Axis5<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeCodec3Axis6(Box<hkcdStaticTreeCodec3Axis6<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeCodecRaw(Box<hkcdStaticTreeCodecRaw<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDefaultTreeStorage32(Box<hkcdStaticTreeDefaultTreeStorage32<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDefaultTreeStorage4(Box<hkcdStaticTreeDefaultTreeStorage4<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDefaultTreeStorage5(Box<hkcdStaticTreeDefaultTreeStorage5<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDefaultTreeStorage6(Box<hkcdStaticTreeDefaultTreeStorage6<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStorage32(Box<hkcdStaticTreeDynamicStorage32<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStorage4(Box<hkcdStaticTreeDynamicStorage4<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStorage5(Box<hkcdStaticTreeDynamicStorage5<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStorage6(Box<hkcdStaticTreeDynamicStorage6<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4(
        Box<hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5(
        Box<hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6(
        Box<hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw(
        Box<hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32(
        Box<hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4(
        Box<hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5(
        Box<hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6(
        Box<hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkClass(Box<hkClass<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkClassEnum(Box<hkClassEnum<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkClassEnumItem(Box<hkClassEnumItem<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkClassMember(Box<hkClassMember<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkCompressedMassProperties(Box<hkCompressedMassProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkContactPoint(Box<hkContactPoint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkContactPointMaterial(Box<hkContactPointMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkCustomAttributes(Box<hkCustomAttributes<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkCustomAttributesAttribute(Box<hkCustomAttributesAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkDataObjectTypeAttribute(Box<hkDataObjectTypeAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkDefaultMemoryTracker(Box<hkDefaultMemoryTracker<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkDescriptionAttribute(Box<hkDescriptionAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkDocumentationAttribute(Box<hkDocumentationAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkFloat16(Box<hkFloat16<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkFloat16Transform(Box<hkFloat16Transform<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkFourTransposedPointsf(Box<hkFourTransposedPointsf<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations(
        Box<
            hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations(
        Box<
            hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance(
        Box<
            hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkGeometry(Box<hkGeometry<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkGeometryTriangle(Box<hkGeometryTriangle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkGizmoAttribute(Box<hkGizmoAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkIndexedTransformSet(Box<hkIndexedTransformSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkIntRealPair(Box<hkIntRealPair<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkLinkAttribute(Box<hkLinkAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkLocalFrame(Box<hkLocalFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkLocalFrameGroup(Box<hkLocalFrameGroup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMassProperties(Box<hkMassProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryMeshBody(Box<hkMemoryMeshBody<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryMeshMaterial(Box<hkMemoryMeshMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryMeshShape(Box<hkMemoryMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryMeshShapeSection(Box<hkMemoryMeshShapeSection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryMeshTexture(Box<hkMemoryMeshTexture<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryMeshVertexBuffer(Box<hkMemoryMeshVertexBuffer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryResourceContainer(Box<hkMemoryResourceContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryResourceHandle(Box<hkMemoryResourceHandle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMemoryResourceHandleExternalLink(Box<hkMemoryResourceHandleExternalLink<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshBody(Box<hkMeshBody<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshBoneIndexMapping(Box<hkMeshBoneIndexMapping<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshMaterial(Box<hkMeshMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshSection(Box<hkMeshSection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshSectionCinfo(Box<hkMeshSectionCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshShape(Box<hkMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshTexture(Box<hkMeshTexture<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshTextureRawBufferDescriptor(Box<hkMeshTextureRawBufferDescriptor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMeshVertexBuffer(Box<hkMeshVertexBuffer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkModelerNodeTypeAttribute(Box<hkModelerNodeTypeAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMonitorStreamColorTable(Box<hkMonitorStreamColorTable<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMonitorStreamColorTableColorPair(Box<hkMonitorStreamColorTableColorPair<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMonitorStreamFrameInfo(Box<hkMonitorStreamFrameInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMonitorStreamStringMap(Box<hkMonitorStreamStringMap<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMonitorStreamStringMapStringMap(Box<hkMonitorStreamStringMapStringMap<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMotionState(Box<hkMotionState<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMultipleVertexBuffer(Box<hkMultipleVertexBuffer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMultipleVertexBufferElementInfo(Box<hkMultipleVertexBufferElementInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMultipleVertexBufferLockedElement(Box<hkMultipleVertexBufferLockedElement<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMultipleVertexBufferVertexBufferInfo(
        Box<hkMultipleVertexBufferVertexBufferInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkMultiThreadCheck(Box<hkMultiThreadCheck<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpAction(Box<hknpAction<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpArrayAction(Box<hknpArrayAction<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBallGun(Box<hknpBallGun<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBinaryAction(Box<hknpBinaryAction<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBody(Box<hknpBody<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBodyCinfo(Box<hknpBodyCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBodyQuality(Box<hknpBodyQuality<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBodyQualityLibrary(Box<hknpBodyQualityLibrary<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBodyReference(Box<hknpBodyReference<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBreakableConstraintData(Box<hknpBreakableConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBridgeConstraintAtom(Box<hknpBridgeConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBroadPhaseConfig(Box<hknpBroadPhaseConfig<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBroadPhaseConfigLayer(Box<hknpBroadPhaseConfigLayer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBSMaterial(Box<hknpBSMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpBSMaterialProperties(Box<hknpBSMaterialProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCapsuleShape(Box<hknpCapsuleShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCharacterProxyCinfo(Box<hknpCharacterProxyCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCharacterRigidBodyCinfo(Box<hknpCharacterRigidBodyCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCircularSurfaceVelocity(Box<hknpCircularSurfaceVelocity<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCollisionDispatchType(Box<hknpCollisionDispatchType<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCollisionFilter(Box<hknpCollisionFilter<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompositeShape(Box<hknpCompositeShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompoundShape(Box<hknpCompoundShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompoundShapeInternalsKeyMask(Box<hknpCompoundShapeInternalsKeyMask<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompoundShapeKeyMask(Box<hknpCompoundShapeKeyMask<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedHeightFieldShape(Box<hknpCompressedHeightFieldShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedMeshShape(Box<hknpCompressedMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedMeshShapeData(Box<hknpCompressedMeshShapeData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedMeshShapeInternalsKeyMask(
        Box<hknpCompressedMeshShapeInternalsKeyMask<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedMeshShapeTree(Box<hknpCompressedMeshShapeTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedMeshShapeTreeDataRun(Box<hknpCompressedMeshShapeTreeDataRun<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpCompressedMeshShapeTreeDataRunData(
        Box<hknpCompressedMeshShapeTreeDataRunData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpConstraint(Box<hknpConstraint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpConstraintCinfo(Box<hknpConstraintCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpConstraintCollisionFilter(Box<hknpConstraintCollisionFilter<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpConvexPolytopeShape(Box<hknpConvexPolytopeShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpConvexPolytopeShapeFace(Box<hknpConvexPolytopeShapeFace<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpConvexShape(Box<hknpConvexShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDecoratorShape(Box<hknpDecoratorShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDefaultBroadPhaseConfig(Box<hknpDefaultBroadPhaseConfig<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDefaultExternMeshShapeGeometry(Box<hknpDefaultExternMeshShapeGeometry<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDestructionShapeProperties(Box<hknpDestructionShapeProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDisableCollisionFilter(Box<hknpDisableCollisionFilter<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDummyShape(Box<hknpDummyShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDynamicCompoundShape(Box<hknpDynamicCompoundShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDynamicCompoundShapeData(Box<hknpDynamicCompoundShapeData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpDynamicCompoundShapeTree(Box<hknpDynamicCompoundShapeTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpExternMeshShape(Box<hknpExternMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpExternMeshShapeData(Box<hknpExternMeshShapeData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpExternMeshShapeGeometry(Box<hknpExternMeshShapeGeometry<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpFirstPersonGun(Box<hknpFirstPersonGun<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpGroupCollisionFilter(Box<hknpGroupCollisionFilter<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516(
        Box<hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpHeightFieldShape(Box<hknpHeightFieldShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpLinearSurfaceVelocity(Box<hknpLinearSurfaceVelocity<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpLodShape(Box<hknpLodShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpLodShapeLevelOfDetailInfo(Box<hknpLodShapeLevelOfDetailInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMalleableConstraintData(Box<hknpMalleableConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMaskedShape(Box<hknpMaskedShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMaskedShapeMaskWrapper(Box<hknpMaskedShapeMaskWrapper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMaterial(Box<hknpMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMaterialDescriptor(Box<hknpMaterialDescriptor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMaterialLibrary(Box<hknpMaterialLibrary<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMaterialPalette(Box<hknpMaterialPalette<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMinMaxQuadTree(Box<hknpMinMaxQuadTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMinMaxQuadTreeMinMaxLevel(Box<hknpMinMaxQuadTreeMinMaxLevel<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMotion(Box<hknpMotion<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMotionCinfo(Box<hknpMotionCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMotionProperties(Box<hknpMotionProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpMotionPropertiesLibrary(Box<hknpMotionPropertiesLibrary<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpPairCollisionFilter(Box<hknpPairCollisionFilter<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpPairCollisionFilterMapPairFilterKeyOverrideType(
        Box<hknpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpPhysicsSceneData(Box<hknpPhysicsSceneData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpPhysicsSystemData(Box<hknpPhysicsSystemData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpProjectileGun(Box<hknpProjectileGun<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpRagdollData(Box<hknpRagdollData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpRagdollKeyFrameHierarchyUtilityControlData(
        Box<hknpRagdollKeyFrameHierarchyUtilityControlData<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpRefMaterial(Box<hknpRefMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpRefWorldCinfo(Box<hknpRefWorldCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpScaledConvexShape(Box<hknpScaledConvexShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpScaledConvexShapeBase(Box<hknpScaledConvexShapeBase<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShape(Box<hknpShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShapeInstance(Box<hknpShapeInstance<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShapeKeyMask(Box<hknpShapeKeyMask<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShapeMassProperties(Box<hknpShapeMassProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShapeSignals(Box<hknpShapeSignals<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShapeTagCodec(Box<hknpShapeTagCodec<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpShapeType(Box<hknpShapeType<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpSparseCompactMapunsignedshort(Box<hknpSparseCompactMapunsignedshort<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpSphereShape(Box<hknpSphereShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpStaticCompoundShape(Box<hknpStaticCompoundShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpStaticCompoundShapeData(Box<hknpStaticCompoundShapeData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpStaticCompoundShapeInternals(Box<hknpStaticCompoundShapeInternals<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpStaticCompoundShapeKeyMask(Box<hknpStaticCompoundShapeKeyMask<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpStaticCompoundShapeTree(Box<hknpStaticCompoundShapeTree<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpSurfaceVelocity(Box<hknpSurfaceVelocity<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpTriangleShape(Box<hknpTriangleShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpTyremarkPoint(Box<hknpTyremarkPoint<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpTyremarksInfo(Box<hknpTyremarksInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpTyremarksWheel(Box<hknpTyremarksWheel<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpUnaryAction(Box<hknpUnaryAction<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleAerodynamics(Box<hknpVehicleAerodynamics<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleBrake(Box<hknpVehicleBrake<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleData(Box<hknpVehicleData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDataWheelComponentParams(Box<hknpVehicleDataWheelComponentParams<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultAerodynamics(Box<hknpVehicleDefaultAerodynamics<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultAnalogDriverInput(Box<hknpVehicleDefaultAnalogDriverInput<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultBrake(Box<hknpVehicleDefaultBrake<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultBrakeWheelBrakingProperties(
        Box<hknpVehicleDefaultBrakeWheelBrakingProperties<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultEngine(Box<hknpVehicleDefaultEngine<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultSteering(Box<hknpVehicleDefaultSteering<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultSuspension(Box<hknpVehicleDefaultSuspension<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
        Box<hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultTransmission(Box<hknpVehicleDefaultTransmission<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDefaultVelocityDamper(Box<hknpVehicleDefaultVelocityDamper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDriverInput(Box<hknpVehicleDriverInput<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDriverInputAnalogStatus(Box<hknpVehicleDriverInputAnalogStatus<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleDriverInputStatus(Box<hknpVehicleDriverInputStatus<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleEngine(Box<hknpVehicleEngine<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleInstance(Box<hknpVehicleInstance<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleInstanceWheelInfo(Box<hknpVehicleInstanceWheelInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleLinearCastWheelCollide(Box<hknpVehicleLinearCastWheelCollide<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleLinearCastWheelCollideWheelState(
        Box<hknpVehicleLinearCastWheelCollideWheelState<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleRayCastWheelCollide(Box<hknpVehicleRayCastWheelCollide<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleSteering(Box<hknpVehicleSteering<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleSuspension(Box<hknpVehicleSuspension<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleSuspensionSuspensionWheelParameters(
        Box<hknpVehicleSuspensionSuspensionWheelParameters<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleTransmission(Box<hknpVehicleTransmission<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleVelocityDamper(Box<hknpVehicleVelocityDamper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpVehicleWheelCollide(Box<hknpVehicleWheelCollide<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpWorldCinfo(Box<hknpWorldCinfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hknpWorldSnapshot(Box<hknpWorldSnapshot<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkOffsetBitField(Box<hkOffsetBitField<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
        Box<hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkp2dAngConstraintAtom(Box<hkp2dAngConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkp3dAngConstraintAtom(Box<hkp3dAngConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkPackedVector3(Box<hkPackedVector3<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkPackedVector8_3(Box<hkPackedVector8_3<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkPackfileHeader(Box<hkPackfileHeader<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkPackfileSectionHeader(Box<hkPackfileSectionHeader<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpAngConstraintAtom(Box<hkpAngConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpAngFrictionConstraintAtom(Box<hkpAngFrictionConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpAngLimitConstraintAtom(Box<hkpAngLimitConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpAngMotorConstraintAtom(Box<hkpAngMotorConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpBallAndSocketConstraintData(Box<hkpBallAndSocketConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpBallAndSocketConstraintDataAtoms(Box<hkpBallAndSocketConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpBallSocketConstraintAtom(Box<hkpBallSocketConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpBridgeAtoms(Box<hkpBridgeAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpBridgeConstraintAtom(Box<hkpBridgeConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpCallbackConstraintMotor(Box<hkpCallbackConstraintMotor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpCogWheelConstraintAtom(Box<hkpCogWheelConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpCogWheelConstraintData(Box<hkpCogWheelConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpCogWheelConstraintDataAtoms(Box<hkpCogWheelConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpConeLimitConstraintAtom(Box<hkpConeLimitConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpConstraintAtom(Box<hkpConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpConstraintData(Box<hkpConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpConstraintMotor(Box<hkpConstraintMotor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpDeformableAngConstraintAtom(Box<hkpDeformableAngConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpDeformableFixedConstraintData(Box<hkpDeformableFixedConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpDeformableFixedConstraintDataAtoms(
        Box<hkpDeformableFixedConstraintDataAtoms<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpDeformableLinConstraintAtom(Box<hkpDeformableLinConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpFixedConstraintData(Box<hkpFixedConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpFixedConstraintDataAtoms(Box<hkpFixedConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpHingeConstraintData(Box<hkpHingeConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpHingeConstraintDataAtoms(Box<hkpHingeConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpHingeLimitsData(Box<hkpHingeLimitsData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpHingeLimitsDataAtoms(Box<hkpHingeLimitsDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLimitedForceConstraintMotor(Box<hkpLimitedForceConstraintMotor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLimitedHingeConstraintData(Box<hkpLimitedHingeConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLimitedHingeConstraintDataAtoms(Box<hkpLimitedHingeConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinConstraintAtom(Box<hkpLinConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinearClearanceConstraintData(Box<hkpLinearClearanceConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinearClearanceConstraintDataAtoms(
        Box<hkpLinearClearanceConstraintDataAtoms<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinearParametricCurve(Box<hkpLinearParametricCurve<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinFrictionConstraintAtom(Box<hkpLinFrictionConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinLimitConstraintAtom(Box<hkpLinLimitConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinMotorConstraintAtom(Box<hkpLinMotorConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpLinSoftConstraintAtom(Box<hkpLinSoftConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkPostFinishAttribute(Box<hkPostFinishAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpOverwritePivotConstraintAtom(Box<hkpOverwritePivotConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpParametricCurve(Box<hkpParametricCurve<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPointToPathConstraintData(Box<hkpPointToPathConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPointToPlaneConstraintData(Box<hkpPointToPlaneConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPointToPlaneConstraintDataAtoms(Box<hkpPointToPlaneConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPositionConstraintMotor(Box<hkpPositionConstraintMotor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPrismaticConstraintData(Box<hkpPrismaticConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPrismaticConstraintDataAtoms(Box<hkpPrismaticConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPulleyConstraintAtom(Box<hkpPulleyConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPulleyConstraintData(Box<hkpPulleyConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpPulleyConstraintDataAtoms(Box<hkpPulleyConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRackAndPinionConstraintAtom(Box<hkpRackAndPinionConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRackAndPinionConstraintData(Box<hkpRackAndPinionConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRackAndPinionConstraintDataAtoms(Box<hkpRackAndPinionConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRagdollConstraintData(Box<hkpRagdollConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRagdollConstraintDataAtoms(Box<hkpRagdollConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRagdollLimitsData(Box<hkpRagdollLimitsData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRagdollLimitsDataAtoms(Box<hkpRagdollLimitsDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRagdollMotorConstraintAtom(Box<hkpRagdollMotorConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRotationalConstraintData(Box<hkpRotationalConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpRotationalConstraintDataAtoms(Box<hkpRotationalConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpSetLocalRotationsConstraintAtom(Box<hkpSetLocalRotationsConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpSetLocalTransformsConstraintAtom(Box<hkpSetLocalTransformsConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpSetLocalTranslationsConstraintAtom(
        Box<hkpSetLocalTranslationsConstraintAtom<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpSetupStabilizationAtom(Box<hkpSetupStabilizationAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpSpringDamperConstraintMotor(Box<hkpSpringDamperConstraintMotor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpStiffSpringConstraintAtom(Box<hkpStiffSpringConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpStiffSpringConstraintData(Box<hkpStiffSpringConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpStiffSpringConstraintDataAtoms(Box<hkpStiffSpringConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpTwistLimitConstraintAtom(Box<hkpTwistLimitConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpVehicleFrictionDescription(Box<hkpVehicleFrictionDescription<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpVehicleFrictionDescriptionAxisDescription(
        Box<hkpVehicleFrictionDescriptionAxisDescription<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpVehicleFrictionStatus(Box<hkpVehicleFrictionStatus<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpVehicleFrictionStatusAxisStatus(Box<hkpVehicleFrictionStatusAxisStatus<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpVelocityConstraintMotor(Box<hkpVelocityConstraintMotor<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWheelConstraintData(Box<hkpWheelConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWheelConstraintDataAtoms(Box<hkpWheelConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWheelFrictionConstraintAtom(Box<hkpWheelFrictionConstraintAtom<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWheelFrictionConstraintAtomAxle(Box<hkpWheelFrictionConstraintAtomAxle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWheelFrictionConstraintData(Box<hkpWheelFrictionConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWheelFrictionConstraintDataAtoms(Box<hkpWheelFrictionConstraintDataAtoms<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkpWrappedConstraintData(Box<hkpWrappedConstraintData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkQTransform(Box<hkQTransform<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkQTransformf(Box<hkQTransformf<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkRangeInt32Attribute(Box<hkRangeInt32Attribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkRangeRealAttribute(Box<hkRangeRealAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkRefCountedProperties(Box<hkRefCountedProperties<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkRefCountedPropertiesEntry(Box<hkRefCountedPropertiesEntry<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkReferencedObject(Box<hkReferencedObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkResourceBase(Box<hkResourceBase<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkResourceContainer(Box<hkResourceContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkResourceHandle(Box<hkResourceHandle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkRootLevelContainer(Box<hkRootLevelContainer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkRootLevelContainerNamedVariant(Box<hkRootLevelContainerNamedVariant<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSemanticsAttribute(Box<hkSemanticsAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair(
        Box<hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSetIntFloatPair(Box<hkSetIntFloatPair<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSetUint32(Box<hkSetUint32<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSetUint64(Box<hkSetUint64<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint(
        Box<hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong(
        Box<
            hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong<
                'a,
            >,
        >,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSimpleLocalFrame(Box<hkSimpleLocalFrame<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSimpleProperty(Box<hkSimpleProperty<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSimplePropertyValue(Box<hkSimplePropertyValue<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSkinBinding(Box<hkSkinBinding<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSkinnedMeshShape(Box<hkSkinnedMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSkinnedMeshShapeBoneSection(Box<hkSkinnedMeshShapeBoneSection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSkinnedMeshShapeBoneSet(Box<hkSkinnedMeshShapeBoneSet<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSkinnedMeshShapePart(Box<hkSkinnedMeshShapePart<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSkinnedRefMeshShape(Box<hkSkinnedRefMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSphere(Box<hkSphere<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkStorageSkinnedMeshShape(Box<hkStorageSkinnedMeshShape<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkStringObject(Box<hkStringObject<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkSweptTransformf(Box<hkSweptTransformf<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkTraceStreamTitle(Box<hkTraceStreamTitle<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkUFloat8(Box<hkUFloat8<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkUiAttribute(Box<hkUiAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkUuid(Box<hkUuid<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVariableTweakingHelper(Box<hkVariableTweakingHelper<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVariableTweakingHelperBoolVariableInfo(
        Box<hkVariableTweakingHelperBoolVariableInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVariableTweakingHelperIntVariableInfo(
        Box<hkVariableTweakingHelperIntVariableInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVariableTweakingHelperRealVariableInfo(
        Box<hkVariableTweakingHelperRealVariableInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVariableTweakingHelperVector4VariableInfo(
        Box<hkVariableTweakingHelperVector4VariableInfo<'a>>,
    ),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVertexFormat(Box<hkVertexFormat<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkVertexFormatElement(Box<hkVertexFormatElement<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAnimatedFloat(Box<hkxAnimatedFloat<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAnimatedMatrix(Box<hkxAnimatedMatrix<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAnimatedQuaternion(Box<hkxAnimatedQuaternion<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAnimatedVector(Box<hkxAnimatedVector<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAttribute(Box<hkxAttribute<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAttributeGroup(Box<hkxAttributeGroup<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxAttributeHolder(Box<hkxAttributeHolder<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxEnum(Box<hkxEnum<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxEnumItem(Box<hkxEnumItem<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxIndexBuffer(Box<hkxIndexBuffer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxMaterial(Box<hkxMaterial<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxMaterialProperty(Box<hkxMaterialProperty<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxMaterialTextureStage(Box<hkxMaterialTextureStage<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxMesh(Box<hkxMesh<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxMeshSection(Box<hkxMeshSection<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxMeshUserChannelInfo(Box<hkxMeshUserChannelInfo<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxSparselyAnimatedBool(Box<hkxSparselyAnimatedBool<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxSparselyAnimatedEnum(Box<hkxSparselyAnimatedEnum<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxSparselyAnimatedInt(Box<hkxSparselyAnimatedInt<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxSparselyAnimatedString(Box<hkxSparselyAnimatedString<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxVertexAnimation(Box<hkxVertexAnimation<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxVertexAnimationUsageMap(Box<hkxVertexAnimationUsageMap<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxVertexBuffer(Box<hkxVertexBuffer<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxVertexBufferVertexData(Box<hkxVertexBufferVertexData<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxVertexDescription(Box<hkxVertexDescription<'a>>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    hkxVertexDescriptionElementDecl(Box<hkxVertexDescriptionElementDecl<'a>>),
}
impl Default for Classes<'_> {
    fn default() -> Self {
        Self::BGSGamebryoSequenceGenerator(
            Box::new(BGSGamebryoSequenceGenerator::default()),
        )
    }
}
impl _serde::HavokClass for Classes<'_> {
    fn name(&self) -> &'static str {
        match &self {
            Classes::BGSGamebryoSequenceGenerator(class) => class.name(),
            Classes::BSAlignBoneModifier(class) => class.name(),
            Classes::BSAssignVariablesModifier(class) => class.name(),
            Classes::BSBehaviorGraphSwapGenerator(class) => class.name(),
            Classes::BSBoneSwitchGenerator(class) => class.name(),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.name(),
            Classes::BSComputeAddBoneAnimModifier(class) => class.name(),
            Classes::BSCyclicBlendTransitionGenerator(class) => class.name(),
            Classes::BSDecomposeVectorModifier(class) => class.name(),
            Classes::BSDirectAtCapturePoseModifier(class) => class.name(),
            Classes::BSDirectAtModifier(class) => class.name(),
            Classes::BSDistTriggerModifier(class) => class.name(),
            Classes::BSEventEveryNEventsModifier(class) => class.name(),
            Classes::BSEventOnDeactivateModifier(class) => class.name(),
            Classes::BSEventOnFalseToTrueModifier(class) => class.name(),
            Classes::BSGetTimeStepModifier(class) => class.name(),
            Classes::BSInterpValueModifier(class) => class.name(),
            Classes::BSIsActiveModifier(class) => class.name(),
            Classes::BSIStateManagerModifier(class) => class.name(),
            Classes::BSIStateManagerModifierBSiStateData(class) => class.name(),
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.name()
            }
            Classes::BSiStateTaggingGenerator(class) => class.name(),
            Classes::BSiStateTaggingStateListener(class) => class.name(),
            Classes::BSLimbCycleModifier(class) => class.name(),
            Classes::BSLimbIKModifier(class) => class.name(),
            Classes::BSLookAtCapturePoseModifier(class) => class.name(),
            Classes::BSLookAtModifier(class) => class.name(),
            Classes::BSLookAtModifierBoneData(class) => class.name(),
            Classes::BSModifyOnceModifier(class) => class.name(),
            Classes::BSOffsetAnimationGenerator(class) => class.name(),
            Classes::BSPassByTargetTriggerModifier(class) => class.name(),
            Classes::BSRagdollContactListenerModifier(class) => class.name(),
            Classes::BSRandomAlarmModifier(class) => class.name(),
            Classes::BSRetargetBodyBlend(class) => class.name(),
            Classes::BSReverseSpineTwistModifier(class) => class.name(),
            Classes::BSRootTwistModifier(class) => class.name(),
            Classes::BSSpeedSamplerModifier(class) => class.name(),
            Classes::BSSpringState(class) => class.name(),
            Classes::BSTimerModifier(class) => class.name(),
            Classes::BSTweenerModifier(class) => class.name(),
            Classes::DynamicAnimationTaggingGenerator(class) => class.name(),
            Classes::hclAction(class) => class.name(),
            Classes::hclAntiPinchConstraintSet(class) => class.name(),
            Classes::hclAntiPinchConstraintSetPerParticle(class) => class.name(),
            Classes::hclBendLinkConstraintSet(class) => class.name(),
            Classes::hclBendLinkConstraintSetLink(class) => class.name(),
            Classes::hclBendLinkConstraintSetMx(class) => class.name(),
            Classes::hclBendLinkConstraintSetMxBatch(class) => class.name(),
            Classes::hclBendLinkConstraintSetMxSingle(class) => class.name(),
            Classes::hclBendStiffnessConstraintSet(class) => class.name(),
            Classes::hclBendStiffnessConstraintSetLink(class) => class.name(),
            Classes::hclBendStiffnessConstraintSetMx(class) => class.name(),
            Classes::hclBendStiffnessConstraintSetMxBatch(class) => class.name(),
            Classes::hclBendStiffnessConstraintSetMxSingle(class) => class.name(),
            Classes::hclBendStiffnessSetupObject(class) => class.name(),
            Classes::hclBlendSomeVerticesOperator(class) => class.name(),
            Classes::hclBlendSomeVerticesOperatorBlendEntry(class) => class.name(),
            Classes::hclBonePlanesConstraintSet(class) => class.name(),
            Classes::hclBonePlanesConstraintSetBonePlane(class) => class.name(),
            Classes::hclBoneSpaceDeformer(class) => class.name(),
            Classes::hclBoneSpaceDeformerFourBlendEntryBlock(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockP(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockPN(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockPNT(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockPNTB(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedP(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPN(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNT(class) => class.name(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNTB(class) => class.name(),
            Classes::hclBoneSpaceDeformerOneBlendEntryBlock(class) => class.name(),
            Classes::hclBoneSpaceDeformerThreeBlendEntryBlock(class) => class.name(),
            Classes::hclBoneSpaceDeformerTwoBlendEntryBlock(class) => class.name(),
            Classes::hclBoneSpaceMeshMeshDeformOperator(class) => class.name(),
            Classes::hclBoneSpaceMeshMeshDeformPNOperator(class) => class.name(),
            Classes::hclBoneSpaceMeshMeshDeformPNTBOperator(class) => class.name(),
            Classes::hclBoneSpaceMeshMeshDeformPNTOperator(class) => class.name(),
            Classes::hclBoneSpaceMeshMeshDeformPOperator(class) => class.name(),
            Classes::hclBoneSpaceSkinOperator(class) => class.name(),
            Classes::hclBoneSpaceSkinPNOperator(class) => class.name(),
            Classes::hclBoneSpaceSkinPNTBOperator(class) => class.name(),
            Classes::hclBoneSpaceSkinPNTOperator(class) => class.name(),
            Classes::hclBoneSpaceSkinPOperator(class) => class.name(),
            Classes::hclBufferDefinition(class) => class.name(),
            Classes::hclBufferLayout(class) => class.name(),
            Classes::hclBufferLayoutBufferElement(class) => class.name(),
            Classes::hclBufferLayoutSlot(class) => class.name(),
            Classes::hclBufferSetupObject(class) => class.name(),
            Classes::hclBufferUsage(class) => class.name(),
            Classes::hclCapsuleShape(class) => class.name(),
            Classes::hclClothContainer(class) => class.name(),
            Classes::hclClothData(class) => class.name(),
            Classes::hclClothSetupContainer(class) => class.name(),
            Classes::hclClothSetupObject(class) => class.name(),
            Classes::hclClothState(class) => class.name(),
            Classes::hclClothStateBufferAccess(class) => class.name(),
            Classes::hclClothStateSetupObject(class) => class.name(),
            Classes::hclClothStateTransformSetAccess(class) => class.name(),
            Classes::hclCollidable(class) => class.name(),
            Classes::hclCompressibleLinkConstraintSet(class) => class.name(),
            Classes::hclCompressibleLinkConstraintSetLink(class) => class.name(),
            Classes::hclCompressibleLinkConstraintSetMx(class) => class.name(),
            Classes::hclCompressibleLinkConstraintSetMxBatch(class) => class.name(),
            Classes::hclCompressibleLinkConstraintSetMxSingle(class) => class.name(),
            Classes::hclConstraintSet(class) => class.name(),
            Classes::hclConstraintSetSetupObject(class) => class.name(),
            Classes::hclConvexGeometryShape(class) => class.name(),
            Classes::hclConvexHeightFieldShape(class) => class.name(),
            Classes::hclConvexPlanesShape(class) => class.name(),
            Classes::hclCopyVerticesOperator(class) => class.name(),
            Classes::hclEdgeSelectionInput(class) => class.name(),
            Classes::hclGatherAllVerticesOperator(class) => class.name(),
            Classes::hclGatherSomeVerticesOperator(class) => class.name(),
            Classes::hclGatherSomeVerticesOperatorVertexPair(class) => class.name(),
            Classes::hclInputConvertOperator(class) => class.name(),
            Classes::hclLocalRangeConstraintSet(class) => class.name(),
            Classes::hclLocalRangeConstraintSetLocalConstraint(class) => class.name(),
            Classes::hclLocalRangeSetupObject(class) => class.name(),
            Classes::hclMeshBoneDeformOperator(class) => class.name(),
            Classes::hclMeshBoneDeformOperatorTriangleBonePair(class) => class.name(),
            Classes::hclMeshBoneDeformSetupObject(class) => class.name(),
            Classes::hclMeshMeshDeformOperator(class) => class.name(),
            Classes::hclMeshMeshDeformOperatorTriangleVertexPair(class) => class.name(),
            Classes::hclMoveFixedParticlesSetupObject(class) => class.name(),
            Classes::hclMoveParticlesOperator(class) => class.name(),
            Classes::hclMoveParticlesOperatorVertexParticlePair(class) => class.name(),
            Classes::hclNamedSetupMesh(class) => class.name(),
            Classes::hclNamedTransformSetSetupObject(class) => class.name(),
            Classes::hclObjectSpaceDeformer(class) => class.name(),
            Classes::hclObjectSpaceDeformerFourBlendEntryBlock(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockP(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockPN(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockPNT(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockPNTB(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedP(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPN(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNT(class) => class.name(),
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNTB(class) => class.name(),
            Classes::hclObjectSpaceDeformerOneBlendEntryBlock(class) => class.name(),
            Classes::hclObjectSpaceDeformerThreeBlendEntryBlock(class) => class.name(),
            Classes::hclObjectSpaceDeformerTwoBlendEntryBlock(class) => class.name(),
            Classes::hclObjectSpaceMeshMeshDeformOperator(class) => class.name(),
            Classes::hclObjectSpaceMeshMeshDeformPNOperator(class) => class.name(),
            Classes::hclObjectSpaceMeshMeshDeformPNTBOperator(class) => class.name(),
            Classes::hclObjectSpaceMeshMeshDeformPNTOperator(class) => class.name(),
            Classes::hclObjectSpaceMeshMeshDeformPOperator(class) => class.name(),
            Classes::hclObjectSpaceSkinOperator(class) => class.name(),
            Classes::hclObjectSpaceSkinPNOperator(class) => class.name(),
            Classes::hclObjectSpaceSkinPNTBOperator(class) => class.name(),
            Classes::hclObjectSpaceSkinPNTOperator(class) => class.name(),
            Classes::hclObjectSpaceSkinPOperator(class) => class.name(),
            Classes::hclOperator(class) => class.name(),
            Classes::hclOperatorSetupObject(class) => class.name(),
            Classes::hclOutputConvertOperator(class) => class.name(),
            Classes::hclPlaneShape(class) => class.name(),
            Classes::hclRuntimeConversionInfo(class) => class.name(),
            Classes::hclRuntimeConversionInfoElementConversion(class) => class.name(),
            Classes::hclRuntimeConversionInfoSlotConversion(class) => class.name(),
            Classes::hclScratchBufferDefinition(class) => class.name(),
            Classes::hclScratchBufferSetupObject(class) => class.name(),
            Classes::hclSetupMesh(class) => class.name(),
            Classes::hclShadowBufferDefinition(class) => class.name(),
            Classes::hclShape(class) => class.name(),
            Classes::hclSimClothBufferSetupObject(class) => class.name(),
            Classes::hclSimClothData(class) => class.name(),
            Classes::hclSimClothDataCollidablePinchingData(class) => class.name(),
            Classes::hclSimClothDataCollidableTransformMap(class) => class.name(),
            Classes::hclSimClothDataLandscapeCollisionData(class) => class.name(),
            Classes::hclSimClothDataOverridableSimulationInfo(class) => class.name(),
            Classes::hclSimClothDataParticleData(class) => class.name(),
            Classes::hclSimClothDataTransferMotionData(class) => class.name(),
            Classes::hclSimClothPose(class) => class.name(),
            Classes::hclSimClothSetupObject(class) => class.name(),
            Classes::hclSimClothSetupObjectPerInstanceCollidable(class) => class.name(),
            Classes::hclSimClothSetupObjectTransferMotionSetupData(class) => class.name(),
            Classes::hclSimpleMeshBoneDeformOperator(class) => class.name(),
            Classes::hclSimpleMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.name()
            }
            Classes::hclSimpleWindAction(class) => class.name(),
            Classes::hclSimulateOperator(class) => class.name(),
            Classes::hclSimulateSetupObject(class) => class.name(),
            Classes::hclSimulationSetupMesh(class) => class.name(),
            Classes::hclSimulationSetupMeshMapOptions(class) => class.name(),
            Classes::hclSkinOperator(class) => class.name(),
            Classes::hclSkinOperatorBoneInfluence(class) => class.name(),
            Classes::hclSkinSetupObject(class) => class.name(),
            Classes::hclSphereShape(class) => class.name(),
            Classes::hclStandardLinkConstraintSet(class) => class.name(),
            Classes::hclStandardLinkConstraintSetLink(class) => class.name(),
            Classes::hclStandardLinkConstraintSetMx(class) => class.name(),
            Classes::hclStandardLinkConstraintSetMxBatch(class) => class.name(),
            Classes::hclStandardLinkConstraintSetMxSingle(class) => class.name(),
            Classes::hclStandardLinkSetupObject(class) => class.name(),
            Classes::hclStaticDisplayBufferSetupObject(class) => class.name(),
            Classes::hclStaticShadowBufferDefinition(class) => class.name(),
            Classes::hclStretchLinkConstraintSet(class) => class.name(),
            Classes::hclStretchLinkConstraintSetLink(class) => class.name(),
            Classes::hclStretchLinkConstraintSetMx(class) => class.name(),
            Classes::hclStretchLinkConstraintSetMxBatch(class) => class.name(),
            Classes::hclStretchLinkConstraintSetMxSingle(class) => class.name(),
            Classes::hclStretchLinkSetupObject(class) => class.name(),
            Classes::hclTaperedCapsuleShape(class) => class.name(),
            Classes::hclTransformSetDefinition(class) => class.name(),
            Classes::hclTransformSetSetupObject(class) => class.name(),
            Classes::hclTransformSetUsage(class) => class.name(),
            Classes::hclTransformSetUsageTransformTracker(class) => class.name(),
            Classes::hclTransitionConstraintSet(class) => class.name(),
            Classes::hclTransitionConstraintSetPerParticle(class) => class.name(),
            Classes::hclTriangleSelectionInput(class) => class.name(),
            Classes::hclUpdateAllVertexFramesOperator(class) => class.name(),
            Classes::hclUpdateSomeVertexFramesOperator(class) => class.name(),
            Classes::hclUpdateSomeVertexFramesOperatorTriangle(class) => class.name(),
            Classes::hclVertexFloatInput(class) => class.name(),
            Classes::hclVertexGatherSetupObject(class) => class.name(),
            Classes::hclVertexSelectionInput(class) => class.name(),
            Classes::hclVolumeConstraint(class) => class.name(),
            Classes::hclVolumeConstraintApplyData(class) => class.name(),
            Classes::hclVolumeConstraintFrameData(class) => class.name(),
            Classes::hclVolumeConstraintMx(class) => class.name(),
            Classes::hclVolumeConstraintMxApplyBatchData(class) => class.name(),
            Classes::hclVolumeConstraintMxApplySingleData(class) => class.name(),
            Classes::hclVolumeConstraintMxFrameBatchData(class) => class.name(),
            Classes::hclVolumeConstraintMxFrameSingleData(class) => class.name(),
            Classes::hclVolumeConstraintSetupObject(class) => class.name(),
            Classes::hkAabb(class) => class.name(),
            Classes::hkAabb16(class) => class.name(),
            Classes::hkAabbHalf(class) => class.name(),
            Classes::hkAabbUint32(class) => class.name(),
            Classes::hkaAngularReferenceFrame(class) => class.name(),
            Classes::hkaAnimatedReferenceFrame(class) => class.name(),
            Classes::hkaAnimation(class) => class.name(),
            Classes::hkaAnimationBinding(class) => class.name(),
            Classes::hkaAnimationContainer(class) => class.name(),
            Classes::hkaAnimationPreviewColorContainer(class) => class.name(),
            Classes::hkaAnnotationTrack(class) => class.name(),
            Classes::hkaAnnotationTrackAnnotation(class) => class.name(),
            Classes::hkaBone(class) => class.name(),
            Classes::hkaBoneAttachment(class) => class.name(),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => class.name(),
            Classes::hkaDirectionalReferenceFrame(class) => class.name(),
            Classes::hkaFootstepAnalysisInfo(class) => class.name(),
            Classes::hkaFootstepAnalysisInfoContainer(class) => class.name(),
            Classes::hkaInterleavedUncompressedAnimation(class) => class.name(),
            Classes::hkaLosslessCompressedAnimation(class) => class.name(),
            Classes::hkaMeshBinding(class) => class.name(),
            Classes::hkaMeshBindingMapping(class) => class.name(),
            Classes::hkaParameterizedAnimationReferenceFrame(class) => class.name(),
            Classes::hkaParameterizedReferenceFrame(class) => class.name(),
            Classes::hkaPredictiveCompressedAnimation(class) => class.name(),
            Classes::hkaPredictiveCompressedAnimationTrackCompressionParams(class) => {
                class.name()
            }
            Classes::hkaQuantizedAnimation(class) => class.name(),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => class.name(),
            Classes::hkaReferencePoseAnimation(class) => class.name(),
            Classes::hkArrayTypeAttribute(class) => class.name(),
            Classes::hkaSkeleton(class) => class.name(),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.name(),
            Classes::hkaSkeletonMapper(class) => class.name(),
            Classes::hkaSkeletonMapperData(class) => class.name(),
            Classes::hkaSkeletonMapperDataChainMapping(class) => class.name(),
            Classes::hkaSkeletonMapperDataPartitionMappingRange(class) => class.name(),
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => class.name(),
            Classes::hkaSkeletonPartition(class) => class.name(),
            Classes::hkaSplineCompressedAnimation(class) => class.name(),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.name()
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.name()
            }
            Classes::hkbAiControlCancelPathCommand(class) => class.name(),
            Classes::hkbAiControlControlData(class) => class.name(),
            Classes::hkbAiControlControlDataBlendable(class) => class.name(),
            Classes::hkbAiControlControlDataNonBlendable(class) => class.name(),
            Classes::hkbAiControlPathToCommand(class) => class.name(),
            Classes::hkbAlignBoneModifier(class) => class.name(),
            Classes::hkbAnimatedSkeletonGenerator(class) => class.name(),
            Classes::hkBaseObject(class) => class.name(),
            Classes::hkbAssetBundle(class) => class.name(),
            Classes::hkbAssetBundleStringData(class) => class.name(),
            Classes::hkbAttachmentModifier(class) => class.name(),
            Classes::hkbAttachmentSetup(class) => class.name(),
            Classes::hkbAttributeModifier(class) => class.name(),
            Classes::hkbAttributeModifierAssignment(class) => class.name(),
            Classes::hkbAuxiliaryNodeInfo(class) => class.name(),
            Classes::hkbBehaviorEventsInfo(class) => class.name(),
            Classes::hkbBehaviorGraph(class) => class.name(),
            Classes::hkbBehaviorGraphData(class) => class.name(),
            Classes::hkbBehaviorGraphInternalState(class) => class.name(),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => class.name(),
            Classes::hkbBehaviorGraphStringData(class) => class.name(),
            Classes::hkbBehaviorInfo(class) => class.name(),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.name(),
            Classes::hkbBehaviorReferenceGenerator(class) => class.name(),
            Classes::hkbBindable(class) => class.name(),
            Classes::hkbBlendCurveUtils(class) => class.name(),
            Classes::hkbBlenderGenerator(class) => class.name(),
            Classes::hkbBlenderGeneratorChild(class) => class.name(),
            Classes::hkbBlenderGeneratorChildInternalState(class) => class.name(),
            Classes::hkbBlenderGeneratorInternalState(class) => class.name(),
            Classes::hkbBlendingTransitionEffect(class) => class.name(),
            Classes::hkbBlendingTransitionEffectInternalState(class) => class.name(),
            Classes::hkbBoneIndexArray(class) => class.name(),
            Classes::hkbBoneWeightArray(class) => class.name(),
            Classes::hkbBoolVariableSequencedData(class) => class.name(),
            Classes::hkbBoolVariableSequencedDataSample(class) => class.name(),
            Classes::hkbCameraShakeEventPayload(class) => class.name(),
            Classes::hkbCameraVariablesChangedCommand(class) => class.name(),
            Classes::hkbCharacter(class) => class.name(),
            Classes::hkbCharacterAddedInfo(class) => class.name(),
            Classes::hkbCharacterControlCommand(class) => class.name(),
            Classes::hkbCharacterControllerModifier(class) => class.name(),
            Classes::hkbCharacterControllerModifierControlData(class) => class.name(),
            Classes::hkbCharacterControllerModifierInternalState(class) => class.name(),
            Classes::hkbCharacterControllerSetup(class) => class.name(),
            Classes::hkbCharacterData(class) => class.name(),
            Classes::hkbCharacterInfo(class) => class.name(),
            Classes::hkbCharacterSelectedInfo(class) => class.name(),
            Classes::hkbCharacterSetup(class) => class.name(),
            Classes::hkbCharacterSkinInfo(class) => class.name(),
            Classes::hkbCharacterSteppedInfo(class) => class.name(),
            Classes::hkbCharacterStringData(class) => class.name(),
            Classes::hkbCharacterStringDataFileNameMeshNamePair(class) => class.name(),
            Classes::hkbClientCharacterState(class) => class.name(),
            Classes::hkbClipGenerator(class) => class.name(),
            Classes::hkbClipGeneratorEcho(class) => class.name(),
            Classes::hkbClipGeneratorInternalState(class) => class.name(),
            Classes::hkbClipTrigger(class) => class.name(),
            Classes::hkbClipTriggerArray(class) => class.name(),
            Classes::hkbCombineTransformsModifier(class) => class.name(),
            Classes::hkbCombineTransformsModifierInternalState(class) => class.name(),
            Classes::hkbCompiledExpressionSet(class) => class.name(),
            Classes::hkbCompiledExpressionSetToken(class) => class.name(),
            Classes::hkbComputeDirectionModifier(class) => class.name(),
            Classes::hkbComputeDirectionModifierInternalState(class) => class.name(),
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => class.name(),
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.name()
            }
            Classes::hkbComputeRotationToTargetModifier(class) => class.name(),
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.name()
            }
            Classes::hkbComputeWorldFromModelModifier(class) => class.name(),
            Classes::hkbCondition(class) => class.name(),
            Classes::hkbConstraintSetup(class) => class.name(),
            Classes::hkbContext(class) => class.name(),
            Classes::hkbCustomIdSelector(class) => class.name(),
            Classes::hkbCustomTestGenerator(class) => class.name(),
            Classes::hkbCustomTestGeneratorAnnotatedTypes(class) => class.name(),
            Classes::hkbCustomTestGeneratorBoneTypes(class) => class.name(),
            Classes::hkbCustomTestGeneratorComplexTypes(class) => class.name(),
            Classes::hkbCustomTestGeneratorHiddenTypes(class) => class.name(),
            Classes::hkbCustomTestGeneratorNestedTypes(class) => class.name(),
            Classes::hkbCustomTestGeneratorNestedTypesBase(class) => class.name(),
            Classes::hkbCustomTestGeneratorSimpleTypes(class) => class.name(),
            Classes::hkbDampingModifier(class) => class.name(),
            Classes::hkbDampingModifierInternalState(class) => class.name(),
            Classes::hkbDelayedModifier(class) => class.name(),
            Classes::hkbDelayedModifierInternalState(class) => class.name(),
            Classes::hkbDetectCloseToGroundModifier(class) => class.name(),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => class.name(),
            Classes::hkbDockingGenerator(class) => class.name(),
            Classes::hkbDockingGeneratorInternalState(class) => class.name(),
            Classes::hkbEvaluateExpressionModifier(class) => class.name(),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.name()
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => class.name(),
            Classes::hkbEvaluateHandleModifier(class) => class.name(),
            Classes::hkbEvent(class) => class.name(),
            Classes::hkbEventBase(class) => class.name(),
            Classes::hkbEventDrivenModifier(class) => class.name(),
            Classes::hkbEventDrivenModifierInternalState(class) => class.name(),
            Classes::hkbEventInfo(class) => class.name(),
            Classes::hkbEventPayload(class) => class.name(),
            Classes::hkbEventPayloadList(class) => class.name(),
            Classes::hkbEventProperty(class) => class.name(),
            Classes::hkbEventRaisedInfo(class) => class.name(),
            Classes::hkbEventRangeData(class) => class.name(),
            Classes::hkbEventRangeDataArray(class) => class.name(),
            Classes::hkbEventSequencedData(class) => class.name(),
            Classes::hkbEventSequencedDataSequencedEvent(class) => class.name(),
            Classes::hkbEventsFromRangeModifier(class) => class.name(),
            Classes::hkbEventsFromRangeModifierInternalState(class) => class.name(),
            Classes::hkbExpressionCondition(class) => class.name(),
            Classes::hkbExpressionData(class) => class.name(),
            Classes::hkbExpressionDataArray(class) => class.name(),
            Classes::hkbExtractRagdollPoseModifier(class) => class.name(),
            Classes::hkbExtrapolatingTransitionEffect(class) => class.name(),
            Classes::hkbExtrapolatingTransitionEffectInternalState(class) => class.name(),
            Classes::hkbFootIkControlData(class) => class.name(),
            Classes::hkbFootIkControlsModifier(class) => class.name(),
            Classes::hkbFootIkControlsModifierLeg(class) => class.name(),
            Classes::hkbFootIkDriverInfo(class) => class.name(),
            Classes::hkbFootIkDriverInfoLeg(class) => class.name(),
            Classes::hkbFootIkGains(class) => class.name(),
            Classes::hkbFootIkModifier(class) => class.name(),
            Classes::hkbFootIkModifierInternalLegData(class) => class.name(),
            Classes::hkbFootIkModifierLeg(class) => class.name(),
            Classes::hkbGenerator(class) => class.name(),
            Classes::hkbGeneratorOutputListener(class) => class.name(),
            Classes::hkbGeneratorPartitionInfo(class) => class.name(),
            Classes::hkbGeneratorSyncInfo(class) => class.name(),
            Classes::hkbGeneratorSyncInfoActiveInterval(class) => class.name(),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.name(),
            Classes::hkbGeneratorTransitionEffect(class) => class.name(),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => class.name(),
            Classes::hkbGetHandleOnBoneModifier(class) => class.name(),
            Classes::hkbGetUpModifier(class) => class.name(),
            Classes::hkbGetUpModifierInternalState(class) => class.name(),
            Classes::hkbGetWorldFromModelModifier(class) => class.name(),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => class.name(),
            Classes::hkbGravityModifier(class) => class.name(),
            Classes::hkbHandIkControlData(class) => class.name(),
            Classes::hkbHandIkControlsModifier(class) => class.name(),
            Classes::hkbHandIkControlsModifierHand(class) => class.name(),
            Classes::hkbHandIkDriverInfo(class) => class.name(),
            Classes::hkbHandIkDriverInfoHand(class) => class.name(),
            Classes::hkbHandIkModifier(class) => class.name(),
            Classes::hkbHandIkModifierHand(class) => class.name(),
            Classes::hkbHandle(class) => class.name(),
            Classes::hkbHoldFromBlendingTransitionEffect(class) => class.name(),
            Classes::hkbIntEventPayload(class) => class.name(),
            Classes::hkbIntVariableSequencedData(class) => class.name(),
            Classes::hkbIntVariableSequencedDataSample(class) => class.name(),
            Classes::hkBitField(class) => class.name(),
            Classes::hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.name(),
            Classes::hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.name(),
            Classes::hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.name(),
            Classes::hkbJigglerGroup(class) => class.name(),
            Classes::hkbJigglerModifier(class) => class.name(),
            Classes::hkbJigglerModifierInternalState(class) => class.name(),
            Classes::hkbKeyframeBonesModifier(class) => class.name(),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => class.name(),
            Classes::hkbKeyFrameControlData(class) => class.name(),
            Classes::hkbLayer(class) => class.name(),
            Classes::hkbLayerGenerator(class) => class.name(),
            Classes::hkbLayerGeneratorInternalState(class) => class.name(),
            Classes::hkbLayerGeneratorLayerInternalState(class) => class.name(),
            Classes::hkbLinkedSymbolInfo(class) => class.name(),
            Classes::hkbLookAtModifier(class) => class.name(),
            Classes::hkbLookAtModifierInternalState(class) => class.name(),
            Classes::hkbManualSelectorGenerator(class) => class.name(),
            Classes::hkbManualSelectorGeneratorInternalState(class) => class.name(),
            Classes::hkbManualSelectorTransitionEffect(class) => class.name(),
            Classes::hkbManualSelectorTransitionEffectInternalState(class) => {
                class.name()
            }
            Classes::hkbMessageLog(class) => class.name(),
            Classes::hkbMirroredSkeletonInfo(class) => class.name(),
            Classes::hkbMirrorModifier(class) => class.name(),
            Classes::hkbModifier(class) => class.name(),
            Classes::hkbModifierGenerator(class) => class.name(),
            Classes::hkbModifierList(class) => class.name(),
            Classes::hkbModifierWrapper(class) => class.name(),
            Classes::hkbMoveBoneAttachmentModifier(class) => class.name(),
            Classes::hkbMoveCharacterModifier(class) => class.name(),
            Classes::hkbMoveCharacterModifierInternalState(class) => class.name(),
            Classes::hkbNamedEventPayload(class) => class.name(),
            Classes::hkbNamedIntEventPayload(class) => class.name(),
            Classes::hkbNamedRealEventPayload(class) => class.name(),
            Classes::hkbNamedStringEventPayload(class) => class.name(),
            Classes::hkbNode(class) => class.name(),
            Classes::hkbNodeInternalStateInfo(class) => class.name(),
            Classes::hkbnpPhysicsInterface(class) => class.name(),
            Classes::hkbnpRagdollInterface(class) => class.name(),
            Classes::hkbNullPhysicsInterface(class) => class.name(),
            Classes::hkbParametricMotionGenerator(class) => class.name(),
            Classes::hkbParticleSystemEventPayload(class) => class.name(),
            Classes::hkbPhysicsInterface(class) => class.name(),
            Classes::hkbPinBonesGenerator(class) => class.name(),
            Classes::hkbPoseMatchingGenerator(class) => class.name(),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => class.name(),
            Classes::hkbPoseStoringGeneratorOutputListener(class) => class.name(),
            Classes::hkbPoseStoringGeneratorOutputListenerStoredPose(class) => {
                class.name()
            }
            Classes::hkbPoweredRagdollControlData(class) => class.name(),
            Classes::hkbPoweredRagdollControlsModifier(class) => class.name(),
            Classes::hkbProceduralBlenderGenerator(class) => class.name(),
            Classes::hkbProjectData(class) => class.name(),
            Classes::hkbProjectStringData(class) => class.name(),
            Classes::hkbRadialSelectorGenerator(class) => class.name(),
            Classes::hkbRadialSelectorGeneratorGeneratorInfo(class) => class.name(),
            Classes::hkbRadialSelectorGeneratorGeneratorPair(class) => class.name(),
            Classes::hkbRagdollControllerSetup(class) => class.name(),
            Classes::hkbRagdollInterface(class) => class.name(),
            Classes::hkbRaiseEventCommand(class) => class.name(),
            Classes::hkbRealEventPayload(class) => class.name(),
            Classes::hkbRealVariableSequencedData(class) => class.name(),
            Classes::hkbRealVariableSequencedDataSample(class) => class.name(),
            Classes::hkbRebuildScriptsCommand(class) => class.name(),
            Classes::hkbReferencedGeneratorSyncInfo(class) => class.name(),
            Classes::hkbReferencePoseGenerator(class) => class.name(),
            Classes::hkbRigidBodyRagdollControlData(class) => class.name(),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => class.name(),
            Classes::hkbRigidBodySetup(class) => class.name(),
            Classes::hkbRoleAttribute(class) => class.name(),
            Classes::hkbRotateCharacterModifier(class) => class.name(),
            Classes::hkbRotateCharacterModifierInternalState(class) => class.name(),
            Classes::hkbScriptCondition(class) => class.name(),
            Classes::hkbScriptGenerator(class) => class.name(),
            Classes::hkbSenseHandleModifier(class) => class.name(),
            Classes::hkbSenseHandleModifierRange(class) => class.name(),
            Classes::hkbSequence(class) => class.name(),
            Classes::hkbSequencedData(class) => class.name(),
            Classes::hkbSequenceInternalState(class) => class.name(),
            Classes::hkbSequenceStringData(class) => class.name(),
            Classes::hkbSetBehaviorCommand(class) => class.name(),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => class.name(),
            Classes::hkbSetNodePropertyCommand(class) => class.name(),
            Classes::hkbSetSelectedCharacterCommand(class) => class.name(),
            Classes::hkbSetWordVariableCommand(class) => class.name(),
            Classes::hkbSetWorldFromModelModifier(class) => class.name(),
            Classes::hkbShapeSetup(class) => class.name(),
            Classes::hkbSimulationControlCommand(class) => class.name(),
            Classes::hkbSimulationStateInfo(class) => class.name(),
            Classes::hkbStateChooser(class) => class.name(),
            Classes::hkbStateChooserWrapper(class) => class.name(),
            Classes::hkbStateDependentModifier(class) => class.name(),
            Classes::hkbStateListener(class) => class.name(),
            Classes::hkbStateMachine(class) => class.name(),
            Classes::hkbStateMachineActiveTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineDelayedTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineEventPropertyArray(class) => class.name(),
            Classes::hkbStateMachineInternalState(class) => class.name(),
            Classes::hkbStateMachineNestedStateMachineData(class) => class.name(),
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineStateInfo(class) => class.name(),
            Classes::hkbStateMachineTimeInterval(class) => class.name(),
            Classes::hkbStateMachineTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineTransitionInfoArray(class) => class.name(),
            Classes::hkbStateMachineTransitionInfoReference(class) => class.name(),
            Classes::hkbStringCondition(class) => class.name(),
            Classes::hkbStringEventPayload(class) => class.name(),
            Classes::hkbTestIdSelector(class) => class.name(),
            Classes::hkbTimerModifier(class) => class.name(),
            Classes::hkbTimerModifierInternalState(class) => class.name(),
            Classes::hkbToolNodeType(class) => class.name(),
            Classes::hkbTransformVectorModifier(class) => class.name(),
            Classes::hkbTransformVectorModifierInternalState(class) => class.name(),
            Classes::hkbTransitionEffect(class) => class.name(),
            Classes::hkbTwistModifier(class) => class.name(),
            Classes::hkbVariableBindingSet(class) => class.name(),
            Classes::hkbVariableBindingSetBinding(class) => class.name(),
            Classes::hkbVariableBounds(class) => class.name(),
            Classes::hkbVariableInfo(class) => class.name(),
            Classes::hkbVariableValue(class) => class.name(),
            Classes::hkbVariableValueSet(class) => class.name(),
            Classes::hkbWorldEnums(class) => class.name(),
            Classes::hkbWorldFromModelModeData(class) => class.name(),
            Classes::hkcdAdf(class) => class.name(),
            Classes::hkcdDynamicAabbTree(class) => class.name(),
            Classes::hkcdDynamicTreeAnisotropicMetric(class) => class.name(),
            Classes::hkcdDynamicTreeCodec18(class) => class.name(),
            Classes::hkcdDynamicTreeCodec32(class) => class.name(),
            Classes::hkcdDynamicTreeCodecInt16(class) => class.name(),
            Classes::hkcdDynamicTreeCodecInt16IntAabb(class) => class.name(),
            Classes::hkcdDynamicTreeCodecRawUint(class) => class.name(),
            Classes::hkcdDynamicTreeCodecRawUlong(class) => class.name(),
            Classes::hkcdDynamicTreeCodecRawunsignedint(class) => class.name(),
            Classes::hkcdDynamicTreeCodecRawunsignedlong(class) => class.name(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDefaultTree32Storage(class) => class.name(),
            Classes::hkcdDynamicTreeDefaultTree48Storage(class) => class.name(),
            Classes::hkcdDynamicTreeDefaultTreeInt16Storage(class) => class.name(),
            Classes::hkcdDynamicTreeDefaultTreePtrStorage(class) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorage16(class) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorage32(class) => class.name(),
            Classes::hkcdDynamicTreeDynamicStorageInt16(class) => class.name(),
            Classes::hkcdDynamicTreeDynamicStoragePtr(class) => class.name(),
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16(class) => {
                class.name()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32(class) => {
                class.name()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16(class) => {
                class.name()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr(class) => {
                class.name()
            }
            Classes::hkcdFourAabb(class) => class.name(),
            Classes::hkcdPlanarCsgOperand(class) => class.name(),
            Classes::hkcdPlanarCsgOperandGeomSource(class) => class.name(),
            Classes::hkcdPlanarEntity(class) => class.name(),
            Classes::hkcdPlanarGeometry(class) => class.name(),
            Classes::hkcdPlanarGeometryPlanesCollection(class) => class.name(),
            Classes::hkcdPlanarGeometryPolygonCollection(class) => class.name(),
            Classes::hkcdPlanarGeometryPrimitivesCollection28(class) => class.name(),
            Classes::hkcdPlanarGeometryPrimitivesPlane(class) => class.name(),
            Classes::hkcdPlanarSolid(class) => class.name(),
            Classes::hkcdPlanarSolidNode(class) => class.name(),
            Classes::hkcdPlanarSolidNodeStorage(class) => class.name(),
            Classes::hkcdShape(class) => class.name(),
            Classes::hkcdShapeDispatchType(class) => class.name(),
            Classes::hkcdShapeInfoCodecType(class) => class.name(),
            Classes::hkcdShapeType(class) => class.name(),
            Classes::hkcdSimdTree(class) => class.name(),
            Classes::hkcdSimdTreeNode(class) => class.name(),
            Classes::hkcdStaticAabbTree(class) => class.name(),
            Classes::hkcdStaticMeshTreeBase(class) => class.name(),
            Classes::hkcdStaticMeshTreeBasePrimitive(class) => class.name(),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData(
                class,
            ) => class.name(),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort(class) => {
                class.name()
            }
            Classes::hkcdStaticMeshTreeBaseSection(class) => class.name(),
            Classes::hkcdStaticMeshTreeBaseSectionDataRuns(class) => class.name(),
            Classes::hkcdStaticMeshTreeBaseSectionPrimitives(class) => class.name(),
            Classes::hkcdStaticMeshTreeBaseSectionSharedVertices(class) => class.name(),
            Classes::hkcdStaticMeshTreeDefaultDataRun(class) => class.name(),
            Classes::hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun(
                class,
            ) => class.name(),
            Classes::hkcdStaticPvs(class) => class.name(),
            Classes::hkcdStaticPvsBlockHeader(class) => class.name(),
            Classes::hkcdStaticTreeCodec3Axis(class) => class.name(),
            Classes::hkcdStaticTreeCodec3Axis4(class) => class.name(),
            Classes::hkcdStaticTreeCodec3Axis5(class) => class.name(),
            Classes::hkcdStaticTreeCodec3Axis6(class) => class.name(),
            Classes::hkcdStaticTreeCodecRaw(class) => class.name(),
            Classes::hkcdStaticTreeDefaultTreeStorage32(class) => class.name(),
            Classes::hkcdStaticTreeDefaultTreeStorage4(class) => class.name(),
            Classes::hkcdStaticTreeDefaultTreeStorage5(class) => class.name(),
            Classes::hkcdStaticTreeDefaultTreeStorage6(class) => class.name(),
            Classes::hkcdStaticTreeDynamicStorage32(class) => class.name(),
            Classes::hkcdStaticTreeDynamicStorage4(class) => class.name(),
            Classes::hkcdStaticTreeDynamicStorage5(class) => class.name(),
            Classes::hkcdStaticTreeDynamicStorage6(class) => class.name(),
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5(class) => {
                class.name()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6(class) => {
                class.name()
            }
            Classes::hkClass(class) => class.name(),
            Classes::hkClassEnum(class) => class.name(),
            Classes::hkClassEnumItem(class) => class.name(),
            Classes::hkClassMember(class) => class.name(),
            Classes::hkCompressedMassProperties(class) => class.name(),
            Classes::hkContactPoint(class) => class.name(),
            Classes::hkContactPointMaterial(class) => class.name(),
            Classes::hkCustomAttributes(class) => class.name(),
            Classes::hkCustomAttributesAttribute(class) => class.name(),
            Classes::hkDataObjectTypeAttribute(class) => class.name(),
            Classes::hkDefaultMemoryTracker(class) => class.name(),
            Classes::hkDescriptionAttribute(class) => class.name(),
            Classes::hkDocumentationAttribute(class) => class.name(),
            Classes::hkFloat16(class) => class.name(),
            Classes::hkFloat16Transform(class) => class.name(),
            Classes::hkFourTransposedPointsf(class) => class.name(),
            Classes::hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations(
                class,
            ) => class.name(),
            Classes::hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations(
                class,
            ) => class.name(),
            Classes::hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance(
                class,
            ) => class.name(),
            Classes::hkGeometry(class) => class.name(),
            Classes::hkGeometryTriangle(class) => class.name(),
            Classes::hkGizmoAttribute(class) => class.name(),
            Classes::hkIndexedTransformSet(class) => class.name(),
            Classes::hkIntRealPair(class) => class.name(),
            Classes::hkLinkAttribute(class) => class.name(),
            Classes::hkLocalFrame(class) => class.name(),
            Classes::hkLocalFrameGroup(class) => class.name(),
            Classes::hkMassProperties(class) => class.name(),
            Classes::hkMemoryMeshBody(class) => class.name(),
            Classes::hkMemoryMeshMaterial(class) => class.name(),
            Classes::hkMemoryMeshShape(class) => class.name(),
            Classes::hkMemoryMeshShapeSection(class) => class.name(),
            Classes::hkMemoryMeshTexture(class) => class.name(),
            Classes::hkMemoryMeshVertexBuffer(class) => class.name(),
            Classes::hkMemoryResourceContainer(class) => class.name(),
            Classes::hkMemoryResourceHandle(class) => class.name(),
            Classes::hkMemoryResourceHandleExternalLink(class) => class.name(),
            Classes::hkMeshBody(class) => class.name(),
            Classes::hkMeshBoneIndexMapping(class) => class.name(),
            Classes::hkMeshMaterial(class) => class.name(),
            Classes::hkMeshSection(class) => class.name(),
            Classes::hkMeshSectionCinfo(class) => class.name(),
            Classes::hkMeshShape(class) => class.name(),
            Classes::hkMeshTexture(class) => class.name(),
            Classes::hkMeshTextureRawBufferDescriptor(class) => class.name(),
            Classes::hkMeshVertexBuffer(class) => class.name(),
            Classes::hkModelerNodeTypeAttribute(class) => class.name(),
            Classes::hkMonitorStreamColorTable(class) => class.name(),
            Classes::hkMonitorStreamColorTableColorPair(class) => class.name(),
            Classes::hkMonitorStreamFrameInfo(class) => class.name(),
            Classes::hkMonitorStreamStringMap(class) => class.name(),
            Classes::hkMonitorStreamStringMapStringMap(class) => class.name(),
            Classes::hkMotionState(class) => class.name(),
            Classes::hkMultipleVertexBuffer(class) => class.name(),
            Classes::hkMultipleVertexBufferElementInfo(class) => class.name(),
            Classes::hkMultipleVertexBufferLockedElement(class) => class.name(),
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => class.name(),
            Classes::hkMultiThreadCheck(class) => class.name(),
            Classes::hknpAction(class) => class.name(),
            Classes::hknpArrayAction(class) => class.name(),
            Classes::hknpBallGun(class) => class.name(),
            Classes::hknpBinaryAction(class) => class.name(),
            Classes::hknpBody(class) => class.name(),
            Classes::hknpBodyCinfo(class) => class.name(),
            Classes::hknpBodyQuality(class) => class.name(),
            Classes::hknpBodyQualityLibrary(class) => class.name(),
            Classes::hknpBodyReference(class) => class.name(),
            Classes::hknpBreakableConstraintData(class) => class.name(),
            Classes::hknpBridgeConstraintAtom(class) => class.name(),
            Classes::hknpBroadPhaseConfig(class) => class.name(),
            Classes::hknpBroadPhaseConfigLayer(class) => class.name(),
            Classes::hknpBSMaterial(class) => class.name(),
            Classes::hknpBSMaterialProperties(class) => class.name(),
            Classes::hknpCapsuleShape(class) => class.name(),
            Classes::hknpCharacterProxyCinfo(class) => class.name(),
            Classes::hknpCharacterRigidBodyCinfo(class) => class.name(),
            Classes::hknpCircularSurfaceVelocity(class) => class.name(),
            Classes::hknpCollisionDispatchType(class) => class.name(),
            Classes::hknpCollisionFilter(class) => class.name(),
            Classes::hknpCompositeShape(class) => class.name(),
            Classes::hknpCompoundShape(class) => class.name(),
            Classes::hknpCompoundShapeInternalsKeyMask(class) => class.name(),
            Classes::hknpCompoundShapeKeyMask(class) => class.name(),
            Classes::hknpCompressedHeightFieldShape(class) => class.name(),
            Classes::hknpCompressedMeshShape(class) => class.name(),
            Classes::hknpCompressedMeshShapeData(class) => class.name(),
            Classes::hknpCompressedMeshShapeInternalsKeyMask(class) => class.name(),
            Classes::hknpCompressedMeshShapeTree(class) => class.name(),
            Classes::hknpCompressedMeshShapeTreeDataRun(class) => class.name(),
            Classes::hknpCompressedMeshShapeTreeDataRunData(class) => class.name(),
            Classes::hknpConstraint(class) => class.name(),
            Classes::hknpConstraintCinfo(class) => class.name(),
            Classes::hknpConstraintCollisionFilter(class) => class.name(),
            Classes::hknpConvexPolytopeShape(class) => class.name(),
            Classes::hknpConvexPolytopeShapeFace(class) => class.name(),
            Classes::hknpConvexShape(class) => class.name(),
            Classes::hknpDecoratorShape(class) => class.name(),
            Classes::hknpDefaultBroadPhaseConfig(class) => class.name(),
            Classes::hknpDefaultExternMeshShapeGeometry(class) => class.name(),
            Classes::hknpDestructionShapeProperties(class) => class.name(),
            Classes::hknpDisableCollisionFilter(class) => class.name(),
            Classes::hknpDummyShape(class) => class.name(),
            Classes::hknpDynamicCompoundShape(class) => class.name(),
            Classes::hknpDynamicCompoundShapeData(class) => class.name(),
            Classes::hknpDynamicCompoundShapeTree(class) => class.name(),
            Classes::hknpExternMeshShape(class) => class.name(),
            Classes::hknpExternMeshShapeData(class) => class.name(),
            Classes::hknpExternMeshShapeGeometry(class) => class.name(),
            Classes::hknpFirstPersonGun(class) => class.name(),
            Classes::hknpGroupCollisionFilter(class) => class.name(),
            Classes::hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516(
                class,
            ) => class.name(),
            Classes::hknpHeightFieldShape(class) => class.name(),
            Classes::hknpLinearSurfaceVelocity(class) => class.name(),
            Classes::hknpLodShape(class) => class.name(),
            Classes::hknpLodShapeLevelOfDetailInfo(class) => class.name(),
            Classes::hknpMalleableConstraintData(class) => class.name(),
            Classes::hknpMaskedShape(class) => class.name(),
            Classes::hknpMaskedShapeMaskWrapper(class) => class.name(),
            Classes::hknpMaterial(class) => class.name(),
            Classes::hknpMaterialDescriptor(class) => class.name(),
            Classes::hknpMaterialLibrary(class) => class.name(),
            Classes::hknpMaterialPalette(class) => class.name(),
            Classes::hknpMinMaxQuadTree(class) => class.name(),
            Classes::hknpMinMaxQuadTreeMinMaxLevel(class) => class.name(),
            Classes::hknpMotion(class) => class.name(),
            Classes::hknpMotionCinfo(class) => class.name(),
            Classes::hknpMotionProperties(class) => class.name(),
            Classes::hknpMotionPropertiesLibrary(class) => class.name(),
            Classes::hknpPairCollisionFilter(class) => class.name(),
            Classes::hknpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.name()
            }
            Classes::hknpPhysicsSceneData(class) => class.name(),
            Classes::hknpPhysicsSystemData(class) => class.name(),
            Classes::hknpProjectileGun(class) => class.name(),
            Classes::hknpRagdollData(class) => class.name(),
            Classes::hknpRagdollKeyFrameHierarchyUtilityControlData(class) => {
                class.name()
            }
            Classes::hknpRefMaterial(class) => class.name(),
            Classes::hknpRefWorldCinfo(class) => class.name(),
            Classes::hknpScaledConvexShape(class) => class.name(),
            Classes::hknpScaledConvexShapeBase(class) => class.name(),
            Classes::hknpShape(class) => class.name(),
            Classes::hknpShapeInstance(class) => class.name(),
            Classes::hknpShapeKeyMask(class) => class.name(),
            Classes::hknpShapeMassProperties(class) => class.name(),
            Classes::hknpShapeSignals(class) => class.name(),
            Classes::hknpShapeTagCodec(class) => class.name(),
            Classes::hknpShapeType(class) => class.name(),
            Classes::hknpSparseCompactMapunsignedshort(class) => class.name(),
            Classes::hknpSphereShape(class) => class.name(),
            Classes::hknpStaticCompoundShape(class) => class.name(),
            Classes::hknpStaticCompoundShapeData(class) => class.name(),
            Classes::hknpStaticCompoundShapeInternals(class) => class.name(),
            Classes::hknpStaticCompoundShapeKeyMask(class) => class.name(),
            Classes::hknpStaticCompoundShapeTree(class) => class.name(),
            Classes::hknpSurfaceVelocity(class) => class.name(),
            Classes::hknpTriangleShape(class) => class.name(),
            Classes::hknpTyremarkPoint(class) => class.name(),
            Classes::hknpTyremarksInfo(class) => class.name(),
            Classes::hknpTyremarksWheel(class) => class.name(),
            Classes::hknpUnaryAction(class) => class.name(),
            Classes::hknpVehicleAerodynamics(class) => class.name(),
            Classes::hknpVehicleBrake(class) => class.name(),
            Classes::hknpVehicleData(class) => class.name(),
            Classes::hknpVehicleDataWheelComponentParams(class) => class.name(),
            Classes::hknpVehicleDefaultAerodynamics(class) => class.name(),
            Classes::hknpVehicleDefaultAnalogDriverInput(class) => class.name(),
            Classes::hknpVehicleDefaultBrake(class) => class.name(),
            Classes::hknpVehicleDefaultBrakeWheelBrakingProperties(class) => class.name(),
            Classes::hknpVehicleDefaultEngine(class) => class.name(),
            Classes::hknpVehicleDefaultSteering(class) => class.name(),
            Classes::hknpVehicleDefaultSuspension(class) => class.name(),
            Classes::hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.name(),
            Classes::hknpVehicleDefaultTransmission(class) => class.name(),
            Classes::hknpVehicleDefaultVelocityDamper(class) => class.name(),
            Classes::hknpVehicleDriverInput(class) => class.name(),
            Classes::hknpVehicleDriverInputAnalogStatus(class) => class.name(),
            Classes::hknpVehicleDriverInputStatus(class) => class.name(),
            Classes::hknpVehicleEngine(class) => class.name(),
            Classes::hknpVehicleInstance(class) => class.name(),
            Classes::hknpVehicleInstanceWheelInfo(class) => class.name(),
            Classes::hknpVehicleLinearCastWheelCollide(class) => class.name(),
            Classes::hknpVehicleLinearCastWheelCollideWheelState(class) => class.name(),
            Classes::hknpVehicleRayCastWheelCollide(class) => class.name(),
            Classes::hknpVehicleSteering(class) => class.name(),
            Classes::hknpVehicleSuspension(class) => class.name(),
            Classes::hknpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.name()
            }
            Classes::hknpVehicleTransmission(class) => class.name(),
            Classes::hknpVehicleVelocityDamper(class) => class.name(),
            Classes::hknpVehicleWheelCollide(class) => class.name(),
            Classes::hknpWorldCinfo(class) => class.name(),
            Classes::hknpWorldSnapshot(class) => class.name(),
            Classes::hkOffsetBitField(class) => class.name(),
            Classes::hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.name(),
            Classes::hkp2dAngConstraintAtom(class) => class.name(),
            Classes::hkp3dAngConstraintAtom(class) => class.name(),
            Classes::hkPackedVector3(class) => class.name(),
            Classes::hkPackedVector8_3(class) => class.name(),
            Classes::hkPackfileHeader(class) => class.name(),
            Classes::hkPackfileSectionHeader(class) => class.name(),
            Classes::hkpAngConstraintAtom(class) => class.name(),
            Classes::hkpAngFrictionConstraintAtom(class) => class.name(),
            Classes::hkpAngLimitConstraintAtom(class) => class.name(),
            Classes::hkpAngMotorConstraintAtom(class) => class.name(),
            Classes::hkpBallAndSocketConstraintData(class) => class.name(),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => class.name(),
            Classes::hkpBallSocketConstraintAtom(class) => class.name(),
            Classes::hkpBridgeAtoms(class) => class.name(),
            Classes::hkpBridgeConstraintAtom(class) => class.name(),
            Classes::hkpCallbackConstraintMotor(class) => class.name(),
            Classes::hkpCogWheelConstraintAtom(class) => class.name(),
            Classes::hkpCogWheelConstraintData(class) => class.name(),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.name(),
            Classes::hkpConeLimitConstraintAtom(class) => class.name(),
            Classes::hkpConstraintAtom(class) => class.name(),
            Classes::hkpConstraintData(class) => class.name(),
            Classes::hkpConstraintMotor(class) => class.name(),
            Classes::hkpDeformableAngConstraintAtom(class) => class.name(),
            Classes::hkpDeformableFixedConstraintData(class) => class.name(),
            Classes::hkpDeformableFixedConstraintDataAtoms(class) => class.name(),
            Classes::hkpDeformableLinConstraintAtom(class) => class.name(),
            Classes::hkpFixedConstraintData(class) => class.name(),
            Classes::hkpFixedConstraintDataAtoms(class) => class.name(),
            Classes::hkpHingeConstraintData(class) => class.name(),
            Classes::hkpHingeConstraintDataAtoms(class) => class.name(),
            Classes::hkpHingeLimitsData(class) => class.name(),
            Classes::hkpHingeLimitsDataAtoms(class) => class.name(),
            Classes::hkpLimitedForceConstraintMotor(class) => class.name(),
            Classes::hkpLimitedHingeConstraintData(class) => class.name(),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => class.name(),
            Classes::hkpLinConstraintAtom(class) => class.name(),
            Classes::hkpLinearClearanceConstraintData(class) => class.name(),
            Classes::hkpLinearClearanceConstraintDataAtoms(class) => class.name(),
            Classes::hkpLinearParametricCurve(class) => class.name(),
            Classes::hkpLinFrictionConstraintAtom(class) => class.name(),
            Classes::hkpLinLimitConstraintAtom(class) => class.name(),
            Classes::hkpLinMotorConstraintAtom(class) => class.name(),
            Classes::hkpLinSoftConstraintAtom(class) => class.name(),
            Classes::hkPostFinishAttribute(class) => class.name(),
            Classes::hkpOverwritePivotConstraintAtom(class) => class.name(),
            Classes::hkpParametricCurve(class) => class.name(),
            Classes::hkpPointToPathConstraintData(class) => class.name(),
            Classes::hkpPointToPlaneConstraintData(class) => class.name(),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => class.name(),
            Classes::hkpPositionConstraintMotor(class) => class.name(),
            Classes::hkpPrismaticConstraintData(class) => class.name(),
            Classes::hkpPrismaticConstraintDataAtoms(class) => class.name(),
            Classes::hkpPulleyConstraintAtom(class) => class.name(),
            Classes::hkpPulleyConstraintData(class) => class.name(),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.name(),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.name(),
            Classes::hkpRackAndPinionConstraintData(class) => class.name(),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => class.name(),
            Classes::hkpRagdollConstraintData(class) => class.name(),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.name(),
            Classes::hkpRagdollLimitsData(class) => class.name(),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.name(),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.name(),
            Classes::hkpRotationalConstraintData(class) => class.name(),
            Classes::hkpRotationalConstraintDataAtoms(class) => class.name(),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => class.name(),
            Classes::hkpSetLocalTransformsConstraintAtom(class) => class.name(),
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => class.name(),
            Classes::hkpSetupStabilizationAtom(class) => class.name(),
            Classes::hkpSpringDamperConstraintMotor(class) => class.name(),
            Classes::hkpStiffSpringConstraintAtom(class) => class.name(),
            Classes::hkpStiffSpringConstraintData(class) => class.name(),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => class.name(),
            Classes::hkpTwistLimitConstraintAtom(class) => class.name(),
            Classes::hkpVehicleFrictionDescription(class) => class.name(),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => class.name(),
            Classes::hkpVehicleFrictionStatus(class) => class.name(),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => class.name(),
            Classes::hkpVelocityConstraintMotor(class) => class.name(),
            Classes::hkpWheelConstraintData(class) => class.name(),
            Classes::hkpWheelConstraintDataAtoms(class) => class.name(),
            Classes::hkpWheelFrictionConstraintAtom(class) => class.name(),
            Classes::hkpWheelFrictionConstraintAtomAxle(class) => class.name(),
            Classes::hkpWheelFrictionConstraintData(class) => class.name(),
            Classes::hkpWheelFrictionConstraintDataAtoms(class) => class.name(),
            Classes::hkpWrappedConstraintData(class) => class.name(),
            Classes::hkQTransform(class) => class.name(),
            Classes::hkQTransformf(class) => class.name(),
            Classes::hkRangeInt32Attribute(class) => class.name(),
            Classes::hkRangeRealAttribute(class) => class.name(),
            Classes::hkRefCountedProperties(class) => class.name(),
            Classes::hkRefCountedPropertiesEntry(class) => class.name(),
            Classes::hkReferencedObject(class) => class.name(),
            Classes::hkResourceBase(class) => class.name(),
            Classes::hkResourceContainer(class) => class.name(),
            Classes::hkResourceHandle(class) => class.name(),
            Classes::hkRootLevelContainer(class) => class.name(),
            Classes::hkRootLevelContainerNamedVariant(class) => class.name(),
            Classes::hkSemanticsAttribute(class) => class.name(),
            Classes::hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair(
                class,
            ) => class.name(),
            Classes::hkSetIntFloatPair(class) => class.name(),
            Classes::hkSetUint32(class) => class.name(),
            Classes::hkSetUint64(class) => class.name(),
            Classes::hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint(
                class,
            ) => class.name(),
            Classes::hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong(
                class,
            ) => class.name(),
            Classes::hkSimpleLocalFrame(class) => class.name(),
            Classes::hkSimpleProperty(class) => class.name(),
            Classes::hkSimplePropertyValue(class) => class.name(),
            Classes::hkSkinBinding(class) => class.name(),
            Classes::hkSkinnedMeshShape(class) => class.name(),
            Classes::hkSkinnedMeshShapeBoneSection(class) => class.name(),
            Classes::hkSkinnedMeshShapeBoneSet(class) => class.name(),
            Classes::hkSkinnedMeshShapePart(class) => class.name(),
            Classes::hkSkinnedRefMeshShape(class) => class.name(),
            Classes::hkSphere(class) => class.name(),
            Classes::hkStorageSkinnedMeshShape(class) => class.name(),
            Classes::hkStringObject(class) => class.name(),
            Classes::hkSweptTransformf(class) => class.name(),
            Classes::hkTraceStreamTitle(class) => class.name(),
            Classes::hkUFloat8(class) => class.name(),
            Classes::hkUiAttribute(class) => class.name(),
            Classes::hkUuid(class) => class.name(),
            Classes::hkVariableTweakingHelper(class) => class.name(),
            Classes::hkVariableTweakingHelperBoolVariableInfo(class) => class.name(),
            Classes::hkVariableTweakingHelperIntVariableInfo(class) => class.name(),
            Classes::hkVariableTweakingHelperRealVariableInfo(class) => class.name(),
            Classes::hkVariableTweakingHelperVector4VariableInfo(class) => class.name(),
            Classes::hkVertexFormat(class) => class.name(),
            Classes::hkVertexFormatElement(class) => class.name(),
            Classes::hkxAnimatedFloat(class) => class.name(),
            Classes::hkxAnimatedMatrix(class) => class.name(),
            Classes::hkxAnimatedQuaternion(class) => class.name(),
            Classes::hkxAnimatedVector(class) => class.name(),
            Classes::hkxAttribute(class) => class.name(),
            Classes::hkxAttributeGroup(class) => class.name(),
            Classes::hkxAttributeHolder(class) => class.name(),
            Classes::hkxEnum(class) => class.name(),
            Classes::hkxEnumItem(class) => class.name(),
            Classes::hkxIndexBuffer(class) => class.name(),
            Classes::hkxMaterial(class) => class.name(),
            Classes::hkxMaterialProperty(class) => class.name(),
            Classes::hkxMaterialTextureStage(class) => class.name(),
            Classes::hkxMesh(class) => class.name(),
            Classes::hkxMeshSection(class) => class.name(),
            Classes::hkxMeshUserChannelInfo(class) => class.name(),
            Classes::hkxSparselyAnimatedBool(class) => class.name(),
            Classes::hkxSparselyAnimatedEnum(class) => class.name(),
            Classes::hkxSparselyAnimatedInt(class) => class.name(),
            Classes::hkxSparselyAnimatedString(class) => class.name(),
            Classes::hkxVertexAnimation(class) => class.name(),
            Classes::hkxVertexAnimationUsageMap(class) => class.name(),
            Classes::hkxVertexBuffer(class) => class.name(),
            Classes::hkxVertexBufferVertexData(class) => class.name(),
            Classes::hkxVertexDescription(class) => class.name(),
            Classes::hkxVertexDescriptionElementDecl(class) => class.name(),
        }
    }
    fn signature(&self) -> _serde::__private::Signature {
        match &self {
            Classes::BGSGamebryoSequenceGenerator(class) => class.signature(),
            Classes::BSAlignBoneModifier(class) => class.signature(),
            Classes::BSAssignVariablesModifier(class) => class.signature(),
            Classes::BSBehaviorGraphSwapGenerator(class) => class.signature(),
            Classes::BSBoneSwitchGenerator(class) => class.signature(),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.signature(),
            Classes::BSComputeAddBoneAnimModifier(class) => class.signature(),
            Classes::BSCyclicBlendTransitionGenerator(class) => class.signature(),
            Classes::BSDecomposeVectorModifier(class) => class.signature(),
            Classes::BSDirectAtCapturePoseModifier(class) => class.signature(),
            Classes::BSDirectAtModifier(class) => class.signature(),
            Classes::BSDistTriggerModifier(class) => class.signature(),
            Classes::BSEventEveryNEventsModifier(class) => class.signature(),
            Classes::BSEventOnDeactivateModifier(class) => class.signature(),
            Classes::BSEventOnFalseToTrueModifier(class) => class.signature(),
            Classes::BSGetTimeStepModifier(class) => class.signature(),
            Classes::BSInterpValueModifier(class) => class.signature(),
            Classes::BSIsActiveModifier(class) => class.signature(),
            Classes::BSIStateManagerModifier(class) => class.signature(),
            Classes::BSIStateManagerModifierBSiStateData(class) => class.signature(),
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.signature()
            }
            Classes::BSiStateTaggingGenerator(class) => class.signature(),
            Classes::BSiStateTaggingStateListener(class) => class.signature(),
            Classes::BSLimbCycleModifier(class) => class.signature(),
            Classes::BSLimbIKModifier(class) => class.signature(),
            Classes::BSLookAtCapturePoseModifier(class) => class.signature(),
            Classes::BSLookAtModifier(class) => class.signature(),
            Classes::BSLookAtModifierBoneData(class) => class.signature(),
            Classes::BSModifyOnceModifier(class) => class.signature(),
            Classes::BSOffsetAnimationGenerator(class) => class.signature(),
            Classes::BSPassByTargetTriggerModifier(class) => class.signature(),
            Classes::BSRagdollContactListenerModifier(class) => class.signature(),
            Classes::BSRandomAlarmModifier(class) => class.signature(),
            Classes::BSRetargetBodyBlend(class) => class.signature(),
            Classes::BSReverseSpineTwistModifier(class) => class.signature(),
            Classes::BSRootTwistModifier(class) => class.signature(),
            Classes::BSSpeedSamplerModifier(class) => class.signature(),
            Classes::BSSpringState(class) => class.signature(),
            Classes::BSTimerModifier(class) => class.signature(),
            Classes::BSTweenerModifier(class) => class.signature(),
            Classes::DynamicAnimationTaggingGenerator(class) => class.signature(),
            Classes::hclAction(class) => class.signature(),
            Classes::hclAntiPinchConstraintSet(class) => class.signature(),
            Classes::hclAntiPinchConstraintSetPerParticle(class) => class.signature(),
            Classes::hclBendLinkConstraintSet(class) => class.signature(),
            Classes::hclBendLinkConstraintSetLink(class) => class.signature(),
            Classes::hclBendLinkConstraintSetMx(class) => class.signature(),
            Classes::hclBendLinkConstraintSetMxBatch(class) => class.signature(),
            Classes::hclBendLinkConstraintSetMxSingle(class) => class.signature(),
            Classes::hclBendStiffnessConstraintSet(class) => class.signature(),
            Classes::hclBendStiffnessConstraintSetLink(class) => class.signature(),
            Classes::hclBendStiffnessConstraintSetMx(class) => class.signature(),
            Classes::hclBendStiffnessConstraintSetMxBatch(class) => class.signature(),
            Classes::hclBendStiffnessConstraintSetMxSingle(class) => class.signature(),
            Classes::hclBendStiffnessSetupObject(class) => class.signature(),
            Classes::hclBlendSomeVerticesOperator(class) => class.signature(),
            Classes::hclBlendSomeVerticesOperatorBlendEntry(class) => class.signature(),
            Classes::hclBonePlanesConstraintSet(class) => class.signature(),
            Classes::hclBonePlanesConstraintSetBonePlane(class) => class.signature(),
            Classes::hclBoneSpaceDeformer(class) => class.signature(),
            Classes::hclBoneSpaceDeformerFourBlendEntryBlock(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockP(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockPN(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockPNT(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockPNTB(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedP(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPN(class) => class.signature(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNT(class) => {
                class.signature()
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNTB(class) => {
                class.signature()
            }
            Classes::hclBoneSpaceDeformerOneBlendEntryBlock(class) => class.signature(),
            Classes::hclBoneSpaceDeformerThreeBlendEntryBlock(class) => class.signature(),
            Classes::hclBoneSpaceDeformerTwoBlendEntryBlock(class) => class.signature(),
            Classes::hclBoneSpaceMeshMeshDeformOperator(class) => class.signature(),
            Classes::hclBoneSpaceMeshMeshDeformPNOperator(class) => class.signature(),
            Classes::hclBoneSpaceMeshMeshDeformPNTBOperator(class) => class.signature(),
            Classes::hclBoneSpaceMeshMeshDeformPNTOperator(class) => class.signature(),
            Classes::hclBoneSpaceMeshMeshDeformPOperator(class) => class.signature(),
            Classes::hclBoneSpaceSkinOperator(class) => class.signature(),
            Classes::hclBoneSpaceSkinPNOperator(class) => class.signature(),
            Classes::hclBoneSpaceSkinPNTBOperator(class) => class.signature(),
            Classes::hclBoneSpaceSkinPNTOperator(class) => class.signature(),
            Classes::hclBoneSpaceSkinPOperator(class) => class.signature(),
            Classes::hclBufferDefinition(class) => class.signature(),
            Classes::hclBufferLayout(class) => class.signature(),
            Classes::hclBufferLayoutBufferElement(class) => class.signature(),
            Classes::hclBufferLayoutSlot(class) => class.signature(),
            Classes::hclBufferSetupObject(class) => class.signature(),
            Classes::hclBufferUsage(class) => class.signature(),
            Classes::hclCapsuleShape(class) => class.signature(),
            Classes::hclClothContainer(class) => class.signature(),
            Classes::hclClothData(class) => class.signature(),
            Classes::hclClothSetupContainer(class) => class.signature(),
            Classes::hclClothSetupObject(class) => class.signature(),
            Classes::hclClothState(class) => class.signature(),
            Classes::hclClothStateBufferAccess(class) => class.signature(),
            Classes::hclClothStateSetupObject(class) => class.signature(),
            Classes::hclClothStateTransformSetAccess(class) => class.signature(),
            Classes::hclCollidable(class) => class.signature(),
            Classes::hclCompressibleLinkConstraintSet(class) => class.signature(),
            Classes::hclCompressibleLinkConstraintSetLink(class) => class.signature(),
            Classes::hclCompressibleLinkConstraintSetMx(class) => class.signature(),
            Classes::hclCompressibleLinkConstraintSetMxBatch(class) => class.signature(),
            Classes::hclCompressibleLinkConstraintSetMxSingle(class) => class.signature(),
            Classes::hclConstraintSet(class) => class.signature(),
            Classes::hclConstraintSetSetupObject(class) => class.signature(),
            Classes::hclConvexGeometryShape(class) => class.signature(),
            Classes::hclConvexHeightFieldShape(class) => class.signature(),
            Classes::hclConvexPlanesShape(class) => class.signature(),
            Classes::hclCopyVerticesOperator(class) => class.signature(),
            Classes::hclEdgeSelectionInput(class) => class.signature(),
            Classes::hclGatherAllVerticesOperator(class) => class.signature(),
            Classes::hclGatherSomeVerticesOperator(class) => class.signature(),
            Classes::hclGatherSomeVerticesOperatorVertexPair(class) => class.signature(),
            Classes::hclInputConvertOperator(class) => class.signature(),
            Classes::hclLocalRangeConstraintSet(class) => class.signature(),
            Classes::hclLocalRangeConstraintSetLocalConstraint(class) => {
                class.signature()
            }
            Classes::hclLocalRangeSetupObject(class) => class.signature(),
            Classes::hclMeshBoneDeformOperator(class) => class.signature(),
            Classes::hclMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.signature()
            }
            Classes::hclMeshBoneDeformSetupObject(class) => class.signature(),
            Classes::hclMeshMeshDeformOperator(class) => class.signature(),
            Classes::hclMeshMeshDeformOperatorTriangleVertexPair(class) => {
                class.signature()
            }
            Classes::hclMoveFixedParticlesSetupObject(class) => class.signature(),
            Classes::hclMoveParticlesOperator(class) => class.signature(),
            Classes::hclMoveParticlesOperatorVertexParticlePair(class) => {
                class.signature()
            }
            Classes::hclNamedSetupMesh(class) => class.signature(),
            Classes::hclNamedTransformSetSetupObject(class) => class.signature(),
            Classes::hclObjectSpaceDeformer(class) => class.signature(),
            Classes::hclObjectSpaceDeformerFourBlendEntryBlock(class) => {
                class.signature()
            }
            Classes::hclObjectSpaceDeformerLocalBlockP(class) => class.signature(),
            Classes::hclObjectSpaceDeformerLocalBlockPN(class) => class.signature(),
            Classes::hclObjectSpaceDeformerLocalBlockPNT(class) => class.signature(),
            Classes::hclObjectSpaceDeformerLocalBlockPNTB(class) => class.signature(),
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedP(class) => {
                class.signature()
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPN(class) => {
                class.signature()
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNT(class) => {
                class.signature()
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNTB(class) => {
                class.signature()
            }
            Classes::hclObjectSpaceDeformerOneBlendEntryBlock(class) => class.signature(),
            Classes::hclObjectSpaceDeformerThreeBlendEntryBlock(class) => {
                class.signature()
            }
            Classes::hclObjectSpaceDeformerTwoBlendEntryBlock(class) => class.signature(),
            Classes::hclObjectSpaceMeshMeshDeformOperator(class) => class.signature(),
            Classes::hclObjectSpaceMeshMeshDeformPNOperator(class) => class.signature(),
            Classes::hclObjectSpaceMeshMeshDeformPNTBOperator(class) => class.signature(),
            Classes::hclObjectSpaceMeshMeshDeformPNTOperator(class) => class.signature(),
            Classes::hclObjectSpaceMeshMeshDeformPOperator(class) => class.signature(),
            Classes::hclObjectSpaceSkinOperator(class) => class.signature(),
            Classes::hclObjectSpaceSkinPNOperator(class) => class.signature(),
            Classes::hclObjectSpaceSkinPNTBOperator(class) => class.signature(),
            Classes::hclObjectSpaceSkinPNTOperator(class) => class.signature(),
            Classes::hclObjectSpaceSkinPOperator(class) => class.signature(),
            Classes::hclOperator(class) => class.signature(),
            Classes::hclOperatorSetupObject(class) => class.signature(),
            Classes::hclOutputConvertOperator(class) => class.signature(),
            Classes::hclPlaneShape(class) => class.signature(),
            Classes::hclRuntimeConversionInfo(class) => class.signature(),
            Classes::hclRuntimeConversionInfoElementConversion(class) => {
                class.signature()
            }
            Classes::hclRuntimeConversionInfoSlotConversion(class) => class.signature(),
            Classes::hclScratchBufferDefinition(class) => class.signature(),
            Classes::hclScratchBufferSetupObject(class) => class.signature(),
            Classes::hclSetupMesh(class) => class.signature(),
            Classes::hclShadowBufferDefinition(class) => class.signature(),
            Classes::hclShape(class) => class.signature(),
            Classes::hclSimClothBufferSetupObject(class) => class.signature(),
            Classes::hclSimClothData(class) => class.signature(),
            Classes::hclSimClothDataCollidablePinchingData(class) => class.signature(),
            Classes::hclSimClothDataCollidableTransformMap(class) => class.signature(),
            Classes::hclSimClothDataLandscapeCollisionData(class) => class.signature(),
            Classes::hclSimClothDataOverridableSimulationInfo(class) => class.signature(),
            Classes::hclSimClothDataParticleData(class) => class.signature(),
            Classes::hclSimClothDataTransferMotionData(class) => class.signature(),
            Classes::hclSimClothPose(class) => class.signature(),
            Classes::hclSimClothSetupObject(class) => class.signature(),
            Classes::hclSimClothSetupObjectPerInstanceCollidable(class) => {
                class.signature()
            }
            Classes::hclSimClothSetupObjectTransferMotionSetupData(class) => {
                class.signature()
            }
            Classes::hclSimpleMeshBoneDeformOperator(class) => class.signature(),
            Classes::hclSimpleMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.signature()
            }
            Classes::hclSimpleWindAction(class) => class.signature(),
            Classes::hclSimulateOperator(class) => class.signature(),
            Classes::hclSimulateSetupObject(class) => class.signature(),
            Classes::hclSimulationSetupMesh(class) => class.signature(),
            Classes::hclSimulationSetupMeshMapOptions(class) => class.signature(),
            Classes::hclSkinOperator(class) => class.signature(),
            Classes::hclSkinOperatorBoneInfluence(class) => class.signature(),
            Classes::hclSkinSetupObject(class) => class.signature(),
            Classes::hclSphereShape(class) => class.signature(),
            Classes::hclStandardLinkConstraintSet(class) => class.signature(),
            Classes::hclStandardLinkConstraintSetLink(class) => class.signature(),
            Classes::hclStandardLinkConstraintSetMx(class) => class.signature(),
            Classes::hclStandardLinkConstraintSetMxBatch(class) => class.signature(),
            Classes::hclStandardLinkConstraintSetMxSingle(class) => class.signature(),
            Classes::hclStandardLinkSetupObject(class) => class.signature(),
            Classes::hclStaticDisplayBufferSetupObject(class) => class.signature(),
            Classes::hclStaticShadowBufferDefinition(class) => class.signature(),
            Classes::hclStretchLinkConstraintSet(class) => class.signature(),
            Classes::hclStretchLinkConstraintSetLink(class) => class.signature(),
            Classes::hclStretchLinkConstraintSetMx(class) => class.signature(),
            Classes::hclStretchLinkConstraintSetMxBatch(class) => class.signature(),
            Classes::hclStretchLinkConstraintSetMxSingle(class) => class.signature(),
            Classes::hclStretchLinkSetupObject(class) => class.signature(),
            Classes::hclTaperedCapsuleShape(class) => class.signature(),
            Classes::hclTransformSetDefinition(class) => class.signature(),
            Classes::hclTransformSetSetupObject(class) => class.signature(),
            Classes::hclTransformSetUsage(class) => class.signature(),
            Classes::hclTransformSetUsageTransformTracker(class) => class.signature(),
            Classes::hclTransitionConstraintSet(class) => class.signature(),
            Classes::hclTransitionConstraintSetPerParticle(class) => class.signature(),
            Classes::hclTriangleSelectionInput(class) => class.signature(),
            Classes::hclUpdateAllVertexFramesOperator(class) => class.signature(),
            Classes::hclUpdateSomeVertexFramesOperator(class) => class.signature(),
            Classes::hclUpdateSomeVertexFramesOperatorTriangle(class) => {
                class.signature()
            }
            Classes::hclVertexFloatInput(class) => class.signature(),
            Classes::hclVertexGatherSetupObject(class) => class.signature(),
            Classes::hclVertexSelectionInput(class) => class.signature(),
            Classes::hclVolumeConstraint(class) => class.signature(),
            Classes::hclVolumeConstraintApplyData(class) => class.signature(),
            Classes::hclVolumeConstraintFrameData(class) => class.signature(),
            Classes::hclVolumeConstraintMx(class) => class.signature(),
            Classes::hclVolumeConstraintMxApplyBatchData(class) => class.signature(),
            Classes::hclVolumeConstraintMxApplySingleData(class) => class.signature(),
            Classes::hclVolumeConstraintMxFrameBatchData(class) => class.signature(),
            Classes::hclVolumeConstraintMxFrameSingleData(class) => class.signature(),
            Classes::hclVolumeConstraintSetupObject(class) => class.signature(),
            Classes::hkAabb(class) => class.signature(),
            Classes::hkAabb16(class) => class.signature(),
            Classes::hkAabbHalf(class) => class.signature(),
            Classes::hkAabbUint32(class) => class.signature(),
            Classes::hkaAngularReferenceFrame(class) => class.signature(),
            Classes::hkaAnimatedReferenceFrame(class) => class.signature(),
            Classes::hkaAnimation(class) => class.signature(),
            Classes::hkaAnimationBinding(class) => class.signature(),
            Classes::hkaAnimationContainer(class) => class.signature(),
            Classes::hkaAnimationPreviewColorContainer(class) => class.signature(),
            Classes::hkaAnnotationTrack(class) => class.signature(),
            Classes::hkaAnnotationTrackAnnotation(class) => class.signature(),
            Classes::hkaBone(class) => class.signature(),
            Classes::hkaBoneAttachment(class) => class.signature(),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => class.signature(),
            Classes::hkaDirectionalReferenceFrame(class) => class.signature(),
            Classes::hkaFootstepAnalysisInfo(class) => class.signature(),
            Classes::hkaFootstepAnalysisInfoContainer(class) => class.signature(),
            Classes::hkaInterleavedUncompressedAnimation(class) => class.signature(),
            Classes::hkaLosslessCompressedAnimation(class) => class.signature(),
            Classes::hkaMeshBinding(class) => class.signature(),
            Classes::hkaMeshBindingMapping(class) => class.signature(),
            Classes::hkaParameterizedAnimationReferenceFrame(class) => class.signature(),
            Classes::hkaParameterizedReferenceFrame(class) => class.signature(),
            Classes::hkaPredictiveCompressedAnimation(class) => class.signature(),
            Classes::hkaPredictiveCompressedAnimationTrackCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaQuantizedAnimation(class) => class.signature(),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaReferencePoseAnimation(class) => class.signature(),
            Classes::hkArrayTypeAttribute(class) => class.signature(),
            Classes::hkaSkeleton(class) => class.signature(),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.signature(),
            Classes::hkaSkeletonMapper(class) => class.signature(),
            Classes::hkaSkeletonMapperData(class) => class.signature(),
            Classes::hkaSkeletonMapperDataChainMapping(class) => class.signature(),
            Classes::hkaSkeletonMapperDataPartitionMappingRange(class) => {
                class.signature()
            }
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => class.signature(),
            Classes::hkaSkeletonPartition(class) => class.signature(),
            Classes::hkaSplineCompressedAnimation(class) => class.signature(),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.signature()
            }
            Classes::hkbAiControlCancelPathCommand(class) => class.signature(),
            Classes::hkbAiControlControlData(class) => class.signature(),
            Classes::hkbAiControlControlDataBlendable(class) => class.signature(),
            Classes::hkbAiControlControlDataNonBlendable(class) => class.signature(),
            Classes::hkbAiControlPathToCommand(class) => class.signature(),
            Classes::hkbAlignBoneModifier(class) => class.signature(),
            Classes::hkbAnimatedSkeletonGenerator(class) => class.signature(),
            Classes::hkBaseObject(class) => class.signature(),
            Classes::hkbAssetBundle(class) => class.signature(),
            Classes::hkbAssetBundleStringData(class) => class.signature(),
            Classes::hkbAttachmentModifier(class) => class.signature(),
            Classes::hkbAttachmentSetup(class) => class.signature(),
            Classes::hkbAttributeModifier(class) => class.signature(),
            Classes::hkbAttributeModifierAssignment(class) => class.signature(),
            Classes::hkbAuxiliaryNodeInfo(class) => class.signature(),
            Classes::hkbBehaviorEventsInfo(class) => class.signature(),
            Classes::hkbBehaviorGraph(class) => class.signature(),
            Classes::hkbBehaviorGraphData(class) => class.signature(),
            Classes::hkbBehaviorGraphInternalState(class) => class.signature(),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => class.signature(),
            Classes::hkbBehaviorGraphStringData(class) => class.signature(),
            Classes::hkbBehaviorInfo(class) => class.signature(),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.signature(),
            Classes::hkbBehaviorReferenceGenerator(class) => class.signature(),
            Classes::hkbBindable(class) => class.signature(),
            Classes::hkbBlendCurveUtils(class) => class.signature(),
            Classes::hkbBlenderGenerator(class) => class.signature(),
            Classes::hkbBlenderGeneratorChild(class) => class.signature(),
            Classes::hkbBlenderGeneratorChildInternalState(class) => class.signature(),
            Classes::hkbBlenderGeneratorInternalState(class) => class.signature(),
            Classes::hkbBlendingTransitionEffect(class) => class.signature(),
            Classes::hkbBlendingTransitionEffectInternalState(class) => class.signature(),
            Classes::hkbBoneIndexArray(class) => class.signature(),
            Classes::hkbBoneWeightArray(class) => class.signature(),
            Classes::hkbBoolVariableSequencedData(class) => class.signature(),
            Classes::hkbBoolVariableSequencedDataSample(class) => class.signature(),
            Classes::hkbCameraShakeEventPayload(class) => class.signature(),
            Classes::hkbCameraVariablesChangedCommand(class) => class.signature(),
            Classes::hkbCharacter(class) => class.signature(),
            Classes::hkbCharacterAddedInfo(class) => class.signature(),
            Classes::hkbCharacterControlCommand(class) => class.signature(),
            Classes::hkbCharacterControllerModifier(class) => class.signature(),
            Classes::hkbCharacterControllerModifierControlData(class) => {
                class.signature()
            }
            Classes::hkbCharacterControllerModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbCharacterControllerSetup(class) => class.signature(),
            Classes::hkbCharacterData(class) => class.signature(),
            Classes::hkbCharacterInfo(class) => class.signature(),
            Classes::hkbCharacterSelectedInfo(class) => class.signature(),
            Classes::hkbCharacterSetup(class) => class.signature(),
            Classes::hkbCharacterSkinInfo(class) => class.signature(),
            Classes::hkbCharacterSteppedInfo(class) => class.signature(),
            Classes::hkbCharacterStringData(class) => class.signature(),
            Classes::hkbCharacterStringDataFileNameMeshNamePair(class) => {
                class.signature()
            }
            Classes::hkbClientCharacterState(class) => class.signature(),
            Classes::hkbClipGenerator(class) => class.signature(),
            Classes::hkbClipGeneratorEcho(class) => class.signature(),
            Classes::hkbClipGeneratorInternalState(class) => class.signature(),
            Classes::hkbClipTrigger(class) => class.signature(),
            Classes::hkbClipTriggerArray(class) => class.signature(),
            Classes::hkbCombineTransformsModifier(class) => class.signature(),
            Classes::hkbCombineTransformsModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbCompiledExpressionSet(class) => class.signature(),
            Classes::hkbCompiledExpressionSetToken(class) => class.signature(),
            Classes::hkbComputeDirectionModifier(class) => class.signature(),
            Classes::hkbComputeDirectionModifierInternalState(class) => class.signature(),
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => class.signature(),
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbComputeRotationToTargetModifier(class) => class.signature(),
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbComputeWorldFromModelModifier(class) => class.signature(),
            Classes::hkbCondition(class) => class.signature(),
            Classes::hkbConstraintSetup(class) => class.signature(),
            Classes::hkbContext(class) => class.signature(),
            Classes::hkbCustomIdSelector(class) => class.signature(),
            Classes::hkbCustomTestGenerator(class) => class.signature(),
            Classes::hkbCustomTestGeneratorAnnotatedTypes(class) => class.signature(),
            Classes::hkbCustomTestGeneratorBoneTypes(class) => class.signature(),
            Classes::hkbCustomTestGeneratorComplexTypes(class) => class.signature(),
            Classes::hkbCustomTestGeneratorHiddenTypes(class) => class.signature(),
            Classes::hkbCustomTestGeneratorNestedTypes(class) => class.signature(),
            Classes::hkbCustomTestGeneratorNestedTypesBase(class) => class.signature(),
            Classes::hkbCustomTestGeneratorSimpleTypes(class) => class.signature(),
            Classes::hkbDampingModifier(class) => class.signature(),
            Classes::hkbDampingModifierInternalState(class) => class.signature(),
            Classes::hkbDelayedModifier(class) => class.signature(),
            Classes::hkbDelayedModifierInternalState(class) => class.signature(),
            Classes::hkbDetectCloseToGroundModifier(class) => class.signature(),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbDockingGenerator(class) => class.signature(),
            Classes::hkbDockingGeneratorInternalState(class) => class.signature(),
            Classes::hkbEvaluateExpressionModifier(class) => class.signature(),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.signature()
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbEvaluateHandleModifier(class) => class.signature(),
            Classes::hkbEvent(class) => class.signature(),
            Classes::hkbEventBase(class) => class.signature(),
            Classes::hkbEventDrivenModifier(class) => class.signature(),
            Classes::hkbEventDrivenModifierInternalState(class) => class.signature(),
            Classes::hkbEventInfo(class) => class.signature(),
            Classes::hkbEventPayload(class) => class.signature(),
            Classes::hkbEventPayloadList(class) => class.signature(),
            Classes::hkbEventProperty(class) => class.signature(),
            Classes::hkbEventRaisedInfo(class) => class.signature(),
            Classes::hkbEventRangeData(class) => class.signature(),
            Classes::hkbEventRangeDataArray(class) => class.signature(),
            Classes::hkbEventSequencedData(class) => class.signature(),
            Classes::hkbEventSequencedDataSequencedEvent(class) => class.signature(),
            Classes::hkbEventsFromRangeModifier(class) => class.signature(),
            Classes::hkbEventsFromRangeModifierInternalState(class) => class.signature(),
            Classes::hkbExpressionCondition(class) => class.signature(),
            Classes::hkbExpressionData(class) => class.signature(),
            Classes::hkbExpressionDataArray(class) => class.signature(),
            Classes::hkbExtractRagdollPoseModifier(class) => class.signature(),
            Classes::hkbExtrapolatingTransitionEffect(class) => class.signature(),
            Classes::hkbExtrapolatingTransitionEffectInternalState(class) => {
                class.signature()
            }
            Classes::hkbFootIkControlData(class) => class.signature(),
            Classes::hkbFootIkControlsModifier(class) => class.signature(),
            Classes::hkbFootIkControlsModifierLeg(class) => class.signature(),
            Classes::hkbFootIkDriverInfo(class) => class.signature(),
            Classes::hkbFootIkDriverInfoLeg(class) => class.signature(),
            Classes::hkbFootIkGains(class) => class.signature(),
            Classes::hkbFootIkModifier(class) => class.signature(),
            Classes::hkbFootIkModifierInternalLegData(class) => class.signature(),
            Classes::hkbFootIkModifierLeg(class) => class.signature(),
            Classes::hkbGenerator(class) => class.signature(),
            Classes::hkbGeneratorOutputListener(class) => class.signature(),
            Classes::hkbGeneratorPartitionInfo(class) => class.signature(),
            Classes::hkbGeneratorSyncInfo(class) => class.signature(),
            Classes::hkbGeneratorSyncInfoActiveInterval(class) => class.signature(),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.signature(),
            Classes::hkbGeneratorTransitionEffect(class) => class.signature(),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => {
                class.signature()
            }
            Classes::hkbGetHandleOnBoneModifier(class) => class.signature(),
            Classes::hkbGetUpModifier(class) => class.signature(),
            Classes::hkbGetUpModifierInternalState(class) => class.signature(),
            Classes::hkbGetWorldFromModelModifier(class) => class.signature(),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbGravityModifier(class) => class.signature(),
            Classes::hkbHandIkControlData(class) => class.signature(),
            Classes::hkbHandIkControlsModifier(class) => class.signature(),
            Classes::hkbHandIkControlsModifierHand(class) => class.signature(),
            Classes::hkbHandIkDriverInfo(class) => class.signature(),
            Classes::hkbHandIkDriverInfoHand(class) => class.signature(),
            Classes::hkbHandIkModifier(class) => class.signature(),
            Classes::hkbHandIkModifierHand(class) => class.signature(),
            Classes::hkbHandle(class) => class.signature(),
            Classes::hkbHoldFromBlendingTransitionEffect(class) => class.signature(),
            Classes::hkbIntEventPayload(class) => class.signature(),
            Classes::hkbIntVariableSequencedData(class) => class.signature(),
            Classes::hkbIntVariableSequencedDataSample(class) => class.signature(),
            Classes::hkBitField(class) => class.signature(),
            Classes::hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.signature(),
            Classes::hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.signature(),
            Classes::hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.signature(),
            Classes::hkbJigglerGroup(class) => class.signature(),
            Classes::hkbJigglerModifier(class) => class.signature(),
            Classes::hkbJigglerModifierInternalState(class) => class.signature(),
            Classes::hkbKeyframeBonesModifier(class) => class.signature(),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => class.signature(),
            Classes::hkbKeyFrameControlData(class) => class.signature(),
            Classes::hkbLayer(class) => class.signature(),
            Classes::hkbLayerGenerator(class) => class.signature(),
            Classes::hkbLayerGeneratorInternalState(class) => class.signature(),
            Classes::hkbLayerGeneratorLayerInternalState(class) => class.signature(),
            Classes::hkbLinkedSymbolInfo(class) => class.signature(),
            Classes::hkbLookAtModifier(class) => class.signature(),
            Classes::hkbLookAtModifierInternalState(class) => class.signature(),
            Classes::hkbManualSelectorGenerator(class) => class.signature(),
            Classes::hkbManualSelectorGeneratorInternalState(class) => class.signature(),
            Classes::hkbManualSelectorTransitionEffect(class) => class.signature(),
            Classes::hkbManualSelectorTransitionEffectInternalState(class) => {
                class.signature()
            }
            Classes::hkbMessageLog(class) => class.signature(),
            Classes::hkbMirroredSkeletonInfo(class) => class.signature(),
            Classes::hkbMirrorModifier(class) => class.signature(),
            Classes::hkbModifier(class) => class.signature(),
            Classes::hkbModifierGenerator(class) => class.signature(),
            Classes::hkbModifierList(class) => class.signature(),
            Classes::hkbModifierWrapper(class) => class.signature(),
            Classes::hkbMoveBoneAttachmentModifier(class) => class.signature(),
            Classes::hkbMoveCharacterModifier(class) => class.signature(),
            Classes::hkbMoveCharacterModifierInternalState(class) => class.signature(),
            Classes::hkbNamedEventPayload(class) => class.signature(),
            Classes::hkbNamedIntEventPayload(class) => class.signature(),
            Classes::hkbNamedRealEventPayload(class) => class.signature(),
            Classes::hkbNamedStringEventPayload(class) => class.signature(),
            Classes::hkbNode(class) => class.signature(),
            Classes::hkbNodeInternalStateInfo(class) => class.signature(),
            Classes::hkbnpPhysicsInterface(class) => class.signature(),
            Classes::hkbnpRagdollInterface(class) => class.signature(),
            Classes::hkbNullPhysicsInterface(class) => class.signature(),
            Classes::hkbParametricMotionGenerator(class) => class.signature(),
            Classes::hkbParticleSystemEventPayload(class) => class.signature(),
            Classes::hkbPhysicsInterface(class) => class.signature(),
            Classes::hkbPinBonesGenerator(class) => class.signature(),
            Classes::hkbPoseMatchingGenerator(class) => class.signature(),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => class.signature(),
            Classes::hkbPoseStoringGeneratorOutputListener(class) => class.signature(),
            Classes::hkbPoseStoringGeneratorOutputListenerStoredPose(class) => {
                class.signature()
            }
            Classes::hkbPoweredRagdollControlData(class) => class.signature(),
            Classes::hkbPoweredRagdollControlsModifier(class) => class.signature(),
            Classes::hkbProceduralBlenderGenerator(class) => class.signature(),
            Classes::hkbProjectData(class) => class.signature(),
            Classes::hkbProjectStringData(class) => class.signature(),
            Classes::hkbRadialSelectorGenerator(class) => class.signature(),
            Classes::hkbRadialSelectorGeneratorGeneratorInfo(class) => class.signature(),
            Classes::hkbRadialSelectorGeneratorGeneratorPair(class) => class.signature(),
            Classes::hkbRagdollControllerSetup(class) => class.signature(),
            Classes::hkbRagdollInterface(class) => class.signature(),
            Classes::hkbRaiseEventCommand(class) => class.signature(),
            Classes::hkbRealEventPayload(class) => class.signature(),
            Classes::hkbRealVariableSequencedData(class) => class.signature(),
            Classes::hkbRealVariableSequencedDataSample(class) => class.signature(),
            Classes::hkbRebuildScriptsCommand(class) => class.signature(),
            Classes::hkbReferencedGeneratorSyncInfo(class) => class.signature(),
            Classes::hkbReferencePoseGenerator(class) => class.signature(),
            Classes::hkbRigidBodyRagdollControlData(class) => class.signature(),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => class.signature(),
            Classes::hkbRigidBodySetup(class) => class.signature(),
            Classes::hkbRoleAttribute(class) => class.signature(),
            Classes::hkbRotateCharacterModifier(class) => class.signature(),
            Classes::hkbRotateCharacterModifierInternalState(class) => class.signature(),
            Classes::hkbScriptCondition(class) => class.signature(),
            Classes::hkbScriptGenerator(class) => class.signature(),
            Classes::hkbSenseHandleModifier(class) => class.signature(),
            Classes::hkbSenseHandleModifierRange(class) => class.signature(),
            Classes::hkbSequence(class) => class.signature(),
            Classes::hkbSequencedData(class) => class.signature(),
            Classes::hkbSequenceInternalState(class) => class.signature(),
            Classes::hkbSequenceStringData(class) => class.signature(),
            Classes::hkbSetBehaviorCommand(class) => class.signature(),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => class.signature(),
            Classes::hkbSetNodePropertyCommand(class) => class.signature(),
            Classes::hkbSetSelectedCharacterCommand(class) => class.signature(),
            Classes::hkbSetWordVariableCommand(class) => class.signature(),
            Classes::hkbSetWorldFromModelModifier(class) => class.signature(),
            Classes::hkbShapeSetup(class) => class.signature(),
            Classes::hkbSimulationControlCommand(class) => class.signature(),
            Classes::hkbSimulationStateInfo(class) => class.signature(),
            Classes::hkbStateChooser(class) => class.signature(),
            Classes::hkbStateChooserWrapper(class) => class.signature(),
            Classes::hkbStateDependentModifier(class) => class.signature(),
            Classes::hkbStateListener(class) => class.signature(),
            Classes::hkbStateMachine(class) => class.signature(),
            Classes::hkbStateMachineActiveTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineDelayedTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineEventPropertyArray(class) => class.signature(),
            Classes::hkbStateMachineInternalState(class) => class.signature(),
            Classes::hkbStateMachineNestedStateMachineData(class) => class.signature(),
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineStateInfo(class) => class.signature(),
            Classes::hkbStateMachineTimeInterval(class) => class.signature(),
            Classes::hkbStateMachineTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineTransitionInfoArray(class) => class.signature(),
            Classes::hkbStateMachineTransitionInfoReference(class) => class.signature(),
            Classes::hkbStringCondition(class) => class.signature(),
            Classes::hkbStringEventPayload(class) => class.signature(),
            Classes::hkbTestIdSelector(class) => class.signature(),
            Classes::hkbTimerModifier(class) => class.signature(),
            Classes::hkbTimerModifierInternalState(class) => class.signature(),
            Classes::hkbToolNodeType(class) => class.signature(),
            Classes::hkbTransformVectorModifier(class) => class.signature(),
            Classes::hkbTransformVectorModifierInternalState(class) => class.signature(),
            Classes::hkbTransitionEffect(class) => class.signature(),
            Classes::hkbTwistModifier(class) => class.signature(),
            Classes::hkbVariableBindingSet(class) => class.signature(),
            Classes::hkbVariableBindingSetBinding(class) => class.signature(),
            Classes::hkbVariableBounds(class) => class.signature(),
            Classes::hkbVariableInfo(class) => class.signature(),
            Classes::hkbVariableValue(class) => class.signature(),
            Classes::hkbVariableValueSet(class) => class.signature(),
            Classes::hkbWorldEnums(class) => class.signature(),
            Classes::hkbWorldFromModelModeData(class) => class.signature(),
            Classes::hkcdAdf(class) => class.signature(),
            Classes::hkcdDynamicAabbTree(class) => class.signature(),
            Classes::hkcdDynamicTreeAnisotropicMetric(class) => class.signature(),
            Classes::hkcdDynamicTreeCodec18(class) => class.signature(),
            Classes::hkcdDynamicTreeCodec32(class) => class.signature(),
            Classes::hkcdDynamicTreeCodecInt16(class) => class.signature(),
            Classes::hkcdDynamicTreeCodecInt16IntAabb(class) => class.signature(),
            Classes::hkcdDynamicTreeCodecRawUint(class) => class.signature(),
            Classes::hkcdDynamicTreeCodecRawUlong(class) => class.signature(),
            Classes::hkcdDynamicTreeCodecRawunsignedint(class) => class.signature(),
            Classes::hkcdDynamicTreeCodecRawunsignedlong(class) => class.signature(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDefaultTree32Storage(class) => class.signature(),
            Classes::hkcdDynamicTreeDefaultTree48Storage(class) => class.signature(),
            Classes::hkcdDynamicTreeDefaultTreeInt16Storage(class) => class.signature(),
            Classes::hkcdDynamicTreeDefaultTreePtrStorage(class) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorage16(class) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorage32(class) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStorageInt16(class) => class.signature(),
            Classes::hkcdDynamicTreeDynamicStoragePtr(class) => class.signature(),
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16(class) => {
                class.signature()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32(class) => {
                class.signature()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16(class) => {
                class.signature()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr(class) => {
                class.signature()
            }
            Classes::hkcdFourAabb(class) => class.signature(),
            Classes::hkcdPlanarCsgOperand(class) => class.signature(),
            Classes::hkcdPlanarCsgOperandGeomSource(class) => class.signature(),
            Classes::hkcdPlanarEntity(class) => class.signature(),
            Classes::hkcdPlanarGeometry(class) => class.signature(),
            Classes::hkcdPlanarGeometryPlanesCollection(class) => class.signature(),
            Classes::hkcdPlanarGeometryPolygonCollection(class) => class.signature(),
            Classes::hkcdPlanarGeometryPrimitivesCollection28(class) => class.signature(),
            Classes::hkcdPlanarGeometryPrimitivesPlane(class) => class.signature(),
            Classes::hkcdPlanarSolid(class) => class.signature(),
            Classes::hkcdPlanarSolidNode(class) => class.signature(),
            Classes::hkcdPlanarSolidNodeStorage(class) => class.signature(),
            Classes::hkcdShape(class) => class.signature(),
            Classes::hkcdShapeDispatchType(class) => class.signature(),
            Classes::hkcdShapeInfoCodecType(class) => class.signature(),
            Classes::hkcdShapeType(class) => class.signature(),
            Classes::hkcdSimdTree(class) => class.signature(),
            Classes::hkcdSimdTreeNode(class) => class.signature(),
            Classes::hkcdStaticAabbTree(class) => class.signature(),
            Classes::hkcdStaticMeshTreeBase(class) => class.signature(),
            Classes::hkcdStaticMeshTreeBasePrimitive(class) => class.signature(),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData(
                class,
            ) => class.signature(),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort(class) => {
                class.signature()
            }
            Classes::hkcdStaticMeshTreeBaseSection(class) => class.signature(),
            Classes::hkcdStaticMeshTreeBaseSectionDataRuns(class) => class.signature(),
            Classes::hkcdStaticMeshTreeBaseSectionPrimitives(class) => class.signature(),
            Classes::hkcdStaticMeshTreeBaseSectionSharedVertices(class) => {
                class.signature()
            }
            Classes::hkcdStaticMeshTreeDefaultDataRun(class) => class.signature(),
            Classes::hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun(
                class,
            ) => class.signature(),
            Classes::hkcdStaticPvs(class) => class.signature(),
            Classes::hkcdStaticPvsBlockHeader(class) => class.signature(),
            Classes::hkcdStaticTreeCodec3Axis(class) => class.signature(),
            Classes::hkcdStaticTreeCodec3Axis4(class) => class.signature(),
            Classes::hkcdStaticTreeCodec3Axis5(class) => class.signature(),
            Classes::hkcdStaticTreeCodec3Axis6(class) => class.signature(),
            Classes::hkcdStaticTreeCodecRaw(class) => class.signature(),
            Classes::hkcdStaticTreeDefaultTreeStorage32(class) => class.signature(),
            Classes::hkcdStaticTreeDefaultTreeStorage4(class) => class.signature(),
            Classes::hkcdStaticTreeDefaultTreeStorage5(class) => class.signature(),
            Classes::hkcdStaticTreeDefaultTreeStorage6(class) => class.signature(),
            Classes::hkcdStaticTreeDynamicStorage32(class) => class.signature(),
            Classes::hkcdStaticTreeDynamicStorage4(class) => class.signature(),
            Classes::hkcdStaticTreeDynamicStorage5(class) => class.signature(),
            Classes::hkcdStaticTreeDynamicStorage6(class) => class.signature(),
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5(class) => {
                class.signature()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6(class) => {
                class.signature()
            }
            Classes::hkClass(class) => class.signature(),
            Classes::hkClassEnum(class) => class.signature(),
            Classes::hkClassEnumItem(class) => class.signature(),
            Classes::hkClassMember(class) => class.signature(),
            Classes::hkCompressedMassProperties(class) => class.signature(),
            Classes::hkContactPoint(class) => class.signature(),
            Classes::hkContactPointMaterial(class) => class.signature(),
            Classes::hkCustomAttributes(class) => class.signature(),
            Classes::hkCustomAttributesAttribute(class) => class.signature(),
            Classes::hkDataObjectTypeAttribute(class) => class.signature(),
            Classes::hkDefaultMemoryTracker(class) => class.signature(),
            Classes::hkDescriptionAttribute(class) => class.signature(),
            Classes::hkDocumentationAttribute(class) => class.signature(),
            Classes::hkFloat16(class) => class.signature(),
            Classes::hkFloat16Transform(class) => class.signature(),
            Classes::hkFourTransposedPointsf(class) => class.signature(),
            Classes::hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations(
                class,
            ) => class.signature(),
            Classes::hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations(
                class,
            ) => class.signature(),
            Classes::hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance(
                class,
            ) => class.signature(),
            Classes::hkGeometry(class) => class.signature(),
            Classes::hkGeometryTriangle(class) => class.signature(),
            Classes::hkGizmoAttribute(class) => class.signature(),
            Classes::hkIndexedTransformSet(class) => class.signature(),
            Classes::hkIntRealPair(class) => class.signature(),
            Classes::hkLinkAttribute(class) => class.signature(),
            Classes::hkLocalFrame(class) => class.signature(),
            Classes::hkLocalFrameGroup(class) => class.signature(),
            Classes::hkMassProperties(class) => class.signature(),
            Classes::hkMemoryMeshBody(class) => class.signature(),
            Classes::hkMemoryMeshMaterial(class) => class.signature(),
            Classes::hkMemoryMeshShape(class) => class.signature(),
            Classes::hkMemoryMeshShapeSection(class) => class.signature(),
            Classes::hkMemoryMeshTexture(class) => class.signature(),
            Classes::hkMemoryMeshVertexBuffer(class) => class.signature(),
            Classes::hkMemoryResourceContainer(class) => class.signature(),
            Classes::hkMemoryResourceHandle(class) => class.signature(),
            Classes::hkMemoryResourceHandleExternalLink(class) => class.signature(),
            Classes::hkMeshBody(class) => class.signature(),
            Classes::hkMeshBoneIndexMapping(class) => class.signature(),
            Classes::hkMeshMaterial(class) => class.signature(),
            Classes::hkMeshSection(class) => class.signature(),
            Classes::hkMeshSectionCinfo(class) => class.signature(),
            Classes::hkMeshShape(class) => class.signature(),
            Classes::hkMeshTexture(class) => class.signature(),
            Classes::hkMeshTextureRawBufferDescriptor(class) => class.signature(),
            Classes::hkMeshVertexBuffer(class) => class.signature(),
            Classes::hkModelerNodeTypeAttribute(class) => class.signature(),
            Classes::hkMonitorStreamColorTable(class) => class.signature(),
            Classes::hkMonitorStreamColorTableColorPair(class) => class.signature(),
            Classes::hkMonitorStreamFrameInfo(class) => class.signature(),
            Classes::hkMonitorStreamStringMap(class) => class.signature(),
            Classes::hkMonitorStreamStringMapStringMap(class) => class.signature(),
            Classes::hkMotionState(class) => class.signature(),
            Classes::hkMultipleVertexBuffer(class) => class.signature(),
            Classes::hkMultipleVertexBufferElementInfo(class) => class.signature(),
            Classes::hkMultipleVertexBufferLockedElement(class) => class.signature(),
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => class.signature(),
            Classes::hkMultiThreadCheck(class) => class.signature(),
            Classes::hknpAction(class) => class.signature(),
            Classes::hknpArrayAction(class) => class.signature(),
            Classes::hknpBallGun(class) => class.signature(),
            Classes::hknpBinaryAction(class) => class.signature(),
            Classes::hknpBody(class) => class.signature(),
            Classes::hknpBodyCinfo(class) => class.signature(),
            Classes::hknpBodyQuality(class) => class.signature(),
            Classes::hknpBodyQualityLibrary(class) => class.signature(),
            Classes::hknpBodyReference(class) => class.signature(),
            Classes::hknpBreakableConstraintData(class) => class.signature(),
            Classes::hknpBridgeConstraintAtom(class) => class.signature(),
            Classes::hknpBroadPhaseConfig(class) => class.signature(),
            Classes::hknpBroadPhaseConfigLayer(class) => class.signature(),
            Classes::hknpBSMaterial(class) => class.signature(),
            Classes::hknpBSMaterialProperties(class) => class.signature(),
            Classes::hknpCapsuleShape(class) => class.signature(),
            Classes::hknpCharacterProxyCinfo(class) => class.signature(),
            Classes::hknpCharacterRigidBodyCinfo(class) => class.signature(),
            Classes::hknpCircularSurfaceVelocity(class) => class.signature(),
            Classes::hknpCollisionDispatchType(class) => class.signature(),
            Classes::hknpCollisionFilter(class) => class.signature(),
            Classes::hknpCompositeShape(class) => class.signature(),
            Classes::hknpCompoundShape(class) => class.signature(),
            Classes::hknpCompoundShapeInternalsKeyMask(class) => class.signature(),
            Classes::hknpCompoundShapeKeyMask(class) => class.signature(),
            Classes::hknpCompressedHeightFieldShape(class) => class.signature(),
            Classes::hknpCompressedMeshShape(class) => class.signature(),
            Classes::hknpCompressedMeshShapeData(class) => class.signature(),
            Classes::hknpCompressedMeshShapeInternalsKeyMask(class) => class.signature(),
            Classes::hknpCompressedMeshShapeTree(class) => class.signature(),
            Classes::hknpCompressedMeshShapeTreeDataRun(class) => class.signature(),
            Classes::hknpCompressedMeshShapeTreeDataRunData(class) => class.signature(),
            Classes::hknpConstraint(class) => class.signature(),
            Classes::hknpConstraintCinfo(class) => class.signature(),
            Classes::hknpConstraintCollisionFilter(class) => class.signature(),
            Classes::hknpConvexPolytopeShape(class) => class.signature(),
            Classes::hknpConvexPolytopeShapeFace(class) => class.signature(),
            Classes::hknpConvexShape(class) => class.signature(),
            Classes::hknpDecoratorShape(class) => class.signature(),
            Classes::hknpDefaultBroadPhaseConfig(class) => class.signature(),
            Classes::hknpDefaultExternMeshShapeGeometry(class) => class.signature(),
            Classes::hknpDestructionShapeProperties(class) => class.signature(),
            Classes::hknpDisableCollisionFilter(class) => class.signature(),
            Classes::hknpDummyShape(class) => class.signature(),
            Classes::hknpDynamicCompoundShape(class) => class.signature(),
            Classes::hknpDynamicCompoundShapeData(class) => class.signature(),
            Classes::hknpDynamicCompoundShapeTree(class) => class.signature(),
            Classes::hknpExternMeshShape(class) => class.signature(),
            Classes::hknpExternMeshShapeData(class) => class.signature(),
            Classes::hknpExternMeshShapeGeometry(class) => class.signature(),
            Classes::hknpFirstPersonGun(class) => class.signature(),
            Classes::hknpGroupCollisionFilter(class) => class.signature(),
            Classes::hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516(
                class,
            ) => class.signature(),
            Classes::hknpHeightFieldShape(class) => class.signature(),
            Classes::hknpLinearSurfaceVelocity(class) => class.signature(),
            Classes::hknpLodShape(class) => class.signature(),
            Classes::hknpLodShapeLevelOfDetailInfo(class) => class.signature(),
            Classes::hknpMalleableConstraintData(class) => class.signature(),
            Classes::hknpMaskedShape(class) => class.signature(),
            Classes::hknpMaskedShapeMaskWrapper(class) => class.signature(),
            Classes::hknpMaterial(class) => class.signature(),
            Classes::hknpMaterialDescriptor(class) => class.signature(),
            Classes::hknpMaterialLibrary(class) => class.signature(),
            Classes::hknpMaterialPalette(class) => class.signature(),
            Classes::hknpMinMaxQuadTree(class) => class.signature(),
            Classes::hknpMinMaxQuadTreeMinMaxLevel(class) => class.signature(),
            Classes::hknpMotion(class) => class.signature(),
            Classes::hknpMotionCinfo(class) => class.signature(),
            Classes::hknpMotionProperties(class) => class.signature(),
            Classes::hknpMotionPropertiesLibrary(class) => class.signature(),
            Classes::hknpPairCollisionFilter(class) => class.signature(),
            Classes::hknpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.signature()
            }
            Classes::hknpPhysicsSceneData(class) => class.signature(),
            Classes::hknpPhysicsSystemData(class) => class.signature(),
            Classes::hknpProjectileGun(class) => class.signature(),
            Classes::hknpRagdollData(class) => class.signature(),
            Classes::hknpRagdollKeyFrameHierarchyUtilityControlData(class) => {
                class.signature()
            }
            Classes::hknpRefMaterial(class) => class.signature(),
            Classes::hknpRefWorldCinfo(class) => class.signature(),
            Classes::hknpScaledConvexShape(class) => class.signature(),
            Classes::hknpScaledConvexShapeBase(class) => class.signature(),
            Classes::hknpShape(class) => class.signature(),
            Classes::hknpShapeInstance(class) => class.signature(),
            Classes::hknpShapeKeyMask(class) => class.signature(),
            Classes::hknpShapeMassProperties(class) => class.signature(),
            Classes::hknpShapeSignals(class) => class.signature(),
            Classes::hknpShapeTagCodec(class) => class.signature(),
            Classes::hknpShapeType(class) => class.signature(),
            Classes::hknpSparseCompactMapunsignedshort(class) => class.signature(),
            Classes::hknpSphereShape(class) => class.signature(),
            Classes::hknpStaticCompoundShape(class) => class.signature(),
            Classes::hknpStaticCompoundShapeData(class) => class.signature(),
            Classes::hknpStaticCompoundShapeInternals(class) => class.signature(),
            Classes::hknpStaticCompoundShapeKeyMask(class) => class.signature(),
            Classes::hknpStaticCompoundShapeTree(class) => class.signature(),
            Classes::hknpSurfaceVelocity(class) => class.signature(),
            Classes::hknpTriangleShape(class) => class.signature(),
            Classes::hknpTyremarkPoint(class) => class.signature(),
            Classes::hknpTyremarksInfo(class) => class.signature(),
            Classes::hknpTyremarksWheel(class) => class.signature(),
            Classes::hknpUnaryAction(class) => class.signature(),
            Classes::hknpVehicleAerodynamics(class) => class.signature(),
            Classes::hknpVehicleBrake(class) => class.signature(),
            Classes::hknpVehicleData(class) => class.signature(),
            Classes::hknpVehicleDataWheelComponentParams(class) => class.signature(),
            Classes::hknpVehicleDefaultAerodynamics(class) => class.signature(),
            Classes::hknpVehicleDefaultAnalogDriverInput(class) => class.signature(),
            Classes::hknpVehicleDefaultBrake(class) => class.signature(),
            Classes::hknpVehicleDefaultBrakeWheelBrakingProperties(class) => {
                class.signature()
            }
            Classes::hknpVehicleDefaultEngine(class) => class.signature(),
            Classes::hknpVehicleDefaultSteering(class) => class.signature(),
            Classes::hknpVehicleDefaultSuspension(class) => class.signature(),
            Classes::hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.signature(),
            Classes::hknpVehicleDefaultTransmission(class) => class.signature(),
            Classes::hknpVehicleDefaultVelocityDamper(class) => class.signature(),
            Classes::hknpVehicleDriverInput(class) => class.signature(),
            Classes::hknpVehicleDriverInputAnalogStatus(class) => class.signature(),
            Classes::hknpVehicleDriverInputStatus(class) => class.signature(),
            Classes::hknpVehicleEngine(class) => class.signature(),
            Classes::hknpVehicleInstance(class) => class.signature(),
            Classes::hknpVehicleInstanceWheelInfo(class) => class.signature(),
            Classes::hknpVehicleLinearCastWheelCollide(class) => class.signature(),
            Classes::hknpVehicleLinearCastWheelCollideWheelState(class) => {
                class.signature()
            }
            Classes::hknpVehicleRayCastWheelCollide(class) => class.signature(),
            Classes::hknpVehicleSteering(class) => class.signature(),
            Classes::hknpVehicleSuspension(class) => class.signature(),
            Classes::hknpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.signature()
            }
            Classes::hknpVehicleTransmission(class) => class.signature(),
            Classes::hknpVehicleVelocityDamper(class) => class.signature(),
            Classes::hknpVehicleWheelCollide(class) => class.signature(),
            Classes::hknpWorldCinfo(class) => class.signature(),
            Classes::hknpWorldSnapshot(class) => class.signature(),
            Classes::hkOffsetBitField(class) => class.signature(),
            Classes::hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.signature(),
            Classes::hkp2dAngConstraintAtom(class) => class.signature(),
            Classes::hkp3dAngConstraintAtom(class) => class.signature(),
            Classes::hkPackedVector3(class) => class.signature(),
            Classes::hkPackedVector8_3(class) => class.signature(),
            Classes::hkPackfileHeader(class) => class.signature(),
            Classes::hkPackfileSectionHeader(class) => class.signature(),
            Classes::hkpAngConstraintAtom(class) => class.signature(),
            Classes::hkpAngFrictionConstraintAtom(class) => class.signature(),
            Classes::hkpAngLimitConstraintAtom(class) => class.signature(),
            Classes::hkpAngMotorConstraintAtom(class) => class.signature(),
            Classes::hkpBallAndSocketConstraintData(class) => class.signature(),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => class.signature(),
            Classes::hkpBallSocketConstraintAtom(class) => class.signature(),
            Classes::hkpBridgeAtoms(class) => class.signature(),
            Classes::hkpBridgeConstraintAtom(class) => class.signature(),
            Classes::hkpCallbackConstraintMotor(class) => class.signature(),
            Classes::hkpCogWheelConstraintAtom(class) => class.signature(),
            Classes::hkpCogWheelConstraintData(class) => class.signature(),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.signature(),
            Classes::hkpConeLimitConstraintAtom(class) => class.signature(),
            Classes::hkpConstraintAtom(class) => class.signature(),
            Classes::hkpConstraintData(class) => class.signature(),
            Classes::hkpConstraintMotor(class) => class.signature(),
            Classes::hkpDeformableAngConstraintAtom(class) => class.signature(),
            Classes::hkpDeformableFixedConstraintData(class) => class.signature(),
            Classes::hkpDeformableFixedConstraintDataAtoms(class) => class.signature(),
            Classes::hkpDeformableLinConstraintAtom(class) => class.signature(),
            Classes::hkpFixedConstraintData(class) => class.signature(),
            Classes::hkpFixedConstraintDataAtoms(class) => class.signature(),
            Classes::hkpHingeConstraintData(class) => class.signature(),
            Classes::hkpHingeConstraintDataAtoms(class) => class.signature(),
            Classes::hkpHingeLimitsData(class) => class.signature(),
            Classes::hkpHingeLimitsDataAtoms(class) => class.signature(),
            Classes::hkpLimitedForceConstraintMotor(class) => class.signature(),
            Classes::hkpLimitedHingeConstraintData(class) => class.signature(),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => class.signature(),
            Classes::hkpLinConstraintAtom(class) => class.signature(),
            Classes::hkpLinearClearanceConstraintData(class) => class.signature(),
            Classes::hkpLinearClearanceConstraintDataAtoms(class) => class.signature(),
            Classes::hkpLinearParametricCurve(class) => class.signature(),
            Classes::hkpLinFrictionConstraintAtom(class) => class.signature(),
            Classes::hkpLinLimitConstraintAtom(class) => class.signature(),
            Classes::hkpLinMotorConstraintAtom(class) => class.signature(),
            Classes::hkpLinSoftConstraintAtom(class) => class.signature(),
            Classes::hkPostFinishAttribute(class) => class.signature(),
            Classes::hkpOverwritePivotConstraintAtom(class) => class.signature(),
            Classes::hkpParametricCurve(class) => class.signature(),
            Classes::hkpPointToPathConstraintData(class) => class.signature(),
            Classes::hkpPointToPlaneConstraintData(class) => class.signature(),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => class.signature(),
            Classes::hkpPositionConstraintMotor(class) => class.signature(),
            Classes::hkpPrismaticConstraintData(class) => class.signature(),
            Classes::hkpPrismaticConstraintDataAtoms(class) => class.signature(),
            Classes::hkpPulleyConstraintAtom(class) => class.signature(),
            Classes::hkpPulleyConstraintData(class) => class.signature(),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.signature(),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.signature(),
            Classes::hkpRackAndPinionConstraintData(class) => class.signature(),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => class.signature(),
            Classes::hkpRagdollConstraintData(class) => class.signature(),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.signature(),
            Classes::hkpRagdollLimitsData(class) => class.signature(),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.signature(),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.signature(),
            Classes::hkpRotationalConstraintData(class) => class.signature(),
            Classes::hkpRotationalConstraintDataAtoms(class) => class.signature(),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => class.signature(),
            Classes::hkpSetLocalTransformsConstraintAtom(class) => class.signature(),
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => class.signature(),
            Classes::hkpSetupStabilizationAtom(class) => class.signature(),
            Classes::hkpSpringDamperConstraintMotor(class) => class.signature(),
            Classes::hkpStiffSpringConstraintAtom(class) => class.signature(),
            Classes::hkpStiffSpringConstraintData(class) => class.signature(),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => class.signature(),
            Classes::hkpTwistLimitConstraintAtom(class) => class.signature(),
            Classes::hkpVehicleFrictionDescription(class) => class.signature(),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => {
                class.signature()
            }
            Classes::hkpVehicleFrictionStatus(class) => class.signature(),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => class.signature(),
            Classes::hkpVelocityConstraintMotor(class) => class.signature(),
            Classes::hkpWheelConstraintData(class) => class.signature(),
            Classes::hkpWheelConstraintDataAtoms(class) => class.signature(),
            Classes::hkpWheelFrictionConstraintAtom(class) => class.signature(),
            Classes::hkpWheelFrictionConstraintAtomAxle(class) => class.signature(),
            Classes::hkpWheelFrictionConstraintData(class) => class.signature(),
            Classes::hkpWheelFrictionConstraintDataAtoms(class) => class.signature(),
            Classes::hkpWrappedConstraintData(class) => class.signature(),
            Classes::hkQTransform(class) => class.signature(),
            Classes::hkQTransformf(class) => class.signature(),
            Classes::hkRangeInt32Attribute(class) => class.signature(),
            Classes::hkRangeRealAttribute(class) => class.signature(),
            Classes::hkRefCountedProperties(class) => class.signature(),
            Classes::hkRefCountedPropertiesEntry(class) => class.signature(),
            Classes::hkReferencedObject(class) => class.signature(),
            Classes::hkResourceBase(class) => class.signature(),
            Classes::hkResourceContainer(class) => class.signature(),
            Classes::hkResourceHandle(class) => class.signature(),
            Classes::hkRootLevelContainer(class) => class.signature(),
            Classes::hkRootLevelContainerNamedVariant(class) => class.signature(),
            Classes::hkSemanticsAttribute(class) => class.signature(),
            Classes::hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair(
                class,
            ) => class.signature(),
            Classes::hkSetIntFloatPair(class) => class.signature(),
            Classes::hkSetUint32(class) => class.signature(),
            Classes::hkSetUint64(class) => class.signature(),
            Classes::hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint(
                class,
            ) => class.signature(),
            Classes::hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong(
                class,
            ) => class.signature(),
            Classes::hkSimpleLocalFrame(class) => class.signature(),
            Classes::hkSimpleProperty(class) => class.signature(),
            Classes::hkSimplePropertyValue(class) => class.signature(),
            Classes::hkSkinBinding(class) => class.signature(),
            Classes::hkSkinnedMeshShape(class) => class.signature(),
            Classes::hkSkinnedMeshShapeBoneSection(class) => class.signature(),
            Classes::hkSkinnedMeshShapeBoneSet(class) => class.signature(),
            Classes::hkSkinnedMeshShapePart(class) => class.signature(),
            Classes::hkSkinnedRefMeshShape(class) => class.signature(),
            Classes::hkSphere(class) => class.signature(),
            Classes::hkStorageSkinnedMeshShape(class) => class.signature(),
            Classes::hkStringObject(class) => class.signature(),
            Classes::hkSweptTransformf(class) => class.signature(),
            Classes::hkTraceStreamTitle(class) => class.signature(),
            Classes::hkUFloat8(class) => class.signature(),
            Classes::hkUiAttribute(class) => class.signature(),
            Classes::hkUuid(class) => class.signature(),
            Classes::hkVariableTweakingHelper(class) => class.signature(),
            Classes::hkVariableTweakingHelperBoolVariableInfo(class) => class.signature(),
            Classes::hkVariableTweakingHelperIntVariableInfo(class) => class.signature(),
            Classes::hkVariableTweakingHelperRealVariableInfo(class) => class.signature(),
            Classes::hkVariableTweakingHelperVector4VariableInfo(class) => {
                class.signature()
            }
            Classes::hkVertexFormat(class) => class.signature(),
            Classes::hkVertexFormatElement(class) => class.signature(),
            Classes::hkxAnimatedFloat(class) => class.signature(),
            Classes::hkxAnimatedMatrix(class) => class.signature(),
            Classes::hkxAnimatedQuaternion(class) => class.signature(),
            Classes::hkxAnimatedVector(class) => class.signature(),
            Classes::hkxAttribute(class) => class.signature(),
            Classes::hkxAttributeGroup(class) => class.signature(),
            Classes::hkxAttributeHolder(class) => class.signature(),
            Classes::hkxEnum(class) => class.signature(),
            Classes::hkxEnumItem(class) => class.signature(),
            Classes::hkxIndexBuffer(class) => class.signature(),
            Classes::hkxMaterial(class) => class.signature(),
            Classes::hkxMaterialProperty(class) => class.signature(),
            Classes::hkxMaterialTextureStage(class) => class.signature(),
            Classes::hkxMesh(class) => class.signature(),
            Classes::hkxMeshSection(class) => class.signature(),
            Classes::hkxMeshUserChannelInfo(class) => class.signature(),
            Classes::hkxSparselyAnimatedBool(class) => class.signature(),
            Classes::hkxSparselyAnimatedEnum(class) => class.signature(),
            Classes::hkxSparselyAnimatedInt(class) => class.signature(),
            Classes::hkxSparselyAnimatedString(class) => class.signature(),
            Classes::hkxVertexAnimation(class) => class.signature(),
            Classes::hkxVertexAnimationUsageMap(class) => class.signature(),
            Classes::hkxVertexBuffer(class) => class.signature(),
            Classes::hkxVertexBufferVertexData(class) => class.signature(),
            Classes::hkxVertexDescription(class) => class.signature(),
            Classes::hkxVertexDescriptionElementDecl(class) => class.signature(),
        }
    }
    fn deps_indexes(&self) -> Vec<&havok_types::Pointer<'_>> {
        match &self {
            Classes::BGSGamebryoSequenceGenerator(class) => class.deps_indexes(),
            Classes::BSAlignBoneModifier(class) => class.deps_indexes(),
            Classes::BSAssignVariablesModifier(class) => class.deps_indexes(),
            Classes::BSBehaviorGraphSwapGenerator(class) => class.deps_indexes(),
            Classes::BSBoneSwitchGenerator(class) => class.deps_indexes(),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.deps_indexes(),
            Classes::BSComputeAddBoneAnimModifier(class) => class.deps_indexes(),
            Classes::BSCyclicBlendTransitionGenerator(class) => class.deps_indexes(),
            Classes::BSDecomposeVectorModifier(class) => class.deps_indexes(),
            Classes::BSDirectAtCapturePoseModifier(class) => class.deps_indexes(),
            Classes::BSDirectAtModifier(class) => class.deps_indexes(),
            Classes::BSDistTriggerModifier(class) => class.deps_indexes(),
            Classes::BSEventEveryNEventsModifier(class) => class.deps_indexes(),
            Classes::BSEventOnDeactivateModifier(class) => class.deps_indexes(),
            Classes::BSEventOnFalseToTrueModifier(class) => class.deps_indexes(),
            Classes::BSGetTimeStepModifier(class) => class.deps_indexes(),
            Classes::BSInterpValueModifier(class) => class.deps_indexes(),
            Classes::BSIsActiveModifier(class) => class.deps_indexes(),
            Classes::BSIStateManagerModifier(class) => class.deps_indexes(),
            Classes::BSIStateManagerModifierBSiStateData(class) => class.deps_indexes(),
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.deps_indexes()
            }
            Classes::BSiStateTaggingGenerator(class) => class.deps_indexes(),
            Classes::BSiStateTaggingStateListener(class) => class.deps_indexes(),
            Classes::BSLimbCycleModifier(class) => class.deps_indexes(),
            Classes::BSLimbIKModifier(class) => class.deps_indexes(),
            Classes::BSLookAtCapturePoseModifier(class) => class.deps_indexes(),
            Classes::BSLookAtModifier(class) => class.deps_indexes(),
            Classes::BSLookAtModifierBoneData(class) => class.deps_indexes(),
            Classes::BSModifyOnceModifier(class) => class.deps_indexes(),
            Classes::BSOffsetAnimationGenerator(class) => class.deps_indexes(),
            Classes::BSPassByTargetTriggerModifier(class) => class.deps_indexes(),
            Classes::BSRagdollContactListenerModifier(class) => class.deps_indexes(),
            Classes::BSRandomAlarmModifier(class) => class.deps_indexes(),
            Classes::BSRetargetBodyBlend(class) => class.deps_indexes(),
            Classes::BSReverseSpineTwistModifier(class) => class.deps_indexes(),
            Classes::BSRootTwistModifier(class) => class.deps_indexes(),
            Classes::BSSpeedSamplerModifier(class) => class.deps_indexes(),
            Classes::BSSpringState(class) => class.deps_indexes(),
            Classes::BSTimerModifier(class) => class.deps_indexes(),
            Classes::BSTweenerModifier(class) => class.deps_indexes(),
            Classes::DynamicAnimationTaggingGenerator(class) => class.deps_indexes(),
            Classes::hclAction(class) => class.deps_indexes(),
            Classes::hclAntiPinchConstraintSet(class) => class.deps_indexes(),
            Classes::hclAntiPinchConstraintSetPerParticle(class) => class.deps_indexes(),
            Classes::hclBendLinkConstraintSet(class) => class.deps_indexes(),
            Classes::hclBendLinkConstraintSetLink(class) => class.deps_indexes(),
            Classes::hclBendLinkConstraintSetMx(class) => class.deps_indexes(),
            Classes::hclBendLinkConstraintSetMxBatch(class) => class.deps_indexes(),
            Classes::hclBendLinkConstraintSetMxSingle(class) => class.deps_indexes(),
            Classes::hclBendStiffnessConstraintSet(class) => class.deps_indexes(),
            Classes::hclBendStiffnessConstraintSetLink(class) => class.deps_indexes(),
            Classes::hclBendStiffnessConstraintSetMx(class) => class.deps_indexes(),
            Classes::hclBendStiffnessConstraintSetMxBatch(class) => class.deps_indexes(),
            Classes::hclBendStiffnessConstraintSetMxSingle(class) => class.deps_indexes(),
            Classes::hclBendStiffnessSetupObject(class) => class.deps_indexes(),
            Classes::hclBlendSomeVerticesOperator(class) => class.deps_indexes(),
            Classes::hclBlendSomeVerticesOperatorBlendEntry(class) => {
                class.deps_indexes()
            }
            Classes::hclBonePlanesConstraintSet(class) => class.deps_indexes(),
            Classes::hclBonePlanesConstraintSetBonePlane(class) => class.deps_indexes(),
            Classes::hclBoneSpaceDeformer(class) => class.deps_indexes(),
            Classes::hclBoneSpaceDeformerFourBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerLocalBlockP(class) => class.deps_indexes(),
            Classes::hclBoneSpaceDeformerLocalBlockPN(class) => class.deps_indexes(),
            Classes::hclBoneSpaceDeformerLocalBlockPNT(class) => class.deps_indexes(),
            Classes::hclBoneSpaceDeformerLocalBlockPNTB(class) => class.deps_indexes(),
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedP(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPN(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNT(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNTB(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerOneBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerThreeBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceDeformerTwoBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceMeshMeshDeformOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceMeshMeshDeformPNOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceMeshMeshDeformPNTBOperator(class) => {
                class.deps_indexes()
            }
            Classes::hclBoneSpaceMeshMeshDeformPNTOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceMeshMeshDeformPOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceSkinOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceSkinPNOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceSkinPNTBOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceSkinPNTOperator(class) => class.deps_indexes(),
            Classes::hclBoneSpaceSkinPOperator(class) => class.deps_indexes(),
            Classes::hclBufferDefinition(class) => class.deps_indexes(),
            Classes::hclBufferLayout(class) => class.deps_indexes(),
            Classes::hclBufferLayoutBufferElement(class) => class.deps_indexes(),
            Classes::hclBufferLayoutSlot(class) => class.deps_indexes(),
            Classes::hclBufferSetupObject(class) => class.deps_indexes(),
            Classes::hclBufferUsage(class) => class.deps_indexes(),
            Classes::hclCapsuleShape(class) => class.deps_indexes(),
            Classes::hclClothContainer(class) => class.deps_indexes(),
            Classes::hclClothData(class) => class.deps_indexes(),
            Classes::hclClothSetupContainer(class) => class.deps_indexes(),
            Classes::hclClothSetupObject(class) => class.deps_indexes(),
            Classes::hclClothState(class) => class.deps_indexes(),
            Classes::hclClothStateBufferAccess(class) => class.deps_indexes(),
            Classes::hclClothStateSetupObject(class) => class.deps_indexes(),
            Classes::hclClothStateTransformSetAccess(class) => class.deps_indexes(),
            Classes::hclCollidable(class) => class.deps_indexes(),
            Classes::hclCompressibleLinkConstraintSet(class) => class.deps_indexes(),
            Classes::hclCompressibleLinkConstraintSetLink(class) => class.deps_indexes(),
            Classes::hclCompressibleLinkConstraintSetMx(class) => class.deps_indexes(),
            Classes::hclCompressibleLinkConstraintSetMxBatch(class) => {
                class.deps_indexes()
            }
            Classes::hclCompressibleLinkConstraintSetMxSingle(class) => {
                class.deps_indexes()
            }
            Classes::hclConstraintSet(class) => class.deps_indexes(),
            Classes::hclConstraintSetSetupObject(class) => class.deps_indexes(),
            Classes::hclConvexGeometryShape(class) => class.deps_indexes(),
            Classes::hclConvexHeightFieldShape(class) => class.deps_indexes(),
            Classes::hclConvexPlanesShape(class) => class.deps_indexes(),
            Classes::hclCopyVerticesOperator(class) => class.deps_indexes(),
            Classes::hclEdgeSelectionInput(class) => class.deps_indexes(),
            Classes::hclGatherAllVerticesOperator(class) => class.deps_indexes(),
            Classes::hclGatherSomeVerticesOperator(class) => class.deps_indexes(),
            Classes::hclGatherSomeVerticesOperatorVertexPair(class) => {
                class.deps_indexes()
            }
            Classes::hclInputConvertOperator(class) => class.deps_indexes(),
            Classes::hclLocalRangeConstraintSet(class) => class.deps_indexes(),
            Classes::hclLocalRangeConstraintSetLocalConstraint(class) => {
                class.deps_indexes()
            }
            Classes::hclLocalRangeSetupObject(class) => class.deps_indexes(),
            Classes::hclMeshBoneDeformOperator(class) => class.deps_indexes(),
            Classes::hclMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.deps_indexes()
            }
            Classes::hclMeshBoneDeformSetupObject(class) => class.deps_indexes(),
            Classes::hclMeshMeshDeformOperator(class) => class.deps_indexes(),
            Classes::hclMeshMeshDeformOperatorTriangleVertexPair(class) => {
                class.deps_indexes()
            }
            Classes::hclMoveFixedParticlesSetupObject(class) => class.deps_indexes(),
            Classes::hclMoveParticlesOperator(class) => class.deps_indexes(),
            Classes::hclMoveParticlesOperatorVertexParticlePair(class) => {
                class.deps_indexes()
            }
            Classes::hclNamedSetupMesh(class) => class.deps_indexes(),
            Classes::hclNamedTransformSetSetupObject(class) => class.deps_indexes(),
            Classes::hclObjectSpaceDeformer(class) => class.deps_indexes(),
            Classes::hclObjectSpaceDeformerFourBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerLocalBlockP(class) => class.deps_indexes(),
            Classes::hclObjectSpaceDeformerLocalBlockPN(class) => class.deps_indexes(),
            Classes::hclObjectSpaceDeformerLocalBlockPNT(class) => class.deps_indexes(),
            Classes::hclObjectSpaceDeformerLocalBlockPNTB(class) => class.deps_indexes(),
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedP(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPN(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNT(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNTB(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerOneBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerThreeBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceDeformerTwoBlendEntryBlock(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceMeshMeshDeformOperator(class) => class.deps_indexes(),
            Classes::hclObjectSpaceMeshMeshDeformPNOperator(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceMeshMeshDeformPNTBOperator(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceMeshMeshDeformPNTOperator(class) => {
                class.deps_indexes()
            }
            Classes::hclObjectSpaceMeshMeshDeformPOperator(class) => class.deps_indexes(),
            Classes::hclObjectSpaceSkinOperator(class) => class.deps_indexes(),
            Classes::hclObjectSpaceSkinPNOperator(class) => class.deps_indexes(),
            Classes::hclObjectSpaceSkinPNTBOperator(class) => class.deps_indexes(),
            Classes::hclObjectSpaceSkinPNTOperator(class) => class.deps_indexes(),
            Classes::hclObjectSpaceSkinPOperator(class) => class.deps_indexes(),
            Classes::hclOperator(class) => class.deps_indexes(),
            Classes::hclOperatorSetupObject(class) => class.deps_indexes(),
            Classes::hclOutputConvertOperator(class) => class.deps_indexes(),
            Classes::hclPlaneShape(class) => class.deps_indexes(),
            Classes::hclRuntimeConversionInfo(class) => class.deps_indexes(),
            Classes::hclRuntimeConversionInfoElementConversion(class) => {
                class.deps_indexes()
            }
            Classes::hclRuntimeConversionInfoSlotConversion(class) => {
                class.deps_indexes()
            }
            Classes::hclScratchBufferDefinition(class) => class.deps_indexes(),
            Classes::hclScratchBufferSetupObject(class) => class.deps_indexes(),
            Classes::hclSetupMesh(class) => class.deps_indexes(),
            Classes::hclShadowBufferDefinition(class) => class.deps_indexes(),
            Classes::hclShape(class) => class.deps_indexes(),
            Classes::hclSimClothBufferSetupObject(class) => class.deps_indexes(),
            Classes::hclSimClothData(class) => class.deps_indexes(),
            Classes::hclSimClothDataCollidablePinchingData(class) => class.deps_indexes(),
            Classes::hclSimClothDataCollidableTransformMap(class) => class.deps_indexes(),
            Classes::hclSimClothDataLandscapeCollisionData(class) => class.deps_indexes(),
            Classes::hclSimClothDataOverridableSimulationInfo(class) => {
                class.deps_indexes()
            }
            Classes::hclSimClothDataParticleData(class) => class.deps_indexes(),
            Classes::hclSimClothDataTransferMotionData(class) => class.deps_indexes(),
            Classes::hclSimClothPose(class) => class.deps_indexes(),
            Classes::hclSimClothSetupObject(class) => class.deps_indexes(),
            Classes::hclSimClothSetupObjectPerInstanceCollidable(class) => {
                class.deps_indexes()
            }
            Classes::hclSimClothSetupObjectTransferMotionSetupData(class) => {
                class.deps_indexes()
            }
            Classes::hclSimpleMeshBoneDeformOperator(class) => class.deps_indexes(),
            Classes::hclSimpleMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.deps_indexes()
            }
            Classes::hclSimpleWindAction(class) => class.deps_indexes(),
            Classes::hclSimulateOperator(class) => class.deps_indexes(),
            Classes::hclSimulateSetupObject(class) => class.deps_indexes(),
            Classes::hclSimulationSetupMesh(class) => class.deps_indexes(),
            Classes::hclSimulationSetupMeshMapOptions(class) => class.deps_indexes(),
            Classes::hclSkinOperator(class) => class.deps_indexes(),
            Classes::hclSkinOperatorBoneInfluence(class) => class.deps_indexes(),
            Classes::hclSkinSetupObject(class) => class.deps_indexes(),
            Classes::hclSphereShape(class) => class.deps_indexes(),
            Classes::hclStandardLinkConstraintSet(class) => class.deps_indexes(),
            Classes::hclStandardLinkConstraintSetLink(class) => class.deps_indexes(),
            Classes::hclStandardLinkConstraintSetMx(class) => class.deps_indexes(),
            Classes::hclStandardLinkConstraintSetMxBatch(class) => class.deps_indexes(),
            Classes::hclStandardLinkConstraintSetMxSingle(class) => class.deps_indexes(),
            Classes::hclStandardLinkSetupObject(class) => class.deps_indexes(),
            Classes::hclStaticDisplayBufferSetupObject(class) => class.deps_indexes(),
            Classes::hclStaticShadowBufferDefinition(class) => class.deps_indexes(),
            Classes::hclStretchLinkConstraintSet(class) => class.deps_indexes(),
            Classes::hclStretchLinkConstraintSetLink(class) => class.deps_indexes(),
            Classes::hclStretchLinkConstraintSetMx(class) => class.deps_indexes(),
            Classes::hclStretchLinkConstraintSetMxBatch(class) => class.deps_indexes(),
            Classes::hclStretchLinkConstraintSetMxSingle(class) => class.deps_indexes(),
            Classes::hclStretchLinkSetupObject(class) => class.deps_indexes(),
            Classes::hclTaperedCapsuleShape(class) => class.deps_indexes(),
            Classes::hclTransformSetDefinition(class) => class.deps_indexes(),
            Classes::hclTransformSetSetupObject(class) => class.deps_indexes(),
            Classes::hclTransformSetUsage(class) => class.deps_indexes(),
            Classes::hclTransformSetUsageTransformTracker(class) => class.deps_indexes(),
            Classes::hclTransitionConstraintSet(class) => class.deps_indexes(),
            Classes::hclTransitionConstraintSetPerParticle(class) => class.deps_indexes(),
            Classes::hclTriangleSelectionInput(class) => class.deps_indexes(),
            Classes::hclUpdateAllVertexFramesOperator(class) => class.deps_indexes(),
            Classes::hclUpdateSomeVertexFramesOperator(class) => class.deps_indexes(),
            Classes::hclUpdateSomeVertexFramesOperatorTriangle(class) => {
                class.deps_indexes()
            }
            Classes::hclVertexFloatInput(class) => class.deps_indexes(),
            Classes::hclVertexGatherSetupObject(class) => class.deps_indexes(),
            Classes::hclVertexSelectionInput(class) => class.deps_indexes(),
            Classes::hclVolumeConstraint(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintApplyData(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintFrameData(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintMx(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintMxApplyBatchData(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintMxApplySingleData(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintMxFrameBatchData(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintMxFrameSingleData(class) => class.deps_indexes(),
            Classes::hclVolumeConstraintSetupObject(class) => class.deps_indexes(),
            Classes::hkAabb(class) => class.deps_indexes(),
            Classes::hkAabb16(class) => class.deps_indexes(),
            Classes::hkAabbHalf(class) => class.deps_indexes(),
            Classes::hkAabbUint32(class) => class.deps_indexes(),
            Classes::hkaAngularReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaAnimatedReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaAnimation(class) => class.deps_indexes(),
            Classes::hkaAnimationBinding(class) => class.deps_indexes(),
            Classes::hkaAnimationContainer(class) => class.deps_indexes(),
            Classes::hkaAnimationPreviewColorContainer(class) => class.deps_indexes(),
            Classes::hkaAnnotationTrack(class) => class.deps_indexes(),
            Classes::hkaAnnotationTrackAnnotation(class) => class.deps_indexes(),
            Classes::hkaBone(class) => class.deps_indexes(),
            Classes::hkaBoneAttachment(class) => class.deps_indexes(),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaDirectionalReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaFootstepAnalysisInfo(class) => class.deps_indexes(),
            Classes::hkaFootstepAnalysisInfoContainer(class) => class.deps_indexes(),
            Classes::hkaInterleavedUncompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaLosslessCompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaMeshBinding(class) => class.deps_indexes(),
            Classes::hkaMeshBindingMapping(class) => class.deps_indexes(),
            Classes::hkaParameterizedAnimationReferenceFrame(class) => {
                class.deps_indexes()
            }
            Classes::hkaParameterizedReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaPredictiveCompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaPredictiveCompressedAnimationTrackCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaQuantizedAnimation(class) => class.deps_indexes(),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaReferencePoseAnimation(class) => class.deps_indexes(),
            Classes::hkArrayTypeAttribute(class) => class.deps_indexes(),
            Classes::hkaSkeleton(class) => class.deps_indexes(),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapper(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapperData(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapperDataChainMapping(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapperDataPartitionMappingRange(class) => {
                class.deps_indexes()
            }
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => class.deps_indexes(),
            Classes::hkaSkeletonPartition(class) => class.deps_indexes(),
            Classes::hkaSplineCompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkbAiControlCancelPathCommand(class) => class.deps_indexes(),
            Classes::hkbAiControlControlData(class) => class.deps_indexes(),
            Classes::hkbAiControlControlDataBlendable(class) => class.deps_indexes(),
            Classes::hkbAiControlControlDataNonBlendable(class) => class.deps_indexes(),
            Classes::hkbAiControlPathToCommand(class) => class.deps_indexes(),
            Classes::hkbAlignBoneModifier(class) => class.deps_indexes(),
            Classes::hkbAnimatedSkeletonGenerator(class) => class.deps_indexes(),
            Classes::hkBaseObject(class) => class.deps_indexes(),
            Classes::hkbAssetBundle(class) => class.deps_indexes(),
            Classes::hkbAssetBundleStringData(class) => class.deps_indexes(),
            Classes::hkbAttachmentModifier(class) => class.deps_indexes(),
            Classes::hkbAttachmentSetup(class) => class.deps_indexes(),
            Classes::hkbAttributeModifier(class) => class.deps_indexes(),
            Classes::hkbAttributeModifierAssignment(class) => class.deps_indexes(),
            Classes::hkbAuxiliaryNodeInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorEventsInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraph(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphData(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphInternalState(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphStringData(class) => class.deps_indexes(),
            Classes::hkbBehaviorInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.deps_indexes(),
            Classes::hkbBehaviorReferenceGenerator(class) => class.deps_indexes(),
            Classes::hkbBindable(class) => class.deps_indexes(),
            Classes::hkbBlendCurveUtils(class) => class.deps_indexes(),
            Classes::hkbBlenderGenerator(class) => class.deps_indexes(),
            Classes::hkbBlenderGeneratorChild(class) => class.deps_indexes(),
            Classes::hkbBlenderGeneratorChildInternalState(class) => class.deps_indexes(),
            Classes::hkbBlenderGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbBlendingTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbBlendingTransitionEffectInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbBoneIndexArray(class) => class.deps_indexes(),
            Classes::hkbBoneWeightArray(class) => class.deps_indexes(),
            Classes::hkbBoolVariableSequencedData(class) => class.deps_indexes(),
            Classes::hkbBoolVariableSequencedDataSample(class) => class.deps_indexes(),
            Classes::hkbCameraShakeEventPayload(class) => class.deps_indexes(),
            Classes::hkbCameraVariablesChangedCommand(class) => class.deps_indexes(),
            Classes::hkbCharacter(class) => class.deps_indexes(),
            Classes::hkbCharacterAddedInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterControlCommand(class) => class.deps_indexes(),
            Classes::hkbCharacterControllerModifier(class) => class.deps_indexes(),
            Classes::hkbCharacterControllerModifierControlData(class) => {
                class.deps_indexes()
            }
            Classes::hkbCharacterControllerModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbCharacterControllerSetup(class) => class.deps_indexes(),
            Classes::hkbCharacterData(class) => class.deps_indexes(),
            Classes::hkbCharacterInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterSelectedInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterSetup(class) => class.deps_indexes(),
            Classes::hkbCharacterSkinInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterSteppedInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterStringData(class) => class.deps_indexes(),
            Classes::hkbCharacterStringDataFileNameMeshNamePair(class) => {
                class.deps_indexes()
            }
            Classes::hkbClientCharacterState(class) => class.deps_indexes(),
            Classes::hkbClipGenerator(class) => class.deps_indexes(),
            Classes::hkbClipGeneratorEcho(class) => class.deps_indexes(),
            Classes::hkbClipGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbClipTrigger(class) => class.deps_indexes(),
            Classes::hkbClipTriggerArray(class) => class.deps_indexes(),
            Classes::hkbCombineTransformsModifier(class) => class.deps_indexes(),
            Classes::hkbCombineTransformsModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbCompiledExpressionSet(class) => class.deps_indexes(),
            Classes::hkbCompiledExpressionSetToken(class) => class.deps_indexes(),
            Classes::hkbComputeDirectionModifier(class) => class.deps_indexes(),
            Classes::hkbComputeDirectionModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeRotationToTargetModifier(class) => class.deps_indexes(),
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeWorldFromModelModifier(class) => class.deps_indexes(),
            Classes::hkbCondition(class) => class.deps_indexes(),
            Classes::hkbConstraintSetup(class) => class.deps_indexes(),
            Classes::hkbContext(class) => class.deps_indexes(),
            Classes::hkbCustomIdSelector(class) => class.deps_indexes(),
            Classes::hkbCustomTestGenerator(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorAnnotatedTypes(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorBoneTypes(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorComplexTypes(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorHiddenTypes(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorNestedTypes(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorNestedTypesBase(class) => class.deps_indexes(),
            Classes::hkbCustomTestGeneratorSimpleTypes(class) => class.deps_indexes(),
            Classes::hkbDampingModifier(class) => class.deps_indexes(),
            Classes::hkbDampingModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbDelayedModifier(class) => class.deps_indexes(),
            Classes::hkbDelayedModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbDetectCloseToGroundModifier(class) => class.deps_indexes(),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbDockingGenerator(class) => class.deps_indexes(),
            Classes::hkbDockingGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbEvaluateExpressionModifier(class) => class.deps_indexes(),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.deps_indexes()
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbEvaluateHandleModifier(class) => class.deps_indexes(),
            Classes::hkbEvent(class) => class.deps_indexes(),
            Classes::hkbEventBase(class) => class.deps_indexes(),
            Classes::hkbEventDrivenModifier(class) => class.deps_indexes(),
            Classes::hkbEventDrivenModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbEventInfo(class) => class.deps_indexes(),
            Classes::hkbEventPayload(class) => class.deps_indexes(),
            Classes::hkbEventPayloadList(class) => class.deps_indexes(),
            Classes::hkbEventProperty(class) => class.deps_indexes(),
            Classes::hkbEventRaisedInfo(class) => class.deps_indexes(),
            Classes::hkbEventRangeData(class) => class.deps_indexes(),
            Classes::hkbEventRangeDataArray(class) => class.deps_indexes(),
            Classes::hkbEventSequencedData(class) => class.deps_indexes(),
            Classes::hkbEventSequencedDataSequencedEvent(class) => class.deps_indexes(),
            Classes::hkbEventsFromRangeModifier(class) => class.deps_indexes(),
            Classes::hkbEventsFromRangeModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbExpressionCondition(class) => class.deps_indexes(),
            Classes::hkbExpressionData(class) => class.deps_indexes(),
            Classes::hkbExpressionDataArray(class) => class.deps_indexes(),
            Classes::hkbExtractRagdollPoseModifier(class) => class.deps_indexes(),
            Classes::hkbExtrapolatingTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbExtrapolatingTransitionEffectInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbFootIkControlData(class) => class.deps_indexes(),
            Classes::hkbFootIkControlsModifier(class) => class.deps_indexes(),
            Classes::hkbFootIkControlsModifierLeg(class) => class.deps_indexes(),
            Classes::hkbFootIkDriverInfo(class) => class.deps_indexes(),
            Classes::hkbFootIkDriverInfoLeg(class) => class.deps_indexes(),
            Classes::hkbFootIkGains(class) => class.deps_indexes(),
            Classes::hkbFootIkModifier(class) => class.deps_indexes(),
            Classes::hkbFootIkModifierInternalLegData(class) => class.deps_indexes(),
            Classes::hkbFootIkModifierLeg(class) => class.deps_indexes(),
            Classes::hkbGenerator(class) => class.deps_indexes(),
            Classes::hkbGeneratorOutputListener(class) => class.deps_indexes(),
            Classes::hkbGeneratorPartitionInfo(class) => class.deps_indexes(),
            Classes::hkbGeneratorSyncInfo(class) => class.deps_indexes(),
            Classes::hkbGeneratorSyncInfoActiveInterval(class) => class.deps_indexes(),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.deps_indexes(),
            Classes::hkbGeneratorTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbGetHandleOnBoneModifier(class) => class.deps_indexes(),
            Classes::hkbGetUpModifier(class) => class.deps_indexes(),
            Classes::hkbGetUpModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbGetWorldFromModelModifier(class) => class.deps_indexes(),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbGravityModifier(class) => class.deps_indexes(),
            Classes::hkbHandIkControlData(class) => class.deps_indexes(),
            Classes::hkbHandIkControlsModifier(class) => class.deps_indexes(),
            Classes::hkbHandIkControlsModifierHand(class) => class.deps_indexes(),
            Classes::hkbHandIkDriverInfo(class) => class.deps_indexes(),
            Classes::hkbHandIkDriverInfoHand(class) => class.deps_indexes(),
            Classes::hkbHandIkModifier(class) => class.deps_indexes(),
            Classes::hkbHandIkModifierHand(class) => class.deps_indexes(),
            Classes::hkbHandle(class) => class.deps_indexes(),
            Classes::hkbHoldFromBlendingTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbIntEventPayload(class) => class.deps_indexes(),
            Classes::hkbIntVariableSequencedData(class) => class.deps_indexes(),
            Classes::hkbIntVariableSequencedDataSample(class) => class.deps_indexes(),
            Classes::hkBitField(class) => class.deps_indexes(),
            Classes::hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.deps_indexes(),
            Classes::hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.deps_indexes(),
            Classes::hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.deps_indexes(),
            Classes::hkbJigglerGroup(class) => class.deps_indexes(),
            Classes::hkbJigglerModifier(class) => class.deps_indexes(),
            Classes::hkbJigglerModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbKeyframeBonesModifier(class) => class.deps_indexes(),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => class.deps_indexes(),
            Classes::hkbKeyFrameControlData(class) => class.deps_indexes(),
            Classes::hkbLayer(class) => class.deps_indexes(),
            Classes::hkbLayerGenerator(class) => class.deps_indexes(),
            Classes::hkbLayerGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbLayerGeneratorLayerInternalState(class) => class.deps_indexes(),
            Classes::hkbLinkedSymbolInfo(class) => class.deps_indexes(),
            Classes::hkbLookAtModifier(class) => class.deps_indexes(),
            Classes::hkbLookAtModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbManualSelectorGenerator(class) => class.deps_indexes(),
            Classes::hkbManualSelectorGeneratorInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbManualSelectorTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbManualSelectorTransitionEffectInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbMessageLog(class) => class.deps_indexes(),
            Classes::hkbMirroredSkeletonInfo(class) => class.deps_indexes(),
            Classes::hkbMirrorModifier(class) => class.deps_indexes(),
            Classes::hkbModifier(class) => class.deps_indexes(),
            Classes::hkbModifierGenerator(class) => class.deps_indexes(),
            Classes::hkbModifierList(class) => class.deps_indexes(),
            Classes::hkbModifierWrapper(class) => class.deps_indexes(),
            Classes::hkbMoveBoneAttachmentModifier(class) => class.deps_indexes(),
            Classes::hkbMoveCharacterModifier(class) => class.deps_indexes(),
            Classes::hkbMoveCharacterModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbNamedEventPayload(class) => class.deps_indexes(),
            Classes::hkbNamedIntEventPayload(class) => class.deps_indexes(),
            Classes::hkbNamedRealEventPayload(class) => class.deps_indexes(),
            Classes::hkbNamedStringEventPayload(class) => class.deps_indexes(),
            Classes::hkbNode(class) => class.deps_indexes(),
            Classes::hkbNodeInternalStateInfo(class) => class.deps_indexes(),
            Classes::hkbnpPhysicsInterface(class) => class.deps_indexes(),
            Classes::hkbnpRagdollInterface(class) => class.deps_indexes(),
            Classes::hkbNullPhysicsInterface(class) => class.deps_indexes(),
            Classes::hkbParametricMotionGenerator(class) => class.deps_indexes(),
            Classes::hkbParticleSystemEventPayload(class) => class.deps_indexes(),
            Classes::hkbPhysicsInterface(class) => class.deps_indexes(),
            Classes::hkbPinBonesGenerator(class) => class.deps_indexes(),
            Classes::hkbPoseMatchingGenerator(class) => class.deps_indexes(),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbPoseStoringGeneratorOutputListener(class) => class.deps_indexes(),
            Classes::hkbPoseStoringGeneratorOutputListenerStoredPose(class) => {
                class.deps_indexes()
            }
            Classes::hkbPoweredRagdollControlData(class) => class.deps_indexes(),
            Classes::hkbPoweredRagdollControlsModifier(class) => class.deps_indexes(),
            Classes::hkbProceduralBlenderGenerator(class) => class.deps_indexes(),
            Classes::hkbProjectData(class) => class.deps_indexes(),
            Classes::hkbProjectStringData(class) => class.deps_indexes(),
            Classes::hkbRadialSelectorGenerator(class) => class.deps_indexes(),
            Classes::hkbRadialSelectorGeneratorGeneratorInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkbRadialSelectorGeneratorGeneratorPair(class) => {
                class.deps_indexes()
            }
            Classes::hkbRagdollControllerSetup(class) => class.deps_indexes(),
            Classes::hkbRagdollInterface(class) => class.deps_indexes(),
            Classes::hkbRaiseEventCommand(class) => class.deps_indexes(),
            Classes::hkbRealEventPayload(class) => class.deps_indexes(),
            Classes::hkbRealVariableSequencedData(class) => class.deps_indexes(),
            Classes::hkbRealVariableSequencedDataSample(class) => class.deps_indexes(),
            Classes::hkbRebuildScriptsCommand(class) => class.deps_indexes(),
            Classes::hkbReferencedGeneratorSyncInfo(class) => class.deps_indexes(),
            Classes::hkbReferencePoseGenerator(class) => class.deps_indexes(),
            Classes::hkbRigidBodyRagdollControlData(class) => class.deps_indexes(),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => class.deps_indexes(),
            Classes::hkbRigidBodySetup(class) => class.deps_indexes(),
            Classes::hkbRoleAttribute(class) => class.deps_indexes(),
            Classes::hkbRotateCharacterModifier(class) => class.deps_indexes(),
            Classes::hkbRotateCharacterModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbScriptCondition(class) => class.deps_indexes(),
            Classes::hkbScriptGenerator(class) => class.deps_indexes(),
            Classes::hkbSenseHandleModifier(class) => class.deps_indexes(),
            Classes::hkbSenseHandleModifierRange(class) => class.deps_indexes(),
            Classes::hkbSequence(class) => class.deps_indexes(),
            Classes::hkbSequencedData(class) => class.deps_indexes(),
            Classes::hkbSequenceInternalState(class) => class.deps_indexes(),
            Classes::hkbSequenceStringData(class) => class.deps_indexes(),
            Classes::hkbSetBehaviorCommand(class) => class.deps_indexes(),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => class.deps_indexes(),
            Classes::hkbSetNodePropertyCommand(class) => class.deps_indexes(),
            Classes::hkbSetSelectedCharacterCommand(class) => class.deps_indexes(),
            Classes::hkbSetWordVariableCommand(class) => class.deps_indexes(),
            Classes::hkbSetWorldFromModelModifier(class) => class.deps_indexes(),
            Classes::hkbShapeSetup(class) => class.deps_indexes(),
            Classes::hkbSimulationControlCommand(class) => class.deps_indexes(),
            Classes::hkbSimulationStateInfo(class) => class.deps_indexes(),
            Classes::hkbStateChooser(class) => class.deps_indexes(),
            Classes::hkbStateChooserWrapper(class) => class.deps_indexes(),
            Classes::hkbStateDependentModifier(class) => class.deps_indexes(),
            Classes::hkbStateListener(class) => class.deps_indexes(),
            Classes::hkbStateMachine(class) => class.deps_indexes(),
            Classes::hkbStateMachineActiveTransitionInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineDelayedTransitionInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineEventPropertyArray(class) => class.deps_indexes(),
            Classes::hkbStateMachineInternalState(class) => class.deps_indexes(),
            Classes::hkbStateMachineNestedStateMachineData(class) => class.deps_indexes(),
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkbStateMachineStateInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineTimeInterval(class) => class.deps_indexes(),
            Classes::hkbStateMachineTransitionInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineTransitionInfoArray(class) => class.deps_indexes(),
            Classes::hkbStateMachineTransitionInfoReference(class) => {
                class.deps_indexes()
            }
            Classes::hkbStringCondition(class) => class.deps_indexes(),
            Classes::hkbStringEventPayload(class) => class.deps_indexes(),
            Classes::hkbTestIdSelector(class) => class.deps_indexes(),
            Classes::hkbTimerModifier(class) => class.deps_indexes(),
            Classes::hkbTimerModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbToolNodeType(class) => class.deps_indexes(),
            Classes::hkbTransformVectorModifier(class) => class.deps_indexes(),
            Classes::hkbTransformVectorModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbTwistModifier(class) => class.deps_indexes(),
            Classes::hkbVariableBindingSet(class) => class.deps_indexes(),
            Classes::hkbVariableBindingSetBinding(class) => class.deps_indexes(),
            Classes::hkbVariableBounds(class) => class.deps_indexes(),
            Classes::hkbVariableInfo(class) => class.deps_indexes(),
            Classes::hkbVariableValue(class) => class.deps_indexes(),
            Classes::hkbVariableValueSet(class) => class.deps_indexes(),
            Classes::hkbWorldEnums(class) => class.deps_indexes(),
            Classes::hkbWorldFromModelModeData(class) => class.deps_indexes(),
            Classes::hkcdAdf(class) => class.deps_indexes(),
            Classes::hkcdDynamicAabbTree(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeAnisotropicMetric(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodec18(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodec32(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodecInt16(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodecInt16IntAabb(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodecRawUint(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodecRawUlong(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodecRawunsignedint(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeCodecRawunsignedlong(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultTree32Storage(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultTree48Storage(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDefaultTreeInt16Storage(class) => {
                class.deps_indexes()
            }
            Classes::hkcdDynamicTreeDefaultTreePtrStorage(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorage16(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorage32(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStorageInt16(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeDynamicStoragePtr(class) => class.deps_indexes(),
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16(class) => {
                class.deps_indexes()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32(class) => {
                class.deps_indexes()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16(class) => {
                class.deps_indexes()
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr(class) => {
                class.deps_indexes()
            }
            Classes::hkcdFourAabb(class) => class.deps_indexes(),
            Classes::hkcdPlanarCsgOperand(class) => class.deps_indexes(),
            Classes::hkcdPlanarCsgOperandGeomSource(class) => class.deps_indexes(),
            Classes::hkcdPlanarEntity(class) => class.deps_indexes(),
            Classes::hkcdPlanarGeometry(class) => class.deps_indexes(),
            Classes::hkcdPlanarGeometryPlanesCollection(class) => class.deps_indexes(),
            Classes::hkcdPlanarGeometryPolygonCollection(class) => class.deps_indexes(),
            Classes::hkcdPlanarGeometryPrimitivesCollection28(class) => {
                class.deps_indexes()
            }
            Classes::hkcdPlanarGeometryPrimitivesPlane(class) => class.deps_indexes(),
            Classes::hkcdPlanarSolid(class) => class.deps_indexes(),
            Classes::hkcdPlanarSolidNode(class) => class.deps_indexes(),
            Classes::hkcdPlanarSolidNodeStorage(class) => class.deps_indexes(),
            Classes::hkcdShape(class) => class.deps_indexes(),
            Classes::hkcdShapeDispatchType(class) => class.deps_indexes(),
            Classes::hkcdShapeInfoCodecType(class) => class.deps_indexes(),
            Classes::hkcdShapeType(class) => class.deps_indexes(),
            Classes::hkcdSimdTree(class) => class.deps_indexes(),
            Classes::hkcdSimdTreeNode(class) => class.deps_indexes(),
            Classes::hkcdStaticAabbTree(class) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreeBase(class) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreeBasePrimitive(class) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticMeshTreeBaseSection(class) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreeBaseSectionDataRuns(class) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreeBaseSectionPrimitives(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticMeshTreeBaseSectionSharedVertices(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticMeshTreeDefaultDataRun(class) => class.deps_indexes(),
            Classes::hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun(
                class,
            ) => class.deps_indexes(),
            Classes::hkcdStaticPvs(class) => class.deps_indexes(),
            Classes::hkcdStaticPvsBlockHeader(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeCodec3Axis(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeCodec3Axis4(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeCodec3Axis5(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeCodec3Axis6(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeCodecRaw(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDefaultTreeStorage32(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDefaultTreeStorage4(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDefaultTreeStorage5(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDefaultTreeStorage6(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDynamicStorage32(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDynamicStorage4(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDynamicStorage5(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDynamicStorage6(class) => class.deps_indexes(),
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5(class) => {
                class.deps_indexes()
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6(class) => {
                class.deps_indexes()
            }
            Classes::hkClass(class) => class.deps_indexes(),
            Classes::hkClassEnum(class) => class.deps_indexes(),
            Classes::hkClassEnumItem(class) => class.deps_indexes(),
            Classes::hkClassMember(class) => class.deps_indexes(),
            Classes::hkCompressedMassProperties(class) => class.deps_indexes(),
            Classes::hkContactPoint(class) => class.deps_indexes(),
            Classes::hkContactPointMaterial(class) => class.deps_indexes(),
            Classes::hkCustomAttributes(class) => class.deps_indexes(),
            Classes::hkCustomAttributesAttribute(class) => class.deps_indexes(),
            Classes::hkDataObjectTypeAttribute(class) => class.deps_indexes(),
            Classes::hkDefaultMemoryTracker(class) => class.deps_indexes(),
            Classes::hkDescriptionAttribute(class) => class.deps_indexes(),
            Classes::hkDocumentationAttribute(class) => class.deps_indexes(),
            Classes::hkFloat16(class) => class.deps_indexes(),
            Classes::hkFloat16Transform(class) => class.deps_indexes(),
            Classes::hkFourTransposedPointsf(class) => class.deps_indexes(),
            Classes::hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations(
                class,
            ) => class.deps_indexes(),
            Classes::hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations(
                class,
            ) => class.deps_indexes(),
            Classes::hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance(
                class,
            ) => class.deps_indexes(),
            Classes::hkGeometry(class) => class.deps_indexes(),
            Classes::hkGeometryTriangle(class) => class.deps_indexes(),
            Classes::hkGizmoAttribute(class) => class.deps_indexes(),
            Classes::hkIndexedTransformSet(class) => class.deps_indexes(),
            Classes::hkIntRealPair(class) => class.deps_indexes(),
            Classes::hkLinkAttribute(class) => class.deps_indexes(),
            Classes::hkLocalFrame(class) => class.deps_indexes(),
            Classes::hkLocalFrameGroup(class) => class.deps_indexes(),
            Classes::hkMassProperties(class) => class.deps_indexes(),
            Classes::hkMemoryMeshBody(class) => class.deps_indexes(),
            Classes::hkMemoryMeshMaterial(class) => class.deps_indexes(),
            Classes::hkMemoryMeshShape(class) => class.deps_indexes(),
            Classes::hkMemoryMeshShapeSection(class) => class.deps_indexes(),
            Classes::hkMemoryMeshTexture(class) => class.deps_indexes(),
            Classes::hkMemoryMeshVertexBuffer(class) => class.deps_indexes(),
            Classes::hkMemoryResourceContainer(class) => class.deps_indexes(),
            Classes::hkMemoryResourceHandle(class) => class.deps_indexes(),
            Classes::hkMemoryResourceHandleExternalLink(class) => class.deps_indexes(),
            Classes::hkMeshBody(class) => class.deps_indexes(),
            Classes::hkMeshBoneIndexMapping(class) => class.deps_indexes(),
            Classes::hkMeshMaterial(class) => class.deps_indexes(),
            Classes::hkMeshSection(class) => class.deps_indexes(),
            Classes::hkMeshSectionCinfo(class) => class.deps_indexes(),
            Classes::hkMeshShape(class) => class.deps_indexes(),
            Classes::hkMeshTexture(class) => class.deps_indexes(),
            Classes::hkMeshTextureRawBufferDescriptor(class) => class.deps_indexes(),
            Classes::hkMeshVertexBuffer(class) => class.deps_indexes(),
            Classes::hkModelerNodeTypeAttribute(class) => class.deps_indexes(),
            Classes::hkMonitorStreamColorTable(class) => class.deps_indexes(),
            Classes::hkMonitorStreamColorTableColorPair(class) => class.deps_indexes(),
            Classes::hkMonitorStreamFrameInfo(class) => class.deps_indexes(),
            Classes::hkMonitorStreamStringMap(class) => class.deps_indexes(),
            Classes::hkMonitorStreamStringMapStringMap(class) => class.deps_indexes(),
            Classes::hkMotionState(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBuffer(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBufferElementInfo(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBufferLockedElement(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkMultiThreadCheck(class) => class.deps_indexes(),
            Classes::hknpAction(class) => class.deps_indexes(),
            Classes::hknpArrayAction(class) => class.deps_indexes(),
            Classes::hknpBallGun(class) => class.deps_indexes(),
            Classes::hknpBinaryAction(class) => class.deps_indexes(),
            Classes::hknpBody(class) => class.deps_indexes(),
            Classes::hknpBodyCinfo(class) => class.deps_indexes(),
            Classes::hknpBodyQuality(class) => class.deps_indexes(),
            Classes::hknpBodyQualityLibrary(class) => class.deps_indexes(),
            Classes::hknpBodyReference(class) => class.deps_indexes(),
            Classes::hknpBreakableConstraintData(class) => class.deps_indexes(),
            Classes::hknpBridgeConstraintAtom(class) => class.deps_indexes(),
            Classes::hknpBroadPhaseConfig(class) => class.deps_indexes(),
            Classes::hknpBroadPhaseConfigLayer(class) => class.deps_indexes(),
            Classes::hknpBSMaterial(class) => class.deps_indexes(),
            Classes::hknpBSMaterialProperties(class) => class.deps_indexes(),
            Classes::hknpCapsuleShape(class) => class.deps_indexes(),
            Classes::hknpCharacterProxyCinfo(class) => class.deps_indexes(),
            Classes::hknpCharacterRigidBodyCinfo(class) => class.deps_indexes(),
            Classes::hknpCircularSurfaceVelocity(class) => class.deps_indexes(),
            Classes::hknpCollisionDispatchType(class) => class.deps_indexes(),
            Classes::hknpCollisionFilter(class) => class.deps_indexes(),
            Classes::hknpCompositeShape(class) => class.deps_indexes(),
            Classes::hknpCompoundShape(class) => class.deps_indexes(),
            Classes::hknpCompoundShapeInternalsKeyMask(class) => class.deps_indexes(),
            Classes::hknpCompoundShapeKeyMask(class) => class.deps_indexes(),
            Classes::hknpCompressedHeightFieldShape(class) => class.deps_indexes(),
            Classes::hknpCompressedMeshShape(class) => class.deps_indexes(),
            Classes::hknpCompressedMeshShapeData(class) => class.deps_indexes(),
            Classes::hknpCompressedMeshShapeInternalsKeyMask(class) => {
                class.deps_indexes()
            }
            Classes::hknpCompressedMeshShapeTree(class) => class.deps_indexes(),
            Classes::hknpCompressedMeshShapeTreeDataRun(class) => class.deps_indexes(),
            Classes::hknpCompressedMeshShapeTreeDataRunData(class) => {
                class.deps_indexes()
            }
            Classes::hknpConstraint(class) => class.deps_indexes(),
            Classes::hknpConstraintCinfo(class) => class.deps_indexes(),
            Classes::hknpConstraintCollisionFilter(class) => class.deps_indexes(),
            Classes::hknpConvexPolytopeShape(class) => class.deps_indexes(),
            Classes::hknpConvexPolytopeShapeFace(class) => class.deps_indexes(),
            Classes::hknpConvexShape(class) => class.deps_indexes(),
            Classes::hknpDecoratorShape(class) => class.deps_indexes(),
            Classes::hknpDefaultBroadPhaseConfig(class) => class.deps_indexes(),
            Classes::hknpDefaultExternMeshShapeGeometry(class) => class.deps_indexes(),
            Classes::hknpDestructionShapeProperties(class) => class.deps_indexes(),
            Classes::hknpDisableCollisionFilter(class) => class.deps_indexes(),
            Classes::hknpDummyShape(class) => class.deps_indexes(),
            Classes::hknpDynamicCompoundShape(class) => class.deps_indexes(),
            Classes::hknpDynamicCompoundShapeData(class) => class.deps_indexes(),
            Classes::hknpDynamicCompoundShapeTree(class) => class.deps_indexes(),
            Classes::hknpExternMeshShape(class) => class.deps_indexes(),
            Classes::hknpExternMeshShapeData(class) => class.deps_indexes(),
            Classes::hknpExternMeshShapeGeometry(class) => class.deps_indexes(),
            Classes::hknpFirstPersonGun(class) => class.deps_indexes(),
            Classes::hknpGroupCollisionFilter(class) => class.deps_indexes(),
            Classes::hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516(
                class,
            ) => class.deps_indexes(),
            Classes::hknpHeightFieldShape(class) => class.deps_indexes(),
            Classes::hknpLinearSurfaceVelocity(class) => class.deps_indexes(),
            Classes::hknpLodShape(class) => class.deps_indexes(),
            Classes::hknpLodShapeLevelOfDetailInfo(class) => class.deps_indexes(),
            Classes::hknpMalleableConstraintData(class) => class.deps_indexes(),
            Classes::hknpMaskedShape(class) => class.deps_indexes(),
            Classes::hknpMaskedShapeMaskWrapper(class) => class.deps_indexes(),
            Classes::hknpMaterial(class) => class.deps_indexes(),
            Classes::hknpMaterialDescriptor(class) => class.deps_indexes(),
            Classes::hknpMaterialLibrary(class) => class.deps_indexes(),
            Classes::hknpMaterialPalette(class) => class.deps_indexes(),
            Classes::hknpMinMaxQuadTree(class) => class.deps_indexes(),
            Classes::hknpMinMaxQuadTreeMinMaxLevel(class) => class.deps_indexes(),
            Classes::hknpMotion(class) => class.deps_indexes(),
            Classes::hknpMotionCinfo(class) => class.deps_indexes(),
            Classes::hknpMotionProperties(class) => class.deps_indexes(),
            Classes::hknpMotionPropertiesLibrary(class) => class.deps_indexes(),
            Classes::hknpPairCollisionFilter(class) => class.deps_indexes(),
            Classes::hknpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.deps_indexes()
            }
            Classes::hknpPhysicsSceneData(class) => class.deps_indexes(),
            Classes::hknpPhysicsSystemData(class) => class.deps_indexes(),
            Classes::hknpProjectileGun(class) => class.deps_indexes(),
            Classes::hknpRagdollData(class) => class.deps_indexes(),
            Classes::hknpRagdollKeyFrameHierarchyUtilityControlData(class) => {
                class.deps_indexes()
            }
            Classes::hknpRefMaterial(class) => class.deps_indexes(),
            Classes::hknpRefWorldCinfo(class) => class.deps_indexes(),
            Classes::hknpScaledConvexShape(class) => class.deps_indexes(),
            Classes::hknpScaledConvexShapeBase(class) => class.deps_indexes(),
            Classes::hknpShape(class) => class.deps_indexes(),
            Classes::hknpShapeInstance(class) => class.deps_indexes(),
            Classes::hknpShapeKeyMask(class) => class.deps_indexes(),
            Classes::hknpShapeMassProperties(class) => class.deps_indexes(),
            Classes::hknpShapeSignals(class) => class.deps_indexes(),
            Classes::hknpShapeTagCodec(class) => class.deps_indexes(),
            Classes::hknpShapeType(class) => class.deps_indexes(),
            Classes::hknpSparseCompactMapunsignedshort(class) => class.deps_indexes(),
            Classes::hknpSphereShape(class) => class.deps_indexes(),
            Classes::hknpStaticCompoundShape(class) => class.deps_indexes(),
            Classes::hknpStaticCompoundShapeData(class) => class.deps_indexes(),
            Classes::hknpStaticCompoundShapeInternals(class) => class.deps_indexes(),
            Classes::hknpStaticCompoundShapeKeyMask(class) => class.deps_indexes(),
            Classes::hknpStaticCompoundShapeTree(class) => class.deps_indexes(),
            Classes::hknpSurfaceVelocity(class) => class.deps_indexes(),
            Classes::hknpTriangleShape(class) => class.deps_indexes(),
            Classes::hknpTyremarkPoint(class) => class.deps_indexes(),
            Classes::hknpTyremarksInfo(class) => class.deps_indexes(),
            Classes::hknpTyremarksWheel(class) => class.deps_indexes(),
            Classes::hknpUnaryAction(class) => class.deps_indexes(),
            Classes::hknpVehicleAerodynamics(class) => class.deps_indexes(),
            Classes::hknpVehicleBrake(class) => class.deps_indexes(),
            Classes::hknpVehicleData(class) => class.deps_indexes(),
            Classes::hknpVehicleDataWheelComponentParams(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultAerodynamics(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultAnalogDriverInput(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultBrake(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultBrakeWheelBrakingProperties(class) => {
                class.deps_indexes()
            }
            Classes::hknpVehicleDefaultEngine(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultSteering(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultSuspension(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.deps_indexes(),
            Classes::hknpVehicleDefaultTransmission(class) => class.deps_indexes(),
            Classes::hknpVehicleDefaultVelocityDamper(class) => class.deps_indexes(),
            Classes::hknpVehicleDriverInput(class) => class.deps_indexes(),
            Classes::hknpVehicleDriverInputAnalogStatus(class) => class.deps_indexes(),
            Classes::hknpVehicleDriverInputStatus(class) => class.deps_indexes(),
            Classes::hknpVehicleEngine(class) => class.deps_indexes(),
            Classes::hknpVehicleInstance(class) => class.deps_indexes(),
            Classes::hknpVehicleInstanceWheelInfo(class) => class.deps_indexes(),
            Classes::hknpVehicleLinearCastWheelCollide(class) => class.deps_indexes(),
            Classes::hknpVehicleLinearCastWheelCollideWheelState(class) => {
                class.deps_indexes()
            }
            Classes::hknpVehicleRayCastWheelCollide(class) => class.deps_indexes(),
            Classes::hknpVehicleSteering(class) => class.deps_indexes(),
            Classes::hknpVehicleSuspension(class) => class.deps_indexes(),
            Classes::hknpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.deps_indexes()
            }
            Classes::hknpVehicleTransmission(class) => class.deps_indexes(),
            Classes::hknpVehicleVelocityDamper(class) => class.deps_indexes(),
            Classes::hknpVehicleWheelCollide(class) => class.deps_indexes(),
            Classes::hknpWorldCinfo(class) => class.deps_indexes(),
            Classes::hknpWorldSnapshot(class) => class.deps_indexes(),
            Classes::hkOffsetBitField(class) => class.deps_indexes(),
            Classes::hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.deps_indexes(),
            Classes::hkp2dAngConstraintAtom(class) => class.deps_indexes(),
            Classes::hkp3dAngConstraintAtom(class) => class.deps_indexes(),
            Classes::hkPackedVector3(class) => class.deps_indexes(),
            Classes::hkPackedVector8_3(class) => class.deps_indexes(),
            Classes::hkPackfileHeader(class) => class.deps_indexes(),
            Classes::hkPackfileSectionHeader(class) => class.deps_indexes(),
            Classes::hkpAngConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngFrictionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngMotorConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpBallAndSocketConstraintData(class) => class.deps_indexes(),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpBallSocketConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpBridgeAtoms(class) => class.deps_indexes(),
            Classes::hkpBridgeConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpCallbackConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpCogWheelConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpCogWheelConstraintData(class) => class.deps_indexes(),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpConeLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpConstraintData(class) => class.deps_indexes(),
            Classes::hkpConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpDeformableAngConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpDeformableFixedConstraintData(class) => class.deps_indexes(),
            Classes::hkpDeformableFixedConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpDeformableLinConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpFixedConstraintData(class) => class.deps_indexes(),
            Classes::hkpFixedConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpHingeConstraintData(class) => class.deps_indexes(),
            Classes::hkpHingeConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpHingeLimitsData(class) => class.deps_indexes(),
            Classes::hkpHingeLimitsDataAtoms(class) => class.deps_indexes(),
            Classes::hkpLimitedForceConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpLimitedHingeConstraintData(class) => class.deps_indexes(),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpLinConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinearClearanceConstraintData(class) => class.deps_indexes(),
            Classes::hkpLinearClearanceConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpLinearParametricCurve(class) => class.deps_indexes(),
            Classes::hkpLinFrictionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinMotorConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinSoftConstraintAtom(class) => class.deps_indexes(),
            Classes::hkPostFinishAttribute(class) => class.deps_indexes(),
            Classes::hkpOverwritePivotConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpParametricCurve(class) => class.deps_indexes(),
            Classes::hkpPointToPathConstraintData(class) => class.deps_indexes(),
            Classes::hkpPointToPlaneConstraintData(class) => class.deps_indexes(),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpPositionConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpPrismaticConstraintData(class) => class.deps_indexes(),
            Classes::hkpPrismaticConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpPulleyConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpPulleyConstraintData(class) => class.deps_indexes(),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpRackAndPinionConstraintData(class) => class.deps_indexes(),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRagdollConstraintData(class) => class.deps_indexes(),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRagdollLimitsData(class) => class.deps_indexes(),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpRotationalConstraintData(class) => class.deps_indexes(),
            Classes::hkpRotationalConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSetLocalTransformsConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSetupStabilizationAtom(class) => class.deps_indexes(),
            Classes::hkpSpringDamperConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpStiffSpringConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpStiffSpringConstraintData(class) => class.deps_indexes(),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpTwistLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpVehicleFrictionDescription(class) => class.deps_indexes(),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => {
                class.deps_indexes()
            }
            Classes::hkpVehicleFrictionStatus(class) => class.deps_indexes(),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => class.deps_indexes(),
            Classes::hkpVelocityConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpWheelConstraintData(class) => class.deps_indexes(),
            Classes::hkpWheelConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpWheelFrictionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpWheelFrictionConstraintAtomAxle(class) => class.deps_indexes(),
            Classes::hkpWheelFrictionConstraintData(class) => class.deps_indexes(),
            Classes::hkpWheelFrictionConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpWrappedConstraintData(class) => class.deps_indexes(),
            Classes::hkQTransform(class) => class.deps_indexes(),
            Classes::hkQTransformf(class) => class.deps_indexes(),
            Classes::hkRangeInt32Attribute(class) => class.deps_indexes(),
            Classes::hkRangeRealAttribute(class) => class.deps_indexes(),
            Classes::hkRefCountedProperties(class) => class.deps_indexes(),
            Classes::hkRefCountedPropertiesEntry(class) => class.deps_indexes(),
            Classes::hkReferencedObject(class) => class.deps_indexes(),
            Classes::hkResourceBase(class) => class.deps_indexes(),
            Classes::hkResourceContainer(class) => class.deps_indexes(),
            Classes::hkResourceHandle(class) => class.deps_indexes(),
            Classes::hkRootLevelContainer(class) => class.deps_indexes(),
            Classes::hkRootLevelContainerNamedVariant(class) => class.deps_indexes(),
            Classes::hkSemanticsAttribute(class) => class.deps_indexes(),
            Classes::hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair(
                class,
            ) => class.deps_indexes(),
            Classes::hkSetIntFloatPair(class) => class.deps_indexes(),
            Classes::hkSetUint32(class) => class.deps_indexes(),
            Classes::hkSetUint64(class) => class.deps_indexes(),
            Classes::hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint(
                class,
            ) => class.deps_indexes(),
            Classes::hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong(
                class,
            ) => class.deps_indexes(),
            Classes::hkSimpleLocalFrame(class) => class.deps_indexes(),
            Classes::hkSimpleProperty(class) => class.deps_indexes(),
            Classes::hkSimplePropertyValue(class) => class.deps_indexes(),
            Classes::hkSkinBinding(class) => class.deps_indexes(),
            Classes::hkSkinnedMeshShape(class) => class.deps_indexes(),
            Classes::hkSkinnedMeshShapeBoneSection(class) => class.deps_indexes(),
            Classes::hkSkinnedMeshShapeBoneSet(class) => class.deps_indexes(),
            Classes::hkSkinnedMeshShapePart(class) => class.deps_indexes(),
            Classes::hkSkinnedRefMeshShape(class) => class.deps_indexes(),
            Classes::hkSphere(class) => class.deps_indexes(),
            Classes::hkStorageSkinnedMeshShape(class) => class.deps_indexes(),
            Classes::hkStringObject(class) => class.deps_indexes(),
            Classes::hkSweptTransformf(class) => class.deps_indexes(),
            Classes::hkTraceStreamTitle(class) => class.deps_indexes(),
            Classes::hkUFloat8(class) => class.deps_indexes(),
            Classes::hkUiAttribute(class) => class.deps_indexes(),
            Classes::hkUuid(class) => class.deps_indexes(),
            Classes::hkVariableTweakingHelper(class) => class.deps_indexes(),
            Classes::hkVariableTweakingHelperBoolVariableInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkVariableTweakingHelperIntVariableInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkVariableTweakingHelperRealVariableInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkVariableTweakingHelperVector4VariableInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkVertexFormat(class) => class.deps_indexes(),
            Classes::hkVertexFormatElement(class) => class.deps_indexes(),
            Classes::hkxAnimatedFloat(class) => class.deps_indexes(),
            Classes::hkxAnimatedMatrix(class) => class.deps_indexes(),
            Classes::hkxAnimatedQuaternion(class) => class.deps_indexes(),
            Classes::hkxAnimatedVector(class) => class.deps_indexes(),
            Classes::hkxAttribute(class) => class.deps_indexes(),
            Classes::hkxAttributeGroup(class) => class.deps_indexes(),
            Classes::hkxAttributeHolder(class) => class.deps_indexes(),
            Classes::hkxEnum(class) => class.deps_indexes(),
            Classes::hkxEnumItem(class) => class.deps_indexes(),
            Classes::hkxIndexBuffer(class) => class.deps_indexes(),
            Classes::hkxMaterial(class) => class.deps_indexes(),
            Classes::hkxMaterialProperty(class) => class.deps_indexes(),
            Classes::hkxMaterialTextureStage(class) => class.deps_indexes(),
            Classes::hkxMesh(class) => class.deps_indexes(),
            Classes::hkxMeshSection(class) => class.deps_indexes(),
            Classes::hkxMeshUserChannelInfo(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedBool(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedEnum(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedInt(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedString(class) => class.deps_indexes(),
            Classes::hkxVertexAnimation(class) => class.deps_indexes(),
            Classes::hkxVertexAnimationUsageMap(class) => class.deps_indexes(),
            Classes::hkxVertexBuffer(class) => class.deps_indexes(),
            Classes::hkxVertexBufferVertexData(class) => class.deps_indexes(),
            Classes::hkxVertexDescription(class) => class.deps_indexes(),
            Classes::hkxVertexDescriptionElementDecl(class) => class.deps_indexes(),
        }
    }
}
impl<'a> _serde::Serialize for Classes<'a> {
    fn serialize<S: _serde::ser::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            Classes::BGSGamebryoSequenceGenerator(class) => class.serialize(serializer),
            Classes::BSAlignBoneModifier(class) => class.serialize(serializer),
            Classes::BSAssignVariablesModifier(class) => class.serialize(serializer),
            Classes::BSBehaviorGraphSwapGenerator(class) => class.serialize(serializer),
            Classes::BSBoneSwitchGenerator(class) => class.serialize(serializer),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.serialize(serializer),
            Classes::BSComputeAddBoneAnimModifier(class) => class.serialize(serializer),
            Classes::BSCyclicBlendTransitionGenerator(class) => {
                class.serialize(serializer)
            }
            Classes::BSDecomposeVectorModifier(class) => class.serialize(serializer),
            Classes::BSDirectAtCapturePoseModifier(class) => class.serialize(serializer),
            Classes::BSDirectAtModifier(class) => class.serialize(serializer),
            Classes::BSDistTriggerModifier(class) => class.serialize(serializer),
            Classes::BSEventEveryNEventsModifier(class) => class.serialize(serializer),
            Classes::BSEventOnDeactivateModifier(class) => class.serialize(serializer),
            Classes::BSEventOnFalseToTrueModifier(class) => class.serialize(serializer),
            Classes::BSGetTimeStepModifier(class) => class.serialize(serializer),
            Classes::BSInterpValueModifier(class) => class.serialize(serializer),
            Classes::BSIsActiveModifier(class) => class.serialize(serializer),
            Classes::BSIStateManagerModifier(class) => class.serialize(serializer),
            Classes::BSIStateManagerModifierBSiStateData(class) => {
                class.serialize(serializer)
            }
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.serialize(serializer)
            }
            Classes::BSiStateTaggingGenerator(class) => class.serialize(serializer),
            Classes::BSiStateTaggingStateListener(class) => class.serialize(serializer),
            Classes::BSLimbCycleModifier(class) => class.serialize(serializer),
            Classes::BSLimbIKModifier(class) => class.serialize(serializer),
            Classes::BSLookAtCapturePoseModifier(class) => class.serialize(serializer),
            Classes::BSLookAtModifier(class) => class.serialize(serializer),
            Classes::BSLookAtModifierBoneData(class) => class.serialize(serializer),
            Classes::BSModifyOnceModifier(class) => class.serialize(serializer),
            Classes::BSOffsetAnimationGenerator(class) => class.serialize(serializer),
            Classes::BSPassByTargetTriggerModifier(class) => class.serialize(serializer),
            Classes::BSRagdollContactListenerModifier(class) => {
                class.serialize(serializer)
            }
            Classes::BSRandomAlarmModifier(class) => class.serialize(serializer),
            Classes::BSRetargetBodyBlend(class) => class.serialize(serializer),
            Classes::BSReverseSpineTwistModifier(class) => class.serialize(serializer),
            Classes::BSRootTwistModifier(class) => class.serialize(serializer),
            Classes::BSSpeedSamplerModifier(class) => class.serialize(serializer),
            Classes::BSSpringState(class) => class.serialize(serializer),
            Classes::BSTimerModifier(class) => class.serialize(serializer),
            Classes::BSTweenerModifier(class) => class.serialize(serializer),
            Classes::DynamicAnimationTaggingGenerator(class) => {
                class.serialize(serializer)
            }
            Classes::hclAction(class) => class.serialize(serializer),
            Classes::hclAntiPinchConstraintSet(class) => class.serialize(serializer),
            Classes::hclAntiPinchConstraintSetPerParticle(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendLinkConstraintSet(class) => class.serialize(serializer),
            Classes::hclBendLinkConstraintSetLink(class) => class.serialize(serializer),
            Classes::hclBendLinkConstraintSetMx(class) => class.serialize(serializer),
            Classes::hclBendLinkConstraintSetMxBatch(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendLinkConstraintSetMxSingle(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendStiffnessConstraintSet(class) => class.serialize(serializer),
            Classes::hclBendStiffnessConstraintSetLink(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendStiffnessConstraintSetMx(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendStiffnessConstraintSetMxBatch(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendStiffnessConstraintSetMxSingle(class) => {
                class.serialize(serializer)
            }
            Classes::hclBendStiffnessSetupObject(class) => class.serialize(serializer),
            Classes::hclBlendSomeVerticesOperator(class) => class.serialize(serializer),
            Classes::hclBlendSomeVerticesOperatorBlendEntry(class) => {
                class.serialize(serializer)
            }
            Classes::hclBonePlanesConstraintSet(class) => class.serialize(serializer),
            Classes::hclBonePlanesConstraintSetBonePlane(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformer(class) => class.serialize(serializer),
            Classes::hclBoneSpaceDeformerFourBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockP(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockPN(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockPNT(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockPNTB(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedP(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPN(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNT(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNTB(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerOneBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerThreeBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceDeformerTwoBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceMeshMeshDeformOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceMeshMeshDeformPNOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceMeshMeshDeformPNTBOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceMeshMeshDeformPNTOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceMeshMeshDeformPOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclBoneSpaceSkinOperator(class) => class.serialize(serializer),
            Classes::hclBoneSpaceSkinPNOperator(class) => class.serialize(serializer),
            Classes::hclBoneSpaceSkinPNTBOperator(class) => class.serialize(serializer),
            Classes::hclBoneSpaceSkinPNTOperator(class) => class.serialize(serializer),
            Classes::hclBoneSpaceSkinPOperator(class) => class.serialize(serializer),
            Classes::hclBufferDefinition(class) => class.serialize(serializer),
            Classes::hclBufferLayout(class) => class.serialize(serializer),
            Classes::hclBufferLayoutBufferElement(class) => class.serialize(serializer),
            Classes::hclBufferLayoutSlot(class) => class.serialize(serializer),
            Classes::hclBufferSetupObject(class) => class.serialize(serializer),
            Classes::hclBufferUsage(class) => class.serialize(serializer),
            Classes::hclCapsuleShape(class) => class.serialize(serializer),
            Classes::hclClothContainer(class) => class.serialize(serializer),
            Classes::hclClothData(class) => class.serialize(serializer),
            Classes::hclClothSetupContainer(class) => class.serialize(serializer),
            Classes::hclClothSetupObject(class) => class.serialize(serializer),
            Classes::hclClothState(class) => class.serialize(serializer),
            Classes::hclClothStateBufferAccess(class) => class.serialize(serializer),
            Classes::hclClothStateSetupObject(class) => class.serialize(serializer),
            Classes::hclClothStateTransformSetAccess(class) => {
                class.serialize(serializer)
            }
            Classes::hclCollidable(class) => class.serialize(serializer),
            Classes::hclCompressibleLinkConstraintSet(class) => {
                class.serialize(serializer)
            }
            Classes::hclCompressibleLinkConstraintSetLink(class) => {
                class.serialize(serializer)
            }
            Classes::hclCompressibleLinkConstraintSetMx(class) => {
                class.serialize(serializer)
            }
            Classes::hclCompressibleLinkConstraintSetMxBatch(class) => {
                class.serialize(serializer)
            }
            Classes::hclCompressibleLinkConstraintSetMxSingle(class) => {
                class.serialize(serializer)
            }
            Classes::hclConstraintSet(class) => class.serialize(serializer),
            Classes::hclConstraintSetSetupObject(class) => class.serialize(serializer),
            Classes::hclConvexGeometryShape(class) => class.serialize(serializer),
            Classes::hclConvexHeightFieldShape(class) => class.serialize(serializer),
            Classes::hclConvexPlanesShape(class) => class.serialize(serializer),
            Classes::hclCopyVerticesOperator(class) => class.serialize(serializer),
            Classes::hclEdgeSelectionInput(class) => class.serialize(serializer),
            Classes::hclGatherAllVerticesOperator(class) => class.serialize(serializer),
            Classes::hclGatherSomeVerticesOperator(class) => class.serialize(serializer),
            Classes::hclGatherSomeVerticesOperatorVertexPair(class) => {
                class.serialize(serializer)
            }
            Classes::hclInputConvertOperator(class) => class.serialize(serializer),
            Classes::hclLocalRangeConstraintSet(class) => class.serialize(serializer),
            Classes::hclLocalRangeConstraintSetLocalConstraint(class) => {
                class.serialize(serializer)
            }
            Classes::hclLocalRangeSetupObject(class) => class.serialize(serializer),
            Classes::hclMeshBoneDeformOperator(class) => class.serialize(serializer),
            Classes::hclMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.serialize(serializer)
            }
            Classes::hclMeshBoneDeformSetupObject(class) => class.serialize(serializer),
            Classes::hclMeshMeshDeformOperator(class) => class.serialize(serializer),
            Classes::hclMeshMeshDeformOperatorTriangleVertexPair(class) => {
                class.serialize(serializer)
            }
            Classes::hclMoveFixedParticlesSetupObject(class) => {
                class.serialize(serializer)
            }
            Classes::hclMoveParticlesOperator(class) => class.serialize(serializer),
            Classes::hclMoveParticlesOperatorVertexParticlePair(class) => {
                class.serialize(serializer)
            }
            Classes::hclNamedSetupMesh(class) => class.serialize(serializer),
            Classes::hclNamedTransformSetSetupObject(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformer(class) => class.serialize(serializer),
            Classes::hclObjectSpaceDeformerFourBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockP(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockPN(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockPNT(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockPNTB(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedP(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPN(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNT(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNTB(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerOneBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerThreeBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceDeformerTwoBlendEntryBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceMeshMeshDeformOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceMeshMeshDeformPNOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceMeshMeshDeformPNTBOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceMeshMeshDeformPNTOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceMeshMeshDeformPOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclObjectSpaceSkinOperator(class) => class.serialize(serializer),
            Classes::hclObjectSpaceSkinPNOperator(class) => class.serialize(serializer),
            Classes::hclObjectSpaceSkinPNTBOperator(class) => class.serialize(serializer),
            Classes::hclObjectSpaceSkinPNTOperator(class) => class.serialize(serializer),
            Classes::hclObjectSpaceSkinPOperator(class) => class.serialize(serializer),
            Classes::hclOperator(class) => class.serialize(serializer),
            Classes::hclOperatorSetupObject(class) => class.serialize(serializer),
            Classes::hclOutputConvertOperator(class) => class.serialize(serializer),
            Classes::hclPlaneShape(class) => class.serialize(serializer),
            Classes::hclRuntimeConversionInfo(class) => class.serialize(serializer),
            Classes::hclRuntimeConversionInfoElementConversion(class) => {
                class.serialize(serializer)
            }
            Classes::hclRuntimeConversionInfoSlotConversion(class) => {
                class.serialize(serializer)
            }
            Classes::hclScratchBufferDefinition(class) => class.serialize(serializer),
            Classes::hclScratchBufferSetupObject(class) => class.serialize(serializer),
            Classes::hclSetupMesh(class) => class.serialize(serializer),
            Classes::hclShadowBufferDefinition(class) => class.serialize(serializer),
            Classes::hclShape(class) => class.serialize(serializer),
            Classes::hclSimClothBufferSetupObject(class) => class.serialize(serializer),
            Classes::hclSimClothData(class) => class.serialize(serializer),
            Classes::hclSimClothDataCollidablePinchingData(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimClothDataCollidableTransformMap(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimClothDataLandscapeCollisionData(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimClothDataOverridableSimulationInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimClothDataParticleData(class) => class.serialize(serializer),
            Classes::hclSimClothDataTransferMotionData(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimClothPose(class) => class.serialize(serializer),
            Classes::hclSimClothSetupObject(class) => class.serialize(serializer),
            Classes::hclSimClothSetupObjectPerInstanceCollidable(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimClothSetupObjectTransferMotionSetupData(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimpleMeshBoneDeformOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimpleMeshBoneDeformOperatorTriangleBonePair(class) => {
                class.serialize(serializer)
            }
            Classes::hclSimpleWindAction(class) => class.serialize(serializer),
            Classes::hclSimulateOperator(class) => class.serialize(serializer),
            Classes::hclSimulateSetupObject(class) => class.serialize(serializer),
            Classes::hclSimulationSetupMesh(class) => class.serialize(serializer),
            Classes::hclSimulationSetupMeshMapOptions(class) => {
                class.serialize(serializer)
            }
            Classes::hclSkinOperator(class) => class.serialize(serializer),
            Classes::hclSkinOperatorBoneInfluence(class) => class.serialize(serializer),
            Classes::hclSkinSetupObject(class) => class.serialize(serializer),
            Classes::hclSphereShape(class) => class.serialize(serializer),
            Classes::hclStandardLinkConstraintSet(class) => class.serialize(serializer),
            Classes::hclStandardLinkConstraintSetLink(class) => {
                class.serialize(serializer)
            }
            Classes::hclStandardLinkConstraintSetMx(class) => class.serialize(serializer),
            Classes::hclStandardLinkConstraintSetMxBatch(class) => {
                class.serialize(serializer)
            }
            Classes::hclStandardLinkConstraintSetMxSingle(class) => {
                class.serialize(serializer)
            }
            Classes::hclStandardLinkSetupObject(class) => class.serialize(serializer),
            Classes::hclStaticDisplayBufferSetupObject(class) => {
                class.serialize(serializer)
            }
            Classes::hclStaticShadowBufferDefinition(class) => {
                class.serialize(serializer)
            }
            Classes::hclStretchLinkConstraintSet(class) => class.serialize(serializer),
            Classes::hclStretchLinkConstraintSetLink(class) => {
                class.serialize(serializer)
            }
            Classes::hclStretchLinkConstraintSetMx(class) => class.serialize(serializer),
            Classes::hclStretchLinkConstraintSetMxBatch(class) => {
                class.serialize(serializer)
            }
            Classes::hclStretchLinkConstraintSetMxSingle(class) => {
                class.serialize(serializer)
            }
            Classes::hclStretchLinkSetupObject(class) => class.serialize(serializer),
            Classes::hclTaperedCapsuleShape(class) => class.serialize(serializer),
            Classes::hclTransformSetDefinition(class) => class.serialize(serializer),
            Classes::hclTransformSetSetupObject(class) => class.serialize(serializer),
            Classes::hclTransformSetUsage(class) => class.serialize(serializer),
            Classes::hclTransformSetUsageTransformTracker(class) => {
                class.serialize(serializer)
            }
            Classes::hclTransitionConstraintSet(class) => class.serialize(serializer),
            Classes::hclTransitionConstraintSetPerParticle(class) => {
                class.serialize(serializer)
            }
            Classes::hclTriangleSelectionInput(class) => class.serialize(serializer),
            Classes::hclUpdateAllVertexFramesOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclUpdateSomeVertexFramesOperator(class) => {
                class.serialize(serializer)
            }
            Classes::hclUpdateSomeVertexFramesOperatorTriangle(class) => {
                class.serialize(serializer)
            }
            Classes::hclVertexFloatInput(class) => class.serialize(serializer),
            Classes::hclVertexGatherSetupObject(class) => class.serialize(serializer),
            Classes::hclVertexSelectionInput(class) => class.serialize(serializer),
            Classes::hclVolumeConstraint(class) => class.serialize(serializer),
            Classes::hclVolumeConstraintApplyData(class) => class.serialize(serializer),
            Classes::hclVolumeConstraintFrameData(class) => class.serialize(serializer),
            Classes::hclVolumeConstraintMx(class) => class.serialize(serializer),
            Classes::hclVolumeConstraintMxApplyBatchData(class) => {
                class.serialize(serializer)
            }
            Classes::hclVolumeConstraintMxApplySingleData(class) => {
                class.serialize(serializer)
            }
            Classes::hclVolumeConstraintMxFrameBatchData(class) => {
                class.serialize(serializer)
            }
            Classes::hclVolumeConstraintMxFrameSingleData(class) => {
                class.serialize(serializer)
            }
            Classes::hclVolumeConstraintSetupObject(class) => class.serialize(serializer),
            Classes::hkAabb(class) => class.serialize(serializer),
            Classes::hkAabb16(class) => class.serialize(serializer),
            Classes::hkAabbHalf(class) => class.serialize(serializer),
            Classes::hkAabbUint32(class) => class.serialize(serializer),
            Classes::hkaAngularReferenceFrame(class) => class.serialize(serializer),
            Classes::hkaAnimatedReferenceFrame(class) => class.serialize(serializer),
            Classes::hkaAnimation(class) => class.serialize(serializer),
            Classes::hkaAnimationBinding(class) => class.serialize(serializer),
            Classes::hkaAnimationContainer(class) => class.serialize(serializer),
            Classes::hkaAnimationPreviewColorContainer(class) => {
                class.serialize(serializer)
            }
            Classes::hkaAnnotationTrack(class) => class.serialize(serializer),
            Classes::hkaAnnotationTrackAnnotation(class) => class.serialize(serializer),
            Classes::hkaBone(class) => class.serialize(serializer),
            Classes::hkaBoneAttachment(class) => class.serialize(serializer),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => {
                class.serialize(serializer)
            }
            Classes::hkaDirectionalReferenceFrame(class) => class.serialize(serializer),
            Classes::hkaFootstepAnalysisInfo(class) => class.serialize(serializer),
            Classes::hkaFootstepAnalysisInfoContainer(class) => {
                class.serialize(serializer)
            }
            Classes::hkaInterleavedUncompressedAnimation(class) => {
                class.serialize(serializer)
            }
            Classes::hkaLosslessCompressedAnimation(class) => class.serialize(serializer),
            Classes::hkaMeshBinding(class) => class.serialize(serializer),
            Classes::hkaMeshBindingMapping(class) => class.serialize(serializer),
            Classes::hkaParameterizedAnimationReferenceFrame(class) => {
                class.serialize(serializer)
            }
            Classes::hkaParameterizedReferenceFrame(class) => class.serialize(serializer),
            Classes::hkaPredictiveCompressedAnimation(class) => {
                class.serialize(serializer)
            }
            Classes::hkaPredictiveCompressedAnimationTrackCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaQuantizedAnimation(class) => class.serialize(serializer),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaReferencePoseAnimation(class) => class.serialize(serializer),
            Classes::hkArrayTypeAttribute(class) => class.serialize(serializer),
            Classes::hkaSkeleton(class) => class.serialize(serializer),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.serialize(serializer),
            Classes::hkaSkeletonMapper(class) => class.serialize(serializer),
            Classes::hkaSkeletonMapperData(class) => class.serialize(serializer),
            Classes::hkaSkeletonMapperDataChainMapping(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSkeletonMapperDataPartitionMappingRange(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSkeletonPartition(class) => class.serialize(serializer),
            Classes::hkaSplineCompressedAnimation(class) => class.serialize(serializer),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkbAiControlCancelPathCommand(class) => class.serialize(serializer),
            Classes::hkbAiControlControlData(class) => class.serialize(serializer),
            Classes::hkbAiControlControlDataBlendable(class) => {
                class.serialize(serializer)
            }
            Classes::hkbAiControlControlDataNonBlendable(class) => {
                class.serialize(serializer)
            }
            Classes::hkbAiControlPathToCommand(class) => class.serialize(serializer),
            Classes::hkbAlignBoneModifier(class) => class.serialize(serializer),
            Classes::hkbAnimatedSkeletonGenerator(class) => class.serialize(serializer),
            Classes::hkBaseObject(class) => class.serialize(serializer),
            Classes::hkbAssetBundle(class) => class.serialize(serializer),
            Classes::hkbAssetBundleStringData(class) => class.serialize(serializer),
            Classes::hkbAttachmentModifier(class) => class.serialize(serializer),
            Classes::hkbAttachmentSetup(class) => class.serialize(serializer),
            Classes::hkbAttributeModifier(class) => class.serialize(serializer),
            Classes::hkbAttributeModifierAssignment(class) => class.serialize(serializer),
            Classes::hkbAuxiliaryNodeInfo(class) => class.serialize(serializer),
            Classes::hkbBehaviorEventsInfo(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraph(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraphData(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraphInternalState(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBehaviorGraphStringData(class) => class.serialize(serializer),
            Classes::hkbBehaviorInfo(class) => class.serialize(serializer),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.serialize(serializer),
            Classes::hkbBehaviorReferenceGenerator(class) => class.serialize(serializer),
            Classes::hkbBindable(class) => class.serialize(serializer),
            Classes::hkbBlendCurveUtils(class) => class.serialize(serializer),
            Classes::hkbBlenderGenerator(class) => class.serialize(serializer),
            Classes::hkbBlenderGeneratorChild(class) => class.serialize(serializer),
            Classes::hkbBlenderGeneratorChildInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBlenderGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBlendingTransitionEffect(class) => class.serialize(serializer),
            Classes::hkbBlendingTransitionEffectInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBoneIndexArray(class) => class.serialize(serializer),
            Classes::hkbBoneWeightArray(class) => class.serialize(serializer),
            Classes::hkbBoolVariableSequencedData(class) => class.serialize(serializer),
            Classes::hkbBoolVariableSequencedDataSample(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCameraShakeEventPayload(class) => class.serialize(serializer),
            Classes::hkbCameraVariablesChangedCommand(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCharacter(class) => class.serialize(serializer),
            Classes::hkbCharacterAddedInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterControlCommand(class) => class.serialize(serializer),
            Classes::hkbCharacterControllerModifier(class) => class.serialize(serializer),
            Classes::hkbCharacterControllerModifierControlData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCharacterControllerModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCharacterControllerSetup(class) => class.serialize(serializer),
            Classes::hkbCharacterData(class) => class.serialize(serializer),
            Classes::hkbCharacterInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterSelectedInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterSetup(class) => class.serialize(serializer),
            Classes::hkbCharacterSkinInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterSteppedInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterStringData(class) => class.serialize(serializer),
            Classes::hkbCharacterStringDataFileNameMeshNamePair(class) => {
                class.serialize(serializer)
            }
            Classes::hkbClientCharacterState(class) => class.serialize(serializer),
            Classes::hkbClipGenerator(class) => class.serialize(serializer),
            Classes::hkbClipGeneratorEcho(class) => class.serialize(serializer),
            Classes::hkbClipGeneratorInternalState(class) => class.serialize(serializer),
            Classes::hkbClipTrigger(class) => class.serialize(serializer),
            Classes::hkbClipTriggerArray(class) => class.serialize(serializer),
            Classes::hkbCombineTransformsModifier(class) => class.serialize(serializer),
            Classes::hkbCombineTransformsModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCompiledExpressionSet(class) => class.serialize(serializer),
            Classes::hkbCompiledExpressionSetToken(class) => class.serialize(serializer),
            Classes::hkbComputeDirectionModifier(class) => class.serialize(serializer),
            Classes::hkbComputeDirectionModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationToTargetModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeWorldFromModelModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCondition(class) => class.serialize(serializer),
            Classes::hkbConstraintSetup(class) => class.serialize(serializer),
            Classes::hkbContext(class) => class.serialize(serializer),
            Classes::hkbCustomIdSelector(class) => class.serialize(serializer),
            Classes::hkbCustomTestGenerator(class) => class.serialize(serializer),
            Classes::hkbCustomTestGeneratorAnnotatedTypes(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCustomTestGeneratorBoneTypes(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCustomTestGeneratorComplexTypes(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCustomTestGeneratorHiddenTypes(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCustomTestGeneratorNestedTypes(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCustomTestGeneratorNestedTypesBase(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCustomTestGeneratorSimpleTypes(class) => {
                class.serialize(serializer)
            }
            Classes::hkbDampingModifier(class) => class.serialize(serializer),
            Classes::hkbDampingModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbDelayedModifier(class) => class.serialize(serializer),
            Classes::hkbDelayedModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbDetectCloseToGroundModifier(class) => class.serialize(serializer),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbDockingGenerator(class) => class.serialize(serializer),
            Classes::hkbDockingGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEvaluateExpressionModifier(class) => class.serialize(serializer),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEvaluateHandleModifier(class) => class.serialize(serializer),
            Classes::hkbEvent(class) => class.serialize(serializer),
            Classes::hkbEventBase(class) => class.serialize(serializer),
            Classes::hkbEventDrivenModifier(class) => class.serialize(serializer),
            Classes::hkbEventDrivenModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEventInfo(class) => class.serialize(serializer),
            Classes::hkbEventPayload(class) => class.serialize(serializer),
            Classes::hkbEventPayloadList(class) => class.serialize(serializer),
            Classes::hkbEventProperty(class) => class.serialize(serializer),
            Classes::hkbEventRaisedInfo(class) => class.serialize(serializer),
            Classes::hkbEventRangeData(class) => class.serialize(serializer),
            Classes::hkbEventRangeDataArray(class) => class.serialize(serializer),
            Classes::hkbEventSequencedData(class) => class.serialize(serializer),
            Classes::hkbEventSequencedDataSequencedEvent(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEventsFromRangeModifier(class) => class.serialize(serializer),
            Classes::hkbEventsFromRangeModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbExpressionCondition(class) => class.serialize(serializer),
            Classes::hkbExpressionData(class) => class.serialize(serializer),
            Classes::hkbExpressionDataArray(class) => class.serialize(serializer),
            Classes::hkbExtractRagdollPoseModifier(class) => class.serialize(serializer),
            Classes::hkbExtrapolatingTransitionEffect(class) => {
                class.serialize(serializer)
            }
            Classes::hkbExtrapolatingTransitionEffectInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbFootIkControlData(class) => class.serialize(serializer),
            Classes::hkbFootIkControlsModifier(class) => class.serialize(serializer),
            Classes::hkbFootIkControlsModifierLeg(class) => class.serialize(serializer),
            Classes::hkbFootIkDriverInfo(class) => class.serialize(serializer),
            Classes::hkbFootIkDriverInfoLeg(class) => class.serialize(serializer),
            Classes::hkbFootIkGains(class) => class.serialize(serializer),
            Classes::hkbFootIkModifier(class) => class.serialize(serializer),
            Classes::hkbFootIkModifierInternalLegData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbFootIkModifierLeg(class) => class.serialize(serializer),
            Classes::hkbGenerator(class) => class.serialize(serializer),
            Classes::hkbGeneratorOutputListener(class) => class.serialize(serializer),
            Classes::hkbGeneratorPartitionInfo(class) => class.serialize(serializer),
            Classes::hkbGeneratorSyncInfo(class) => class.serialize(serializer),
            Classes::hkbGeneratorSyncInfoActiveInterval(class) => {
                class.serialize(serializer)
            }
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.serialize(serializer),
            Classes::hkbGeneratorTransitionEffect(class) => class.serialize(serializer),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbGetHandleOnBoneModifier(class) => class.serialize(serializer),
            Classes::hkbGetUpModifier(class) => class.serialize(serializer),
            Classes::hkbGetUpModifierInternalState(class) => class.serialize(serializer),
            Classes::hkbGetWorldFromModelModifier(class) => class.serialize(serializer),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbGravityModifier(class) => class.serialize(serializer),
            Classes::hkbHandIkControlData(class) => class.serialize(serializer),
            Classes::hkbHandIkControlsModifier(class) => class.serialize(serializer),
            Classes::hkbHandIkControlsModifierHand(class) => class.serialize(serializer),
            Classes::hkbHandIkDriverInfo(class) => class.serialize(serializer),
            Classes::hkbHandIkDriverInfoHand(class) => class.serialize(serializer),
            Classes::hkbHandIkModifier(class) => class.serialize(serializer),
            Classes::hkbHandIkModifierHand(class) => class.serialize(serializer),
            Classes::hkbHandle(class) => class.serialize(serializer),
            Classes::hkbHoldFromBlendingTransitionEffect(class) => {
                class.serialize(serializer)
            }
            Classes::hkbIntEventPayload(class) => class.serialize(serializer),
            Classes::hkbIntVariableSequencedData(class) => class.serialize(serializer),
            Classes::hkbIntVariableSequencedDataSample(class) => {
                class.serialize(serializer)
            }
            Classes::hkBitField(class) => class.serialize(serializer),
            Classes::hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.serialize(serializer),
            Classes::hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.serialize(serializer),
            Classes::hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.serialize(serializer),
            Classes::hkbJigglerGroup(class) => class.serialize(serializer),
            Classes::hkbJigglerModifier(class) => class.serialize(serializer),
            Classes::hkbJigglerModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbKeyframeBonesModifier(class) => class.serialize(serializer),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbKeyFrameControlData(class) => class.serialize(serializer),
            Classes::hkbLayer(class) => class.serialize(serializer),
            Classes::hkbLayerGenerator(class) => class.serialize(serializer),
            Classes::hkbLayerGeneratorInternalState(class) => class.serialize(serializer),
            Classes::hkbLayerGeneratorLayerInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbLinkedSymbolInfo(class) => class.serialize(serializer),
            Classes::hkbLookAtModifier(class) => class.serialize(serializer),
            Classes::hkbLookAtModifierInternalState(class) => class.serialize(serializer),
            Classes::hkbManualSelectorGenerator(class) => class.serialize(serializer),
            Classes::hkbManualSelectorGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbManualSelectorTransitionEffect(class) => {
                class.serialize(serializer)
            }
            Classes::hkbManualSelectorTransitionEffectInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbMessageLog(class) => class.serialize(serializer),
            Classes::hkbMirroredSkeletonInfo(class) => class.serialize(serializer),
            Classes::hkbMirrorModifier(class) => class.serialize(serializer),
            Classes::hkbModifier(class) => class.serialize(serializer),
            Classes::hkbModifierGenerator(class) => class.serialize(serializer),
            Classes::hkbModifierList(class) => class.serialize(serializer),
            Classes::hkbModifierWrapper(class) => class.serialize(serializer),
            Classes::hkbMoveBoneAttachmentModifier(class) => class.serialize(serializer),
            Classes::hkbMoveCharacterModifier(class) => class.serialize(serializer),
            Classes::hkbMoveCharacterModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbNamedEventPayload(class) => class.serialize(serializer),
            Classes::hkbNamedIntEventPayload(class) => class.serialize(serializer),
            Classes::hkbNamedRealEventPayload(class) => class.serialize(serializer),
            Classes::hkbNamedStringEventPayload(class) => class.serialize(serializer),
            Classes::hkbNode(class) => class.serialize(serializer),
            Classes::hkbNodeInternalStateInfo(class) => class.serialize(serializer),
            Classes::hkbnpPhysicsInterface(class) => class.serialize(serializer),
            Classes::hkbnpRagdollInterface(class) => class.serialize(serializer),
            Classes::hkbNullPhysicsInterface(class) => class.serialize(serializer),
            Classes::hkbParametricMotionGenerator(class) => class.serialize(serializer),
            Classes::hkbParticleSystemEventPayload(class) => class.serialize(serializer),
            Classes::hkbPhysicsInterface(class) => class.serialize(serializer),
            Classes::hkbPinBonesGenerator(class) => class.serialize(serializer),
            Classes::hkbPoseMatchingGenerator(class) => class.serialize(serializer),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbPoseStoringGeneratorOutputListener(class) => {
                class.serialize(serializer)
            }
            Classes::hkbPoseStoringGeneratorOutputListenerStoredPose(class) => {
                class.serialize(serializer)
            }
            Classes::hkbPoweredRagdollControlData(class) => class.serialize(serializer),
            Classes::hkbPoweredRagdollControlsModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbProceduralBlenderGenerator(class) => class.serialize(serializer),
            Classes::hkbProjectData(class) => class.serialize(serializer),
            Classes::hkbProjectStringData(class) => class.serialize(serializer),
            Classes::hkbRadialSelectorGenerator(class) => class.serialize(serializer),
            Classes::hkbRadialSelectorGeneratorGeneratorInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbRadialSelectorGeneratorGeneratorPair(class) => {
                class.serialize(serializer)
            }
            Classes::hkbRagdollControllerSetup(class) => class.serialize(serializer),
            Classes::hkbRagdollInterface(class) => class.serialize(serializer),
            Classes::hkbRaiseEventCommand(class) => class.serialize(serializer),
            Classes::hkbRealEventPayload(class) => class.serialize(serializer),
            Classes::hkbRealVariableSequencedData(class) => class.serialize(serializer),
            Classes::hkbRealVariableSequencedDataSample(class) => {
                class.serialize(serializer)
            }
            Classes::hkbRebuildScriptsCommand(class) => class.serialize(serializer),
            Classes::hkbReferencedGeneratorSyncInfo(class) => class.serialize(serializer),
            Classes::hkbReferencePoseGenerator(class) => class.serialize(serializer),
            Classes::hkbRigidBodyRagdollControlData(class) => class.serialize(serializer),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbRigidBodySetup(class) => class.serialize(serializer),
            Classes::hkbRoleAttribute(class) => class.serialize(serializer),
            Classes::hkbRotateCharacterModifier(class) => class.serialize(serializer),
            Classes::hkbRotateCharacterModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbScriptCondition(class) => class.serialize(serializer),
            Classes::hkbScriptGenerator(class) => class.serialize(serializer),
            Classes::hkbSenseHandleModifier(class) => class.serialize(serializer),
            Classes::hkbSenseHandleModifierRange(class) => class.serialize(serializer),
            Classes::hkbSequence(class) => class.serialize(serializer),
            Classes::hkbSequencedData(class) => class.serialize(serializer),
            Classes::hkbSequenceInternalState(class) => class.serialize(serializer),
            Classes::hkbSequenceStringData(class) => class.serialize(serializer),
            Classes::hkbSetBehaviorCommand(class) => class.serialize(serializer),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => {
                class.serialize(serializer)
            }
            Classes::hkbSetNodePropertyCommand(class) => class.serialize(serializer),
            Classes::hkbSetSelectedCharacterCommand(class) => class.serialize(serializer),
            Classes::hkbSetWordVariableCommand(class) => class.serialize(serializer),
            Classes::hkbSetWorldFromModelModifier(class) => class.serialize(serializer),
            Classes::hkbShapeSetup(class) => class.serialize(serializer),
            Classes::hkbSimulationControlCommand(class) => class.serialize(serializer),
            Classes::hkbSimulationStateInfo(class) => class.serialize(serializer),
            Classes::hkbStateChooser(class) => class.serialize(serializer),
            Classes::hkbStateChooserWrapper(class) => class.serialize(serializer),
            Classes::hkbStateDependentModifier(class) => class.serialize(serializer),
            Classes::hkbStateListener(class) => class.serialize(serializer),
            Classes::hkbStateMachine(class) => class.serialize(serializer),
            Classes::hkbStateMachineActiveTransitionInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineDelayedTransitionInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineEventPropertyArray(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineInternalState(class) => class.serialize(serializer),
            Classes::hkbStateMachineNestedStateMachineData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineStateInfo(class) => class.serialize(serializer),
            Classes::hkbStateMachineTimeInterval(class) => class.serialize(serializer),
            Classes::hkbStateMachineTransitionInfo(class) => class.serialize(serializer),
            Classes::hkbStateMachineTransitionInfoArray(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineTransitionInfoReference(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStringCondition(class) => class.serialize(serializer),
            Classes::hkbStringEventPayload(class) => class.serialize(serializer),
            Classes::hkbTestIdSelector(class) => class.serialize(serializer),
            Classes::hkbTimerModifier(class) => class.serialize(serializer),
            Classes::hkbTimerModifierInternalState(class) => class.serialize(serializer),
            Classes::hkbToolNodeType(class) => class.serialize(serializer),
            Classes::hkbTransformVectorModifier(class) => class.serialize(serializer),
            Classes::hkbTransformVectorModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbTransitionEffect(class) => class.serialize(serializer),
            Classes::hkbTwistModifier(class) => class.serialize(serializer),
            Classes::hkbVariableBindingSet(class) => class.serialize(serializer),
            Classes::hkbVariableBindingSetBinding(class) => class.serialize(serializer),
            Classes::hkbVariableBounds(class) => class.serialize(serializer),
            Classes::hkbVariableInfo(class) => class.serialize(serializer),
            Classes::hkbVariableValue(class) => class.serialize(serializer),
            Classes::hkbVariableValueSet(class) => class.serialize(serializer),
            Classes::hkbWorldEnums(class) => class.serialize(serializer),
            Classes::hkbWorldFromModelModeData(class) => class.serialize(serializer),
            Classes::hkcdAdf(class) => class.serialize(serializer),
            Classes::hkcdDynamicAabbTree(class) => class.serialize(serializer),
            Classes::hkcdDynamicTreeAnisotropicMetric(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeCodec18(class) => class.serialize(serializer),
            Classes::hkcdDynamicTreeCodec32(class) => class.serialize(serializer),
            Classes::hkcdDynamicTreeCodecInt16(class) => class.serialize(serializer),
            Classes::hkcdDynamicTreeCodecInt16IntAabb(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeCodecRawUint(class) => class.serialize(serializer),
            Classes::hkcdDynamicTreeCodecRawUlong(class) => class.serialize(serializer),
            Classes::hkcdDynamicTreeCodecRawunsignedint(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeCodecRawunsignedlong(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDefaultTree32Storage(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDefaultTree48Storage(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDefaultTreeInt16Storage(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDefaultTreePtrStorage(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdDynamicTreeDynamicStorage16(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDynamicStorage32(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDynamicStorageInt16(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeDynamicStoragePtr(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdFourAabb(class) => class.serialize(serializer),
            Classes::hkcdPlanarCsgOperand(class) => class.serialize(serializer),
            Classes::hkcdPlanarCsgOperandGeomSource(class) => class.serialize(serializer),
            Classes::hkcdPlanarEntity(class) => class.serialize(serializer),
            Classes::hkcdPlanarGeometry(class) => class.serialize(serializer),
            Classes::hkcdPlanarGeometryPlanesCollection(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdPlanarGeometryPolygonCollection(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdPlanarGeometryPrimitivesCollection28(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdPlanarGeometryPrimitivesPlane(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdPlanarSolid(class) => class.serialize(serializer),
            Classes::hkcdPlanarSolidNode(class) => class.serialize(serializer),
            Classes::hkcdPlanarSolidNodeStorage(class) => class.serialize(serializer),
            Classes::hkcdShape(class) => class.serialize(serializer),
            Classes::hkcdShapeDispatchType(class) => class.serialize(serializer),
            Classes::hkcdShapeInfoCodecType(class) => class.serialize(serializer),
            Classes::hkcdShapeType(class) => class.serialize(serializer),
            Classes::hkcdSimdTree(class) => class.serialize(serializer),
            Classes::hkcdSimdTreeNode(class) => class.serialize(serializer),
            Classes::hkcdStaticAabbTree(class) => class.serialize(serializer),
            Classes::hkcdStaticMeshTreeBase(class) => class.serialize(serializer),
            Classes::hkcdStaticMeshTreeBasePrimitive(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticMeshTreeBaseSection(class) => class.serialize(serializer),
            Classes::hkcdStaticMeshTreeBaseSectionDataRuns(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticMeshTreeBaseSectionPrimitives(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticMeshTreeBaseSectionSharedVertices(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticMeshTreeDefaultDataRun(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun(
                class,
            ) => class.serialize(serializer),
            Classes::hkcdStaticPvs(class) => class.serialize(serializer),
            Classes::hkcdStaticPvsBlockHeader(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeCodec3Axis(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeCodec3Axis4(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeCodec3Axis5(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeCodec3Axis6(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeCodecRaw(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeDefaultTreeStorage32(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDefaultTreeStorage4(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDefaultTreeStorage5(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDefaultTreeStorage6(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDynamicStorage32(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeDynamicStorage4(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeDynamicStorage5(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeDynamicStorage6(class) => class.serialize(serializer),
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5(class) => {
                class.serialize(serializer)
            }
            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6(class) => {
                class.serialize(serializer)
            }
            Classes::hkClass(class) => class.serialize(serializer),
            Classes::hkClassEnum(class) => class.serialize(serializer),
            Classes::hkClassEnumItem(class) => class.serialize(serializer),
            Classes::hkClassMember(class) => class.serialize(serializer),
            Classes::hkCompressedMassProperties(class) => class.serialize(serializer),
            Classes::hkContactPoint(class) => class.serialize(serializer),
            Classes::hkContactPointMaterial(class) => class.serialize(serializer),
            Classes::hkCustomAttributes(class) => class.serialize(serializer),
            Classes::hkCustomAttributesAttribute(class) => class.serialize(serializer),
            Classes::hkDataObjectTypeAttribute(class) => class.serialize(serializer),
            Classes::hkDefaultMemoryTracker(class) => class.serialize(serializer),
            Classes::hkDescriptionAttribute(class) => class.serialize(serializer),
            Classes::hkDocumentationAttribute(class) => class.serialize(serializer),
            Classes::hkFloat16(class) => class.serialize(serializer),
            Classes::hkFloat16Transform(class) => class.serialize(serializer),
            Classes::hkFourTransposedPointsf(class) => class.serialize(serializer),
            Classes::hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations(
                class,
            ) => class.serialize(serializer),
            Classes::hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations(
                class,
            ) => class.serialize(serializer),
            Classes::hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance(
                class,
            ) => class.serialize(serializer),
            Classes::hkGeometry(class) => class.serialize(serializer),
            Classes::hkGeometryTriangle(class) => class.serialize(serializer),
            Classes::hkGizmoAttribute(class) => class.serialize(serializer),
            Classes::hkIndexedTransformSet(class) => class.serialize(serializer),
            Classes::hkIntRealPair(class) => class.serialize(serializer),
            Classes::hkLinkAttribute(class) => class.serialize(serializer),
            Classes::hkLocalFrame(class) => class.serialize(serializer),
            Classes::hkLocalFrameGroup(class) => class.serialize(serializer),
            Classes::hkMassProperties(class) => class.serialize(serializer),
            Classes::hkMemoryMeshBody(class) => class.serialize(serializer),
            Classes::hkMemoryMeshMaterial(class) => class.serialize(serializer),
            Classes::hkMemoryMeshShape(class) => class.serialize(serializer),
            Classes::hkMemoryMeshShapeSection(class) => class.serialize(serializer),
            Classes::hkMemoryMeshTexture(class) => class.serialize(serializer),
            Classes::hkMemoryMeshVertexBuffer(class) => class.serialize(serializer),
            Classes::hkMemoryResourceContainer(class) => class.serialize(serializer),
            Classes::hkMemoryResourceHandle(class) => class.serialize(serializer),
            Classes::hkMemoryResourceHandleExternalLink(class) => {
                class.serialize(serializer)
            }
            Classes::hkMeshBody(class) => class.serialize(serializer),
            Classes::hkMeshBoneIndexMapping(class) => class.serialize(serializer),
            Classes::hkMeshMaterial(class) => class.serialize(serializer),
            Classes::hkMeshSection(class) => class.serialize(serializer),
            Classes::hkMeshSectionCinfo(class) => class.serialize(serializer),
            Classes::hkMeshShape(class) => class.serialize(serializer),
            Classes::hkMeshTexture(class) => class.serialize(serializer),
            Classes::hkMeshTextureRawBufferDescriptor(class) => {
                class.serialize(serializer)
            }
            Classes::hkMeshVertexBuffer(class) => class.serialize(serializer),
            Classes::hkModelerNodeTypeAttribute(class) => class.serialize(serializer),
            Classes::hkMonitorStreamColorTable(class) => class.serialize(serializer),
            Classes::hkMonitorStreamColorTableColorPair(class) => {
                class.serialize(serializer)
            }
            Classes::hkMonitorStreamFrameInfo(class) => class.serialize(serializer),
            Classes::hkMonitorStreamStringMap(class) => class.serialize(serializer),
            Classes::hkMonitorStreamStringMapStringMap(class) => {
                class.serialize(serializer)
            }
            Classes::hkMotionState(class) => class.serialize(serializer),
            Classes::hkMultipleVertexBuffer(class) => class.serialize(serializer),
            Classes::hkMultipleVertexBufferElementInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkMultipleVertexBufferLockedElement(class) => {
                class.serialize(serializer)
            }
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkMultiThreadCheck(class) => class.serialize(serializer),
            Classes::hknpAction(class) => class.serialize(serializer),
            Classes::hknpArrayAction(class) => class.serialize(serializer),
            Classes::hknpBallGun(class) => class.serialize(serializer),
            Classes::hknpBinaryAction(class) => class.serialize(serializer),
            Classes::hknpBody(class) => class.serialize(serializer),
            Classes::hknpBodyCinfo(class) => class.serialize(serializer),
            Classes::hknpBodyQuality(class) => class.serialize(serializer),
            Classes::hknpBodyQualityLibrary(class) => class.serialize(serializer),
            Classes::hknpBodyReference(class) => class.serialize(serializer),
            Classes::hknpBreakableConstraintData(class) => class.serialize(serializer),
            Classes::hknpBridgeConstraintAtom(class) => class.serialize(serializer),
            Classes::hknpBroadPhaseConfig(class) => class.serialize(serializer),
            Classes::hknpBroadPhaseConfigLayer(class) => class.serialize(serializer),
            Classes::hknpBSMaterial(class) => class.serialize(serializer),
            Classes::hknpBSMaterialProperties(class) => class.serialize(serializer),
            Classes::hknpCapsuleShape(class) => class.serialize(serializer),
            Classes::hknpCharacterProxyCinfo(class) => class.serialize(serializer),
            Classes::hknpCharacterRigidBodyCinfo(class) => class.serialize(serializer),
            Classes::hknpCircularSurfaceVelocity(class) => class.serialize(serializer),
            Classes::hknpCollisionDispatchType(class) => class.serialize(serializer),
            Classes::hknpCollisionFilter(class) => class.serialize(serializer),
            Classes::hknpCompositeShape(class) => class.serialize(serializer),
            Classes::hknpCompoundShape(class) => class.serialize(serializer),
            Classes::hknpCompoundShapeInternalsKeyMask(class) => {
                class.serialize(serializer)
            }
            Classes::hknpCompoundShapeKeyMask(class) => class.serialize(serializer),
            Classes::hknpCompressedHeightFieldShape(class) => class.serialize(serializer),
            Classes::hknpCompressedMeshShape(class) => class.serialize(serializer),
            Classes::hknpCompressedMeshShapeData(class) => class.serialize(serializer),
            Classes::hknpCompressedMeshShapeInternalsKeyMask(class) => {
                class.serialize(serializer)
            }
            Classes::hknpCompressedMeshShapeTree(class) => class.serialize(serializer),
            Classes::hknpCompressedMeshShapeTreeDataRun(class) => {
                class.serialize(serializer)
            }
            Classes::hknpCompressedMeshShapeTreeDataRunData(class) => {
                class.serialize(serializer)
            }
            Classes::hknpConstraint(class) => class.serialize(serializer),
            Classes::hknpConstraintCinfo(class) => class.serialize(serializer),
            Classes::hknpConstraintCollisionFilter(class) => class.serialize(serializer),
            Classes::hknpConvexPolytopeShape(class) => class.serialize(serializer),
            Classes::hknpConvexPolytopeShapeFace(class) => class.serialize(serializer),
            Classes::hknpConvexShape(class) => class.serialize(serializer),
            Classes::hknpDecoratorShape(class) => class.serialize(serializer),
            Classes::hknpDefaultBroadPhaseConfig(class) => class.serialize(serializer),
            Classes::hknpDefaultExternMeshShapeGeometry(class) => {
                class.serialize(serializer)
            }
            Classes::hknpDestructionShapeProperties(class) => class.serialize(serializer),
            Classes::hknpDisableCollisionFilter(class) => class.serialize(serializer),
            Classes::hknpDummyShape(class) => class.serialize(serializer),
            Classes::hknpDynamicCompoundShape(class) => class.serialize(serializer),
            Classes::hknpDynamicCompoundShapeData(class) => class.serialize(serializer),
            Classes::hknpDynamicCompoundShapeTree(class) => class.serialize(serializer),
            Classes::hknpExternMeshShape(class) => class.serialize(serializer),
            Classes::hknpExternMeshShapeData(class) => class.serialize(serializer),
            Classes::hknpExternMeshShapeGeometry(class) => class.serialize(serializer),
            Classes::hknpFirstPersonGun(class) => class.serialize(serializer),
            Classes::hknpGroupCollisionFilter(class) => class.serialize(serializer),
            Classes::hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516(
                class,
            ) => class.serialize(serializer),
            Classes::hknpHeightFieldShape(class) => class.serialize(serializer),
            Classes::hknpLinearSurfaceVelocity(class) => class.serialize(serializer),
            Classes::hknpLodShape(class) => class.serialize(serializer),
            Classes::hknpLodShapeLevelOfDetailInfo(class) => class.serialize(serializer),
            Classes::hknpMalleableConstraintData(class) => class.serialize(serializer),
            Classes::hknpMaskedShape(class) => class.serialize(serializer),
            Classes::hknpMaskedShapeMaskWrapper(class) => class.serialize(serializer),
            Classes::hknpMaterial(class) => class.serialize(serializer),
            Classes::hknpMaterialDescriptor(class) => class.serialize(serializer),
            Classes::hknpMaterialLibrary(class) => class.serialize(serializer),
            Classes::hknpMaterialPalette(class) => class.serialize(serializer),
            Classes::hknpMinMaxQuadTree(class) => class.serialize(serializer),
            Classes::hknpMinMaxQuadTreeMinMaxLevel(class) => class.serialize(serializer),
            Classes::hknpMotion(class) => class.serialize(serializer),
            Classes::hknpMotionCinfo(class) => class.serialize(serializer),
            Classes::hknpMotionProperties(class) => class.serialize(serializer),
            Classes::hknpMotionPropertiesLibrary(class) => class.serialize(serializer),
            Classes::hknpPairCollisionFilter(class) => class.serialize(serializer),
            Classes::hknpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.serialize(serializer)
            }
            Classes::hknpPhysicsSceneData(class) => class.serialize(serializer),
            Classes::hknpPhysicsSystemData(class) => class.serialize(serializer),
            Classes::hknpProjectileGun(class) => class.serialize(serializer),
            Classes::hknpRagdollData(class) => class.serialize(serializer),
            Classes::hknpRagdollKeyFrameHierarchyUtilityControlData(class) => {
                class.serialize(serializer)
            }
            Classes::hknpRefMaterial(class) => class.serialize(serializer),
            Classes::hknpRefWorldCinfo(class) => class.serialize(serializer),
            Classes::hknpScaledConvexShape(class) => class.serialize(serializer),
            Classes::hknpScaledConvexShapeBase(class) => class.serialize(serializer),
            Classes::hknpShape(class) => class.serialize(serializer),
            Classes::hknpShapeInstance(class) => class.serialize(serializer),
            Classes::hknpShapeKeyMask(class) => class.serialize(serializer),
            Classes::hknpShapeMassProperties(class) => class.serialize(serializer),
            Classes::hknpShapeSignals(class) => class.serialize(serializer),
            Classes::hknpShapeTagCodec(class) => class.serialize(serializer),
            Classes::hknpShapeType(class) => class.serialize(serializer),
            Classes::hknpSparseCompactMapunsignedshort(class) => {
                class.serialize(serializer)
            }
            Classes::hknpSphereShape(class) => class.serialize(serializer),
            Classes::hknpStaticCompoundShape(class) => class.serialize(serializer),
            Classes::hknpStaticCompoundShapeData(class) => class.serialize(serializer),
            Classes::hknpStaticCompoundShapeInternals(class) => {
                class.serialize(serializer)
            }
            Classes::hknpStaticCompoundShapeKeyMask(class) => class.serialize(serializer),
            Classes::hknpStaticCompoundShapeTree(class) => class.serialize(serializer),
            Classes::hknpSurfaceVelocity(class) => class.serialize(serializer),
            Classes::hknpTriangleShape(class) => class.serialize(serializer),
            Classes::hknpTyremarkPoint(class) => class.serialize(serializer),
            Classes::hknpTyremarksInfo(class) => class.serialize(serializer),
            Classes::hknpTyremarksWheel(class) => class.serialize(serializer),
            Classes::hknpUnaryAction(class) => class.serialize(serializer),
            Classes::hknpVehicleAerodynamics(class) => class.serialize(serializer),
            Classes::hknpVehicleBrake(class) => class.serialize(serializer),
            Classes::hknpVehicleData(class) => class.serialize(serializer),
            Classes::hknpVehicleDataWheelComponentParams(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleDefaultAerodynamics(class) => class.serialize(serializer),
            Classes::hknpVehicleDefaultAnalogDriverInput(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleDefaultBrake(class) => class.serialize(serializer),
            Classes::hknpVehicleDefaultBrakeWheelBrakingProperties(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleDefaultEngine(class) => class.serialize(serializer),
            Classes::hknpVehicleDefaultSteering(class) => class.serialize(serializer),
            Classes::hknpVehicleDefaultSuspension(class) => class.serialize(serializer),
            Classes::hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.serialize(serializer),
            Classes::hknpVehicleDefaultTransmission(class) => class.serialize(serializer),
            Classes::hknpVehicleDefaultVelocityDamper(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleDriverInput(class) => class.serialize(serializer),
            Classes::hknpVehicleDriverInputAnalogStatus(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleDriverInputStatus(class) => class.serialize(serializer),
            Classes::hknpVehicleEngine(class) => class.serialize(serializer),
            Classes::hknpVehicleInstance(class) => class.serialize(serializer),
            Classes::hknpVehicleInstanceWheelInfo(class) => class.serialize(serializer),
            Classes::hknpVehicleLinearCastWheelCollide(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleLinearCastWheelCollideWheelState(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleRayCastWheelCollide(class) => class.serialize(serializer),
            Classes::hknpVehicleSteering(class) => class.serialize(serializer),
            Classes::hknpVehicleSuspension(class) => class.serialize(serializer),
            Classes::hknpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.serialize(serializer)
            }
            Classes::hknpVehicleTransmission(class) => class.serialize(serializer),
            Classes::hknpVehicleVelocityDamper(class) => class.serialize(serializer),
            Classes::hknpVehicleWheelCollide(class) => class.serialize(serializer),
            Classes::hknpWorldCinfo(class) => class.serialize(serializer),
            Classes::hknpWorldSnapshot(class) => class.serialize(serializer),
            Classes::hkOffsetBitField(class) => class.serialize(serializer),
            Classes::hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                class,
            ) => class.serialize(serializer),
            Classes::hkp2dAngConstraintAtom(class) => class.serialize(serializer),
            Classes::hkp3dAngConstraintAtom(class) => class.serialize(serializer),
            Classes::hkPackedVector3(class) => class.serialize(serializer),
            Classes::hkPackedVector8_3(class) => class.serialize(serializer),
            Classes::hkPackfileHeader(class) => class.serialize(serializer),
            Classes::hkPackfileSectionHeader(class) => class.serialize(serializer),
            Classes::hkpAngConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngFrictionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngMotorConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpBallAndSocketConstraintData(class) => class.serialize(serializer),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpBallSocketConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpBridgeAtoms(class) => class.serialize(serializer),
            Classes::hkpBridgeConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpCallbackConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpCogWheelConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpCogWheelConstraintData(class) => class.serialize(serializer),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpConeLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpConstraintData(class) => class.serialize(serializer),
            Classes::hkpConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpDeformableAngConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpDeformableFixedConstraintData(class) => {
                class.serialize(serializer)
            }
            Classes::hkpDeformableFixedConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpDeformableLinConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpFixedConstraintData(class) => class.serialize(serializer),
            Classes::hkpFixedConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpHingeConstraintData(class) => class.serialize(serializer),
            Classes::hkpHingeConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpHingeLimitsData(class) => class.serialize(serializer),
            Classes::hkpHingeLimitsDataAtoms(class) => class.serialize(serializer),
            Classes::hkpLimitedForceConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpLimitedHingeConstraintData(class) => class.serialize(serializer),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpLinConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinearClearanceConstraintData(class) => {
                class.serialize(serializer)
            }
            Classes::hkpLinearClearanceConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpLinearParametricCurve(class) => class.serialize(serializer),
            Classes::hkpLinFrictionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinMotorConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinSoftConstraintAtom(class) => class.serialize(serializer),
            Classes::hkPostFinishAttribute(class) => class.serialize(serializer),
            Classes::hkpOverwritePivotConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpParametricCurve(class) => class.serialize(serializer),
            Classes::hkpPointToPathConstraintData(class) => class.serialize(serializer),
            Classes::hkpPointToPlaneConstraintData(class) => class.serialize(serializer),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpPositionConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpPrismaticConstraintData(class) => class.serialize(serializer),
            Classes::hkpPrismaticConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpPulleyConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpPulleyConstraintData(class) => class.serialize(serializer),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpRackAndPinionConstraintData(class) => class.serialize(serializer),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpRagdollConstraintData(class) => class.serialize(serializer),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpRagdollLimitsData(class) => class.serialize(serializer),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.serialize(serializer),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpRotationalConstraintData(class) => class.serialize(serializer),
            Classes::hkpRotationalConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetLocalRotationsConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetLocalTransformsConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetupStabilizationAtom(class) => class.serialize(serializer),
            Classes::hkpSpringDamperConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpStiffSpringConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpStiffSpringConstraintData(class) => class.serialize(serializer),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpTwistLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpVehicleFrictionDescription(class) => class.serialize(serializer),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleFrictionStatus(class) => class.serialize(serializer),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVelocityConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpWheelConstraintData(class) => class.serialize(serializer),
            Classes::hkpWheelConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpWheelFrictionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpWheelFrictionConstraintAtomAxle(class) => {
                class.serialize(serializer)
            }
            Classes::hkpWheelFrictionConstraintData(class) => class.serialize(serializer),
            Classes::hkpWheelFrictionConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpWrappedConstraintData(class) => class.serialize(serializer),
            Classes::hkQTransform(class) => class.serialize(serializer),
            Classes::hkQTransformf(class) => class.serialize(serializer),
            Classes::hkRangeInt32Attribute(class) => class.serialize(serializer),
            Classes::hkRangeRealAttribute(class) => class.serialize(serializer),
            Classes::hkRefCountedProperties(class) => class.serialize(serializer),
            Classes::hkRefCountedPropertiesEntry(class) => class.serialize(serializer),
            Classes::hkReferencedObject(class) => class.serialize(serializer),
            Classes::hkResourceBase(class) => class.serialize(serializer),
            Classes::hkResourceContainer(class) => class.serialize(serializer),
            Classes::hkResourceHandle(class) => class.serialize(serializer),
            Classes::hkRootLevelContainer(class) => class.serialize(serializer),
            Classes::hkRootLevelContainerNamedVariant(class) => {
                class.serialize(serializer)
            }
            Classes::hkSemanticsAttribute(class) => class.serialize(serializer),
            Classes::hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair(
                class,
            ) => class.serialize(serializer),
            Classes::hkSetIntFloatPair(class) => class.serialize(serializer),
            Classes::hkSetUint32(class) => class.serialize(serializer),
            Classes::hkSetUint64(class) => class.serialize(serializer),
            Classes::hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint(
                class,
            ) => class.serialize(serializer),
            Classes::hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong(
                class,
            ) => class.serialize(serializer),
            Classes::hkSimpleLocalFrame(class) => class.serialize(serializer),
            Classes::hkSimpleProperty(class) => class.serialize(serializer),
            Classes::hkSimplePropertyValue(class) => class.serialize(serializer),
            Classes::hkSkinBinding(class) => class.serialize(serializer),
            Classes::hkSkinnedMeshShape(class) => class.serialize(serializer),
            Classes::hkSkinnedMeshShapeBoneSection(class) => class.serialize(serializer),
            Classes::hkSkinnedMeshShapeBoneSet(class) => class.serialize(serializer),
            Classes::hkSkinnedMeshShapePart(class) => class.serialize(serializer),
            Classes::hkSkinnedRefMeshShape(class) => class.serialize(serializer),
            Classes::hkSphere(class) => class.serialize(serializer),
            Classes::hkStorageSkinnedMeshShape(class) => class.serialize(serializer),
            Classes::hkStringObject(class) => class.serialize(serializer),
            Classes::hkSweptTransformf(class) => class.serialize(serializer),
            Classes::hkTraceStreamTitle(class) => class.serialize(serializer),
            Classes::hkUFloat8(class) => class.serialize(serializer),
            Classes::hkUiAttribute(class) => class.serialize(serializer),
            Classes::hkUuid(class) => class.serialize(serializer),
            Classes::hkVariableTweakingHelper(class) => class.serialize(serializer),
            Classes::hkVariableTweakingHelperBoolVariableInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkVariableTweakingHelperIntVariableInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkVariableTweakingHelperRealVariableInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkVariableTweakingHelperVector4VariableInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkVertexFormat(class) => class.serialize(serializer),
            Classes::hkVertexFormatElement(class) => class.serialize(serializer),
            Classes::hkxAnimatedFloat(class) => class.serialize(serializer),
            Classes::hkxAnimatedMatrix(class) => class.serialize(serializer),
            Classes::hkxAnimatedQuaternion(class) => class.serialize(serializer),
            Classes::hkxAnimatedVector(class) => class.serialize(serializer),
            Classes::hkxAttribute(class) => class.serialize(serializer),
            Classes::hkxAttributeGroup(class) => class.serialize(serializer),
            Classes::hkxAttributeHolder(class) => class.serialize(serializer),
            Classes::hkxEnum(class) => class.serialize(serializer),
            Classes::hkxEnumItem(class) => class.serialize(serializer),
            Classes::hkxIndexBuffer(class) => class.serialize(serializer),
            Classes::hkxMaterial(class) => class.serialize(serializer),
            Classes::hkxMaterialProperty(class) => class.serialize(serializer),
            Classes::hkxMaterialTextureStage(class) => class.serialize(serializer),
            Classes::hkxMesh(class) => class.serialize(serializer),
            Classes::hkxMeshSection(class) => class.serialize(serializer),
            Classes::hkxMeshUserChannelInfo(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedBool(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedEnum(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedInt(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedString(class) => class.serialize(serializer),
            Classes::hkxVertexAnimation(class) => class.serialize(serializer),
            Classes::hkxVertexAnimationUsageMap(class) => class.serialize(serializer),
            Classes::hkxVertexBuffer(class) => class.serialize(serializer),
            Classes::hkxVertexBufferVertexData(class) => class.serialize(serializer),
            Classes::hkxVertexDescription(class) => class.serialize(serializer),
            Classes::hkxVertexDescriptionElementDecl(class) => {
                class.serialize(serializer)
            }
        }
    }
}
/// Deserializes the next class directly into a `Box`.
///
/// # Why a separate, non-inlined function?
/// Without optimization (debug builds) every arm of the big `match` in
/// `visit_class_index` keeps its own stack slot for the unboxed class value, so that
/// frame grows to the sum of all class sizes and overflows the stack
/// (e.g., `thread 'main' has overflowed its stack` with Windows' 1 MiB main stack).
/// Here, each frame holds only one class.
#[inline(never)]
fn next_boxed<'de, A, T>(map: &mut A) -> Result<Box<T>, A::Error>
where
    A: _serde::de::ClassIndexAccess<'de>,
    T: _serde::Deserialize<'de>,
{
    map.next_value().map(Box::new)
}
impl<'a, 'de: 'a> _serde::Deserialize<'de> for Classes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: _serde::Deserializer<'de>,
    {
        struct ClassesVisitor<'a> {
            marker: core::marker::PhantomData<Classes<'a>>,
        }
        impl<'a, 'de: 'a> _serde::de::Visitor<'de> for ClassesVisitor<'a> {
            type Value = Classes<'a>;
            fn expecting(
                &self,
                formatter: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
                formatter.write_str("a valid class enum")
            }
            fn visit_class_index<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: _serde::de::ClassIndexAccess<'de>,
            {
                let class_name = map.next_key()?;
                match class_name {
                    "BGSGamebryoSequenceGenerator" => {
                        Ok(Classes::BGSGamebryoSequenceGenerator(next_boxed(&mut map)?))
                    }
                    "BSAlignBoneModifier" => {
                        Ok(Classes::BSAlignBoneModifier(next_boxed(&mut map)?))
                    }
                    "BSAssignVariablesModifier" => {
                        Ok(Classes::BSAssignVariablesModifier(next_boxed(&mut map)?))
                    }
                    "BSBehaviorGraphSwapGenerator" => {
                        Ok(Classes::BSBehaviorGraphSwapGenerator(next_boxed(&mut map)?))
                    }
                    "BSBoneSwitchGenerator" => {
                        Ok(Classes::BSBoneSwitchGenerator(next_boxed(&mut map)?))
                    }
                    "BSBoneSwitchGeneratorBoneData" => {
                        Ok(Classes::BSBoneSwitchGeneratorBoneData(next_boxed(&mut map)?))
                    }
                    "BSComputeAddBoneAnimModifier" => {
                        Ok(Classes::BSComputeAddBoneAnimModifier(next_boxed(&mut map)?))
                    }
                    "BSCyclicBlendTransitionGenerator" => {
                        Ok(
                            Classes::BSCyclicBlendTransitionGenerator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "BSDecomposeVectorModifier" => {
                        Ok(Classes::BSDecomposeVectorModifier(next_boxed(&mut map)?))
                    }
                    "BSDirectAtCapturePoseModifier" => {
                        Ok(Classes::BSDirectAtCapturePoseModifier(next_boxed(&mut map)?))
                    }
                    "BSDirectAtModifier" => {
                        Ok(Classes::BSDirectAtModifier(next_boxed(&mut map)?))
                    }
                    "BSDistTriggerModifier" => {
                        Ok(Classes::BSDistTriggerModifier(next_boxed(&mut map)?))
                    }
                    "BSEventEveryNEventsModifier" => {
                        Ok(Classes::BSEventEveryNEventsModifier(next_boxed(&mut map)?))
                    }
                    "BSEventOnDeactivateModifier" => {
                        Ok(Classes::BSEventOnDeactivateModifier(next_boxed(&mut map)?))
                    }
                    "BSEventOnFalseToTrueModifier" => {
                        Ok(Classes::BSEventOnFalseToTrueModifier(next_boxed(&mut map)?))
                    }
                    "BSGetTimeStepModifier" => {
                        Ok(Classes::BSGetTimeStepModifier(next_boxed(&mut map)?))
                    }
                    "BSInterpValueModifier" => {
                        Ok(Classes::BSInterpValueModifier(next_boxed(&mut map)?))
                    }
                    "BSIsActiveModifier" => {
                        Ok(Classes::BSIsActiveModifier(next_boxed(&mut map)?))
                    }
                    "BSIStateManagerModifier" => {
                        Ok(Classes::BSIStateManagerModifier(next_boxed(&mut map)?))
                    }
                    "BSIStateManagerModifierBSiStateData" => {
                        Ok(
                            Classes::BSIStateManagerModifierBSiStateData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "BSIStateManagerModifierBSIStateManagerStateListener" => {
                        Ok(
                            Classes::BSIStateManagerModifierBSIStateManagerStateListener(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "BSiStateTaggingGenerator" => {
                        Ok(Classes::BSiStateTaggingGenerator(next_boxed(&mut map)?))
                    }
                    "BSiStateTaggingStateListener" => {
                        Ok(Classes::BSiStateTaggingStateListener(next_boxed(&mut map)?))
                    }
                    "BSLimbCycleModifier" => {
                        Ok(Classes::BSLimbCycleModifier(next_boxed(&mut map)?))
                    }
                    "BSLimbIKModifier" => {
                        Ok(Classes::BSLimbIKModifier(next_boxed(&mut map)?))
                    }
                    "BSLookAtCapturePoseModifier" => {
                        Ok(Classes::BSLookAtCapturePoseModifier(next_boxed(&mut map)?))
                    }
                    "BSLookAtModifier" => {
                        Ok(Classes::BSLookAtModifier(next_boxed(&mut map)?))
                    }
                    "BSLookAtModifierBoneData" => {
                        Ok(Classes::BSLookAtModifierBoneData(next_boxed(&mut map)?))
                    }
                    "BSModifyOnceModifier" => {
                        Ok(Classes::BSModifyOnceModifier(next_boxed(&mut map)?))
                    }
                    "BSOffsetAnimationGenerator" => {
                        Ok(Classes::BSOffsetAnimationGenerator(next_boxed(&mut map)?))
                    }
                    "BSPassByTargetTriggerModifier" => {
                        Ok(Classes::BSPassByTargetTriggerModifier(next_boxed(&mut map)?))
                    }
                    "BSRagdollContactListenerModifier" => {
                        Ok(
                            Classes::BSRagdollContactListenerModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "BSRandomAlarmModifier" => {
                        Ok(Classes::BSRandomAlarmModifier(next_boxed(&mut map)?))
                    }
                    "BSRetargetBodyBlend" => {
                        Ok(Classes::BSRetargetBodyBlend(next_boxed(&mut map)?))
                    }
                    "BSReverseSpineTwistModifier" => {
                        Ok(Classes::BSReverseSpineTwistModifier(next_boxed(&mut map)?))
                    }
                    "BSRootTwistModifier" => {
                        Ok(Classes::BSRootTwistModifier(next_boxed(&mut map)?))
                    }
                    "BSSpeedSamplerModifier" => {
                        Ok(Classes::BSSpeedSamplerModifier(next_boxed(&mut map)?))
                    }
                    "BSSpringState" => Ok(Classes::BSSpringState(next_boxed(&mut map)?)),
                    "BSTimerModifier" => {
                        Ok(Classes::BSTimerModifier(next_boxed(&mut map)?))
                    }
                    "BSTweenerModifier" => {
                        Ok(Classes::BSTweenerModifier(next_boxed(&mut map)?))
                    }
                    "DynamicAnimationTaggingGenerator" => {
                        Ok(
                            Classes::DynamicAnimationTaggingGenerator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclAction" => Ok(Classes::hclAction(next_boxed(&mut map)?)),
                    "hclAntiPinchConstraintSet" => {
                        Ok(Classes::hclAntiPinchConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclAntiPinchConstraintSetPerParticle" => {
                        Ok(
                            Classes::hclAntiPinchConstraintSetPerParticle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendLinkConstraintSet" => {
                        Ok(Classes::hclBendLinkConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclBendLinkConstraintSetLink" => {
                        Ok(Classes::hclBendLinkConstraintSetLink(next_boxed(&mut map)?))
                    }
                    "hclBendLinkConstraintSetMx" => {
                        Ok(Classes::hclBendLinkConstraintSetMx(next_boxed(&mut map)?))
                    }
                    "hclBendLinkConstraintSetMxBatch" => {
                        Ok(
                            Classes::hclBendLinkConstraintSetMxBatch(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendLinkConstraintSetMxSingle" => {
                        Ok(
                            Classes::hclBendLinkConstraintSetMxSingle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendStiffnessConstraintSet" => {
                        Ok(Classes::hclBendStiffnessConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclBendStiffnessConstraintSetLink" => {
                        Ok(
                            Classes::hclBendStiffnessConstraintSetLink(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendStiffnessConstraintSetMx" => {
                        Ok(
                            Classes::hclBendStiffnessConstraintSetMx(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendStiffnessConstraintSetMxBatch" => {
                        Ok(
                            Classes::hclBendStiffnessConstraintSetMxBatch(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendStiffnessConstraintSetMxSingle" => {
                        Ok(
                            Classes::hclBendStiffnessConstraintSetMxSingle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBendStiffnessSetupObject" => {
                        Ok(Classes::hclBendStiffnessSetupObject(next_boxed(&mut map)?))
                    }
                    "hclBlendSomeVerticesOperator" => {
                        Ok(Classes::hclBlendSomeVerticesOperator(next_boxed(&mut map)?))
                    }
                    "hclBlendSomeVerticesOperatorBlendEntry" => {
                        Ok(
                            Classes::hclBlendSomeVerticesOperatorBlendEntry(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBonePlanesConstraintSet" => {
                        Ok(Classes::hclBonePlanesConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclBonePlanesConstraintSetBonePlane" => {
                        Ok(
                            Classes::hclBonePlanesConstraintSetBonePlane(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformer" => {
                        Ok(Classes::hclBoneSpaceDeformer(next_boxed(&mut map)?))
                    }
                    "hclBoneSpaceDeformerFourBlendEntryBlock" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerFourBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockP" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockP(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockPN" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockPN(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockPNT" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockPNT(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockPNTB" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockPNTB(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockUnpackedP" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockUnpackedP(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockUnpackedPN" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPN(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockUnpackedPNT" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNT(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerLocalBlockUnpackedPNTB" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerLocalBlockUnpackedPNTB(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerOneBlendEntryBlock" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerOneBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerThreeBlendEntryBlock" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerThreeBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceDeformerTwoBlendEntryBlock" => {
                        Ok(
                            Classes::hclBoneSpaceDeformerTwoBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceMeshMeshDeformOperator" => {
                        Ok(
                            Classes::hclBoneSpaceMeshMeshDeformOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceMeshMeshDeformPNOperator" => {
                        Ok(
                            Classes::hclBoneSpaceMeshMeshDeformPNOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceMeshMeshDeformPNTBOperator" => {
                        Ok(
                            Classes::hclBoneSpaceMeshMeshDeformPNTBOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceMeshMeshDeformPNTOperator" => {
                        Ok(
                            Classes::hclBoneSpaceMeshMeshDeformPNTOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceMeshMeshDeformPOperator" => {
                        Ok(
                            Classes::hclBoneSpaceMeshMeshDeformPOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclBoneSpaceSkinOperator" => {
                        Ok(Classes::hclBoneSpaceSkinOperator(next_boxed(&mut map)?))
                    }
                    "hclBoneSpaceSkinPNOperator" => {
                        Ok(Classes::hclBoneSpaceSkinPNOperator(next_boxed(&mut map)?))
                    }
                    "hclBoneSpaceSkinPNTBOperator" => {
                        Ok(Classes::hclBoneSpaceSkinPNTBOperator(next_boxed(&mut map)?))
                    }
                    "hclBoneSpaceSkinPNTOperator" => {
                        Ok(Classes::hclBoneSpaceSkinPNTOperator(next_boxed(&mut map)?))
                    }
                    "hclBoneSpaceSkinPOperator" => {
                        Ok(Classes::hclBoneSpaceSkinPOperator(next_boxed(&mut map)?))
                    }
                    "hclBufferDefinition" => {
                        Ok(Classes::hclBufferDefinition(next_boxed(&mut map)?))
                    }
                    "hclBufferLayout" => {
                        Ok(Classes::hclBufferLayout(next_boxed(&mut map)?))
                    }
                    "hclBufferLayoutBufferElement" => {
                        Ok(Classes::hclBufferLayoutBufferElement(next_boxed(&mut map)?))
                    }
                    "hclBufferLayoutSlot" => {
                        Ok(Classes::hclBufferLayoutSlot(next_boxed(&mut map)?))
                    }
                    "hclBufferSetupObject" => {
                        Ok(Classes::hclBufferSetupObject(next_boxed(&mut map)?))
                    }
                    "hclBufferUsage" => {
                        Ok(Classes::hclBufferUsage(next_boxed(&mut map)?))
                    }
                    "hclCapsuleShape" => {
                        Ok(Classes::hclCapsuleShape(next_boxed(&mut map)?))
                    }
                    "hclClothContainer" => {
                        Ok(Classes::hclClothContainer(next_boxed(&mut map)?))
                    }
                    "hclClothData" => Ok(Classes::hclClothData(next_boxed(&mut map)?)),
                    "hclClothSetupContainer" => {
                        Ok(Classes::hclClothSetupContainer(next_boxed(&mut map)?))
                    }
                    "hclClothSetupObject" => {
                        Ok(Classes::hclClothSetupObject(next_boxed(&mut map)?))
                    }
                    "hclClothState" => Ok(Classes::hclClothState(next_boxed(&mut map)?)),
                    "hclClothStateBufferAccess" => {
                        Ok(Classes::hclClothStateBufferAccess(next_boxed(&mut map)?))
                    }
                    "hclClothStateSetupObject" => {
                        Ok(Classes::hclClothStateSetupObject(next_boxed(&mut map)?))
                    }
                    "hclClothStateTransformSetAccess" => {
                        Ok(
                            Classes::hclClothStateTransformSetAccess(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclCollidable" => Ok(Classes::hclCollidable(next_boxed(&mut map)?)),
                    "hclCompressibleLinkConstraintSet" => {
                        Ok(
                            Classes::hclCompressibleLinkConstraintSet(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclCompressibleLinkConstraintSetLink" => {
                        Ok(
                            Classes::hclCompressibleLinkConstraintSetLink(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclCompressibleLinkConstraintSetMx" => {
                        Ok(
                            Classes::hclCompressibleLinkConstraintSetMx(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclCompressibleLinkConstraintSetMxBatch" => {
                        Ok(
                            Classes::hclCompressibleLinkConstraintSetMxBatch(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclCompressibleLinkConstraintSetMxSingle" => {
                        Ok(
                            Classes::hclCompressibleLinkConstraintSetMxSingle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclConstraintSet" => {
                        Ok(Classes::hclConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclConstraintSetSetupObject" => {
                        Ok(Classes::hclConstraintSetSetupObject(next_boxed(&mut map)?))
                    }
                    "hclConvexGeometryShape" => {
                        Ok(Classes::hclConvexGeometryShape(next_boxed(&mut map)?))
                    }
                    "hclConvexHeightFieldShape" => {
                        Ok(Classes::hclConvexHeightFieldShape(next_boxed(&mut map)?))
                    }
                    "hclConvexPlanesShape" => {
                        Ok(Classes::hclConvexPlanesShape(next_boxed(&mut map)?))
                    }
                    "hclCopyVerticesOperator" => {
                        Ok(Classes::hclCopyVerticesOperator(next_boxed(&mut map)?))
                    }
                    "hclEdgeSelectionInput" => {
                        Ok(Classes::hclEdgeSelectionInput(next_boxed(&mut map)?))
                    }
                    "hclGatherAllVerticesOperator" => {
                        Ok(Classes::hclGatherAllVerticesOperator(next_boxed(&mut map)?))
                    }
                    "hclGatherSomeVerticesOperator" => {
                        Ok(Classes::hclGatherSomeVerticesOperator(next_boxed(&mut map)?))
                    }
                    "hclGatherSomeVerticesOperatorVertexPair" => {
                        Ok(
                            Classes::hclGatherSomeVerticesOperatorVertexPair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclInputConvertOperator" => {
                        Ok(Classes::hclInputConvertOperator(next_boxed(&mut map)?))
                    }
                    "hclLocalRangeConstraintSet" => {
                        Ok(Classes::hclLocalRangeConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclLocalRangeConstraintSetLocalConstraint" => {
                        Ok(
                            Classes::hclLocalRangeConstraintSetLocalConstraint(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclLocalRangeSetupObject" => {
                        Ok(Classes::hclLocalRangeSetupObject(next_boxed(&mut map)?))
                    }
                    "hclMeshBoneDeformOperator" => {
                        Ok(Classes::hclMeshBoneDeformOperator(next_boxed(&mut map)?))
                    }
                    "hclMeshBoneDeformOperatorTriangleBonePair" => {
                        Ok(
                            Classes::hclMeshBoneDeformOperatorTriangleBonePair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclMeshBoneDeformSetupObject" => {
                        Ok(Classes::hclMeshBoneDeformSetupObject(next_boxed(&mut map)?))
                    }
                    "hclMeshMeshDeformOperator" => {
                        Ok(Classes::hclMeshMeshDeformOperator(next_boxed(&mut map)?))
                    }
                    "hclMeshMeshDeformOperatorTriangleVertexPair" => {
                        Ok(
                            Classes::hclMeshMeshDeformOperatorTriangleVertexPair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclMoveFixedParticlesSetupObject" => {
                        Ok(
                            Classes::hclMoveFixedParticlesSetupObject(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclMoveParticlesOperator" => {
                        Ok(Classes::hclMoveParticlesOperator(next_boxed(&mut map)?))
                    }
                    "hclMoveParticlesOperatorVertexParticlePair" => {
                        Ok(
                            Classes::hclMoveParticlesOperatorVertexParticlePair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclNamedSetupMesh" => {
                        Ok(Classes::hclNamedSetupMesh(next_boxed(&mut map)?))
                    }
                    "hclNamedTransformSetSetupObject" => {
                        Ok(
                            Classes::hclNamedTransformSetSetupObject(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformer" => {
                        Ok(Classes::hclObjectSpaceDeformer(next_boxed(&mut map)?))
                    }
                    "hclObjectSpaceDeformerFourBlendEntryBlock" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerFourBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockP" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockP(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockPN" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockPN(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockPNT" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockPNT(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockPNTB" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockPNTB(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockUnpackedP" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockUnpackedP(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockUnpackedPN" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPN(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockUnpackedPNT" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNT(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerLocalBlockUnpackedPNTB" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerLocalBlockUnpackedPNTB(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerOneBlendEntryBlock" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerOneBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerThreeBlendEntryBlock" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerThreeBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceDeformerTwoBlendEntryBlock" => {
                        Ok(
                            Classes::hclObjectSpaceDeformerTwoBlendEntryBlock(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceMeshMeshDeformOperator" => {
                        Ok(
                            Classes::hclObjectSpaceMeshMeshDeformOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceMeshMeshDeformPNOperator" => {
                        Ok(
                            Classes::hclObjectSpaceMeshMeshDeformPNOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceMeshMeshDeformPNTBOperator" => {
                        Ok(
                            Classes::hclObjectSpaceMeshMeshDeformPNTBOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceMeshMeshDeformPNTOperator" => {
                        Ok(
                            Classes::hclObjectSpaceMeshMeshDeformPNTOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceMeshMeshDeformPOperator" => {
                        Ok(
                            Classes::hclObjectSpaceMeshMeshDeformPOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceSkinOperator" => {
                        Ok(Classes::hclObjectSpaceSkinOperator(next_boxed(&mut map)?))
                    }
                    "hclObjectSpaceSkinPNOperator" => {
                        Ok(Classes::hclObjectSpaceSkinPNOperator(next_boxed(&mut map)?))
                    }
                    "hclObjectSpaceSkinPNTBOperator" => {
                        Ok(
                            Classes::hclObjectSpaceSkinPNTBOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclObjectSpaceSkinPNTOperator" => {
                        Ok(Classes::hclObjectSpaceSkinPNTOperator(next_boxed(&mut map)?))
                    }
                    "hclObjectSpaceSkinPOperator" => {
                        Ok(Classes::hclObjectSpaceSkinPOperator(next_boxed(&mut map)?))
                    }
                    "hclOperator" => Ok(Classes::hclOperator(next_boxed(&mut map)?)),
                    "hclOperatorSetupObject" => {
                        Ok(Classes::hclOperatorSetupObject(next_boxed(&mut map)?))
                    }
                    "hclOutputConvertOperator" => {
                        Ok(Classes::hclOutputConvertOperator(next_boxed(&mut map)?))
                    }
                    "hclPlaneShape" => Ok(Classes::hclPlaneShape(next_boxed(&mut map)?)),
                    "hclRuntimeConversionInfo" => {
                        Ok(Classes::hclRuntimeConversionInfo(next_boxed(&mut map)?))
                    }
                    "hclRuntimeConversionInfoElementConversion" => {
                        Ok(
                            Classes::hclRuntimeConversionInfoElementConversion(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclRuntimeConversionInfoSlotConversion" => {
                        Ok(
                            Classes::hclRuntimeConversionInfoSlotConversion(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclScratchBufferDefinition" => {
                        Ok(Classes::hclScratchBufferDefinition(next_boxed(&mut map)?))
                    }
                    "hclScratchBufferSetupObject" => {
                        Ok(Classes::hclScratchBufferSetupObject(next_boxed(&mut map)?))
                    }
                    "hclSetupMesh" => Ok(Classes::hclSetupMesh(next_boxed(&mut map)?)),
                    "hclShadowBufferDefinition" => {
                        Ok(Classes::hclShadowBufferDefinition(next_boxed(&mut map)?))
                    }
                    "hclShape" => Ok(Classes::hclShape(next_boxed(&mut map)?)),
                    "hclSimClothBufferSetupObject" => {
                        Ok(Classes::hclSimClothBufferSetupObject(next_boxed(&mut map)?))
                    }
                    "hclSimClothData" => {
                        Ok(Classes::hclSimClothData(next_boxed(&mut map)?))
                    }
                    "hclSimClothDataCollidablePinchingData" => {
                        Ok(
                            Classes::hclSimClothDataCollidablePinchingData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimClothDataCollidableTransformMap" => {
                        Ok(
                            Classes::hclSimClothDataCollidableTransformMap(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimClothDataLandscapeCollisionData" => {
                        Ok(
                            Classes::hclSimClothDataLandscapeCollisionData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimClothDataOverridableSimulationInfo" => {
                        Ok(
                            Classes::hclSimClothDataOverridableSimulationInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimClothDataParticleData" => {
                        Ok(Classes::hclSimClothDataParticleData(next_boxed(&mut map)?))
                    }
                    "hclSimClothDataTransferMotionData" => {
                        Ok(
                            Classes::hclSimClothDataTransferMotionData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimClothPose" => {
                        Ok(Classes::hclSimClothPose(next_boxed(&mut map)?))
                    }
                    "hclSimClothSetupObject" => {
                        Ok(Classes::hclSimClothSetupObject(next_boxed(&mut map)?))
                    }
                    "hclSimClothSetupObjectPerInstanceCollidable" => {
                        Ok(
                            Classes::hclSimClothSetupObjectPerInstanceCollidable(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimClothSetupObjectTransferMotionSetupData" => {
                        Ok(
                            Classes::hclSimClothSetupObjectTransferMotionSetupData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimpleMeshBoneDeformOperator" => {
                        Ok(
                            Classes::hclSimpleMeshBoneDeformOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimpleMeshBoneDeformOperatorTriangleBonePair" => {
                        Ok(
                            Classes::hclSimpleMeshBoneDeformOperatorTriangleBonePair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSimpleWindAction" => {
                        Ok(Classes::hclSimpleWindAction(next_boxed(&mut map)?))
                    }
                    "hclSimulateOperator" => {
                        Ok(Classes::hclSimulateOperator(next_boxed(&mut map)?))
                    }
                    "hclSimulateSetupObject" => {
                        Ok(Classes::hclSimulateSetupObject(next_boxed(&mut map)?))
                    }
                    "hclSimulationSetupMesh" => {
                        Ok(Classes::hclSimulationSetupMesh(next_boxed(&mut map)?))
                    }
                    "hclSimulationSetupMeshMapOptions" => {
                        Ok(
                            Classes::hclSimulationSetupMeshMapOptions(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclSkinOperator" => {
                        Ok(Classes::hclSkinOperator(next_boxed(&mut map)?))
                    }
                    "hclSkinOperatorBoneInfluence" => {
                        Ok(Classes::hclSkinOperatorBoneInfluence(next_boxed(&mut map)?))
                    }
                    "hclSkinSetupObject" => {
                        Ok(Classes::hclSkinSetupObject(next_boxed(&mut map)?))
                    }
                    "hclSphereShape" => {
                        Ok(Classes::hclSphereShape(next_boxed(&mut map)?))
                    }
                    "hclStandardLinkConstraintSet" => {
                        Ok(Classes::hclStandardLinkConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclStandardLinkConstraintSetLink" => {
                        Ok(
                            Classes::hclStandardLinkConstraintSetLink(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStandardLinkConstraintSetMx" => {
                        Ok(
                            Classes::hclStandardLinkConstraintSetMx(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStandardLinkConstraintSetMxBatch" => {
                        Ok(
                            Classes::hclStandardLinkConstraintSetMxBatch(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStandardLinkConstraintSetMxSingle" => {
                        Ok(
                            Classes::hclStandardLinkConstraintSetMxSingle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStandardLinkSetupObject" => {
                        Ok(Classes::hclStandardLinkSetupObject(next_boxed(&mut map)?))
                    }
                    "hclStaticDisplayBufferSetupObject" => {
                        Ok(
                            Classes::hclStaticDisplayBufferSetupObject(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStaticShadowBufferDefinition" => {
                        Ok(
                            Classes::hclStaticShadowBufferDefinition(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStretchLinkConstraintSet" => {
                        Ok(Classes::hclStretchLinkConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclStretchLinkConstraintSetLink" => {
                        Ok(
                            Classes::hclStretchLinkConstraintSetLink(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStretchLinkConstraintSetMx" => {
                        Ok(Classes::hclStretchLinkConstraintSetMx(next_boxed(&mut map)?))
                    }
                    "hclStretchLinkConstraintSetMxBatch" => {
                        Ok(
                            Classes::hclStretchLinkConstraintSetMxBatch(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStretchLinkConstraintSetMxSingle" => {
                        Ok(
                            Classes::hclStretchLinkConstraintSetMxSingle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclStretchLinkSetupObject" => {
                        Ok(Classes::hclStretchLinkSetupObject(next_boxed(&mut map)?))
                    }
                    "hclTaperedCapsuleShape" => {
                        Ok(Classes::hclTaperedCapsuleShape(next_boxed(&mut map)?))
                    }
                    "hclTransformSetDefinition" => {
                        Ok(Classes::hclTransformSetDefinition(next_boxed(&mut map)?))
                    }
                    "hclTransformSetSetupObject" => {
                        Ok(Classes::hclTransformSetSetupObject(next_boxed(&mut map)?))
                    }
                    "hclTransformSetUsage" => {
                        Ok(Classes::hclTransformSetUsage(next_boxed(&mut map)?))
                    }
                    "hclTransformSetUsageTransformTracker" => {
                        Ok(
                            Classes::hclTransformSetUsageTransformTracker(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclTransitionConstraintSet" => {
                        Ok(Classes::hclTransitionConstraintSet(next_boxed(&mut map)?))
                    }
                    "hclTransitionConstraintSetPerParticle" => {
                        Ok(
                            Classes::hclTransitionConstraintSetPerParticle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclTriangleSelectionInput" => {
                        Ok(Classes::hclTriangleSelectionInput(next_boxed(&mut map)?))
                    }
                    "hclUpdateAllVertexFramesOperator" => {
                        Ok(
                            Classes::hclUpdateAllVertexFramesOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclUpdateSomeVertexFramesOperator" => {
                        Ok(
                            Classes::hclUpdateSomeVertexFramesOperator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclUpdateSomeVertexFramesOperatorTriangle" => {
                        Ok(
                            Classes::hclUpdateSomeVertexFramesOperatorTriangle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclVertexFloatInput" => {
                        Ok(Classes::hclVertexFloatInput(next_boxed(&mut map)?))
                    }
                    "hclVertexGatherSetupObject" => {
                        Ok(Classes::hclVertexGatherSetupObject(next_boxed(&mut map)?))
                    }
                    "hclVertexSelectionInput" => {
                        Ok(Classes::hclVertexSelectionInput(next_boxed(&mut map)?))
                    }
                    "hclVolumeConstraint" => {
                        Ok(Classes::hclVolumeConstraint(next_boxed(&mut map)?))
                    }
                    "hclVolumeConstraintApplyData" => {
                        Ok(Classes::hclVolumeConstraintApplyData(next_boxed(&mut map)?))
                    }
                    "hclVolumeConstraintFrameData" => {
                        Ok(Classes::hclVolumeConstraintFrameData(next_boxed(&mut map)?))
                    }
                    "hclVolumeConstraintMx" => {
                        Ok(Classes::hclVolumeConstraintMx(next_boxed(&mut map)?))
                    }
                    "hclVolumeConstraintMxApplyBatchData" => {
                        Ok(
                            Classes::hclVolumeConstraintMxApplyBatchData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclVolumeConstraintMxApplySingleData" => {
                        Ok(
                            Classes::hclVolumeConstraintMxApplySingleData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclVolumeConstraintMxFrameBatchData" => {
                        Ok(
                            Classes::hclVolumeConstraintMxFrameBatchData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclVolumeConstraintMxFrameSingleData" => {
                        Ok(
                            Classes::hclVolumeConstraintMxFrameSingleData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hclVolumeConstraintSetupObject" => {
                        Ok(
                            Classes::hclVolumeConstraintSetupObject(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkAabb" => Ok(Classes::hkAabb(next_boxed(&mut map)?)),
                    "hkAabb16" => Ok(Classes::hkAabb16(next_boxed(&mut map)?)),
                    "hkAabbHalf" => Ok(Classes::hkAabbHalf(next_boxed(&mut map)?)),
                    "hkAabbUint32" => Ok(Classes::hkAabbUint32(next_boxed(&mut map)?)),
                    "hkaAngularReferenceFrame" => {
                        Ok(Classes::hkaAngularReferenceFrame(next_boxed(&mut map)?))
                    }
                    "hkaAnimatedReferenceFrame" => {
                        Ok(Classes::hkaAnimatedReferenceFrame(next_boxed(&mut map)?))
                    }
                    "hkaAnimation" => Ok(Classes::hkaAnimation(next_boxed(&mut map)?)),
                    "hkaAnimationBinding" => {
                        Ok(Classes::hkaAnimationBinding(next_boxed(&mut map)?))
                    }
                    "hkaAnimationContainer" => {
                        Ok(Classes::hkaAnimationContainer(next_boxed(&mut map)?))
                    }
                    "hkaAnimationPreviewColorContainer" => {
                        Ok(
                            Classes::hkaAnimationPreviewColorContainer(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaAnnotationTrack" => {
                        Ok(Classes::hkaAnnotationTrack(next_boxed(&mut map)?))
                    }
                    "hkaAnnotationTrackAnnotation" => {
                        Ok(Classes::hkaAnnotationTrackAnnotation(next_boxed(&mut map)?))
                    }
                    "hkaBone" => Ok(Classes::hkaBone(next_boxed(&mut map)?)),
                    "hkaBoneAttachment" => {
                        Ok(Classes::hkaBoneAttachment(next_boxed(&mut map)?))
                    }
                    "hkaDefaultAnimatedReferenceFrame" => {
                        Ok(
                            Classes::hkaDefaultAnimatedReferenceFrame(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaDirectionalReferenceFrame" => {
                        Ok(Classes::hkaDirectionalReferenceFrame(next_boxed(&mut map)?))
                    }
                    "hkaFootstepAnalysisInfo" => {
                        Ok(Classes::hkaFootstepAnalysisInfo(next_boxed(&mut map)?))
                    }
                    "hkaFootstepAnalysisInfoContainer" => {
                        Ok(
                            Classes::hkaFootstepAnalysisInfoContainer(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaInterleavedUncompressedAnimation" => {
                        Ok(
                            Classes::hkaInterleavedUncompressedAnimation(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaLosslessCompressedAnimation" => {
                        Ok(
                            Classes::hkaLosslessCompressedAnimation(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaMeshBinding" => {
                        Ok(Classes::hkaMeshBinding(next_boxed(&mut map)?))
                    }
                    "hkaMeshBindingMapping" => {
                        Ok(Classes::hkaMeshBindingMapping(next_boxed(&mut map)?))
                    }
                    "hkaParameterizedAnimationReferenceFrame" => {
                        Ok(
                            Classes::hkaParameterizedAnimationReferenceFrame(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaParameterizedReferenceFrame" => {
                        Ok(
                            Classes::hkaParameterizedReferenceFrame(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaPredictiveCompressedAnimation" => {
                        Ok(
                            Classes::hkaPredictiveCompressedAnimation(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaPredictiveCompressedAnimationTrackCompressionParams" => {
                        Ok(
                            Classes::hkaPredictiveCompressedAnimationTrackCompressionParams(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaQuantizedAnimation" => {
                        Ok(Classes::hkaQuantizedAnimation(next_boxed(&mut map)?))
                    }
                    "hkaQuantizedAnimationTrackCompressionParams" => {
                        Ok(
                            Classes::hkaQuantizedAnimationTrackCompressionParams(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaReferencePoseAnimation" => {
                        Ok(Classes::hkaReferencePoseAnimation(next_boxed(&mut map)?))
                    }
                    "hkArrayTypeAttribute" => {
                        Ok(Classes::hkArrayTypeAttribute(next_boxed(&mut map)?))
                    }
                    "hkaSkeleton" => Ok(Classes::hkaSkeleton(next_boxed(&mut map)?)),
                    "hkaSkeletonLocalFrameOnBone" => {
                        Ok(Classes::hkaSkeletonLocalFrameOnBone(next_boxed(&mut map)?))
                    }
                    "hkaSkeletonMapper" => {
                        Ok(Classes::hkaSkeletonMapper(next_boxed(&mut map)?))
                    }
                    "hkaSkeletonMapperData" => {
                        Ok(Classes::hkaSkeletonMapperData(next_boxed(&mut map)?))
                    }
                    "hkaSkeletonMapperDataChainMapping" => {
                        Ok(
                            Classes::hkaSkeletonMapperDataChainMapping(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaSkeletonMapperDataPartitionMappingRange" => {
                        Ok(
                            Classes::hkaSkeletonMapperDataPartitionMappingRange(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaSkeletonMapperDataSimpleMapping" => {
                        Ok(
                            Classes::hkaSkeletonMapperDataSimpleMapping(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaSkeletonPartition" => {
                        Ok(Classes::hkaSkeletonPartition(next_boxed(&mut map)?))
                    }
                    "hkaSplineCompressedAnimation" => {
                        Ok(Classes::hkaSplineCompressedAnimation(next_boxed(&mut map)?))
                    }
                    "hkaSplineCompressedAnimationAnimationCompressionParams" => {
                        Ok(
                            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkaSplineCompressedAnimationTrackCompressionParams" => {
                        Ok(
                            Classes::hkaSplineCompressedAnimationTrackCompressionParams(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbAiControlCancelPathCommand" => {
                        Ok(Classes::hkbAiControlCancelPathCommand(next_boxed(&mut map)?))
                    }
                    "hkbAiControlControlData" => {
                        Ok(Classes::hkbAiControlControlData(next_boxed(&mut map)?))
                    }
                    "hkbAiControlControlDataBlendable" => {
                        Ok(
                            Classes::hkbAiControlControlDataBlendable(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbAiControlControlDataNonBlendable" => {
                        Ok(
                            Classes::hkbAiControlControlDataNonBlendable(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbAiControlPathToCommand" => {
                        Ok(Classes::hkbAiControlPathToCommand(next_boxed(&mut map)?))
                    }
                    "hkbAlignBoneModifier" => {
                        Ok(Classes::hkbAlignBoneModifier(next_boxed(&mut map)?))
                    }
                    "hkbAnimatedSkeletonGenerator" => {
                        Ok(Classes::hkbAnimatedSkeletonGenerator(next_boxed(&mut map)?))
                    }
                    "hkBaseObject" => Ok(Classes::hkBaseObject(next_boxed(&mut map)?)),
                    "hkbAssetBundle" => {
                        Ok(Classes::hkbAssetBundle(next_boxed(&mut map)?))
                    }
                    "hkbAssetBundleStringData" => {
                        Ok(Classes::hkbAssetBundleStringData(next_boxed(&mut map)?))
                    }
                    "hkbAttachmentModifier" => {
                        Ok(Classes::hkbAttachmentModifier(next_boxed(&mut map)?))
                    }
                    "hkbAttachmentSetup" => {
                        Ok(Classes::hkbAttachmentSetup(next_boxed(&mut map)?))
                    }
                    "hkbAttributeModifier" => {
                        Ok(Classes::hkbAttributeModifier(next_boxed(&mut map)?))
                    }
                    "hkbAttributeModifierAssignment" => {
                        Ok(
                            Classes::hkbAttributeModifierAssignment(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbAuxiliaryNodeInfo" => {
                        Ok(Classes::hkbAuxiliaryNodeInfo(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorEventsInfo" => {
                        Ok(Classes::hkbBehaviorEventsInfo(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorGraph" => {
                        Ok(Classes::hkbBehaviorGraph(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorGraphData" => {
                        Ok(Classes::hkbBehaviorGraphData(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorGraphInternalState" => {
                        Ok(Classes::hkbBehaviorGraphInternalState(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorGraphInternalStateInfo" => {
                        Ok(
                            Classes::hkbBehaviorGraphInternalStateInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbBehaviorGraphStringData" => {
                        Ok(Classes::hkbBehaviorGraphStringData(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorInfo" => {
                        Ok(Classes::hkbBehaviorInfo(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorInfoIdToNamePair" => {
                        Ok(Classes::hkbBehaviorInfoIdToNamePair(next_boxed(&mut map)?))
                    }
                    "hkbBehaviorReferenceGenerator" => {
                        Ok(Classes::hkbBehaviorReferenceGenerator(next_boxed(&mut map)?))
                    }
                    "hkbBindable" => Ok(Classes::hkbBindable(next_boxed(&mut map)?)),
                    "hkbBlendCurveUtils" => {
                        Ok(Classes::hkbBlendCurveUtils(next_boxed(&mut map)?))
                    }
                    "hkbBlenderGenerator" => {
                        Ok(Classes::hkbBlenderGenerator(next_boxed(&mut map)?))
                    }
                    "hkbBlenderGeneratorChild" => {
                        Ok(Classes::hkbBlenderGeneratorChild(next_boxed(&mut map)?))
                    }
                    "hkbBlenderGeneratorChildInternalState" => {
                        Ok(
                            Classes::hkbBlenderGeneratorChildInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbBlenderGeneratorInternalState" => {
                        Ok(
                            Classes::hkbBlenderGeneratorInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbBlendingTransitionEffect" => {
                        Ok(Classes::hkbBlendingTransitionEffect(next_boxed(&mut map)?))
                    }
                    "hkbBlendingTransitionEffectInternalState" => {
                        Ok(
                            Classes::hkbBlendingTransitionEffectInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbBoneIndexArray" => {
                        Ok(Classes::hkbBoneIndexArray(next_boxed(&mut map)?))
                    }
                    "hkbBoneWeightArray" => {
                        Ok(Classes::hkbBoneWeightArray(next_boxed(&mut map)?))
                    }
                    "hkbBoolVariableSequencedData" => {
                        Ok(Classes::hkbBoolVariableSequencedData(next_boxed(&mut map)?))
                    }
                    "hkbBoolVariableSequencedDataSample" => {
                        Ok(
                            Classes::hkbBoolVariableSequencedDataSample(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCameraShakeEventPayload" => {
                        Ok(Classes::hkbCameraShakeEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbCameraVariablesChangedCommand" => {
                        Ok(
                            Classes::hkbCameraVariablesChangedCommand(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCharacter" => Ok(Classes::hkbCharacter(next_boxed(&mut map)?)),
                    "hkbCharacterAddedInfo" => {
                        Ok(Classes::hkbCharacterAddedInfo(next_boxed(&mut map)?))
                    }
                    "hkbCharacterControlCommand" => {
                        Ok(Classes::hkbCharacterControlCommand(next_boxed(&mut map)?))
                    }
                    "hkbCharacterControllerModifier" => {
                        Ok(
                            Classes::hkbCharacterControllerModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCharacterControllerModifierControlData" => {
                        Ok(
                            Classes::hkbCharacterControllerModifierControlData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCharacterControllerModifierInternalState" => {
                        Ok(
                            Classes::hkbCharacterControllerModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCharacterControllerSetup" => {
                        Ok(Classes::hkbCharacterControllerSetup(next_boxed(&mut map)?))
                    }
                    "hkbCharacterData" => {
                        Ok(Classes::hkbCharacterData(next_boxed(&mut map)?))
                    }
                    "hkbCharacterInfo" => {
                        Ok(Classes::hkbCharacterInfo(next_boxed(&mut map)?))
                    }
                    "hkbCharacterSelectedInfo" => {
                        Ok(Classes::hkbCharacterSelectedInfo(next_boxed(&mut map)?))
                    }
                    "hkbCharacterSetup" => {
                        Ok(Classes::hkbCharacterSetup(next_boxed(&mut map)?))
                    }
                    "hkbCharacterSkinInfo" => {
                        Ok(Classes::hkbCharacterSkinInfo(next_boxed(&mut map)?))
                    }
                    "hkbCharacterSteppedInfo" => {
                        Ok(Classes::hkbCharacterSteppedInfo(next_boxed(&mut map)?))
                    }
                    "hkbCharacterStringData" => {
                        Ok(Classes::hkbCharacterStringData(next_boxed(&mut map)?))
                    }
                    "hkbCharacterStringDataFileNameMeshNamePair" => {
                        Ok(
                            Classes::hkbCharacterStringDataFileNameMeshNamePair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbClientCharacterState" => {
                        Ok(Classes::hkbClientCharacterState(next_boxed(&mut map)?))
                    }
                    "hkbClipGenerator" => {
                        Ok(Classes::hkbClipGenerator(next_boxed(&mut map)?))
                    }
                    "hkbClipGeneratorEcho" => {
                        Ok(Classes::hkbClipGeneratorEcho(next_boxed(&mut map)?))
                    }
                    "hkbClipGeneratorInternalState" => {
                        Ok(Classes::hkbClipGeneratorInternalState(next_boxed(&mut map)?))
                    }
                    "hkbClipTrigger" => {
                        Ok(Classes::hkbClipTrigger(next_boxed(&mut map)?))
                    }
                    "hkbClipTriggerArray" => {
                        Ok(Classes::hkbClipTriggerArray(next_boxed(&mut map)?))
                    }
                    "hkbCombineTransformsModifier" => {
                        Ok(Classes::hkbCombineTransformsModifier(next_boxed(&mut map)?))
                    }
                    "hkbCombineTransformsModifierInternalState" => {
                        Ok(
                            Classes::hkbCombineTransformsModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCompiledExpressionSet" => {
                        Ok(Classes::hkbCompiledExpressionSet(next_boxed(&mut map)?))
                    }
                    "hkbCompiledExpressionSetToken" => {
                        Ok(Classes::hkbCompiledExpressionSetToken(next_boxed(&mut map)?))
                    }
                    "hkbComputeDirectionModifier" => {
                        Ok(Classes::hkbComputeDirectionModifier(next_boxed(&mut map)?))
                    }
                    "hkbComputeDirectionModifierInternalState" => {
                        Ok(
                            Classes::hkbComputeDirectionModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbComputeRotationFromAxisAngleModifier" => {
                        Ok(
                            Classes::hkbComputeRotationFromAxisAngleModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbComputeRotationFromAxisAngleModifierInternalState" => {
                        Ok(
                            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbComputeRotationToTargetModifier" => {
                        Ok(
                            Classes::hkbComputeRotationToTargetModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbComputeRotationToTargetModifierInternalState" => {
                        Ok(
                            Classes::hkbComputeRotationToTargetModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbComputeWorldFromModelModifier" => {
                        Ok(
                            Classes::hkbComputeWorldFromModelModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCondition" => Ok(Classes::hkbCondition(next_boxed(&mut map)?)),
                    "hkbConstraintSetup" => {
                        Ok(Classes::hkbConstraintSetup(next_boxed(&mut map)?))
                    }
                    "hkbContext" => Ok(Classes::hkbContext(next_boxed(&mut map)?)),
                    "hkbCustomIdSelector" => {
                        Ok(Classes::hkbCustomIdSelector(next_boxed(&mut map)?))
                    }
                    "hkbCustomTestGenerator" => {
                        Ok(Classes::hkbCustomTestGenerator(next_boxed(&mut map)?))
                    }
                    "hkbCustomTestGeneratorAnnotatedTypes" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorAnnotatedTypes(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCustomTestGeneratorBoneTypes" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorBoneTypes(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCustomTestGeneratorComplexTypes" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorComplexTypes(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCustomTestGeneratorHiddenTypes" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorHiddenTypes(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCustomTestGeneratorNestedTypes" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorNestedTypes(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCustomTestGeneratorNestedTypesBase" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorNestedTypesBase(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbCustomTestGeneratorSimpleTypes" => {
                        Ok(
                            Classes::hkbCustomTestGeneratorSimpleTypes(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbDampingModifier" => {
                        Ok(Classes::hkbDampingModifier(next_boxed(&mut map)?))
                    }
                    "hkbDampingModifierInternalState" => {
                        Ok(
                            Classes::hkbDampingModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbDelayedModifier" => {
                        Ok(Classes::hkbDelayedModifier(next_boxed(&mut map)?))
                    }
                    "hkbDelayedModifierInternalState" => {
                        Ok(
                            Classes::hkbDelayedModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbDetectCloseToGroundModifier" => {
                        Ok(
                            Classes::hkbDetectCloseToGroundModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbDetectCloseToGroundModifierInternalState" => {
                        Ok(
                            Classes::hkbDetectCloseToGroundModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbDockingGenerator" => {
                        Ok(Classes::hkbDockingGenerator(next_boxed(&mut map)?))
                    }
                    "hkbDockingGeneratorInternalState" => {
                        Ok(
                            Classes::hkbDockingGeneratorInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbEvaluateExpressionModifier" => {
                        Ok(Classes::hkbEvaluateExpressionModifier(next_boxed(&mut map)?))
                    }
                    "hkbEvaluateExpressionModifierInternalExpressionData" => {
                        Ok(
                            Classes::hkbEvaluateExpressionModifierInternalExpressionData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbEvaluateExpressionModifierInternalState" => {
                        Ok(
                            Classes::hkbEvaluateExpressionModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbEvaluateHandleModifier" => {
                        Ok(Classes::hkbEvaluateHandleModifier(next_boxed(&mut map)?))
                    }
                    "hkbEvent" => Ok(Classes::hkbEvent(next_boxed(&mut map)?)),
                    "hkbEventBase" => Ok(Classes::hkbEventBase(next_boxed(&mut map)?)),
                    "hkbEventDrivenModifier" => {
                        Ok(Classes::hkbEventDrivenModifier(next_boxed(&mut map)?))
                    }
                    "hkbEventDrivenModifierInternalState" => {
                        Ok(
                            Classes::hkbEventDrivenModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbEventInfo" => Ok(Classes::hkbEventInfo(next_boxed(&mut map)?)),
                    "hkbEventPayload" => {
                        Ok(Classes::hkbEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbEventPayloadList" => {
                        Ok(Classes::hkbEventPayloadList(next_boxed(&mut map)?))
                    }
                    "hkbEventProperty" => {
                        Ok(Classes::hkbEventProperty(next_boxed(&mut map)?))
                    }
                    "hkbEventRaisedInfo" => {
                        Ok(Classes::hkbEventRaisedInfo(next_boxed(&mut map)?))
                    }
                    "hkbEventRangeData" => {
                        Ok(Classes::hkbEventRangeData(next_boxed(&mut map)?))
                    }
                    "hkbEventRangeDataArray" => {
                        Ok(Classes::hkbEventRangeDataArray(next_boxed(&mut map)?))
                    }
                    "hkbEventSequencedData" => {
                        Ok(Classes::hkbEventSequencedData(next_boxed(&mut map)?))
                    }
                    "hkbEventSequencedDataSequencedEvent" => {
                        Ok(
                            Classes::hkbEventSequencedDataSequencedEvent(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbEventsFromRangeModifier" => {
                        Ok(Classes::hkbEventsFromRangeModifier(next_boxed(&mut map)?))
                    }
                    "hkbEventsFromRangeModifierInternalState" => {
                        Ok(
                            Classes::hkbEventsFromRangeModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbExpressionCondition" => {
                        Ok(Classes::hkbExpressionCondition(next_boxed(&mut map)?))
                    }
                    "hkbExpressionData" => {
                        Ok(Classes::hkbExpressionData(next_boxed(&mut map)?))
                    }
                    "hkbExpressionDataArray" => {
                        Ok(Classes::hkbExpressionDataArray(next_boxed(&mut map)?))
                    }
                    "hkbExtractRagdollPoseModifier" => {
                        Ok(Classes::hkbExtractRagdollPoseModifier(next_boxed(&mut map)?))
                    }
                    "hkbExtrapolatingTransitionEffect" => {
                        Ok(
                            Classes::hkbExtrapolatingTransitionEffect(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbExtrapolatingTransitionEffectInternalState" => {
                        Ok(
                            Classes::hkbExtrapolatingTransitionEffectInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbFootIkControlData" => {
                        Ok(Classes::hkbFootIkControlData(next_boxed(&mut map)?))
                    }
                    "hkbFootIkControlsModifier" => {
                        Ok(Classes::hkbFootIkControlsModifier(next_boxed(&mut map)?))
                    }
                    "hkbFootIkControlsModifierLeg" => {
                        Ok(Classes::hkbFootIkControlsModifierLeg(next_boxed(&mut map)?))
                    }
                    "hkbFootIkDriverInfo" => {
                        Ok(Classes::hkbFootIkDriverInfo(next_boxed(&mut map)?))
                    }
                    "hkbFootIkDriverInfoLeg" => {
                        Ok(Classes::hkbFootIkDriverInfoLeg(next_boxed(&mut map)?))
                    }
                    "hkbFootIkGains" => {
                        Ok(Classes::hkbFootIkGains(next_boxed(&mut map)?))
                    }
                    "hkbFootIkModifier" => {
                        Ok(Classes::hkbFootIkModifier(next_boxed(&mut map)?))
                    }
                    "hkbFootIkModifierInternalLegData" => {
                        Ok(
                            Classes::hkbFootIkModifierInternalLegData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbFootIkModifierLeg" => {
                        Ok(Classes::hkbFootIkModifierLeg(next_boxed(&mut map)?))
                    }
                    "hkbGenerator" => Ok(Classes::hkbGenerator(next_boxed(&mut map)?)),
                    "hkbGeneratorOutputListener" => {
                        Ok(Classes::hkbGeneratorOutputListener(next_boxed(&mut map)?))
                    }
                    "hkbGeneratorPartitionInfo" => {
                        Ok(Classes::hkbGeneratorPartitionInfo(next_boxed(&mut map)?))
                    }
                    "hkbGeneratorSyncInfo" => {
                        Ok(Classes::hkbGeneratorSyncInfo(next_boxed(&mut map)?))
                    }
                    "hkbGeneratorSyncInfoActiveInterval" => {
                        Ok(
                            Classes::hkbGeneratorSyncInfoActiveInterval(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbGeneratorSyncInfoSyncPoint" => {
                        Ok(Classes::hkbGeneratorSyncInfoSyncPoint(next_boxed(&mut map)?))
                    }
                    "hkbGeneratorTransitionEffect" => {
                        Ok(Classes::hkbGeneratorTransitionEffect(next_boxed(&mut map)?))
                    }
                    "hkbGeneratorTransitionEffectInternalState" => {
                        Ok(
                            Classes::hkbGeneratorTransitionEffectInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbGetHandleOnBoneModifier" => {
                        Ok(Classes::hkbGetHandleOnBoneModifier(next_boxed(&mut map)?))
                    }
                    "hkbGetUpModifier" => {
                        Ok(Classes::hkbGetUpModifier(next_boxed(&mut map)?))
                    }
                    "hkbGetUpModifierInternalState" => {
                        Ok(Classes::hkbGetUpModifierInternalState(next_boxed(&mut map)?))
                    }
                    "hkbGetWorldFromModelModifier" => {
                        Ok(Classes::hkbGetWorldFromModelModifier(next_boxed(&mut map)?))
                    }
                    "hkbGetWorldFromModelModifierInternalState" => {
                        Ok(
                            Classes::hkbGetWorldFromModelModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbGravityModifier" => {
                        Ok(Classes::hkbGravityModifier(next_boxed(&mut map)?))
                    }
                    "hkbHandIkControlData" => {
                        Ok(Classes::hkbHandIkControlData(next_boxed(&mut map)?))
                    }
                    "hkbHandIkControlsModifier" => {
                        Ok(Classes::hkbHandIkControlsModifier(next_boxed(&mut map)?))
                    }
                    "hkbHandIkControlsModifierHand" => {
                        Ok(Classes::hkbHandIkControlsModifierHand(next_boxed(&mut map)?))
                    }
                    "hkbHandIkDriverInfo" => {
                        Ok(Classes::hkbHandIkDriverInfo(next_boxed(&mut map)?))
                    }
                    "hkbHandIkDriverInfoHand" => {
                        Ok(Classes::hkbHandIkDriverInfoHand(next_boxed(&mut map)?))
                    }
                    "hkbHandIkModifier" => {
                        Ok(Classes::hkbHandIkModifier(next_boxed(&mut map)?))
                    }
                    "hkbHandIkModifierHand" => {
                        Ok(Classes::hkbHandIkModifierHand(next_boxed(&mut map)?))
                    }
                    "hkbHandle" => Ok(Classes::hkbHandle(next_boxed(&mut map)?)),
                    "hkbHoldFromBlendingTransitionEffect" => {
                        Ok(
                            Classes::hkbHoldFromBlendingTransitionEffect(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbIntEventPayload" => {
                        Ok(Classes::hkbIntEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbIntVariableSequencedData" => {
                        Ok(Classes::hkbIntVariableSequencedData(next_boxed(&mut map)?))
                    }
                    "hkbIntVariableSequencedDataSample" => {
                        Ok(
                            Classes::hkbIntVariableSequencedDataSample(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkBitField" => Ok(Classes::hkBitField(next_boxed(&mut map)?)),
                    "hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator" => {
                        Ok(
                            Classes::hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator" => {
                        Ok(
                            Classes::hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator" => {
                        Ok(
                            Classes::hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbJigglerGroup" => {
                        Ok(Classes::hkbJigglerGroup(next_boxed(&mut map)?))
                    }
                    "hkbJigglerModifier" => {
                        Ok(Classes::hkbJigglerModifier(next_boxed(&mut map)?))
                    }
                    "hkbJigglerModifierInternalState" => {
                        Ok(
                            Classes::hkbJigglerModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbKeyframeBonesModifier" => {
                        Ok(Classes::hkbKeyframeBonesModifier(next_boxed(&mut map)?))
                    }
                    "hkbKeyframeBonesModifierKeyframeInfo" => {
                        Ok(
                            Classes::hkbKeyframeBonesModifierKeyframeInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbKeyFrameControlData" => {
                        Ok(Classes::hkbKeyFrameControlData(next_boxed(&mut map)?))
                    }
                    "hkbLayer" => Ok(Classes::hkbLayer(next_boxed(&mut map)?)),
                    "hkbLayerGenerator" => {
                        Ok(Classes::hkbLayerGenerator(next_boxed(&mut map)?))
                    }
                    "hkbLayerGeneratorInternalState" => {
                        Ok(
                            Classes::hkbLayerGeneratorInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbLayerGeneratorLayerInternalState" => {
                        Ok(
                            Classes::hkbLayerGeneratorLayerInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbLinkedSymbolInfo" => {
                        Ok(Classes::hkbLinkedSymbolInfo(next_boxed(&mut map)?))
                    }
                    "hkbLookAtModifier" => {
                        Ok(Classes::hkbLookAtModifier(next_boxed(&mut map)?))
                    }
                    "hkbLookAtModifierInternalState" => {
                        Ok(
                            Classes::hkbLookAtModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbManualSelectorGenerator" => {
                        Ok(Classes::hkbManualSelectorGenerator(next_boxed(&mut map)?))
                    }
                    "hkbManualSelectorGeneratorInternalState" => {
                        Ok(
                            Classes::hkbManualSelectorGeneratorInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbManualSelectorTransitionEffect" => {
                        Ok(
                            Classes::hkbManualSelectorTransitionEffect(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbManualSelectorTransitionEffectInternalState" => {
                        Ok(
                            Classes::hkbManualSelectorTransitionEffectInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbMessageLog" => Ok(Classes::hkbMessageLog(next_boxed(&mut map)?)),
                    "hkbMirroredSkeletonInfo" => {
                        Ok(Classes::hkbMirroredSkeletonInfo(next_boxed(&mut map)?))
                    }
                    "hkbMirrorModifier" => {
                        Ok(Classes::hkbMirrorModifier(next_boxed(&mut map)?))
                    }
                    "hkbModifier" => Ok(Classes::hkbModifier(next_boxed(&mut map)?)),
                    "hkbModifierGenerator" => {
                        Ok(Classes::hkbModifierGenerator(next_boxed(&mut map)?))
                    }
                    "hkbModifierList" => {
                        Ok(Classes::hkbModifierList(next_boxed(&mut map)?))
                    }
                    "hkbModifierWrapper" => {
                        Ok(Classes::hkbModifierWrapper(next_boxed(&mut map)?))
                    }
                    "hkbMoveBoneAttachmentModifier" => {
                        Ok(Classes::hkbMoveBoneAttachmentModifier(next_boxed(&mut map)?))
                    }
                    "hkbMoveCharacterModifier" => {
                        Ok(Classes::hkbMoveCharacterModifier(next_boxed(&mut map)?))
                    }
                    "hkbMoveCharacterModifierInternalState" => {
                        Ok(
                            Classes::hkbMoveCharacterModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbNamedEventPayload" => {
                        Ok(Classes::hkbNamedEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbNamedIntEventPayload" => {
                        Ok(Classes::hkbNamedIntEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbNamedRealEventPayload" => {
                        Ok(Classes::hkbNamedRealEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbNamedStringEventPayload" => {
                        Ok(Classes::hkbNamedStringEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbNode" => Ok(Classes::hkbNode(next_boxed(&mut map)?)),
                    "hkbNodeInternalStateInfo" => {
                        Ok(Classes::hkbNodeInternalStateInfo(next_boxed(&mut map)?))
                    }
                    "hkbnpPhysicsInterface" => {
                        Ok(Classes::hkbnpPhysicsInterface(next_boxed(&mut map)?))
                    }
                    "hkbnpRagdollInterface" => {
                        Ok(Classes::hkbnpRagdollInterface(next_boxed(&mut map)?))
                    }
                    "hkbNullPhysicsInterface" => {
                        Ok(Classes::hkbNullPhysicsInterface(next_boxed(&mut map)?))
                    }
                    "hkbParametricMotionGenerator" => {
                        Ok(Classes::hkbParametricMotionGenerator(next_boxed(&mut map)?))
                    }
                    "hkbParticleSystemEventPayload" => {
                        Ok(Classes::hkbParticleSystemEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbPhysicsInterface" => {
                        Ok(Classes::hkbPhysicsInterface(next_boxed(&mut map)?))
                    }
                    "hkbPinBonesGenerator" => {
                        Ok(Classes::hkbPinBonesGenerator(next_boxed(&mut map)?))
                    }
                    "hkbPoseMatchingGenerator" => {
                        Ok(Classes::hkbPoseMatchingGenerator(next_boxed(&mut map)?))
                    }
                    "hkbPoseMatchingGeneratorInternalState" => {
                        Ok(
                            Classes::hkbPoseMatchingGeneratorInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbPoseStoringGeneratorOutputListener" => {
                        Ok(
                            Classes::hkbPoseStoringGeneratorOutputListener(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbPoseStoringGeneratorOutputListenerStoredPose" => {
                        Ok(
                            Classes::hkbPoseStoringGeneratorOutputListenerStoredPose(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbPoweredRagdollControlData" => {
                        Ok(Classes::hkbPoweredRagdollControlData(next_boxed(&mut map)?))
                    }
                    "hkbPoweredRagdollControlsModifier" => {
                        Ok(
                            Classes::hkbPoweredRagdollControlsModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbProceduralBlenderGenerator" => {
                        Ok(Classes::hkbProceduralBlenderGenerator(next_boxed(&mut map)?))
                    }
                    "hkbProjectData" => {
                        Ok(Classes::hkbProjectData(next_boxed(&mut map)?))
                    }
                    "hkbProjectStringData" => {
                        Ok(Classes::hkbProjectStringData(next_boxed(&mut map)?))
                    }
                    "hkbRadialSelectorGenerator" => {
                        Ok(Classes::hkbRadialSelectorGenerator(next_boxed(&mut map)?))
                    }
                    "hkbRadialSelectorGeneratorGeneratorInfo" => {
                        Ok(
                            Classes::hkbRadialSelectorGeneratorGeneratorInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbRadialSelectorGeneratorGeneratorPair" => {
                        Ok(
                            Classes::hkbRadialSelectorGeneratorGeneratorPair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbRagdollControllerSetup" => {
                        Ok(Classes::hkbRagdollControllerSetup(next_boxed(&mut map)?))
                    }
                    "hkbRagdollInterface" => {
                        Ok(Classes::hkbRagdollInterface(next_boxed(&mut map)?))
                    }
                    "hkbRaiseEventCommand" => {
                        Ok(Classes::hkbRaiseEventCommand(next_boxed(&mut map)?))
                    }
                    "hkbRealEventPayload" => {
                        Ok(Classes::hkbRealEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbRealVariableSequencedData" => {
                        Ok(Classes::hkbRealVariableSequencedData(next_boxed(&mut map)?))
                    }
                    "hkbRealVariableSequencedDataSample" => {
                        Ok(
                            Classes::hkbRealVariableSequencedDataSample(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbRebuildScriptsCommand" => {
                        Ok(Classes::hkbRebuildScriptsCommand(next_boxed(&mut map)?))
                    }
                    "hkbReferencedGeneratorSyncInfo" => {
                        Ok(
                            Classes::hkbReferencedGeneratorSyncInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbReferencePoseGenerator" => {
                        Ok(Classes::hkbReferencePoseGenerator(next_boxed(&mut map)?))
                    }
                    "hkbRigidBodyRagdollControlData" => {
                        Ok(
                            Classes::hkbRigidBodyRagdollControlData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbRigidBodyRagdollControlsModifier" => {
                        Ok(
                            Classes::hkbRigidBodyRagdollControlsModifier(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbRigidBodySetup" => {
                        Ok(Classes::hkbRigidBodySetup(next_boxed(&mut map)?))
                    }
                    "hkbRoleAttribute" => {
                        Ok(Classes::hkbRoleAttribute(next_boxed(&mut map)?))
                    }
                    "hkbRotateCharacterModifier" => {
                        Ok(Classes::hkbRotateCharacterModifier(next_boxed(&mut map)?))
                    }
                    "hkbRotateCharacterModifierInternalState" => {
                        Ok(
                            Classes::hkbRotateCharacterModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbScriptCondition" => {
                        Ok(Classes::hkbScriptCondition(next_boxed(&mut map)?))
                    }
                    "hkbScriptGenerator" => {
                        Ok(Classes::hkbScriptGenerator(next_boxed(&mut map)?))
                    }
                    "hkbSenseHandleModifier" => {
                        Ok(Classes::hkbSenseHandleModifier(next_boxed(&mut map)?))
                    }
                    "hkbSenseHandleModifierRange" => {
                        Ok(Classes::hkbSenseHandleModifierRange(next_boxed(&mut map)?))
                    }
                    "hkbSequence" => Ok(Classes::hkbSequence(next_boxed(&mut map)?)),
                    "hkbSequencedData" => {
                        Ok(Classes::hkbSequencedData(next_boxed(&mut map)?))
                    }
                    "hkbSequenceInternalState" => {
                        Ok(Classes::hkbSequenceInternalState(next_boxed(&mut map)?))
                    }
                    "hkbSequenceStringData" => {
                        Ok(Classes::hkbSequenceStringData(next_boxed(&mut map)?))
                    }
                    "hkbSetBehaviorCommand" => {
                        Ok(Classes::hkbSetBehaviorCommand(next_boxed(&mut map)?))
                    }
                    "hkbSetLocalTimeOfClipGeneratorCommand" => {
                        Ok(
                            Classes::hkbSetLocalTimeOfClipGeneratorCommand(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbSetNodePropertyCommand" => {
                        Ok(Classes::hkbSetNodePropertyCommand(next_boxed(&mut map)?))
                    }
                    "hkbSetSelectedCharacterCommand" => {
                        Ok(
                            Classes::hkbSetSelectedCharacterCommand(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbSetWordVariableCommand" => {
                        Ok(Classes::hkbSetWordVariableCommand(next_boxed(&mut map)?))
                    }
                    "hkbSetWorldFromModelModifier" => {
                        Ok(Classes::hkbSetWorldFromModelModifier(next_boxed(&mut map)?))
                    }
                    "hkbShapeSetup" => Ok(Classes::hkbShapeSetup(next_boxed(&mut map)?)),
                    "hkbSimulationControlCommand" => {
                        Ok(Classes::hkbSimulationControlCommand(next_boxed(&mut map)?))
                    }
                    "hkbSimulationStateInfo" => {
                        Ok(Classes::hkbSimulationStateInfo(next_boxed(&mut map)?))
                    }
                    "hkbStateChooser" => {
                        Ok(Classes::hkbStateChooser(next_boxed(&mut map)?))
                    }
                    "hkbStateChooserWrapper" => {
                        Ok(Classes::hkbStateChooserWrapper(next_boxed(&mut map)?))
                    }
                    "hkbStateDependentModifier" => {
                        Ok(Classes::hkbStateDependentModifier(next_boxed(&mut map)?))
                    }
                    "hkbStateListener" => {
                        Ok(Classes::hkbStateListener(next_boxed(&mut map)?))
                    }
                    "hkbStateMachine" => {
                        Ok(Classes::hkbStateMachine(next_boxed(&mut map)?))
                    }
                    "hkbStateMachineActiveTransitionInfo" => {
                        Ok(
                            Classes::hkbStateMachineActiveTransitionInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStateMachineDelayedTransitionInfo" => {
                        Ok(
                            Classes::hkbStateMachineDelayedTransitionInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStateMachineEventPropertyArray" => {
                        Ok(
                            Classes::hkbStateMachineEventPropertyArray(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStateMachineInternalState" => {
                        Ok(Classes::hkbStateMachineInternalState(next_boxed(&mut map)?))
                    }
                    "hkbStateMachineNestedStateMachineData" => {
                        Ok(
                            Classes::hkbStateMachineNestedStateMachineData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStateMachineProspectiveTransitionInfo" => {
                        Ok(
                            Classes::hkbStateMachineProspectiveTransitionInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStateMachineStateInfo" => {
                        Ok(Classes::hkbStateMachineStateInfo(next_boxed(&mut map)?))
                    }
                    "hkbStateMachineTimeInterval" => {
                        Ok(Classes::hkbStateMachineTimeInterval(next_boxed(&mut map)?))
                    }
                    "hkbStateMachineTransitionInfo" => {
                        Ok(Classes::hkbStateMachineTransitionInfo(next_boxed(&mut map)?))
                    }
                    "hkbStateMachineTransitionInfoArray" => {
                        Ok(
                            Classes::hkbStateMachineTransitionInfoArray(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStateMachineTransitionInfoReference" => {
                        Ok(
                            Classes::hkbStateMachineTransitionInfoReference(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbStringCondition" => {
                        Ok(Classes::hkbStringCondition(next_boxed(&mut map)?))
                    }
                    "hkbStringEventPayload" => {
                        Ok(Classes::hkbStringEventPayload(next_boxed(&mut map)?))
                    }
                    "hkbTestIdSelector" => {
                        Ok(Classes::hkbTestIdSelector(next_boxed(&mut map)?))
                    }
                    "hkbTimerModifier" => {
                        Ok(Classes::hkbTimerModifier(next_boxed(&mut map)?))
                    }
                    "hkbTimerModifierInternalState" => {
                        Ok(Classes::hkbTimerModifierInternalState(next_boxed(&mut map)?))
                    }
                    "hkbToolNodeType" => {
                        Ok(Classes::hkbToolNodeType(next_boxed(&mut map)?))
                    }
                    "hkbTransformVectorModifier" => {
                        Ok(Classes::hkbTransformVectorModifier(next_boxed(&mut map)?))
                    }
                    "hkbTransformVectorModifierInternalState" => {
                        Ok(
                            Classes::hkbTransformVectorModifierInternalState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkbTransitionEffect" => {
                        Ok(Classes::hkbTransitionEffect(next_boxed(&mut map)?))
                    }
                    "hkbTwistModifier" => {
                        Ok(Classes::hkbTwistModifier(next_boxed(&mut map)?))
                    }
                    "hkbVariableBindingSet" => {
                        Ok(Classes::hkbVariableBindingSet(next_boxed(&mut map)?))
                    }
                    "hkbVariableBindingSetBinding" => {
                        Ok(Classes::hkbVariableBindingSetBinding(next_boxed(&mut map)?))
                    }
                    "hkbVariableBounds" => {
                        Ok(Classes::hkbVariableBounds(next_boxed(&mut map)?))
                    }
                    "hkbVariableInfo" => {
                        Ok(Classes::hkbVariableInfo(next_boxed(&mut map)?))
                    }
                    "hkbVariableValue" => {
                        Ok(Classes::hkbVariableValue(next_boxed(&mut map)?))
                    }
                    "hkbVariableValueSet" => {
                        Ok(Classes::hkbVariableValueSet(next_boxed(&mut map)?))
                    }
                    "hkbWorldEnums" => Ok(Classes::hkbWorldEnums(next_boxed(&mut map)?)),
                    "hkbWorldFromModelModeData" => {
                        Ok(Classes::hkbWorldFromModelModeData(next_boxed(&mut map)?))
                    }
                    "hkcdAdf" => Ok(Classes::hkcdAdf(next_boxed(&mut map)?)),
                    "hkcdDynamicAabbTree" => {
                        Ok(Classes::hkcdDynamicAabbTree(next_boxed(&mut map)?))
                    }
                    "hkcdDynamicTreeAnisotropicMetric" => {
                        Ok(
                            Classes::hkcdDynamicTreeAnisotropicMetric(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeCodec18" => {
                        Ok(Classes::hkcdDynamicTreeCodec18(next_boxed(&mut map)?))
                    }
                    "hkcdDynamicTreeCodec32" => {
                        Ok(Classes::hkcdDynamicTreeCodec32(next_boxed(&mut map)?))
                    }
                    "hkcdDynamicTreeCodecInt16" => {
                        Ok(Classes::hkcdDynamicTreeCodecInt16(next_boxed(&mut map)?))
                    }
                    "hkcdDynamicTreeCodecInt16IntAabb" => {
                        Ok(
                            Classes::hkcdDynamicTreeCodecInt16IntAabb(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeCodecRawUint" => {
                        Ok(Classes::hkcdDynamicTreeCodecRawUint(next_boxed(&mut map)?))
                    }
                    "hkcdDynamicTreeCodecRawUlong" => {
                        Ok(Classes::hkcdDynamicTreeCodecRawUlong(next_boxed(&mut map)?))
                    }
                    "hkcdDynamicTreeCodecRawunsignedint" => {
                        Ok(
                            Classes::hkcdDynamicTreeCodecRawunsignedint(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeCodecRawunsignedlong" => {
                        Ok(
                            Classes::hkcdDynamicTreeCodecRawunsignedlong(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultTree32Storage" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultTree32Storage(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultTree48Storage" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultTree48Storage(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultTreeInt16Storage" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultTreeInt16Storage(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDefaultTreePtrStorage" => {
                        Ok(
                            Classes::hkcdDynamicTreeDefaultTreePtrStorage(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorage16" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorage16(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorage32" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorage32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStorageInt16" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStorageInt16(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeDynamicStoragePtr" => {
                        Ok(
                            Classes::hkcdDynamicTreeDynamicStoragePtr(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16" => {
                        Ok(
                            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32" => {
                        Ok(
                            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16" => {
                        Ok(
                            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr" => {
                        Ok(
                            Classes::hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdFourAabb" => Ok(Classes::hkcdFourAabb(next_boxed(&mut map)?)),
                    "hkcdPlanarCsgOperand" => {
                        Ok(Classes::hkcdPlanarCsgOperand(next_boxed(&mut map)?))
                    }
                    "hkcdPlanarCsgOperandGeomSource" => {
                        Ok(
                            Classes::hkcdPlanarCsgOperandGeomSource(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdPlanarEntity" => {
                        Ok(Classes::hkcdPlanarEntity(next_boxed(&mut map)?))
                    }
                    "hkcdPlanarGeometry" => {
                        Ok(Classes::hkcdPlanarGeometry(next_boxed(&mut map)?))
                    }
                    "hkcdPlanarGeometryPlanesCollection" => {
                        Ok(
                            Classes::hkcdPlanarGeometryPlanesCollection(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdPlanarGeometryPolygonCollection" => {
                        Ok(
                            Classes::hkcdPlanarGeometryPolygonCollection(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdPlanarGeometryPrimitivesCollection28" => {
                        Ok(
                            Classes::hkcdPlanarGeometryPrimitivesCollection28(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdPlanarGeometryPrimitivesPlane" => {
                        Ok(
                            Classes::hkcdPlanarGeometryPrimitivesPlane(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdPlanarSolid" => {
                        Ok(Classes::hkcdPlanarSolid(next_boxed(&mut map)?))
                    }
                    "hkcdPlanarSolidNode" => {
                        Ok(Classes::hkcdPlanarSolidNode(next_boxed(&mut map)?))
                    }
                    "hkcdPlanarSolidNodeStorage" => {
                        Ok(Classes::hkcdPlanarSolidNodeStorage(next_boxed(&mut map)?))
                    }
                    "hkcdShape" => Ok(Classes::hkcdShape(next_boxed(&mut map)?)),
                    "hkcdShapeDispatchType" => {
                        Ok(Classes::hkcdShapeDispatchType(next_boxed(&mut map)?))
                    }
                    "hkcdShapeInfoCodecType" => {
                        Ok(Classes::hkcdShapeInfoCodecType(next_boxed(&mut map)?))
                    }
                    "hkcdShapeType" => Ok(Classes::hkcdShapeType(next_boxed(&mut map)?)),
                    "hkcdSimdTree" => Ok(Classes::hkcdSimdTree(next_boxed(&mut map)?)),
                    "hkcdSimdTreeNode" => {
                        Ok(Classes::hkcdSimdTreeNode(next_boxed(&mut map)?))
                    }
                    "hkcdStaticAabbTree" => {
                        Ok(Classes::hkcdStaticAabbTree(next_boxed(&mut map)?))
                    }
                    "hkcdStaticMeshTreeBase" => {
                        Ok(Classes::hkcdStaticMeshTreeBase(next_boxed(&mut map)?))
                    }
                    "hkcdStaticMeshTreeBasePrimitive" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeBasePrimitive(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreeBaseSection" => {
                        Ok(Classes::hkcdStaticMeshTreeBaseSection(next_boxed(&mut map)?))
                    }
                    "hkcdStaticMeshTreeBaseSectionDataRuns" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeBaseSectionDataRuns(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreeBaseSectionPrimitives" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeBaseSectionPrimitives(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreeBaseSectionSharedVertices" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeBaseSectionSharedVertices(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreeDefaultDataRun" => {
                        Ok(
                            Classes::hkcdStaticMeshTreeDefaultDataRun(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun" => {
                        Ok(
                            Classes::hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticPvs" => Ok(Classes::hkcdStaticPvs(next_boxed(&mut map)?)),
                    "hkcdStaticPvsBlockHeader" => {
                        Ok(Classes::hkcdStaticPvsBlockHeader(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeCodec3Axis" => {
                        Ok(Classes::hkcdStaticTreeCodec3Axis(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeCodec3Axis4" => {
                        Ok(Classes::hkcdStaticTreeCodec3Axis4(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeCodec3Axis5" => {
                        Ok(Classes::hkcdStaticTreeCodec3Axis5(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeCodec3Axis6" => {
                        Ok(Classes::hkcdStaticTreeCodec3Axis6(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeCodecRaw" => {
                        Ok(Classes::hkcdStaticTreeCodecRaw(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeDefaultTreeStorage32" => {
                        Ok(
                            Classes::hkcdStaticTreeDefaultTreeStorage32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDefaultTreeStorage4" => {
                        Ok(
                            Classes::hkcdStaticTreeDefaultTreeStorage4(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDefaultTreeStorage5" => {
                        Ok(
                            Classes::hkcdStaticTreeDefaultTreeStorage5(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDefaultTreeStorage6" => {
                        Ok(
                            Classes::hkcdStaticTreeDefaultTreeStorage6(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDynamicStorage32" => {
                        Ok(
                            Classes::hkcdStaticTreeDynamicStorage32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDynamicStorage4" => {
                        Ok(Classes::hkcdStaticTreeDynamicStorage4(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeDynamicStorage5" => {
                        Ok(Classes::hkcdStaticTreeDynamicStorage5(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeDynamicStorage6" => {
                        Ok(Classes::hkcdStaticTreeDynamicStorage6(next_boxed(&mut map)?))
                    }
                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4" => {
                        Ok(
                            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5" => {
                        Ok(
                            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6" => {
                        Ok(
                            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw" => {
                        Ok(
                            Classes::hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32" => {
                        Ok(
                            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4" => {
                        Ok(
                            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5" => {
                        Ok(
                            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6" => {
                        Ok(
                            Classes::hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkClass" => Ok(Classes::hkClass(next_boxed(&mut map)?)),
                    "hkClassEnum" => Ok(Classes::hkClassEnum(next_boxed(&mut map)?)),
                    "hkClassEnumItem" => {
                        Ok(Classes::hkClassEnumItem(next_boxed(&mut map)?))
                    }
                    "hkClassMember" => Ok(Classes::hkClassMember(next_boxed(&mut map)?)),
                    "hkCompressedMassProperties" => {
                        Ok(Classes::hkCompressedMassProperties(next_boxed(&mut map)?))
                    }
                    "hkContactPoint" => {
                        Ok(Classes::hkContactPoint(next_boxed(&mut map)?))
                    }
                    "hkContactPointMaterial" => {
                        Ok(Classes::hkContactPointMaterial(next_boxed(&mut map)?))
                    }
                    "hkCustomAttributes" => {
                        Ok(Classes::hkCustomAttributes(next_boxed(&mut map)?))
                    }
                    "hkCustomAttributesAttribute" => {
                        Ok(Classes::hkCustomAttributesAttribute(next_boxed(&mut map)?))
                    }
                    "hkDataObjectTypeAttribute" => {
                        Ok(Classes::hkDataObjectTypeAttribute(next_boxed(&mut map)?))
                    }
                    "hkDefaultMemoryTracker" => {
                        Ok(Classes::hkDefaultMemoryTracker(next_boxed(&mut map)?))
                    }
                    "hkDescriptionAttribute" => {
                        Ok(Classes::hkDescriptionAttribute(next_boxed(&mut map)?))
                    }
                    "hkDocumentationAttribute" => {
                        Ok(Classes::hkDocumentationAttribute(next_boxed(&mut map)?))
                    }
                    "hkFloat16" => Ok(Classes::hkFloat16(next_boxed(&mut map)?)),
                    "hkFloat16Transform" => {
                        Ok(Classes::hkFloat16Transform(next_boxed(&mut map)?))
                    }
                    "hkFourTransposedPointsf" => {
                        Ok(Classes::hkFourTransposedPointsf(next_boxed(&mut map)?))
                    }
                    "hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations" => {
                        Ok(
                            Classes::hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations" => {
                        Ok(
                            Classes::hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance" => {
                        Ok(
                            Classes::hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkGeometry" => Ok(Classes::hkGeometry(next_boxed(&mut map)?)),
                    "hkGeometryTriangle" => {
                        Ok(Classes::hkGeometryTriangle(next_boxed(&mut map)?))
                    }
                    "hkGizmoAttribute" => {
                        Ok(Classes::hkGizmoAttribute(next_boxed(&mut map)?))
                    }
                    "hkIndexedTransformSet" => {
                        Ok(Classes::hkIndexedTransformSet(next_boxed(&mut map)?))
                    }
                    "hkIntRealPair" => Ok(Classes::hkIntRealPair(next_boxed(&mut map)?)),
                    "hkLinkAttribute" => {
                        Ok(Classes::hkLinkAttribute(next_boxed(&mut map)?))
                    }
                    "hkLocalFrame" => Ok(Classes::hkLocalFrame(next_boxed(&mut map)?)),
                    "hkLocalFrameGroup" => {
                        Ok(Classes::hkLocalFrameGroup(next_boxed(&mut map)?))
                    }
                    "hkMassProperties" => {
                        Ok(Classes::hkMassProperties(next_boxed(&mut map)?))
                    }
                    "hkMemoryMeshBody" => {
                        Ok(Classes::hkMemoryMeshBody(next_boxed(&mut map)?))
                    }
                    "hkMemoryMeshMaterial" => {
                        Ok(Classes::hkMemoryMeshMaterial(next_boxed(&mut map)?))
                    }
                    "hkMemoryMeshShape" => {
                        Ok(Classes::hkMemoryMeshShape(next_boxed(&mut map)?))
                    }
                    "hkMemoryMeshShapeSection" => {
                        Ok(Classes::hkMemoryMeshShapeSection(next_boxed(&mut map)?))
                    }
                    "hkMemoryMeshTexture" => {
                        Ok(Classes::hkMemoryMeshTexture(next_boxed(&mut map)?))
                    }
                    "hkMemoryMeshVertexBuffer" => {
                        Ok(Classes::hkMemoryMeshVertexBuffer(next_boxed(&mut map)?))
                    }
                    "hkMemoryResourceContainer" => {
                        Ok(Classes::hkMemoryResourceContainer(next_boxed(&mut map)?))
                    }
                    "hkMemoryResourceHandle" => {
                        Ok(Classes::hkMemoryResourceHandle(next_boxed(&mut map)?))
                    }
                    "hkMemoryResourceHandleExternalLink" => {
                        Ok(
                            Classes::hkMemoryResourceHandleExternalLink(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMeshBody" => Ok(Classes::hkMeshBody(next_boxed(&mut map)?)),
                    "hkMeshBoneIndexMapping" => {
                        Ok(Classes::hkMeshBoneIndexMapping(next_boxed(&mut map)?))
                    }
                    "hkMeshMaterial" => {
                        Ok(Classes::hkMeshMaterial(next_boxed(&mut map)?))
                    }
                    "hkMeshSection" => Ok(Classes::hkMeshSection(next_boxed(&mut map)?)),
                    "hkMeshSectionCinfo" => {
                        Ok(Classes::hkMeshSectionCinfo(next_boxed(&mut map)?))
                    }
                    "hkMeshShape" => Ok(Classes::hkMeshShape(next_boxed(&mut map)?)),
                    "hkMeshTexture" => Ok(Classes::hkMeshTexture(next_boxed(&mut map)?)),
                    "hkMeshTextureRawBufferDescriptor" => {
                        Ok(
                            Classes::hkMeshTextureRawBufferDescriptor(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMeshVertexBuffer" => {
                        Ok(Classes::hkMeshVertexBuffer(next_boxed(&mut map)?))
                    }
                    "hkModelerNodeTypeAttribute" => {
                        Ok(Classes::hkModelerNodeTypeAttribute(next_boxed(&mut map)?))
                    }
                    "hkMonitorStreamColorTable" => {
                        Ok(Classes::hkMonitorStreamColorTable(next_boxed(&mut map)?))
                    }
                    "hkMonitorStreamColorTableColorPair" => {
                        Ok(
                            Classes::hkMonitorStreamColorTableColorPair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMonitorStreamFrameInfo" => {
                        Ok(Classes::hkMonitorStreamFrameInfo(next_boxed(&mut map)?))
                    }
                    "hkMonitorStreamStringMap" => {
                        Ok(Classes::hkMonitorStreamStringMap(next_boxed(&mut map)?))
                    }
                    "hkMonitorStreamStringMapStringMap" => {
                        Ok(
                            Classes::hkMonitorStreamStringMapStringMap(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMotionState" => Ok(Classes::hkMotionState(next_boxed(&mut map)?)),
                    "hkMultipleVertexBuffer" => {
                        Ok(Classes::hkMultipleVertexBuffer(next_boxed(&mut map)?))
                    }
                    "hkMultipleVertexBufferElementInfo" => {
                        Ok(
                            Classes::hkMultipleVertexBufferElementInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMultipleVertexBufferLockedElement" => {
                        Ok(
                            Classes::hkMultipleVertexBufferLockedElement(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMultipleVertexBufferVertexBufferInfo" => {
                        Ok(
                            Classes::hkMultipleVertexBufferVertexBufferInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkMultiThreadCheck" => {
                        Ok(Classes::hkMultiThreadCheck(next_boxed(&mut map)?))
                    }
                    "hknpAction" => Ok(Classes::hknpAction(next_boxed(&mut map)?)),
                    "hknpArrayAction" => {
                        Ok(Classes::hknpArrayAction(next_boxed(&mut map)?))
                    }
                    "hknpBallGun" => Ok(Classes::hknpBallGun(next_boxed(&mut map)?)),
                    "hknpBinaryAction" => {
                        Ok(Classes::hknpBinaryAction(next_boxed(&mut map)?))
                    }
                    "hknpBody" => Ok(Classes::hknpBody(next_boxed(&mut map)?)),
                    "hknpBodyCinfo" => Ok(Classes::hknpBodyCinfo(next_boxed(&mut map)?)),
                    "hknpBodyQuality" => {
                        Ok(Classes::hknpBodyQuality(next_boxed(&mut map)?))
                    }
                    "hknpBodyQualityLibrary" => {
                        Ok(Classes::hknpBodyQualityLibrary(next_boxed(&mut map)?))
                    }
                    "hknpBodyReference" => {
                        Ok(Classes::hknpBodyReference(next_boxed(&mut map)?))
                    }
                    "hknpBreakableConstraintData" => {
                        Ok(Classes::hknpBreakableConstraintData(next_boxed(&mut map)?))
                    }
                    "hknpBridgeConstraintAtom" => {
                        Ok(Classes::hknpBridgeConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hknpBroadPhaseConfig" => {
                        Ok(Classes::hknpBroadPhaseConfig(next_boxed(&mut map)?))
                    }
                    "hknpBroadPhaseConfigLayer" => {
                        Ok(Classes::hknpBroadPhaseConfigLayer(next_boxed(&mut map)?))
                    }
                    "hknpBSMaterial" => {
                        Ok(Classes::hknpBSMaterial(next_boxed(&mut map)?))
                    }
                    "hknpBSMaterialProperties" => {
                        Ok(Classes::hknpBSMaterialProperties(next_boxed(&mut map)?))
                    }
                    "hknpCapsuleShape" => {
                        Ok(Classes::hknpCapsuleShape(next_boxed(&mut map)?))
                    }
                    "hknpCharacterProxyCinfo" => {
                        Ok(Classes::hknpCharacterProxyCinfo(next_boxed(&mut map)?))
                    }
                    "hknpCharacterRigidBodyCinfo" => {
                        Ok(Classes::hknpCharacterRigidBodyCinfo(next_boxed(&mut map)?))
                    }
                    "hknpCircularSurfaceVelocity" => {
                        Ok(Classes::hknpCircularSurfaceVelocity(next_boxed(&mut map)?))
                    }
                    "hknpCollisionDispatchType" => {
                        Ok(Classes::hknpCollisionDispatchType(next_boxed(&mut map)?))
                    }
                    "hknpCollisionFilter" => {
                        Ok(Classes::hknpCollisionFilter(next_boxed(&mut map)?))
                    }
                    "hknpCompositeShape" => {
                        Ok(Classes::hknpCompositeShape(next_boxed(&mut map)?))
                    }
                    "hknpCompoundShape" => {
                        Ok(Classes::hknpCompoundShape(next_boxed(&mut map)?))
                    }
                    "hknpCompoundShapeInternalsKeyMask" => {
                        Ok(
                            Classes::hknpCompoundShapeInternalsKeyMask(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpCompoundShapeKeyMask" => {
                        Ok(Classes::hknpCompoundShapeKeyMask(next_boxed(&mut map)?))
                    }
                    "hknpCompressedHeightFieldShape" => {
                        Ok(
                            Classes::hknpCompressedHeightFieldShape(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpCompressedMeshShape" => {
                        Ok(Classes::hknpCompressedMeshShape(next_boxed(&mut map)?))
                    }
                    "hknpCompressedMeshShapeData" => {
                        Ok(Classes::hknpCompressedMeshShapeData(next_boxed(&mut map)?))
                    }
                    "hknpCompressedMeshShapeInternalsKeyMask" => {
                        Ok(
                            Classes::hknpCompressedMeshShapeInternalsKeyMask(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpCompressedMeshShapeTree" => {
                        Ok(Classes::hknpCompressedMeshShapeTree(next_boxed(&mut map)?))
                    }
                    "hknpCompressedMeshShapeTreeDataRun" => {
                        Ok(
                            Classes::hknpCompressedMeshShapeTreeDataRun(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpCompressedMeshShapeTreeDataRunData" => {
                        Ok(
                            Classes::hknpCompressedMeshShapeTreeDataRunData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpConstraint" => {
                        Ok(Classes::hknpConstraint(next_boxed(&mut map)?))
                    }
                    "hknpConstraintCinfo" => {
                        Ok(Classes::hknpConstraintCinfo(next_boxed(&mut map)?))
                    }
                    "hknpConstraintCollisionFilter" => {
                        Ok(Classes::hknpConstraintCollisionFilter(next_boxed(&mut map)?))
                    }
                    "hknpConvexPolytopeShape" => {
                        Ok(Classes::hknpConvexPolytopeShape(next_boxed(&mut map)?))
                    }
                    "hknpConvexPolytopeShapeFace" => {
                        Ok(Classes::hknpConvexPolytopeShapeFace(next_boxed(&mut map)?))
                    }
                    "hknpConvexShape" => {
                        Ok(Classes::hknpConvexShape(next_boxed(&mut map)?))
                    }
                    "hknpDecoratorShape" => {
                        Ok(Classes::hknpDecoratorShape(next_boxed(&mut map)?))
                    }
                    "hknpDefaultBroadPhaseConfig" => {
                        Ok(Classes::hknpDefaultBroadPhaseConfig(next_boxed(&mut map)?))
                    }
                    "hknpDefaultExternMeshShapeGeometry" => {
                        Ok(
                            Classes::hknpDefaultExternMeshShapeGeometry(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpDestructionShapeProperties" => {
                        Ok(
                            Classes::hknpDestructionShapeProperties(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpDisableCollisionFilter" => {
                        Ok(Classes::hknpDisableCollisionFilter(next_boxed(&mut map)?))
                    }
                    "hknpDummyShape" => {
                        Ok(Classes::hknpDummyShape(next_boxed(&mut map)?))
                    }
                    "hknpDynamicCompoundShape" => {
                        Ok(Classes::hknpDynamicCompoundShape(next_boxed(&mut map)?))
                    }
                    "hknpDynamicCompoundShapeData" => {
                        Ok(Classes::hknpDynamicCompoundShapeData(next_boxed(&mut map)?))
                    }
                    "hknpDynamicCompoundShapeTree" => {
                        Ok(Classes::hknpDynamicCompoundShapeTree(next_boxed(&mut map)?))
                    }
                    "hknpExternMeshShape" => {
                        Ok(Classes::hknpExternMeshShape(next_boxed(&mut map)?))
                    }
                    "hknpExternMeshShapeData" => {
                        Ok(Classes::hknpExternMeshShapeData(next_boxed(&mut map)?))
                    }
                    "hknpExternMeshShapeGeometry" => {
                        Ok(Classes::hknpExternMeshShapeGeometry(next_boxed(&mut map)?))
                    }
                    "hknpFirstPersonGun" => {
                        Ok(Classes::hknpFirstPersonGun(next_boxed(&mut map)?))
                    }
                    "hknpGroupCollisionFilter" => {
                        Ok(Classes::hknpGroupCollisionFilter(next_boxed(&mut map)?))
                    }
                    "hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516" => {
                        Ok(
                            Classes::hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpHeightFieldShape" => {
                        Ok(Classes::hknpHeightFieldShape(next_boxed(&mut map)?))
                    }
                    "hknpLinearSurfaceVelocity" => {
                        Ok(Classes::hknpLinearSurfaceVelocity(next_boxed(&mut map)?))
                    }
                    "hknpLodShape" => Ok(Classes::hknpLodShape(next_boxed(&mut map)?)),
                    "hknpLodShapeLevelOfDetailInfo" => {
                        Ok(Classes::hknpLodShapeLevelOfDetailInfo(next_boxed(&mut map)?))
                    }
                    "hknpMalleableConstraintData" => {
                        Ok(Classes::hknpMalleableConstraintData(next_boxed(&mut map)?))
                    }
                    "hknpMaskedShape" => {
                        Ok(Classes::hknpMaskedShape(next_boxed(&mut map)?))
                    }
                    "hknpMaskedShapeMaskWrapper" => {
                        Ok(Classes::hknpMaskedShapeMaskWrapper(next_boxed(&mut map)?))
                    }
                    "hknpMaterial" => Ok(Classes::hknpMaterial(next_boxed(&mut map)?)),
                    "hknpMaterialDescriptor" => {
                        Ok(Classes::hknpMaterialDescriptor(next_boxed(&mut map)?))
                    }
                    "hknpMaterialLibrary" => {
                        Ok(Classes::hknpMaterialLibrary(next_boxed(&mut map)?))
                    }
                    "hknpMaterialPalette" => {
                        Ok(Classes::hknpMaterialPalette(next_boxed(&mut map)?))
                    }
                    "hknpMinMaxQuadTree" => {
                        Ok(Classes::hknpMinMaxQuadTree(next_boxed(&mut map)?))
                    }
                    "hknpMinMaxQuadTreeMinMaxLevel" => {
                        Ok(Classes::hknpMinMaxQuadTreeMinMaxLevel(next_boxed(&mut map)?))
                    }
                    "hknpMotion" => Ok(Classes::hknpMotion(next_boxed(&mut map)?)),
                    "hknpMotionCinfo" => {
                        Ok(Classes::hknpMotionCinfo(next_boxed(&mut map)?))
                    }
                    "hknpMotionProperties" => {
                        Ok(Classes::hknpMotionProperties(next_boxed(&mut map)?))
                    }
                    "hknpMotionPropertiesLibrary" => {
                        Ok(Classes::hknpMotionPropertiesLibrary(next_boxed(&mut map)?))
                    }
                    "hknpPairCollisionFilter" => {
                        Ok(Classes::hknpPairCollisionFilter(next_boxed(&mut map)?))
                    }
                    "hknpPairCollisionFilterMapPairFilterKeyOverrideType" => {
                        Ok(
                            Classes::hknpPairCollisionFilterMapPairFilterKeyOverrideType(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpPhysicsSceneData" => {
                        Ok(Classes::hknpPhysicsSceneData(next_boxed(&mut map)?))
                    }
                    "hknpPhysicsSystemData" => {
                        Ok(Classes::hknpPhysicsSystemData(next_boxed(&mut map)?))
                    }
                    "hknpProjectileGun" => {
                        Ok(Classes::hknpProjectileGun(next_boxed(&mut map)?))
                    }
                    "hknpRagdollData" => {
                        Ok(Classes::hknpRagdollData(next_boxed(&mut map)?))
                    }
                    "hknpRagdollKeyFrameHierarchyUtilityControlData" => {
                        Ok(
                            Classes::hknpRagdollKeyFrameHierarchyUtilityControlData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpRefMaterial" => {
                        Ok(Classes::hknpRefMaterial(next_boxed(&mut map)?))
                    }
                    "hknpRefWorldCinfo" => {
                        Ok(Classes::hknpRefWorldCinfo(next_boxed(&mut map)?))
                    }
                    "hknpScaledConvexShape" => {
                        Ok(Classes::hknpScaledConvexShape(next_boxed(&mut map)?))
                    }
                    "hknpScaledConvexShapeBase" => {
                        Ok(Classes::hknpScaledConvexShapeBase(next_boxed(&mut map)?))
                    }
                    "hknpShape" => Ok(Classes::hknpShape(next_boxed(&mut map)?)),
                    "hknpShapeInstance" => {
                        Ok(Classes::hknpShapeInstance(next_boxed(&mut map)?))
                    }
                    "hknpShapeKeyMask" => {
                        Ok(Classes::hknpShapeKeyMask(next_boxed(&mut map)?))
                    }
                    "hknpShapeMassProperties" => {
                        Ok(Classes::hknpShapeMassProperties(next_boxed(&mut map)?))
                    }
                    "hknpShapeSignals" => {
                        Ok(Classes::hknpShapeSignals(next_boxed(&mut map)?))
                    }
                    "hknpShapeTagCodec" => {
                        Ok(Classes::hknpShapeTagCodec(next_boxed(&mut map)?))
                    }
                    "hknpShapeType" => Ok(Classes::hknpShapeType(next_boxed(&mut map)?)),
                    "hknpSparseCompactMapunsignedshort" => {
                        Ok(
                            Classes::hknpSparseCompactMapunsignedshort(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpSphereShape" => {
                        Ok(Classes::hknpSphereShape(next_boxed(&mut map)?))
                    }
                    "hknpStaticCompoundShape" => {
                        Ok(Classes::hknpStaticCompoundShape(next_boxed(&mut map)?))
                    }
                    "hknpStaticCompoundShapeData" => {
                        Ok(Classes::hknpStaticCompoundShapeData(next_boxed(&mut map)?))
                    }
                    "hknpStaticCompoundShapeInternals" => {
                        Ok(
                            Classes::hknpStaticCompoundShapeInternals(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpStaticCompoundShapeKeyMask" => {
                        Ok(
                            Classes::hknpStaticCompoundShapeKeyMask(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpStaticCompoundShapeTree" => {
                        Ok(Classes::hknpStaticCompoundShapeTree(next_boxed(&mut map)?))
                    }
                    "hknpSurfaceVelocity" => {
                        Ok(Classes::hknpSurfaceVelocity(next_boxed(&mut map)?))
                    }
                    "hknpTriangleShape" => {
                        Ok(Classes::hknpTriangleShape(next_boxed(&mut map)?))
                    }
                    "hknpTyremarkPoint" => {
                        Ok(Classes::hknpTyremarkPoint(next_boxed(&mut map)?))
                    }
                    "hknpTyremarksInfo" => {
                        Ok(Classes::hknpTyremarksInfo(next_boxed(&mut map)?))
                    }
                    "hknpTyremarksWheel" => {
                        Ok(Classes::hknpTyremarksWheel(next_boxed(&mut map)?))
                    }
                    "hknpUnaryAction" => {
                        Ok(Classes::hknpUnaryAction(next_boxed(&mut map)?))
                    }
                    "hknpVehicleAerodynamics" => {
                        Ok(Classes::hknpVehicleAerodynamics(next_boxed(&mut map)?))
                    }
                    "hknpVehicleBrake" => {
                        Ok(Classes::hknpVehicleBrake(next_boxed(&mut map)?))
                    }
                    "hknpVehicleData" => {
                        Ok(Classes::hknpVehicleData(next_boxed(&mut map)?))
                    }
                    "hknpVehicleDataWheelComponentParams" => {
                        Ok(
                            Classes::hknpVehicleDataWheelComponentParams(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDefaultAerodynamics" => {
                        Ok(
                            Classes::hknpVehicleDefaultAerodynamics(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDefaultAnalogDriverInput" => {
                        Ok(
                            Classes::hknpVehicleDefaultAnalogDriverInput(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDefaultBrake" => {
                        Ok(Classes::hknpVehicleDefaultBrake(next_boxed(&mut map)?))
                    }
                    "hknpVehicleDefaultBrakeWheelBrakingProperties" => {
                        Ok(
                            Classes::hknpVehicleDefaultBrakeWheelBrakingProperties(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDefaultEngine" => {
                        Ok(Classes::hknpVehicleDefaultEngine(next_boxed(&mut map)?))
                    }
                    "hknpVehicleDefaultSteering" => {
                        Ok(Classes::hknpVehicleDefaultSteering(next_boxed(&mut map)?))
                    }
                    "hknpVehicleDefaultSuspension" => {
                        Ok(Classes::hknpVehicleDefaultSuspension(next_boxed(&mut map)?))
                    }
                    "hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters" => {
                        Ok(
                            Classes::hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDefaultTransmission" => {
                        Ok(
                            Classes::hknpVehicleDefaultTransmission(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDefaultVelocityDamper" => {
                        Ok(
                            Classes::hknpVehicleDefaultVelocityDamper(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDriverInput" => {
                        Ok(Classes::hknpVehicleDriverInput(next_boxed(&mut map)?))
                    }
                    "hknpVehicleDriverInputAnalogStatus" => {
                        Ok(
                            Classes::hknpVehicleDriverInputAnalogStatus(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleDriverInputStatus" => {
                        Ok(Classes::hknpVehicleDriverInputStatus(next_boxed(&mut map)?))
                    }
                    "hknpVehicleEngine" => {
                        Ok(Classes::hknpVehicleEngine(next_boxed(&mut map)?))
                    }
                    "hknpVehicleInstance" => {
                        Ok(Classes::hknpVehicleInstance(next_boxed(&mut map)?))
                    }
                    "hknpVehicleInstanceWheelInfo" => {
                        Ok(Classes::hknpVehicleInstanceWheelInfo(next_boxed(&mut map)?))
                    }
                    "hknpVehicleLinearCastWheelCollide" => {
                        Ok(
                            Classes::hknpVehicleLinearCastWheelCollide(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleLinearCastWheelCollideWheelState" => {
                        Ok(
                            Classes::hknpVehicleLinearCastWheelCollideWheelState(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleRayCastWheelCollide" => {
                        Ok(
                            Classes::hknpVehicleRayCastWheelCollide(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleSteering" => {
                        Ok(Classes::hknpVehicleSteering(next_boxed(&mut map)?))
                    }
                    "hknpVehicleSuspension" => {
                        Ok(Classes::hknpVehicleSuspension(next_boxed(&mut map)?))
                    }
                    "hknpVehicleSuspensionSuspensionWheelParameters" => {
                        Ok(
                            Classes::hknpVehicleSuspensionSuspensionWheelParameters(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hknpVehicleTransmission" => {
                        Ok(Classes::hknpVehicleTransmission(next_boxed(&mut map)?))
                    }
                    "hknpVehicleVelocityDamper" => {
                        Ok(Classes::hknpVehicleVelocityDamper(next_boxed(&mut map)?))
                    }
                    "hknpVehicleWheelCollide" => {
                        Ok(Classes::hknpVehicleWheelCollide(next_boxed(&mut map)?))
                    }
                    "hknpWorldCinfo" => {
                        Ok(Classes::hknpWorldCinfo(next_boxed(&mut map)?))
                    }
                    "hknpWorldSnapshot" => {
                        Ok(Classes::hknpWorldSnapshot(next_boxed(&mut map)?))
                    }
                    "hkOffsetBitField" => {
                        Ok(Classes::hkOffsetBitField(next_boxed(&mut map)?))
                    }
                    "hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator" => {
                        Ok(
                            Classes::hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkp2dAngConstraintAtom" => {
                        Ok(Classes::hkp2dAngConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkp3dAngConstraintAtom" => {
                        Ok(Classes::hkp3dAngConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkPackedVector3" => {
                        Ok(Classes::hkPackedVector3(next_boxed(&mut map)?))
                    }
                    "hkPackedVector8_3" => {
                        Ok(Classes::hkPackedVector8_3(next_boxed(&mut map)?))
                    }
                    "hkPackfileHeader" => {
                        Ok(Classes::hkPackfileHeader(next_boxed(&mut map)?))
                    }
                    "hkPackfileSectionHeader" => {
                        Ok(Classes::hkPackfileSectionHeader(next_boxed(&mut map)?))
                    }
                    "hkpAngConstraintAtom" => {
                        Ok(Classes::hkpAngConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpAngFrictionConstraintAtom" => {
                        Ok(Classes::hkpAngFrictionConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpAngLimitConstraintAtom" => {
                        Ok(Classes::hkpAngLimitConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpAngMotorConstraintAtom" => {
                        Ok(Classes::hkpAngMotorConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpBallAndSocketConstraintData" => {
                        Ok(
                            Classes::hkpBallAndSocketConstraintData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpBallAndSocketConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpBallAndSocketConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpBallSocketConstraintAtom" => {
                        Ok(Classes::hkpBallSocketConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpBridgeAtoms" => {
                        Ok(Classes::hkpBridgeAtoms(next_boxed(&mut map)?))
                    }
                    "hkpBridgeConstraintAtom" => {
                        Ok(Classes::hkpBridgeConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpCallbackConstraintMotor" => {
                        Ok(Classes::hkpCallbackConstraintMotor(next_boxed(&mut map)?))
                    }
                    "hkpCogWheelConstraintAtom" => {
                        Ok(Classes::hkpCogWheelConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpCogWheelConstraintData" => {
                        Ok(Classes::hkpCogWheelConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpCogWheelConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpCogWheelConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpConeLimitConstraintAtom" => {
                        Ok(Classes::hkpConeLimitConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpConstraintAtom" => {
                        Ok(Classes::hkpConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpConstraintData" => {
                        Ok(Classes::hkpConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpConstraintMotor" => {
                        Ok(Classes::hkpConstraintMotor(next_boxed(&mut map)?))
                    }
                    "hkpDeformableAngConstraintAtom" => {
                        Ok(
                            Classes::hkpDeformableAngConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpDeformableFixedConstraintData" => {
                        Ok(
                            Classes::hkpDeformableFixedConstraintData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpDeformableFixedConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpDeformableFixedConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpDeformableLinConstraintAtom" => {
                        Ok(
                            Classes::hkpDeformableLinConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpFixedConstraintData" => {
                        Ok(Classes::hkpFixedConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpFixedConstraintDataAtoms" => {
                        Ok(Classes::hkpFixedConstraintDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpHingeConstraintData" => {
                        Ok(Classes::hkpHingeConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpHingeConstraintDataAtoms" => {
                        Ok(Classes::hkpHingeConstraintDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpHingeLimitsData" => {
                        Ok(Classes::hkpHingeLimitsData(next_boxed(&mut map)?))
                    }
                    "hkpHingeLimitsDataAtoms" => {
                        Ok(Classes::hkpHingeLimitsDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpLimitedForceConstraintMotor" => {
                        Ok(
                            Classes::hkpLimitedForceConstraintMotor(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpLimitedHingeConstraintData" => {
                        Ok(Classes::hkpLimitedHingeConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpLimitedHingeConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpLimitedHingeConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpLinConstraintAtom" => {
                        Ok(Classes::hkpLinConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpLinearClearanceConstraintData" => {
                        Ok(
                            Classes::hkpLinearClearanceConstraintData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpLinearClearanceConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpLinearClearanceConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpLinearParametricCurve" => {
                        Ok(Classes::hkpLinearParametricCurve(next_boxed(&mut map)?))
                    }
                    "hkpLinFrictionConstraintAtom" => {
                        Ok(Classes::hkpLinFrictionConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpLinLimitConstraintAtom" => {
                        Ok(Classes::hkpLinLimitConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpLinMotorConstraintAtom" => {
                        Ok(Classes::hkpLinMotorConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpLinSoftConstraintAtom" => {
                        Ok(Classes::hkpLinSoftConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkPostFinishAttribute" => {
                        Ok(Classes::hkPostFinishAttribute(next_boxed(&mut map)?))
                    }
                    "hkpOverwritePivotConstraintAtom" => {
                        Ok(
                            Classes::hkpOverwritePivotConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpParametricCurve" => {
                        Ok(Classes::hkpParametricCurve(next_boxed(&mut map)?))
                    }
                    "hkpPointToPathConstraintData" => {
                        Ok(Classes::hkpPointToPathConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpPointToPlaneConstraintData" => {
                        Ok(Classes::hkpPointToPlaneConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpPointToPlaneConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpPointToPlaneConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpPositionConstraintMotor" => {
                        Ok(Classes::hkpPositionConstraintMotor(next_boxed(&mut map)?))
                    }
                    "hkpPrismaticConstraintData" => {
                        Ok(Classes::hkpPrismaticConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpPrismaticConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpPrismaticConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpPulleyConstraintAtom" => {
                        Ok(Classes::hkpPulleyConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpPulleyConstraintData" => {
                        Ok(Classes::hkpPulleyConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpPulleyConstraintDataAtoms" => {
                        Ok(Classes::hkpPulleyConstraintDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpRackAndPinionConstraintAtom" => {
                        Ok(
                            Classes::hkpRackAndPinionConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpRackAndPinionConstraintData" => {
                        Ok(
                            Classes::hkpRackAndPinionConstraintData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpRackAndPinionConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpRackAndPinionConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpRagdollConstraintData" => {
                        Ok(Classes::hkpRagdollConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpRagdollConstraintDataAtoms" => {
                        Ok(Classes::hkpRagdollConstraintDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpRagdollLimitsData" => {
                        Ok(Classes::hkpRagdollLimitsData(next_boxed(&mut map)?))
                    }
                    "hkpRagdollLimitsDataAtoms" => {
                        Ok(Classes::hkpRagdollLimitsDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpRagdollMotorConstraintAtom" => {
                        Ok(Classes::hkpRagdollMotorConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpRotationalConstraintData" => {
                        Ok(Classes::hkpRotationalConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpRotationalConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpRotationalConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpSetLocalRotationsConstraintAtom" => {
                        Ok(
                            Classes::hkpSetLocalRotationsConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpSetLocalTransformsConstraintAtom" => {
                        Ok(
                            Classes::hkpSetLocalTransformsConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpSetLocalTranslationsConstraintAtom" => {
                        Ok(
                            Classes::hkpSetLocalTranslationsConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpSetupStabilizationAtom" => {
                        Ok(Classes::hkpSetupStabilizationAtom(next_boxed(&mut map)?))
                    }
                    "hkpSpringDamperConstraintMotor" => {
                        Ok(
                            Classes::hkpSpringDamperConstraintMotor(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpStiffSpringConstraintAtom" => {
                        Ok(Classes::hkpStiffSpringConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpStiffSpringConstraintData" => {
                        Ok(Classes::hkpStiffSpringConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpStiffSpringConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpStiffSpringConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpTwistLimitConstraintAtom" => {
                        Ok(Classes::hkpTwistLimitConstraintAtom(next_boxed(&mut map)?))
                    }
                    "hkpVehicleFrictionDescription" => {
                        Ok(Classes::hkpVehicleFrictionDescription(next_boxed(&mut map)?))
                    }
                    "hkpVehicleFrictionDescriptionAxisDescription" => {
                        Ok(
                            Classes::hkpVehicleFrictionDescriptionAxisDescription(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpVehicleFrictionStatus" => {
                        Ok(Classes::hkpVehicleFrictionStatus(next_boxed(&mut map)?))
                    }
                    "hkpVehicleFrictionStatusAxisStatus" => {
                        Ok(
                            Classes::hkpVehicleFrictionStatusAxisStatus(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpVelocityConstraintMotor" => {
                        Ok(Classes::hkpVelocityConstraintMotor(next_boxed(&mut map)?))
                    }
                    "hkpWheelConstraintData" => {
                        Ok(Classes::hkpWheelConstraintData(next_boxed(&mut map)?))
                    }
                    "hkpWheelConstraintDataAtoms" => {
                        Ok(Classes::hkpWheelConstraintDataAtoms(next_boxed(&mut map)?))
                    }
                    "hkpWheelFrictionConstraintAtom" => {
                        Ok(
                            Classes::hkpWheelFrictionConstraintAtom(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpWheelFrictionConstraintAtomAxle" => {
                        Ok(
                            Classes::hkpWheelFrictionConstraintAtomAxle(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpWheelFrictionConstraintData" => {
                        Ok(
                            Classes::hkpWheelFrictionConstraintData(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpWheelFrictionConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpWheelFrictionConstraintDataAtoms(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkpWrappedConstraintData" => {
                        Ok(Classes::hkpWrappedConstraintData(next_boxed(&mut map)?))
                    }
                    "hkQTransform" => Ok(Classes::hkQTransform(next_boxed(&mut map)?)),
                    "hkQTransformf" => Ok(Classes::hkQTransformf(next_boxed(&mut map)?)),
                    "hkRangeInt32Attribute" => {
                        Ok(Classes::hkRangeInt32Attribute(next_boxed(&mut map)?))
                    }
                    "hkRangeRealAttribute" => {
                        Ok(Classes::hkRangeRealAttribute(next_boxed(&mut map)?))
                    }
                    "hkRefCountedProperties" => {
                        Ok(Classes::hkRefCountedProperties(next_boxed(&mut map)?))
                    }
                    "hkRefCountedPropertiesEntry" => {
                        Ok(Classes::hkRefCountedPropertiesEntry(next_boxed(&mut map)?))
                    }
                    "hkReferencedObject" => {
                        Ok(Classes::hkReferencedObject(next_boxed(&mut map)?))
                    }
                    "hkResourceBase" => {
                        Ok(Classes::hkResourceBase(next_boxed(&mut map)?))
                    }
                    "hkResourceContainer" => {
                        Ok(Classes::hkResourceContainer(next_boxed(&mut map)?))
                    }
                    "hkResourceHandle" => {
                        Ok(Classes::hkResourceHandle(next_boxed(&mut map)?))
                    }
                    "hkRootLevelContainer" => {
                        Ok(Classes::hkRootLevelContainer(next_boxed(&mut map)?))
                    }
                    "hkRootLevelContainerNamedVariant" => {
                        Ok(
                            Classes::hkRootLevelContainerNamedVariant(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkSemanticsAttribute" => {
                        Ok(Classes::hkSemanticsAttribute(next_boxed(&mut map)?))
                    }
                    "hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair" => {
                        Ok(
                            Classes::hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkSetIntFloatPair" => {
                        Ok(Classes::hkSetIntFloatPair(next_boxed(&mut map)?))
                    }
                    "hkSetUint32" => Ok(Classes::hkSetUint32(next_boxed(&mut map)?)),
                    "hkSetUint64" => Ok(Classes::hkSetUint64(next_boxed(&mut map)?)),
                    "hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint" => {
                        Ok(
                            Classes::hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong" => {
                        Ok(
                            Classes::hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkSimpleLocalFrame" => {
                        Ok(Classes::hkSimpleLocalFrame(next_boxed(&mut map)?))
                    }
                    "hkSimpleProperty" => {
                        Ok(Classes::hkSimpleProperty(next_boxed(&mut map)?))
                    }
                    "hkSimplePropertyValue" => {
                        Ok(Classes::hkSimplePropertyValue(next_boxed(&mut map)?))
                    }
                    "hkSkinBinding" => Ok(Classes::hkSkinBinding(next_boxed(&mut map)?)),
                    "hkSkinnedMeshShape" => {
                        Ok(Classes::hkSkinnedMeshShape(next_boxed(&mut map)?))
                    }
                    "hkSkinnedMeshShapeBoneSection" => {
                        Ok(Classes::hkSkinnedMeshShapeBoneSection(next_boxed(&mut map)?))
                    }
                    "hkSkinnedMeshShapeBoneSet" => {
                        Ok(Classes::hkSkinnedMeshShapeBoneSet(next_boxed(&mut map)?))
                    }
                    "hkSkinnedMeshShapePart" => {
                        Ok(Classes::hkSkinnedMeshShapePart(next_boxed(&mut map)?))
                    }
                    "hkSkinnedRefMeshShape" => {
                        Ok(Classes::hkSkinnedRefMeshShape(next_boxed(&mut map)?))
                    }
                    "hkSphere" => Ok(Classes::hkSphere(next_boxed(&mut map)?)),
                    "hkStorageSkinnedMeshShape" => {
                        Ok(Classes::hkStorageSkinnedMeshShape(next_boxed(&mut map)?))
                    }
                    "hkStringObject" => {
                        Ok(Classes::hkStringObject(next_boxed(&mut map)?))
                    }
                    "hkSweptTransformf" => {
                        Ok(Classes::hkSweptTransformf(next_boxed(&mut map)?))
                    }
                    "hkTraceStreamTitle" => {
                        Ok(Classes::hkTraceStreamTitle(next_boxed(&mut map)?))
                    }
                    "hkUFloat8" => Ok(Classes::hkUFloat8(next_boxed(&mut map)?)),
                    "hkUiAttribute" => Ok(Classes::hkUiAttribute(next_boxed(&mut map)?)),
                    "hkUuid" => Ok(Classes::hkUuid(next_boxed(&mut map)?)),
                    "hkVariableTweakingHelper" => {
                        Ok(Classes::hkVariableTweakingHelper(next_boxed(&mut map)?))
                    }
                    "hkVariableTweakingHelperBoolVariableInfo" => {
                        Ok(
                            Classes::hkVariableTweakingHelperBoolVariableInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkVariableTweakingHelperIntVariableInfo" => {
                        Ok(
                            Classes::hkVariableTweakingHelperIntVariableInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkVariableTweakingHelperRealVariableInfo" => {
                        Ok(
                            Classes::hkVariableTweakingHelperRealVariableInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkVariableTweakingHelperVector4VariableInfo" => {
                        Ok(
                            Classes::hkVariableTweakingHelperVector4VariableInfo(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    "hkVertexFormat" => {
                        Ok(Classes::hkVertexFormat(next_boxed(&mut map)?))
                    }
                    "hkVertexFormatElement" => {
                        Ok(Classes::hkVertexFormatElement(next_boxed(&mut map)?))
                    }
                    "hkxAnimatedFloat" => {
                        Ok(Classes::hkxAnimatedFloat(next_boxed(&mut map)?))
                    }
                    "hkxAnimatedMatrix" => {
                        Ok(Classes::hkxAnimatedMatrix(next_boxed(&mut map)?))
                    }
                    "hkxAnimatedQuaternion" => {
                        Ok(Classes::hkxAnimatedQuaternion(next_boxed(&mut map)?))
                    }
                    "hkxAnimatedVector" => {
                        Ok(Classes::hkxAnimatedVector(next_boxed(&mut map)?))
                    }
                    "hkxAttribute" => Ok(Classes::hkxAttribute(next_boxed(&mut map)?)),
                    "hkxAttributeGroup" => {
                        Ok(Classes::hkxAttributeGroup(next_boxed(&mut map)?))
                    }
                    "hkxAttributeHolder" => {
                        Ok(Classes::hkxAttributeHolder(next_boxed(&mut map)?))
                    }
                    "hkxEnum" => Ok(Classes::hkxEnum(next_boxed(&mut map)?)),
                    "hkxEnumItem" => Ok(Classes::hkxEnumItem(next_boxed(&mut map)?)),
                    "hkxIndexBuffer" => {
                        Ok(Classes::hkxIndexBuffer(next_boxed(&mut map)?))
                    }
                    "hkxMaterial" => Ok(Classes::hkxMaterial(next_boxed(&mut map)?)),
                    "hkxMaterialProperty" => {
                        Ok(Classes::hkxMaterialProperty(next_boxed(&mut map)?))
                    }
                    "hkxMaterialTextureStage" => {
                        Ok(Classes::hkxMaterialTextureStage(next_boxed(&mut map)?))
                    }
                    "hkxMesh" => Ok(Classes::hkxMesh(next_boxed(&mut map)?)),
                    "hkxMeshSection" => {
                        Ok(Classes::hkxMeshSection(next_boxed(&mut map)?))
                    }
                    "hkxMeshUserChannelInfo" => {
                        Ok(Classes::hkxMeshUserChannelInfo(next_boxed(&mut map)?))
                    }
                    "hkxSparselyAnimatedBool" => {
                        Ok(Classes::hkxSparselyAnimatedBool(next_boxed(&mut map)?))
                    }
                    "hkxSparselyAnimatedEnum" => {
                        Ok(Classes::hkxSparselyAnimatedEnum(next_boxed(&mut map)?))
                    }
                    "hkxSparselyAnimatedInt" => {
                        Ok(Classes::hkxSparselyAnimatedInt(next_boxed(&mut map)?))
                    }
                    "hkxSparselyAnimatedString" => {
                        Ok(Classes::hkxSparselyAnimatedString(next_boxed(&mut map)?))
                    }
                    "hkxVertexAnimation" => {
                        Ok(Classes::hkxVertexAnimation(next_boxed(&mut map)?))
                    }
                    "hkxVertexAnimationUsageMap" => {
                        Ok(Classes::hkxVertexAnimationUsageMap(next_boxed(&mut map)?))
                    }
                    "hkxVertexBuffer" => {
                        Ok(Classes::hkxVertexBuffer(next_boxed(&mut map)?))
                    }
                    "hkxVertexBufferVertexData" => {
                        Ok(Classes::hkxVertexBufferVertexData(next_boxed(&mut map)?))
                    }
                    "hkxVertexDescription" => {
                        Ok(Classes::hkxVertexDescription(next_boxed(&mut map)?))
                    }
                    "hkxVertexDescriptionElementDecl" => {
                        Ok(
                            Classes::hkxVertexDescriptionElementDecl(
                                next_boxed(&mut map)?,
                            ),
                        )
                    }
                    _ => {
                        Err(
                            _serde::de::Error::unknown_field(
                                class_name,
                                &[
                                    "BGSGamebryoSequenceGenerator",
                                    "BSAlignBoneModifier",
                                    "BSAssignVariablesModifier",
                                    "BSBehaviorGraphSwapGenerator",
                                    "BSBoneSwitchGenerator",
                                    "BSBoneSwitchGeneratorBoneData",
                                    "BSComputeAddBoneAnimModifier",
                                    "BSCyclicBlendTransitionGenerator",
                                    "BSDecomposeVectorModifier",
                                    "BSDirectAtCapturePoseModifier",
                                    "BSDirectAtModifier",
                                    "BSDistTriggerModifier",
                                    "BSEventEveryNEventsModifier",
                                    "BSEventOnDeactivateModifier",
                                    "BSEventOnFalseToTrueModifier",
                                    "BSGetTimeStepModifier",
                                    "BSInterpValueModifier",
                                    "BSIsActiveModifier",
                                    "BSIStateManagerModifier",
                                    "BSIStateManagerModifierBSiStateData",
                                    "BSIStateManagerModifierBSIStateManagerStateListener",
                                    "BSiStateTaggingGenerator",
                                    "BSiStateTaggingStateListener",
                                    "BSLimbCycleModifier",
                                    "BSLimbIKModifier",
                                    "BSLookAtCapturePoseModifier",
                                    "BSLookAtModifier",
                                    "BSLookAtModifierBoneData",
                                    "BSModifyOnceModifier",
                                    "BSOffsetAnimationGenerator",
                                    "BSPassByTargetTriggerModifier",
                                    "BSRagdollContactListenerModifier",
                                    "BSRandomAlarmModifier",
                                    "BSRetargetBodyBlend",
                                    "BSReverseSpineTwistModifier",
                                    "BSRootTwistModifier",
                                    "BSSpeedSamplerModifier",
                                    "BSSpringState",
                                    "BSTimerModifier",
                                    "BSTweenerModifier",
                                    "DynamicAnimationTaggingGenerator",
                                    "hclAction",
                                    "hclAntiPinchConstraintSet",
                                    "hclAntiPinchConstraintSetPerParticle",
                                    "hclBendLinkConstraintSet",
                                    "hclBendLinkConstraintSetLink",
                                    "hclBendLinkConstraintSetMx",
                                    "hclBendLinkConstraintSetMxBatch",
                                    "hclBendLinkConstraintSetMxSingle",
                                    "hclBendStiffnessConstraintSet",
                                    "hclBendStiffnessConstraintSetLink",
                                    "hclBendStiffnessConstraintSetMx",
                                    "hclBendStiffnessConstraintSetMxBatch",
                                    "hclBendStiffnessConstraintSetMxSingle",
                                    "hclBendStiffnessSetupObject",
                                    "hclBlendSomeVerticesOperator",
                                    "hclBlendSomeVerticesOperatorBlendEntry",
                                    "hclBonePlanesConstraintSet",
                                    "hclBonePlanesConstraintSetBonePlane",
                                    "hclBoneSpaceDeformer",
                                    "hclBoneSpaceDeformerFourBlendEntryBlock",
                                    "hclBoneSpaceDeformerLocalBlockP",
                                    "hclBoneSpaceDeformerLocalBlockPN",
                                    "hclBoneSpaceDeformerLocalBlockPNT",
                                    "hclBoneSpaceDeformerLocalBlockPNTB",
                                    "hclBoneSpaceDeformerLocalBlockUnpackedP",
                                    "hclBoneSpaceDeformerLocalBlockUnpackedPN",
                                    "hclBoneSpaceDeformerLocalBlockUnpackedPNT",
                                    "hclBoneSpaceDeformerLocalBlockUnpackedPNTB",
                                    "hclBoneSpaceDeformerOneBlendEntryBlock",
                                    "hclBoneSpaceDeformerThreeBlendEntryBlock",
                                    "hclBoneSpaceDeformerTwoBlendEntryBlock",
                                    "hclBoneSpaceMeshMeshDeformOperator",
                                    "hclBoneSpaceMeshMeshDeformPNOperator",
                                    "hclBoneSpaceMeshMeshDeformPNTBOperator",
                                    "hclBoneSpaceMeshMeshDeformPNTOperator",
                                    "hclBoneSpaceMeshMeshDeformPOperator",
                                    "hclBoneSpaceSkinOperator",
                                    "hclBoneSpaceSkinPNOperator",
                                    "hclBoneSpaceSkinPNTBOperator",
                                    "hclBoneSpaceSkinPNTOperator",
                                    "hclBoneSpaceSkinPOperator",
                                    "hclBufferDefinition",
                                    "hclBufferLayout",
                                    "hclBufferLayoutBufferElement",
                                    "hclBufferLayoutSlot",
                                    "hclBufferSetupObject",
                                    "hclBufferUsage",
                                    "hclCapsuleShape",
                                    "hclClothContainer",
                                    "hclClothData",
                                    "hclClothSetupContainer",
                                    "hclClothSetupObject",
                                    "hclClothState",
                                    "hclClothStateBufferAccess",
                                    "hclClothStateSetupObject",
                                    "hclClothStateTransformSetAccess",
                                    "hclCollidable",
                                    "hclCompressibleLinkConstraintSet",
                                    "hclCompressibleLinkConstraintSetLink",
                                    "hclCompressibleLinkConstraintSetMx",
                                    "hclCompressibleLinkConstraintSetMxBatch",
                                    "hclCompressibleLinkConstraintSetMxSingle",
                                    "hclConstraintSet",
                                    "hclConstraintSetSetupObject",
                                    "hclConvexGeometryShape",
                                    "hclConvexHeightFieldShape",
                                    "hclConvexPlanesShape",
                                    "hclCopyVerticesOperator",
                                    "hclEdgeSelectionInput",
                                    "hclGatherAllVerticesOperator",
                                    "hclGatherSomeVerticesOperator",
                                    "hclGatherSomeVerticesOperatorVertexPair",
                                    "hclInputConvertOperator",
                                    "hclLocalRangeConstraintSet",
                                    "hclLocalRangeConstraintSetLocalConstraint",
                                    "hclLocalRangeSetupObject",
                                    "hclMeshBoneDeformOperator",
                                    "hclMeshBoneDeformOperatorTriangleBonePair",
                                    "hclMeshBoneDeformSetupObject",
                                    "hclMeshMeshDeformOperator",
                                    "hclMeshMeshDeformOperatorTriangleVertexPair",
                                    "hclMoveFixedParticlesSetupObject",
                                    "hclMoveParticlesOperator",
                                    "hclMoveParticlesOperatorVertexParticlePair",
                                    "hclNamedSetupMesh",
                                    "hclNamedTransformSetSetupObject",
                                    "hclObjectSpaceDeformer",
                                    "hclObjectSpaceDeformerFourBlendEntryBlock",
                                    "hclObjectSpaceDeformerLocalBlockP",
                                    "hclObjectSpaceDeformerLocalBlockPN",
                                    "hclObjectSpaceDeformerLocalBlockPNT",
                                    "hclObjectSpaceDeformerLocalBlockPNTB",
                                    "hclObjectSpaceDeformerLocalBlockUnpackedP",
                                    "hclObjectSpaceDeformerLocalBlockUnpackedPN",
                                    "hclObjectSpaceDeformerLocalBlockUnpackedPNT",
                                    "hclObjectSpaceDeformerLocalBlockUnpackedPNTB",
                                    "hclObjectSpaceDeformerOneBlendEntryBlock",
                                    "hclObjectSpaceDeformerThreeBlendEntryBlock",
                                    "hclObjectSpaceDeformerTwoBlendEntryBlock",
                                    "hclObjectSpaceMeshMeshDeformOperator",
                                    "hclObjectSpaceMeshMeshDeformPNOperator",
                                    "hclObjectSpaceMeshMeshDeformPNTBOperator",
                                    "hclObjectSpaceMeshMeshDeformPNTOperator",
                                    "hclObjectSpaceMeshMeshDeformPOperator",
                                    "hclObjectSpaceSkinOperator",
                                    "hclObjectSpaceSkinPNOperator",
                                    "hclObjectSpaceSkinPNTBOperator",
                                    "hclObjectSpaceSkinPNTOperator",
                                    "hclObjectSpaceSkinPOperator",
                                    "hclOperator",
                                    "hclOperatorSetupObject",
                                    "hclOutputConvertOperator",
                                    "hclPlaneShape",
                                    "hclRuntimeConversionInfo",
                                    "hclRuntimeConversionInfoElementConversion",
                                    "hclRuntimeConversionInfoSlotConversion",
                                    "hclScratchBufferDefinition",
                                    "hclScratchBufferSetupObject",
                                    "hclSetupMesh",
                                    "hclShadowBufferDefinition",
                                    "hclShape",
                                    "hclSimClothBufferSetupObject",
                                    "hclSimClothData",
                                    "hclSimClothDataCollidablePinchingData",
                                    "hclSimClothDataCollidableTransformMap",
                                    "hclSimClothDataLandscapeCollisionData",
                                    "hclSimClothDataOverridableSimulationInfo",
                                    "hclSimClothDataParticleData",
                                    "hclSimClothDataTransferMotionData",
                                    "hclSimClothPose",
                                    "hclSimClothSetupObject",
                                    "hclSimClothSetupObjectPerInstanceCollidable",
                                    "hclSimClothSetupObjectTransferMotionSetupData",
                                    "hclSimpleMeshBoneDeformOperator",
                                    "hclSimpleMeshBoneDeformOperatorTriangleBonePair",
                                    "hclSimpleWindAction",
                                    "hclSimulateOperator",
                                    "hclSimulateSetupObject",
                                    "hclSimulationSetupMesh",
                                    "hclSimulationSetupMeshMapOptions",
                                    "hclSkinOperator",
                                    "hclSkinOperatorBoneInfluence",
                                    "hclSkinSetupObject",
                                    "hclSphereShape",
                                    "hclStandardLinkConstraintSet",
                                    "hclStandardLinkConstraintSetLink",
                                    "hclStandardLinkConstraintSetMx",
                                    "hclStandardLinkConstraintSetMxBatch",
                                    "hclStandardLinkConstraintSetMxSingle",
                                    "hclStandardLinkSetupObject",
                                    "hclStaticDisplayBufferSetupObject",
                                    "hclStaticShadowBufferDefinition",
                                    "hclStretchLinkConstraintSet",
                                    "hclStretchLinkConstraintSetLink",
                                    "hclStretchLinkConstraintSetMx",
                                    "hclStretchLinkConstraintSetMxBatch",
                                    "hclStretchLinkConstraintSetMxSingle",
                                    "hclStretchLinkSetupObject",
                                    "hclTaperedCapsuleShape",
                                    "hclTransformSetDefinition",
                                    "hclTransformSetSetupObject",
                                    "hclTransformSetUsage",
                                    "hclTransformSetUsageTransformTracker",
                                    "hclTransitionConstraintSet",
                                    "hclTransitionConstraintSetPerParticle",
                                    "hclTriangleSelectionInput",
                                    "hclUpdateAllVertexFramesOperator",
                                    "hclUpdateSomeVertexFramesOperator",
                                    "hclUpdateSomeVertexFramesOperatorTriangle",
                                    "hclVertexFloatInput",
                                    "hclVertexGatherSetupObject",
                                    "hclVertexSelectionInput",
                                    "hclVolumeConstraint",
                                    "hclVolumeConstraintApplyData",
                                    "hclVolumeConstraintFrameData",
                                    "hclVolumeConstraintMx",
                                    "hclVolumeConstraintMxApplyBatchData",
                                    "hclVolumeConstraintMxApplySingleData",
                                    "hclVolumeConstraintMxFrameBatchData",
                                    "hclVolumeConstraintMxFrameSingleData",
                                    "hclVolumeConstraintSetupObject",
                                    "hkAabb",
                                    "hkAabb16",
                                    "hkAabbHalf",
                                    "hkAabbUint32",
                                    "hkaAngularReferenceFrame",
                                    "hkaAnimatedReferenceFrame",
                                    "hkaAnimation",
                                    "hkaAnimationBinding",
                                    "hkaAnimationContainer",
                                    "hkaAnimationPreviewColorContainer",
                                    "hkaAnnotationTrack",
                                    "hkaAnnotationTrackAnnotation",
                                    "hkaBone",
                                    "hkaBoneAttachment",
                                    "hkaDefaultAnimatedReferenceFrame",
                                    "hkaDirectionalReferenceFrame",
                                    "hkaFootstepAnalysisInfo",
                                    "hkaFootstepAnalysisInfoContainer",
                                    "hkaInterleavedUncompressedAnimation",
                                    "hkaLosslessCompressedAnimation",
                                    "hkaMeshBinding",
                                    "hkaMeshBindingMapping",
                                    "hkaParameterizedAnimationReferenceFrame",
                                    "hkaParameterizedReferenceFrame",
                                    "hkaPredictiveCompressedAnimation",
                                    "hkaPredictiveCompressedAnimationTrackCompressionParams",
                                    "hkaQuantizedAnimation",
                                    "hkaQuantizedAnimationTrackCompressionParams",
                                    "hkaReferencePoseAnimation",
                                    "hkArrayTypeAttribute",
                                    "hkaSkeleton",
                                    "hkaSkeletonLocalFrameOnBone",
                                    "hkaSkeletonMapper",
                                    "hkaSkeletonMapperData",
                                    "hkaSkeletonMapperDataChainMapping",
                                    "hkaSkeletonMapperDataPartitionMappingRange",
                                    "hkaSkeletonMapperDataSimpleMapping",
                                    "hkaSkeletonPartition",
                                    "hkaSplineCompressedAnimation",
                                    "hkaSplineCompressedAnimationAnimationCompressionParams",
                                    "hkaSplineCompressedAnimationTrackCompressionParams",
                                    "hkbAiControlCancelPathCommand",
                                    "hkbAiControlControlData",
                                    "hkbAiControlControlDataBlendable",
                                    "hkbAiControlControlDataNonBlendable",
                                    "hkbAiControlPathToCommand",
                                    "hkbAlignBoneModifier",
                                    "hkbAnimatedSkeletonGenerator",
                                    "hkBaseObject",
                                    "hkbAssetBundle",
                                    "hkbAssetBundleStringData",
                                    "hkbAttachmentModifier",
                                    "hkbAttachmentSetup",
                                    "hkbAttributeModifier",
                                    "hkbAttributeModifierAssignment",
                                    "hkbAuxiliaryNodeInfo",
                                    "hkbBehaviorEventsInfo",
                                    "hkbBehaviorGraph",
                                    "hkbBehaviorGraphData",
                                    "hkbBehaviorGraphInternalState",
                                    "hkbBehaviorGraphInternalStateInfo",
                                    "hkbBehaviorGraphStringData",
                                    "hkbBehaviorInfo",
                                    "hkbBehaviorInfoIdToNamePair",
                                    "hkbBehaviorReferenceGenerator",
                                    "hkbBindable",
                                    "hkbBlendCurveUtils",
                                    "hkbBlenderGenerator",
                                    "hkbBlenderGeneratorChild",
                                    "hkbBlenderGeneratorChildInternalState",
                                    "hkbBlenderGeneratorInternalState",
                                    "hkbBlendingTransitionEffect",
                                    "hkbBlendingTransitionEffectInternalState",
                                    "hkbBoneIndexArray",
                                    "hkbBoneWeightArray",
                                    "hkbBoolVariableSequencedData",
                                    "hkbBoolVariableSequencedDataSample",
                                    "hkbCameraShakeEventPayload",
                                    "hkbCameraVariablesChangedCommand",
                                    "hkbCharacter",
                                    "hkbCharacterAddedInfo",
                                    "hkbCharacterControlCommand",
                                    "hkbCharacterControllerModifier",
                                    "hkbCharacterControllerModifierControlData",
                                    "hkbCharacterControllerModifierInternalState",
                                    "hkbCharacterControllerSetup",
                                    "hkbCharacterData",
                                    "hkbCharacterInfo",
                                    "hkbCharacterSelectedInfo",
                                    "hkbCharacterSetup",
                                    "hkbCharacterSkinInfo",
                                    "hkbCharacterSteppedInfo",
                                    "hkbCharacterStringData",
                                    "hkbCharacterStringDataFileNameMeshNamePair",
                                    "hkbClientCharacterState",
                                    "hkbClipGenerator",
                                    "hkbClipGeneratorEcho",
                                    "hkbClipGeneratorInternalState",
                                    "hkbClipTrigger",
                                    "hkbClipTriggerArray",
                                    "hkbCombineTransformsModifier",
                                    "hkbCombineTransformsModifierInternalState",
                                    "hkbCompiledExpressionSet",
                                    "hkbCompiledExpressionSetToken",
                                    "hkbComputeDirectionModifier",
                                    "hkbComputeDirectionModifierInternalState",
                                    "hkbComputeRotationFromAxisAngleModifier",
                                    "hkbComputeRotationFromAxisAngleModifierInternalState",
                                    "hkbComputeRotationToTargetModifier",
                                    "hkbComputeRotationToTargetModifierInternalState",
                                    "hkbComputeWorldFromModelModifier",
                                    "hkbCondition",
                                    "hkbConstraintSetup",
                                    "hkbContext",
                                    "hkbCustomIdSelector",
                                    "hkbCustomTestGenerator",
                                    "hkbCustomTestGeneratorAnnotatedTypes",
                                    "hkbCustomTestGeneratorBoneTypes",
                                    "hkbCustomTestGeneratorComplexTypes",
                                    "hkbCustomTestGeneratorHiddenTypes",
                                    "hkbCustomTestGeneratorNestedTypes",
                                    "hkbCustomTestGeneratorNestedTypesBase",
                                    "hkbCustomTestGeneratorSimpleTypes",
                                    "hkbDampingModifier",
                                    "hkbDampingModifierInternalState",
                                    "hkbDelayedModifier",
                                    "hkbDelayedModifierInternalState",
                                    "hkbDetectCloseToGroundModifier",
                                    "hkbDetectCloseToGroundModifierInternalState",
                                    "hkbDockingGenerator",
                                    "hkbDockingGeneratorInternalState",
                                    "hkbEvaluateExpressionModifier",
                                    "hkbEvaluateExpressionModifierInternalExpressionData",
                                    "hkbEvaluateExpressionModifierInternalState",
                                    "hkbEvaluateHandleModifier",
                                    "hkbEvent",
                                    "hkbEventBase",
                                    "hkbEventDrivenModifier",
                                    "hkbEventDrivenModifierInternalState",
                                    "hkbEventInfo",
                                    "hkbEventPayload",
                                    "hkbEventPayloadList",
                                    "hkbEventProperty",
                                    "hkbEventRaisedInfo",
                                    "hkbEventRangeData",
                                    "hkbEventRangeDataArray",
                                    "hkbEventSequencedData",
                                    "hkbEventSequencedDataSequencedEvent",
                                    "hkbEventsFromRangeModifier",
                                    "hkbEventsFromRangeModifierInternalState",
                                    "hkbExpressionCondition",
                                    "hkbExpressionData",
                                    "hkbExpressionDataArray",
                                    "hkbExtractRagdollPoseModifier",
                                    "hkbExtrapolatingTransitionEffect",
                                    "hkbExtrapolatingTransitionEffectInternalState",
                                    "hkbFootIkControlData",
                                    "hkbFootIkControlsModifier",
                                    "hkbFootIkControlsModifierLeg",
                                    "hkbFootIkDriverInfo",
                                    "hkbFootIkDriverInfoLeg",
                                    "hkbFootIkGains",
                                    "hkbFootIkModifier",
                                    "hkbFootIkModifierInternalLegData",
                                    "hkbFootIkModifierLeg",
                                    "hkbGenerator",
                                    "hkbGeneratorOutputListener",
                                    "hkbGeneratorPartitionInfo",
                                    "hkbGeneratorSyncInfo",
                                    "hkbGeneratorSyncInfoActiveInterval",
                                    "hkbGeneratorSyncInfoSyncPoint",
                                    "hkbGeneratorTransitionEffect",
                                    "hkbGeneratorTransitionEffectInternalState",
                                    "hkbGetHandleOnBoneModifier",
                                    "hkbGetUpModifier",
                                    "hkbGetUpModifierInternalState",
                                    "hkbGetWorldFromModelModifier",
                                    "hkbGetWorldFromModelModifierInternalState",
                                    "hkbGravityModifier",
                                    "hkbHandIkControlData",
                                    "hkbHandIkControlsModifier",
                                    "hkbHandIkControlsModifierHand",
                                    "hkbHandIkDriverInfo",
                                    "hkbHandIkDriverInfoHand",
                                    "hkbHandIkModifier",
                                    "hkbHandIkModifierHand",
                                    "hkbHandle",
                                    "hkbHoldFromBlendingTransitionEffect",
                                    "hkbIntEventPayload",
                                    "hkbIntVariableSequencedData",
                                    "hkbIntVariableSequencedDataSample",
                                    "hkBitField",
                                    "hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator",
                                    "hkBitFieldBasehkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator",
                                    "hkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator",
                                    "hkbJigglerGroup",
                                    "hkbJigglerModifier",
                                    "hkbJigglerModifierInternalState",
                                    "hkbKeyframeBonesModifier",
                                    "hkbKeyframeBonesModifierKeyframeInfo",
                                    "hkbKeyFrameControlData",
                                    "hkbLayer",
                                    "hkbLayerGenerator",
                                    "hkbLayerGeneratorInternalState",
                                    "hkbLayerGeneratorLayerInternalState",
                                    "hkbLinkedSymbolInfo",
                                    "hkbLookAtModifier",
                                    "hkbLookAtModifierInternalState",
                                    "hkbManualSelectorGenerator",
                                    "hkbManualSelectorGeneratorInternalState",
                                    "hkbManualSelectorTransitionEffect",
                                    "hkbManualSelectorTransitionEffectInternalState",
                                    "hkbMessageLog",
                                    "hkbMirroredSkeletonInfo",
                                    "hkbMirrorModifier",
                                    "hkbModifier",
                                    "hkbModifierGenerator",
                                    "hkbModifierList",
                                    "hkbModifierWrapper",
                                    "hkbMoveBoneAttachmentModifier",
                                    "hkbMoveCharacterModifier",
                                    "hkbMoveCharacterModifierInternalState",
                                    "hkbNamedEventPayload",
                                    "hkbNamedIntEventPayload",
                                    "hkbNamedRealEventPayload",
                                    "hkbNamedStringEventPayload",
                                    "hkbNode",
                                    "hkbNodeInternalStateInfo",
                                    "hkbnpPhysicsInterface",
                                    "hkbnpRagdollInterface",
                                    "hkbNullPhysicsInterface",
                                    "hkbParametricMotionGenerator",
                                    "hkbParticleSystemEventPayload",
                                    "hkbPhysicsInterface",
                                    "hkbPinBonesGenerator",
                                    "hkbPoseMatchingGenerator",
                                    "hkbPoseMatchingGeneratorInternalState",
                                    "hkbPoseStoringGeneratorOutputListener",
                                    "hkbPoseStoringGeneratorOutputListenerStoredPose",
                                    "hkbPoweredRagdollControlData",
                                    "hkbPoweredRagdollControlsModifier",
                                    "hkbProceduralBlenderGenerator",
                                    "hkbProjectData",
                                    "hkbProjectStringData",
                                    "hkbRadialSelectorGenerator",
                                    "hkbRadialSelectorGeneratorGeneratorInfo",
                                    "hkbRadialSelectorGeneratorGeneratorPair",
                                    "hkbRagdollControllerSetup",
                                    "hkbRagdollInterface",
                                    "hkbRaiseEventCommand",
                                    "hkbRealEventPayload",
                                    "hkbRealVariableSequencedData",
                                    "hkbRealVariableSequencedDataSample",
                                    "hkbRebuildScriptsCommand",
                                    "hkbReferencedGeneratorSyncInfo",
                                    "hkbReferencePoseGenerator",
                                    "hkbRigidBodyRagdollControlData",
                                    "hkbRigidBodyRagdollControlsModifier",
                                    "hkbRigidBodySetup",
                                    "hkbRoleAttribute",
                                    "hkbRotateCharacterModifier",
                                    "hkbRotateCharacterModifierInternalState",
                                    "hkbScriptCondition",
                                    "hkbScriptGenerator",
                                    "hkbSenseHandleModifier",
                                    "hkbSenseHandleModifierRange",
                                    "hkbSequence",
                                    "hkbSequencedData",
                                    "hkbSequenceInternalState",
                                    "hkbSequenceStringData",
                                    "hkbSetBehaviorCommand",
                                    "hkbSetLocalTimeOfClipGeneratorCommand",
                                    "hkbSetNodePropertyCommand",
                                    "hkbSetSelectedCharacterCommand",
                                    "hkbSetWordVariableCommand",
                                    "hkbSetWorldFromModelModifier",
                                    "hkbShapeSetup",
                                    "hkbSimulationControlCommand",
                                    "hkbSimulationStateInfo",
                                    "hkbStateChooser",
                                    "hkbStateChooserWrapper",
                                    "hkbStateDependentModifier",
                                    "hkbStateListener",
                                    "hkbStateMachine",
                                    "hkbStateMachineActiveTransitionInfo",
                                    "hkbStateMachineDelayedTransitionInfo",
                                    "hkbStateMachineEventPropertyArray",
                                    "hkbStateMachineInternalState",
                                    "hkbStateMachineNestedStateMachineData",
                                    "hkbStateMachineProspectiveTransitionInfo",
                                    "hkbStateMachineStateInfo",
                                    "hkbStateMachineTimeInterval",
                                    "hkbStateMachineTransitionInfo",
                                    "hkbStateMachineTransitionInfoArray",
                                    "hkbStateMachineTransitionInfoReference",
                                    "hkbStringCondition",
                                    "hkbStringEventPayload",
                                    "hkbTestIdSelector",
                                    "hkbTimerModifier",
                                    "hkbTimerModifierInternalState",
                                    "hkbToolNodeType",
                                    "hkbTransformVectorModifier",
                                    "hkbTransformVectorModifierInternalState",
                                    "hkbTransitionEffect",
                                    "hkbTwistModifier",
                                    "hkbVariableBindingSet",
                                    "hkbVariableBindingSetBinding",
                                    "hkbVariableBounds",
                                    "hkbVariableInfo",
                                    "hkbVariableValue",
                                    "hkbVariableValueSet",
                                    "hkbWorldEnums",
                                    "hkbWorldFromModelModeData",
                                    "hkcdAdf",
                                    "hkcdDynamicAabbTree",
                                    "hkcdDynamicTreeAnisotropicMetric",
                                    "hkcdDynamicTreeCodec18",
                                    "hkcdDynamicTreeCodec32",
                                    "hkcdDynamicTreeCodecInt16",
                                    "hkcdDynamicTreeCodecInt16IntAabb",
                                    "hkcdDynamicTreeCodecRawUint",
                                    "hkcdDynamicTreeCodecRawUlong",
                                    "hkcdDynamicTreeCodecRawunsignedint",
                                    "hkcdDynamicTreeCodecRawunsignedlong",
                                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodec32",
                                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecInt16",
                                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUint",
                                    "hkcdDynamicTreeDefaultDynamicStoragehkcdDynamicTreeCodecRawUlong",
                                    "hkcdDynamicTreeDefaultTree32Storage",
                                    "hkcdDynamicTreeDefaultTree48Storage",
                                    "hkcdDynamicTreeDefaultTreeInt16Storage",
                                    "hkcdDynamicTreeDefaultTreePtrStorage",
                                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodec32",
                                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecInt16",
                                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUint",
                                    "hkcdDynamicTreeDynamicStorage0hkcdDynamicTreeAnisotropicMetrichkcdDynamicTreeCodecRawUlong",
                                    "hkcdDynamicTreeDynamicStorage16",
                                    "hkcdDynamicTreeDynamicStorage32",
                                    "hkcdDynamicTreeDynamicStorageInt16",
                                    "hkcdDynamicTreeDynamicStoragePtr",
                                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage16",
                                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorage32",
                                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStorageInt16",
                                    "hkcdDynamicTreeTreehkcdDynamicTreeDynamicStoragePtr",
                                    "hkcdFourAabb",
                                    "hkcdPlanarCsgOperand",
                                    "hkcdPlanarCsgOperandGeomSource",
                                    "hkcdPlanarEntity",
                                    "hkcdPlanarGeometry",
                                    "hkcdPlanarGeometryPlanesCollection",
                                    "hkcdPlanarGeometryPolygonCollection",
                                    "hkcdPlanarGeometryPrimitivesCollection28",
                                    "hkcdPlanarGeometryPrimitivesPlane",
                                    "hkcdPlanarSolid",
                                    "hkcdPlanarSolidNode",
                                    "hkcdPlanarSolidNodeStorage",
                                    "hkcdShape",
                                    "hkcdShapeDispatchType",
                                    "hkcdShapeInfoCodecType",
                                    "hkcdShapeType",
                                    "hkcdSimdTree",
                                    "hkcdSimdTreeNode",
                                    "hkcdStaticAabbTree",
                                    "hkcdStaticMeshTreeBase",
                                    "hkcdStaticMeshTreeBasePrimitive",
                                    "hkcdStaticMeshTreeBasePrimitiveDataRunBasehknpCompressedMeshShapeTreeDataRunData",
                                    "hkcdStaticMeshTreeBasePrimitiveDataRunBaseunsignedshort",
                                    "hkcdStaticMeshTreeBaseSection",
                                    "hkcdStaticMeshTreeBaseSectionDataRuns",
                                    "hkcdStaticMeshTreeBaseSectionPrimitives",
                                    "hkcdStaticMeshTreeBaseSectionSharedVertices",
                                    "hkcdStaticMeshTreeDefaultDataRun",
                                    "hkcdStaticMeshTreehkcdStaticMeshTreeCommonConfigunsignedintunsignedlonglong1121hknpCompressedMeshShapeTreeDataRun",
                                    "hkcdStaticPvs",
                                    "hkcdStaticPvsBlockHeader",
                                    "hkcdStaticTreeCodec3Axis",
                                    "hkcdStaticTreeCodec3Axis4",
                                    "hkcdStaticTreeCodec3Axis5",
                                    "hkcdStaticTreeCodec3Axis6",
                                    "hkcdStaticTreeCodecRaw",
                                    "hkcdStaticTreeDefaultTreeStorage32",
                                    "hkcdStaticTreeDefaultTreeStorage4",
                                    "hkcdStaticTreeDefaultTreeStorage5",
                                    "hkcdStaticTreeDefaultTreeStorage6",
                                    "hkcdStaticTreeDynamicStorage32",
                                    "hkcdStaticTreeDynamicStorage4",
                                    "hkcdStaticTreeDynamicStorage5",
                                    "hkcdStaticTreeDynamicStorage6",
                                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4",
                                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis5",
                                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis6",
                                    "hkcdStaticTreeDynamicStoragehkcdStaticTreeCodecRaw",
                                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage32",
                                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4",
                                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage5",
                                    "hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6",
                                    "hkClass",
                                    "hkClassEnum",
                                    "hkClassEnumItem",
                                    "hkClassMember",
                                    "hkCompressedMassProperties",
                                    "hkContactPoint",
                                    "hkContactPointMaterial",
                                    "hkCustomAttributes",
                                    "hkCustomAttributesAttribute",
                                    "hkDataObjectTypeAttribute",
                                    "hkDefaultMemoryTracker",
                                    "hkDescriptionAttribute",
                                    "hkDocumentationAttribute",
                                    "hkFloat16",
                                    "hkFloat16Transform",
                                    "hkFourTransposedPointsf",
                                    "hkFreeListArrayhknpMaterialhknpMaterialId8hknpMaterialFreeListArrayOperations",
                                    "hkFreeListArrayhknpMotionPropertieshknpMotionPropertiesId8hknpMotionPropertiesFreeListArrayOperations",
                                    "hkFreeListArrayhknpShapeInstancehkHandleshort32767hknpShapeInstanceIdDiscriminant8hknpShapeInstance",
                                    "hkGeometry",
                                    "hkGeometryTriangle",
                                    "hkGizmoAttribute",
                                    "hkIndexedTransformSet",
                                    "hkIntRealPair",
                                    "hkLinkAttribute",
                                    "hkLocalFrame",
                                    "hkLocalFrameGroup",
                                    "hkMassProperties",
                                    "hkMemoryMeshBody",
                                    "hkMemoryMeshMaterial",
                                    "hkMemoryMeshShape",
                                    "hkMemoryMeshShapeSection",
                                    "hkMemoryMeshTexture",
                                    "hkMemoryMeshVertexBuffer",
                                    "hkMemoryResourceContainer",
                                    "hkMemoryResourceHandle",
                                    "hkMemoryResourceHandleExternalLink",
                                    "hkMeshBody",
                                    "hkMeshBoneIndexMapping",
                                    "hkMeshMaterial",
                                    "hkMeshSection",
                                    "hkMeshSectionCinfo",
                                    "hkMeshShape",
                                    "hkMeshTexture",
                                    "hkMeshTextureRawBufferDescriptor",
                                    "hkMeshVertexBuffer",
                                    "hkModelerNodeTypeAttribute",
                                    "hkMonitorStreamColorTable",
                                    "hkMonitorStreamColorTableColorPair",
                                    "hkMonitorStreamFrameInfo",
                                    "hkMonitorStreamStringMap",
                                    "hkMonitorStreamStringMapStringMap",
                                    "hkMotionState",
                                    "hkMultipleVertexBuffer",
                                    "hkMultipleVertexBufferElementInfo",
                                    "hkMultipleVertexBufferLockedElement",
                                    "hkMultipleVertexBufferVertexBufferInfo",
                                    "hkMultiThreadCheck",
                                    "hknpAction",
                                    "hknpArrayAction",
                                    "hknpBallGun",
                                    "hknpBinaryAction",
                                    "hknpBody",
                                    "hknpBodyCinfo",
                                    "hknpBodyQuality",
                                    "hknpBodyQualityLibrary",
                                    "hknpBodyReference",
                                    "hknpBreakableConstraintData",
                                    "hknpBridgeConstraintAtom",
                                    "hknpBroadPhaseConfig",
                                    "hknpBroadPhaseConfigLayer",
                                    "hknpBSMaterial",
                                    "hknpBSMaterialProperties",
                                    "hknpCapsuleShape",
                                    "hknpCharacterProxyCinfo",
                                    "hknpCharacterRigidBodyCinfo",
                                    "hknpCircularSurfaceVelocity",
                                    "hknpCollisionDispatchType",
                                    "hknpCollisionFilter",
                                    "hknpCompositeShape",
                                    "hknpCompoundShape",
                                    "hknpCompoundShapeInternalsKeyMask",
                                    "hknpCompoundShapeKeyMask",
                                    "hknpCompressedHeightFieldShape",
                                    "hknpCompressedMeshShape",
                                    "hknpCompressedMeshShapeData",
                                    "hknpCompressedMeshShapeInternalsKeyMask",
                                    "hknpCompressedMeshShapeTree",
                                    "hknpCompressedMeshShapeTreeDataRun",
                                    "hknpCompressedMeshShapeTreeDataRunData",
                                    "hknpConstraint",
                                    "hknpConstraintCinfo",
                                    "hknpConstraintCollisionFilter",
                                    "hknpConvexPolytopeShape",
                                    "hknpConvexPolytopeShapeFace",
                                    "hknpConvexShape",
                                    "hknpDecoratorShape",
                                    "hknpDefaultBroadPhaseConfig",
                                    "hknpDefaultExternMeshShapeGeometry",
                                    "hknpDestructionShapeProperties",
                                    "hknpDisableCollisionFilter",
                                    "hknpDummyShape",
                                    "hknpDynamicCompoundShape",
                                    "hknpDynamicCompoundShapeData",
                                    "hknpDynamicCompoundShapeTree",
                                    "hknpExternMeshShape",
                                    "hknpExternMeshShapeData",
                                    "hknpExternMeshShapeGeometry",
                                    "hknpFirstPersonGun",
                                    "hknpGroupCollisionFilter",
                                    "hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516",
                                    "hknpHeightFieldShape",
                                    "hknpLinearSurfaceVelocity",
                                    "hknpLodShape",
                                    "hknpLodShapeLevelOfDetailInfo",
                                    "hknpMalleableConstraintData",
                                    "hknpMaskedShape",
                                    "hknpMaskedShapeMaskWrapper",
                                    "hknpMaterial",
                                    "hknpMaterialDescriptor",
                                    "hknpMaterialLibrary",
                                    "hknpMaterialPalette",
                                    "hknpMinMaxQuadTree",
                                    "hknpMinMaxQuadTreeMinMaxLevel",
                                    "hknpMotion",
                                    "hknpMotionCinfo",
                                    "hknpMotionProperties",
                                    "hknpMotionPropertiesLibrary",
                                    "hknpPairCollisionFilter",
                                    "hknpPairCollisionFilterMapPairFilterKeyOverrideType",
                                    "hknpPhysicsSceneData",
                                    "hknpPhysicsSystemData",
                                    "hknpProjectileGun",
                                    "hknpRagdollData",
                                    "hknpRagdollKeyFrameHierarchyUtilityControlData",
                                    "hknpRefMaterial",
                                    "hknpRefWorldCinfo",
                                    "hknpScaledConvexShape",
                                    "hknpScaledConvexShapeBase",
                                    "hknpShape",
                                    "hknpShapeInstance",
                                    "hknpShapeKeyMask",
                                    "hknpShapeMassProperties",
                                    "hknpShapeSignals",
                                    "hknpShapeTagCodec",
                                    "hknpShapeType",
                                    "hknpSparseCompactMapunsignedshort",
                                    "hknpSphereShape",
                                    "hknpStaticCompoundShape",
                                    "hknpStaticCompoundShapeData",
                                    "hknpStaticCompoundShapeInternals",
                                    "hknpStaticCompoundShapeKeyMask",
                                    "hknpStaticCompoundShapeTree",
                                    "hknpSurfaceVelocity",
                                    "hknpTriangleShape",
                                    "hknpTyremarkPoint",
                                    "hknpTyremarksInfo",
                                    "hknpTyremarksWheel",
                                    "hknpUnaryAction",
                                    "hknpVehicleAerodynamics",
                                    "hknpVehicleBrake",
                                    "hknpVehicleData",
                                    "hknpVehicleDataWheelComponentParams",
                                    "hknpVehicleDefaultAerodynamics",
                                    "hknpVehicleDefaultAnalogDriverInput",
                                    "hknpVehicleDefaultBrake",
                                    "hknpVehicleDefaultBrakeWheelBrakingProperties",
                                    "hknpVehicleDefaultEngine",
                                    "hknpVehicleDefaultSteering",
                                    "hknpVehicleDefaultSuspension",
                                    "hknpVehicleDefaultSuspensionWheelSpringSuspensionParameters",
                                    "hknpVehicleDefaultTransmission",
                                    "hknpVehicleDefaultVelocityDamper",
                                    "hknpVehicleDriverInput",
                                    "hknpVehicleDriverInputAnalogStatus",
                                    "hknpVehicleDriverInputStatus",
                                    "hknpVehicleEngine",
                                    "hknpVehicleInstance",
                                    "hknpVehicleInstanceWheelInfo",
                                    "hknpVehicleLinearCastWheelCollide",
                                    "hknpVehicleLinearCastWheelCollideWheelState",
                                    "hknpVehicleRayCastWheelCollide",
                                    "hknpVehicleSteering",
                                    "hknpVehicleSuspension",
                                    "hknpVehicleSuspensionSuspensionWheelParameters",
                                    "hknpVehicleTransmission",
                                    "hknpVehicleVelocityDamper",
                                    "hknpVehicleWheelCollide",
                                    "hknpWorldCinfo",
                                    "hknpWorldSnapshot",
                                    "hkOffsetBitField",
                                    "hkOffsetBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator",
                                    "hkp2dAngConstraintAtom",
                                    "hkp3dAngConstraintAtom",
                                    "hkPackedVector3",
                                    "hkPackedVector8_3",
                                    "hkPackfileHeader",
                                    "hkPackfileSectionHeader",
                                    "hkpAngConstraintAtom",
                                    "hkpAngFrictionConstraintAtom",
                                    "hkpAngLimitConstraintAtom",
                                    "hkpAngMotorConstraintAtom",
                                    "hkpBallAndSocketConstraintData",
                                    "hkpBallAndSocketConstraintDataAtoms",
                                    "hkpBallSocketConstraintAtom",
                                    "hkpBridgeAtoms",
                                    "hkpBridgeConstraintAtom",
                                    "hkpCallbackConstraintMotor",
                                    "hkpCogWheelConstraintAtom",
                                    "hkpCogWheelConstraintData",
                                    "hkpCogWheelConstraintDataAtoms",
                                    "hkpConeLimitConstraintAtom",
                                    "hkpConstraintAtom",
                                    "hkpConstraintData",
                                    "hkpConstraintMotor",
                                    "hkpDeformableAngConstraintAtom",
                                    "hkpDeformableFixedConstraintData",
                                    "hkpDeformableFixedConstraintDataAtoms",
                                    "hkpDeformableLinConstraintAtom",
                                    "hkpFixedConstraintData",
                                    "hkpFixedConstraintDataAtoms",
                                    "hkpHingeConstraintData",
                                    "hkpHingeConstraintDataAtoms",
                                    "hkpHingeLimitsData",
                                    "hkpHingeLimitsDataAtoms",
                                    "hkpLimitedForceConstraintMotor",
                                    "hkpLimitedHingeConstraintData",
                                    "hkpLimitedHingeConstraintDataAtoms",
                                    "hkpLinConstraintAtom",
                                    "hkpLinearClearanceConstraintData",
                                    "hkpLinearClearanceConstraintDataAtoms",
                                    "hkpLinearParametricCurve",
                                    "hkpLinFrictionConstraintAtom",
                                    "hkpLinLimitConstraintAtom",
                                    "hkpLinMotorConstraintAtom",
                                    "hkpLinSoftConstraintAtom",
                                    "hkPostFinishAttribute",
                                    "hkpOverwritePivotConstraintAtom",
                                    "hkpParametricCurve",
                                    "hkpPointToPathConstraintData",
                                    "hkpPointToPlaneConstraintData",
                                    "hkpPointToPlaneConstraintDataAtoms",
                                    "hkpPositionConstraintMotor",
                                    "hkpPrismaticConstraintData",
                                    "hkpPrismaticConstraintDataAtoms",
                                    "hkpPulleyConstraintAtom",
                                    "hkpPulleyConstraintData",
                                    "hkpPulleyConstraintDataAtoms",
                                    "hkpRackAndPinionConstraintAtom",
                                    "hkpRackAndPinionConstraintData",
                                    "hkpRackAndPinionConstraintDataAtoms",
                                    "hkpRagdollConstraintData",
                                    "hkpRagdollConstraintDataAtoms",
                                    "hkpRagdollLimitsData",
                                    "hkpRagdollLimitsDataAtoms",
                                    "hkpRagdollMotorConstraintAtom",
                                    "hkpRotationalConstraintData",
                                    "hkpRotationalConstraintDataAtoms",
                                    "hkpSetLocalRotationsConstraintAtom",
                                    "hkpSetLocalTransformsConstraintAtom",
                                    "hkpSetLocalTranslationsConstraintAtom",
                                    "hkpSetupStabilizationAtom",
                                    "hkpSpringDamperConstraintMotor",
                                    "hkpStiffSpringConstraintAtom",
                                    "hkpStiffSpringConstraintData",
                                    "hkpStiffSpringConstraintDataAtoms",
                                    "hkpTwistLimitConstraintAtom",
                                    "hkpVehicleFrictionDescription",
                                    "hkpVehicleFrictionDescriptionAxisDescription",
                                    "hkpVehicleFrictionStatus",
                                    "hkpVehicleFrictionStatusAxisStatus",
                                    "hkpVelocityConstraintMotor",
                                    "hkpWheelConstraintData",
                                    "hkpWheelConstraintDataAtoms",
                                    "hkpWheelFrictionConstraintAtom",
                                    "hkpWheelFrictionConstraintAtomAxle",
                                    "hkpWheelFrictionConstraintData",
                                    "hkpWheelFrictionConstraintDataAtoms",
                                    "hkpWrappedConstraintData",
                                    "hkQTransform",
                                    "hkQTransformf",
                                    "hkRangeInt32Attribute",
                                    "hkRangeRealAttribute",
                                    "hkRefCountedProperties",
                                    "hkRefCountedPropertiesEntry",
                                    "hkReferencedObject",
                                    "hkResourceBase",
                                    "hkResourceContainer",
                                    "hkResourceHandle",
                                    "hkRootLevelContainer",
                                    "hkRootLevelContainerNamedVariant",
                                    "hkSemanticsAttribute",
                                    "hkSethkIntRealPairhkContainerHeapAllocatorhkMapOperationshkIntRealPair",
                                    "hkSetIntFloatPair",
                                    "hkSetUint32",
                                    "hkSetUint64",
                                    "hkSetunsignedinthkContainerHeapAllocatorhkMapOperationsunsignedint",
                                    "hkSetunsignedlonglonghkContainerHeapAllocatorhkMapOperationsunsignedlonglong",
                                    "hkSimpleLocalFrame",
                                    "hkSimpleProperty",
                                    "hkSimplePropertyValue",
                                    "hkSkinBinding",
                                    "hkSkinnedMeshShape",
                                    "hkSkinnedMeshShapeBoneSection",
                                    "hkSkinnedMeshShapeBoneSet",
                                    "hkSkinnedMeshShapePart",
                                    "hkSkinnedRefMeshShape",
                                    "hkSphere",
                                    "hkStorageSkinnedMeshShape",
                                    "hkStringObject",
                                    "hkSweptTransformf",
                                    "hkTraceStreamTitle",
                                    "hkUFloat8",
                                    "hkUiAttribute",
                                    "hkUuid",
                                    "hkVariableTweakingHelper",
                                    "hkVariableTweakingHelperBoolVariableInfo",
                                    "hkVariableTweakingHelperIntVariableInfo",
                                    "hkVariableTweakingHelperRealVariableInfo",
                                    "hkVariableTweakingHelperVector4VariableInfo",
                                    "hkVertexFormat",
                                    "hkVertexFormatElement",
                                    "hkxAnimatedFloat",
                                    "hkxAnimatedMatrix",
                                    "hkxAnimatedQuaternion",
                                    "hkxAnimatedVector",
                                    "hkxAttribute",
                                    "hkxAttributeGroup",
                                    "hkxAttributeHolder",
                                    "hkxEnum",
                                    "hkxEnumItem",
                                    "hkxIndexBuffer",
                                    "hkxMaterial",
                                    "hkxMaterialProperty",
                                    "hkxMaterialTextureStage",
                                    "hkxMesh",
                                    "hkxMeshSection",
                                    "hkxMeshUserChannelInfo",
                                    "hkxSparselyAnimatedBool",
                                    "hkxSparselyAnimatedEnum",
                                    "hkxSparselyAnimatedInt",
                                    "hkxSparselyAnimatedString",
                                    "hkxVertexAnimation",
                                    "hkxVertexAnimationUsageMap",
                                    "hkxVertexBuffer",
                                    "hkxVertexBufferVertexData",
                                    "hkxVertexDescription",
                                    "hkxVertexDescriptionElementDecl",
                                ],
                            ),
                        )
                    }
                }
            }
        }
        deserializer
            .deserialize_class_index(ClassesVisitor {
                marker: core::marker::PhantomData,
            })
    }
}
