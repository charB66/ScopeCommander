use uuid::Uuid;

/// # IPv4
/// ## Parameters
/// Types and meaning:
/// - `uuid` (Uuid): unique identifier, generated during building.
/// - `name` (String): name representing the ip address.
/// - `ip` (String): the ip address.
/// - `comment` (String): useful to add some information about the ip.
///
/// ## Basic usage
/// ### Building
///
/// ```
/// let ip = Ip::new(
///         String::from("target 1"),
///         String::from("10.10.10.2"),
///         String::from("")
///     );
/// ```
///
/// ### Fields accessibility
/// Fields are private, it is necessary to use getters and setters to interact with them.
pub struct Ip {
    uuid: String,
    name: String,
    ip: String,
    comment: String,
}

impl Ip {
    /// # Builder
    /// Function to use to build a new IPv4.
    ///
    /// ## How to?
    /// ```
    /// Ip::new(
    ///     String::from("target 1"),
    ///     String::from("10.10.10.2"),
    ///     String::from("Main target IP!")
    /// );
    ///```
    pub fn new(name: String, ip: String, comment: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            ip,
            comment,
        }
    }

    /// # Uuid getter
    ///
    /// ## Return
    /// Returns the `uuid` field automatically generated.
    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    pub fn get_comment(&self) -> &str {
        &self.comment
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
}
