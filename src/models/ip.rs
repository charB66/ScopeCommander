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
    /// Returns the `uuid` field automatically generated as `&str`.
    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }

    /// # Name getter
    ///
    /// ## Return
    /// Returns the `name` field as `&str`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// # IP getter
    ///
    /// ## Return
    /// Returns the `ip` field as `&str`.
    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    /// # Comment getter
    ///
    /// ## Return
    /// Returns the `comment` field as `&str`.
    pub fn get_comment(&self) -> &str {
        &self.comment
    }

    /// # Uuid setter
    ///
    /// ## Setting
    /// Setting the `uuid` field value.
    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    /// # Name setter
    ///
    /// ## Setting
    /// Setting the `name` field value.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// # IP setter
    ///
    /// ## Setting
    /// Setting the `IP` field value.
    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    /// # Comment setter
    ///
    /// ## Setting
    /// Setting the `comment` field value.
    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
}
