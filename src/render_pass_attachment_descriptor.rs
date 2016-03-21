use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use sys::{MTLLoadAction, MTLRenderPassAttachmentDescriptor, MTLStoreAction};
use {FromRaw, IntoRaw, Texture};

pub struct RenderPassAttachmentDescriptor(id);

impl RenderPassAttachmentDescriptor {
    pub fn texture(&self) -> Option<Texture> {
        unsafe { FromRaw::from_raw(self.0.texture()).ok() }
    }

    pub fn set_texture(&mut self, texture: Texture) {
        unsafe { self.0.setTexture(texture.into_raw()) }
    }

    pub fn level(&self) -> usize {
        unsafe { self.0.level() as usize }
    }

    pub fn set_level(&mut self, level: usize) {
        unsafe { self.0.setLevel(level as NSUInteger) }
    }

    pub fn slice(&self) -> usize {
        unsafe { self.0.slice() as usize }
    }

    pub fn set_slice(&mut self, slice: usize) {
        unsafe { self.0.setSlice(slice as NSUInteger) }
    }

    pub fn depth_plane(&self) -> usize {
        unsafe { self.0.depthPlane() as usize }
    }

    pub fn set_depth_plane(&mut self, depth_plane: usize) {
        unsafe { self.0.setDepthPlane(depth_plane as NSUInteger) }
    }

    pub fn load_action(&self) -> LoadAction {
        unsafe { self.0.loadAction().into() }
    }

    pub fn set_load_action(&mut self, load_action: LoadAction) {
        unsafe { self.0.setLoadAction(load_action.into()) }
    }

    pub fn store_action(&self) -> StoreAction {
        unsafe { self.0.storeAction().into() }
    }

    pub fn set_store_action(&mut self, store_action: StoreAction) {
        unsafe { self.0.setStoreAction(store_action.into()) }
    }

    pub fn resolve_texture(&self) -> Option<Texture> {
        unsafe { FromRaw::from_raw(self.0.resolveTexture()).ok() }
    }

    pub fn set_resolve_texture(&mut self, texture: Texture) {
        unsafe { self.0.setResolveTexture(texture.into_raw()) }
    }

    pub fn resolve_level(&self) -> usize {
        unsafe { self.0.resolveLevel() as usize }
    }

    pub fn set_resolve_level(&mut self, level: usize) {
        unsafe { self.0.setResolveLevel(level as NSUInteger) }
    }

    pub fn resolve_slice(&self) -> usize {
        unsafe { self.0.resolveSlice() as usize }
    }

    pub fn set_resolve_slice(&mut self, slice: usize) {
        unsafe { self.0.setResolveSlice(slice as NSUInteger) }
    }

    pub fn resolve_depth_plane(&self) -> usize {
        unsafe { self.0.resolveDepthPlane() as usize }
    }

    pub fn set_resolve_depth_plane(&mut self, depth_plane: usize) {
        unsafe { self.0.setResolveDepthPlane(depth_plane as NSUInteger) }
    }
}

impl Clone for RenderPassAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe { FromRaw::from_raw(self.0.copy()).unwrap() }
    }
}

impl_from_into_raw!(RenderPassAttachmentDescriptor, of class "MTLRenderPassAttachmentDescriptor");

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum LoadAction: MTLLoadAction {
        DontCare => MTLLoadActionDontCare,
        Load => MTLLoadActionLoad,
        Clear => MTLLoadActionClear
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum StoreAction: MTLStoreAction {
        DontCare => MTLStoreActionDontCare,
        Store => MTLStoreActionStore,
        MultisampleResolve => MTLStoreActionMultisampleResolve
    }
}
