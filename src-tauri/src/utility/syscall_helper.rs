use crate::types::middleware_types::SyscallDataType;

pub fn syscall_type_to_string(acquire_type: &SyscallDataType) -> String {
    match acquire_type {
        SyscallDataType::Char(_) => "Char".to_string(),
        SyscallDataType::Int(_) => "Int".to_string(),
        SyscallDataType::Long(_) => "Long".to_string(),
        SyscallDataType::String(_) => "String".to_string(),
        SyscallDataType::Float(_) => "Float".to_string(),
        SyscallDataType::Double(_) => "Double".to_string(),
    }
}
