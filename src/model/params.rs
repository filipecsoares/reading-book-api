pub struct Params {
    pub service: String,
    pub command: Option<String>,
    pub book_name: Option<String>,
}

impl Params {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // the first argument is the program name and can be ignored
        args.next();
        let service = args.next().ok_or("missing service")?;
        if service == "-cli" {
            let command = args.next().ok_or("missing command")?;
            let book_name = args.next().ok_or("missing book name")?;
            return Ok(Self {
                service,
                command: Some(command),
                book_name: Some(book_name),
            });
        }
        Ok(Self {
            service,
            command: None,
            book_name: None,
        })
    }
}