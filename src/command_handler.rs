use crate::session_service::SessionService;
use anyhow::Result;

pub struct CommandHandler {
    session_service: SessionService,
}

impl CommandHandler {
    pub fn new(session_service: SessionService) -> Self {
        Self { session_service }
    }

    pub fn handle_init(&self, project_name: String) -> Result<()> {
        self.session_service.init_project(project_name)?;
        println!("✓ Project initialized successfully!");
        println!("You can now use 'clock-me now' to start tracking time.");
        Ok(())
    }

    pub fn handle_clock_in(&self) -> Result<()> {
        let project = self.session_service.start_session()?;
        println!("✓ Clocked in to project: {}", project.name);
        println!(
            "Started tracking time at {}",
            project
                .current_session
                .as_ref()
                .unwrap()
                .start
                .format("%H:%M:%S")
        );
        Ok(())
    }

    pub fn handle_clock_out(&self) -> Result<()> {
        let (project, duration) = self.session_service.end_session()?;
        println!("✓ Clocked out from project: {}", project.name);
        println!(
            "Session duration: {:.2} hours",
            duration.num_minutes() as f64 / 60.0
        );
        Ok(())
    }

    pub fn handle_status(&self) -> Result<()> {
        let status = self.session_service.get_status()?;

        println!("Project: {}", status.project_name);

        if let Some(session) = status.current_session {
            println!("Status: Clocked IN");
            println!("Started at: {}", session.start.format("%Y-%m-%d %H:%M:%S"));

            let duration = status.current_time.signed_duration_since(session.start);
            println!(
                "Working for: {:.2} hours",
                duration.num_minutes() as f64 / 60.0
            );
        } else {
            println!("Status: Clocked OUT");

            if let Some(last_session) = status.last_session {
                println!("\nLast session:");
                println!(
                    "  Started: {}",
                    last_session.start.format("%Y-%m-%d %H:%M:%S")
                );
                if let Some(end) = last_session.end {
                    println!("  Ended: {}", end.format("%Y-%m-%d %H:%M:%S"));
                    if let Some(duration) = last_session.duration() {
                        println!(
                            "  Duration: {:.2} hours",
                            duration.num_minutes() as f64 / 60.0
                        );
                    }
                }
            }
        }

        println!("\nTotal sessions: {}", status.total_sessions);

        Ok(())
    }
}
