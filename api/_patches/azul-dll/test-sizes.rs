    impl AzString {
        #[inline]
        pub fn as_str(&self) -> &str {
            unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
        }
        #[inline]
        pub fn as_bytes(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.vec.ptr, self.vec.len) }
        }
    }

    impl ::std::fmt::Debug for AzCallback                   { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzLayoutCallback             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzRenderImageCallback        { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzIFrameCallback             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzTimerCallback              { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzWriteBackCallback          { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzThreadDestructorFn         { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzLibraryReceiveThreadMsgFn  { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzLibrarySendThreadMsgFn     { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzCheckThreadFinishedFn      { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzGetSystemTimeFn            { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzCreateThreadFn             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzThreadRecvFn               { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzThreadReceiverDestructorFn { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzThreadSenderDestructorFn   { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzInstantPtrDestructorFn     { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzInstantPtrCloneFn          { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzThreadSendFn               { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}

    impl ::std::fmt::Debug for AzDomVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzDomVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzIdOrClassVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzIdOrClassVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeDataInlineCssPropertyVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeDataInlineCssPropertyVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundContentVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundContentVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundPositionVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundPositionVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundRepeatVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundRepeatVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundSizeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundSizeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleTransformVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleTransformVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssPropertyVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssPropertyVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgMultiPolygonVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgMultiPolygonVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgPathVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgPathVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzVertexAttributeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzVertexAttributeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgPathElementVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgPathElementVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgVertexVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgVertexVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzU32VecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzU32VecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzXWindowTypeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzXWindowTypeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzVirtualKeyCodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzVirtualKeyCodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCascadeInfoVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCascadeInfoVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzScanCodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzScanCodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssDeclarationVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssDeclarationVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssPathSelectorVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssPathSelectorVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStylesheetVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStylesheetVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssRuleBlockVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssRuleBlockVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzU8VecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzU8VecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCallbackDataVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCallbackDataVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzDebugMessageVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzDebugMessageVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzGLuintVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzGLuintVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzGLintVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzGLintVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStringVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStringVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStringPairVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStringPairVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNormalizedLinearColorStopVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNormalizedLinearColorStopVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNormalizedRadialColorStopVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNormalizedRadialColorStopVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeIdVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeIdVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyledNodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyledNodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzTagIdsToNodeIdsMappingVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzTagIdsToNodeIdsMappingVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzParentWithNodeDepthVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzParentWithNodeDepthVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeDataVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeDataVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
