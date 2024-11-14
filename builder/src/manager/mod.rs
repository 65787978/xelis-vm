mod r#struct;
mod r#enum;

pub use r#struct::*;
pub use r#enum::*;

use std::{borrow::Cow, marker::PhantomData};
use xelis_types::IdentifierType;
use crate::{
    BuilderError,
    IdMapper
};

pub trait BuilderType<D>: Eq + Clone {
    fn with(id: IdentifierType, data: Vec<D>) -> Self;

    fn type_id(&self) -> IdentifierType;
}

pub trait Builder<'a, D> {
    type InnerType: BuilderType<D>;

    fn new(inner: Self::InnerType, fields_names: Vec<&'a str>) -> Self;

    fn inner(&self) -> &Self::InnerType;

    fn fields_names(&self) -> &Vec<&'a str>;

    fn get_id_for_field(&self, name: &str) -> Option<IdentifierType> {
        self.fields_names().iter().position(|k| *k == name).map(|v| v as IdentifierType)
    }

    fn into_inner(self) -> Self::InnerType;
}

#[derive(Debug)]
pub struct TypeManager<'a, D, T: Builder<'a, D>> {
    parent: Option<&'a Self>,
    // All structs registered in the manager
    types: Vec<T>,
    // mapper to map each string name into a unique identifier
    mapper: IdMapper<'a>,
    phantom: PhantomData<D>
}

impl<'a, D, T: Builder<'a, D>> TypeManager<'a, D, T> {
    // Create a new struct manager
    pub fn new() -> Self {
        Self {
            parent: None,
            types: Vec::new(),
            mapper: IdMapper::new(),
            phantom: PhantomData
        }
    }

    pub fn with_parent(parent: &'a Self) -> Self {
        Self {
            parent: Some(parent),
            types: Vec::new(),
            mapper: IdMapper::with_parent(&parent.mapper),
            phantom: PhantomData
        }
    }

    fn build_struct_internal(&mut self, name: Cow<'a, str>, fields: Vec<(&'a str, D)>) -> Result<T, BuilderError> {
        if self.mapper.has_variable(&name) {
            return Err(BuilderError::StructNameAlreadyUsed);
        }

        let (fields_names, fields_types) = split_vec(fields);

        let id = self.mapper.register(name)?;
        let inner = T::InnerType::with(id, fields_types);

        Ok(T::new(
            inner,
            fields_names
        ))
    }
    // register a new struct in the manager
    pub fn add(&mut self, name: Cow<'a, str>, fields: Vec<(&'a str, D)>) -> Result<(), BuilderError> {
        let builder = self.build_struct_internal(name, fields)?;
        self.types.push(builder);

        Ok(())
    }

    // Same as `add` but returns its identifier and the final struct
    pub fn build_struct(&mut self, name: Cow<'a, str>, fields: Vec<(&'a str, D)>) -> Result<T::InnerType, BuilderError> {
        let builder = self.build_struct_internal(name, fields)?;
        let inner = builder.inner().clone();
        self.types.push(builder);

        Ok(inner)
    }

    pub fn get_by_id(&self, id: &IdentifierType) -> Result<&T, BuilderError> {
        if let Some(parent) = self.parent {
            if let Ok(s) = parent.get_by_id(id) {
                return Ok(s);
            }
        }

        self.types.iter().find(|b| b.inner().type_id() == *id).ok_or(BuilderError::StructNotFound)
    }

    // Get a struct by name
    pub fn get_by_name(&self, name: &str) -> Result<&T, BuilderError> {
        let id = self.mapper.get(name)?;
        self.get_by_id(&id)
    }

    pub fn get_by_ref(&self, _type: &T::InnerType) -> Result<&T, BuilderError> {
        if let Some(parent) = self.parent {
            if let Ok(s) = parent.get_by_ref(_type) {
                return Ok(s);
            }
        }

        self.types.iter().find(|v| v.inner() == _type).ok_or(BuilderError::StructNotFound)   
    }

    // Convert the struct manager into a list of structs
    pub fn finalize(self) -> Vec<T::InnerType> {
        self.types.into_iter().map(T::into_inner).collect()
    }
}

fn split_vec<A, B>(input: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    let mut vec_a = Vec::with_capacity(input.len());
    let mut vec_b = Vec::with_capacity(input.len());

    for (a, b) in input {
        vec_a.push(a);
        vec_b.push(b);
    }

    (vec_a, vec_b)
}