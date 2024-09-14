use crate::{
    ast::{Expression, Token},
    types::Type,
    IdentifierType,
    LexerError
};

#[derive(Debug)]
pub enum ParserError<'a> {
    UnexpectedPathInFunctionCall,
    InvalidImport,
    InvalidImportPath(String),
    ImportNotFound(String),
    ImportLexerError(String, LexerError),
    MappingExists(IdentifierType),
    ConstantNameNotUppercase(String),
    StructNotFound(IdentifierType),
    AssignReturnNothing,
    EntryFunctionCannotHaveForType,
    ExpectedToken,
    VariableTooLong(String),
    VariableMustStartWithAlphabetic(String),
    ExpectedIdentifierToken(Token<'a>),
    UnexpectedToken(Token<'a>),
    InvalidToken(Token<'a>, Token<'a>),
    TypeNotFound,
    NoIfBeforeElse,
    StructNameAlreadyUsed(String),
    VariableNameAlreadyUsed(String),
    VariableIdAlreadyUsed(IdentifierType),
    FunctionSignatureAlreadyExist,
    UnexpectedVariable(String),
    UnexpectedMappedVariableId(IdentifierType),
    MappingNotFound(String),
    UnexpectedType(Type),
    InvalidStructField(String),
    InvalidStructureName(String),
    FunctionNotFound(IdentifierType),
    LastFunction,
    FunctionNoReturnType,
    InvalidTypeT,
    NoScopeFound,
    NoReturnFound,
    ReturnAlreadyInElse,
    EmptyValue,
    IncompatibleNullWith(Type),
    EmptyStructName,
    InvalidArrayCall,
    NotImplemented,
    InvalidOperation,
    InvalidTernaryNoPreviousExpression,
    DeadCodeNotAllowed,
    InvalidForExpression(Expression),
    OperatorNotFound(Token<'a>),
    InvalidCondition(Type, Expression),
    InvalidOperationNotSameType(Type, Type),
    CastError(Type, Type),
    InvalidArrayCallIndexType(Type),
    InvalidTypeInArray(Type, Type),
    InvalidValueType(Type, Type),
    NoValueType,
    ExpectedArrayType,
    InvalidFunctionType(Type),
    EmptyArrayConstructor,
    ExpectedNumberType,
    InvalidNumberValueForType
}
