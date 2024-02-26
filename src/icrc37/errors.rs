pub enum ApproveTokenError{
    InvalidSpender,
    Unauthorized,
    NonExistingTokenId,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String },
}

pub enum ApproveCollectionError{
    InvalidSpender,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String },
}

pub enum RevokeTokenApprovalError{
    ApprovalDoesNotExist,
    Unauthorized,
    NonExistingTokenId,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String },
}

pub enum RevokeCollectionApprovalError{
    ApprovalDoesNotExist,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String },
}

pub enum TransferFromError{
    InvalidSpender,
    Unauthorized,
    NonExistingTokenId,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    Duplicate{ duplicate_of: u128 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String },
}