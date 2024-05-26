pub struct Args<'a> {
    pub builtins: Vec<String>,
    pub list: Vec<&'a str>,
    pub path: String,
}
