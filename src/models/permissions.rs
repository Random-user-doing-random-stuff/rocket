use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct UserPermissions: u8 {
        const Default = 1; // 0b1
        const ManageTags = 2; // 0b10
        const ManageComments = 4; // 0b100
        const ManageWords = 8; // 0b1000
        const ManageUsers = 16; // 0b10000
        const Admin = 32; // 0b100000
    }
}

impl UserPermissions {
    pub fn get_name(&self) -> String {
        self.0.to_string()
    }
}
