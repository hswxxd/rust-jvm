use crate::types::RcRefCell;
use super::class_reader::ClassReader;
use crate::classfile{
    ConstantPool,
    ClassReader,
    AttributeInfo, read_attributes
}



pub struct MemberInfo {
    pub constant_pool: RcRefCell<ConstantPool>, /// 保存常量池
    pub access_flags: u16, // 成员访问标志
    pub name_index: u16, // 成员名称index
    pub descriptor_index: u16,
    pub attributes: Vec<Box <dyn AttributeInfo>>,
}

impl MemberInfo {
    pub fn read_members(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Vec<Self> {
        let member_count = reader.read_u16();
        let mut members = vec![];
        for _i in 0..member_count {
            members.push(MemberInfo::read_member(reader, cp));
        }
        members
    }

    pub fn read_member(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Self {
        MemberInfo { 
            constant_pool: cp.clone(), 
            access_flags: reader.read_u16(),
            name_index:reader.read_u16(), 
            descriptor_index: reader.read_u16(), 
            attributes: read_attributes(reader, cp.clone()),
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> u16 {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }

    pub fn descriptor(&self) -> u16 {
        self.constant_pool.borrow().get_utf8(self.descriptor_index)
    }
}