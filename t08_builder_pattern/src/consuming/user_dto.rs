
#[allow(dead_code)]
pub trait UserDTO {
    fn get_name(&self) -> String;
    fn get_address(&self) -> String;
    fn get_age(&self) -> String;
}
