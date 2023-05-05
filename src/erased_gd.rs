use crate::prelude::*;
use bevy::reflect::TypeUuid;
use godot::{
    engine::{InputEvent, Resource},
    obj::mem::{Memory, StaticRefCount},
    sys,
};

#[derive(Debug, Component, Clone)]
pub struct ErasedGd {
    instance_id: InstanceId,
}

impl ErasedGd {
    pub fn get<T: Inherits<Node>>(&mut self) -> Gd<T> {
        self.try_get()
            .unwrap_or_else(|| panic!("failed to get godot ref as {}", std::any::type_name::<T>()))
    }

    /// # SAFETY
    /// The caller must uphold the contract of the constructors to ensure exclusive access
    pub fn try_get<T: Inherits<Node>>(&mut self) -> Option<Gd<T>> {
        Gd::try_from_instance_id(self.instance_id)
    }

    /// # SAFETY
    /// When using ErasedGodotRef as a Bevy Resource or Component, do not create duplicate references
    /// to the same instance because Godot is not completely thread-safe.
    ///
    /// TODO
    /// Could these type bounds be more flexible to accomodate other types that are not ref-counted
    /// but don't inherit Node
    pub fn new<T: Inherits<Node>>(reference: Gd<T>) -> Self {
        Self {
            instance_id: reference.instance_id(),
        }
    }
}

#[derive(Debug, TypeUuid, Resource)]
#[uuid = "c3bd07de-eade-4cb0-9392-7c21394286f8"]
pub struct ErasedGdResource {
    resource_id: InstanceId,
}

impl ErasedGdResource {
    pub fn get(&mut self) -> Gd<Resource> {
        self.try_get().unwrap()
    }

    pub fn try_get(&mut self) -> Option<Gd<Resource>> {
        Gd::try_from_instance_id(self.resource_id)
    }

    pub fn new(reference: Gd<Resource>) -> Self {
        StaticRefCount::maybe_inc_ref(&reference.share());

        Self {
            resource_id: reference.instance_id(),
        }
    }
}

impl Clone for ErasedGdResource {
    fn clone(&self) -> Self {
        StaticRefCount::maybe_inc_ref::<Resource>(
            &Gd::try_from_instance_id(self.resource_id).unwrap(),
        );

        Self {
            resource_id: self.resource_id.clone(),
        }
    }
}

impl Drop for ErasedGdResource {
    fn drop(&mut self) {
        let gd = self.get();
        let is_last = StaticRefCount::maybe_dec_ref(&gd); // may drop
        if is_last {
            unsafe {
                sys::interface_fn!(object_destroy)(gd.obj_sys());
            }
        }
    }
}

#[derive(Debug)]
pub struct ErasedInputEvent {
    event_id: InstanceId,
}

impl ErasedInputEvent {
    pub fn get(&self) -> Gd<Object> {
        self.try_get().unwrap()
    }

    pub fn try_get(&self) -> Option<Gd<Object>> {
        println!("{:?}", self.event_id);
        Gd::try_from_instance_id(self.event_id)
    }

    pub fn new(reference: Gd<InputEvent>) -> Self {
        println!("id: {:?}", reference.instance_id_or_none());
        println!("isvalid: {:?}", reference.is_instance_valid());

        // println!(
        //     "pre inc: {:?}",
        //     reference
        //         .share()
        //         .upcast::<RefCounted>()
        //         .get_reference_count()
        // );

        println!("pre inc: {:?}", reference.get_reference_count());

        StaticRefCount::maybe_inc_ref(&reference);

        println!("post inc: {:?}", reference.get_reference_count());

        // println!(
        //     "post inc: {:?}",
        //     reference
        //         .share()
        //         .upcast::<RefCounted>()
        //         .get_reference_count()
        // );

        Self {
            event_id: reference.instance_id(),
        }
    }
}

// impl Clone for ErasedInputEvent {
//     fn clone(&self) -> Self {
//         StaticRefCount::maybe_inc_ref::<InputEvent>(
//             &Gd::try_from_instance_id(self.event_id).unwrap(),
//         );

//         Self {
//             event_id: self.event_id.clone(),
//         }
//     }
// }

impl Drop for ErasedInputEvent {
    fn drop(&mut self) {
        let gd = self.get();
        let is_last = StaticRefCount::maybe_dec_ref(&gd); // may drop
        if is_last {
            unsafe {
                sys::interface_fn!(object_destroy)(gd.obj_sys());
            }
        }
    }
}
