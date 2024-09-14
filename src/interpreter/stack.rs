use crate::{IdentifierType, InterpreterError, NoHashMap, variable::Path};

pub type Scope<'a> = NoHashMap<Path<'a>>;

#[derive(Debug)]
pub struct Stack<'a> {
    // Each scope is a HashMap storing variables
    // This is done to easily push/pop scopes
    scopes: Vec<Scope<'a>>,
    // Flag to break a loop
    loop_break: bool,
    // Flag to continue in loop
    loop_continue: bool,
}

impl<'a> Stack<'a> {
    // Create a new stack
    pub fn new() -> Self {
        Self {
            scopes: Vec::with_capacity(4),
            loop_break: false,
            loop_continue: false,
        }
    }

    // Get the latest scope created
    #[inline(always)]
    fn get_last_scope<'b>(&'b mut self) -> Result<&'b mut Scope<'a>, InterpreterError> {
        self.scopes.last_mut().ok_or(InterpreterError::NoScopeFound)
    }

    // Create a new scope
    #[inline(always)]
    pub fn begin_scope(&mut self) {
        self.scopes.push(Scope::with_capacity_and_hasher(16, Default::default()));
    }

    // Remove the latest scope created
    #[inline(always)]
    fn remove_last_scope(&mut self) -> Result<Scope<'a>, InterpreterError> {
        self.scopes.pop().ok_or(InterpreterError::NoScopeFound)
    }

    // End the latest scope created
    #[inline(always)]
    pub fn end_scope(&mut self) -> Result<(), InterpreterError> {
        self.remove_last_scope()?;
        Ok(())
    }

    // Clear the latest scope created without deleting it
    #[inline(always)]
    pub fn clear_last_scope(&mut self) -> Result<(), InterpreterError> {
        let scope = self.get_last_scope()?;
        scope.clear();
        Ok(())
    }

    // Remove a variable from the stack
    #[inline(always)]
    pub fn remove_variable(&mut self, name: &IdentifierType) -> Result<Path<'a>, InterpreterError> {
        self.scopes.iter_mut()
            .rev()
            .find_map(|scope| scope.remove(name))
            .ok_or_else(|| InterpreterError::VariableNotFound(name.clone()))
    }

    // Get a variable from the stack
    #[inline(always)]
    #[cfg(test)]
    pub fn get_variable<'b>(&'b self, name: &'b IdentifierType) -> Result<&'b Path<'a>, InterpreterError> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
            .ok_or_else(|| InterpreterError::VariableNotFound(name.clone()))
    }

    // Get a path access to a variable from the stack
    #[inline(always)]
    pub fn get_variable_path<'b>(&'b mut self, name: &'b IdentifierType) -> Result<Path<'a>, InterpreterError> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name).map(Path::shareable))
            .ok_or_else(|| InterpreterError::VariableNotFound(name.clone()))
    }

    // Check if a variable exists in the stack
    #[inline(always)]
    #[cfg(test)]
    pub fn has_variable(&self, name: &IdentifierType) -> bool {
        self.get_variable(name).is_ok()
    }

    // Register a variable in the stack
    pub fn register_variable(&mut self, name: IdentifierType, value: Path<'a>) -> Result<(), InterpreterError> {
        // We don't verify if the variable already exists
        // Parser should take care of that
        // if self.has_variable(&name) {
        //     return Err(InterpreterError::VariableAlreadyExists(name))
        // }

        let scope = self.get_last_scope()?;
        scope.insert(name, value);

        Ok(())
    }

    // Get the loop break flag
    #[inline(always)]
    pub fn get_loop_break(&self) -> bool {
        self.loop_break
    }

    // Set the loop break flag
    #[inline(always)]
    pub fn set_loop_break(&mut self, value: bool) {
        self.loop_break = value;
    }

    // Get the loop continue flag
    #[inline(always)]
    pub fn get_loop_continue(&self) -> bool {
        self.loop_continue
    }

    // Set the loop continue flag
    #[inline(always)]
    pub fn set_loop_continue(&mut self, value: bool) {
        self.loop_continue = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::values::Value;
    use super::*;

    #[test]
    fn test_variable_exists() {
        let mut stack = Stack::new();
        stack.begin_scope();
        stack.register_variable(0, Path::Owned(Value::U64(42))).unwrap();

        assert!(stack.has_variable(&0));
        assert!(!stack.has_variable(&1));

        stack.end_scope().unwrap();

        assert!(!stack.has_variable(&0));
    }
}