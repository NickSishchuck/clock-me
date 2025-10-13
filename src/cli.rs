use crate::clock::SystemClock;
use crate::command_handler::CommandHandler;
use crate::repository::FileRepository;
use crate::session_service::SessionService;
use crate::validators::ProjectValidator;
use anyhow::Result;
use std::io::{self, Write};

pub struct CLI {
    handler: CommandHandler,
}

impl CLI {
    pub fn new() -> Self {
        let repository = Box::new(FileRepository::new());
        let clock = Box::new(SystemClock);
        let service = SessionService::new(repository, clock);
        let handler = CommandHandler::new(service);

        Self { handler }
    }

    pub fn handle_init(&self, name: Option<String>) -> Result<()> {
        let project_name = match name {
            Some(n) => n,
            None => {
                print!("Enter project name: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            }
        };

        // Validate the project name using regex
        ProjectValidator::validate_name(&project_name)?;

        self.handler.handle_init(project_name)
    }

    pub fn handle_now(&self) -> Result<()> {
        self.handler.handle_clock_in()
    }

    pub fn handle_out(&self) -> Result<()> {
        self.handler.handle_clock_out()
    }

    pub fn handle_status(&self) -> Result<()> {
        self.handler.handle_status()
    }
}
