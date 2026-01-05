use crate::parsers::DurationParser;
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

        // Check if we just ended a break
        if project.current_session.is_some()
            && project.current_session.as_ref().unwrap().breaks.len() > 0
            && !project.is_on_break()
        {
            println!("✓ Break ended, continuing work on: {}", project.name);

            if let Some(ref session) = project.current_session {
                if let Some(last_break) = session.breaks.last() {
                    if let Some(duration) = last_break.duration() {
                        println!("Break duration: {}", DurationParser::format(duration));
                    }
                }

                // Show accumulated break time in this session
                let total_break_time = session.total_break_time();
                if total_break_time.num_minutes() > 0 {
                    println!(
                        "Total break time this session: {}",
                        DurationParser::format(total_break_time)
                    );
                }
            }
        } else {
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
        }
        Ok(())
    }

    pub fn handle_clock_out(&self) -> Result<()> {
        let (project, duration) = self.session_service.end_session()?;
        println!("✓ Clocked out from project: {}", project.name);

        println!("Session work time: {}", DurationParser::format(duration));
        println!("  ({:.2} hours)", duration.num_minutes() as f64 / 60.0);

        // Show break info if there were breaks
        if let Some(last_session) = project.sessions.last() {
            let break_time = last_session.total_break_time();
            if break_time.num_minutes() > 0 {
                println!("Break time: {}", DurationParser::format(break_time));
                println!("  (Breaks taken: {})", last_session.breaks.len());
            }
        }

        Ok(())
    }

    pub fn handle_break(&self) -> Result<()> {
        let (project, work_time_before_break) = self.session_service.start_break()?;
        println!("✓ Break started for project: {}", project.name);
        println!(
            "Work time before break: {}",
            DurationParser::format(work_time_before_break)
        );

        if let Some(ref session) = project.current_session {
            let accumulated_breaks = session.total_break_time();
            if accumulated_breaks.num_minutes() > 0 {
                println!(
                    "Previous breaks this session: {}",
                    DurationParser::format(accumulated_breaks)
                );
            }
        }

        println!("\nUse 'clock-me start' to continue working");
        Ok(())
    }

    pub fn handle_status(&self) -> Result<()> {
        let status = self.session_service.get_status()?;

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Project: {}", status.project_name);

        if let Some(session) = status.current_session {
            if let Some(break_start) = status.current_break_start {
                println!("Status: On BREAK 🔴");
                println!("Break started at: {}", break_start.format("%H:%M:%S"));

                let break_duration = status.current_time.signed_duration_since(break_start);
                println!(
                    "Current break duration: {}",
                    DurationParser::format(break_duration)
                );

                // Show accumulated break time
                let accumulated_breaks = session.total_break_time();
                if accumulated_breaks.num_minutes() > 0 {
                    println!(
                        "Previous breaks this session: {}",
                        DurationParser::format(accumulated_breaks)
                    );
                }
            } else {
                println!("Status: Clocked IN ✓");
                println!("Started at: {}", session.start.format("%Y-%m-%d %H:%M:%S"));

                let elapsed = status.current_time.signed_duration_since(session.start);
                let break_time = session.total_break_time();
                let work_time = elapsed - break_time;

                println!("Working for: {}", DurationParser::format(work_time));

                if break_time.num_minutes() > 0 {
                    println!(
                        "Break time this session: {} ({} breaks)",
                        DurationParser::format(break_time),
                        session.breaks.len()
                    );
                }
            }
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
                    if let Some(work_time) = last_session.work_time() {
                        println!("  Work time: {}", DurationParser::format(work_time));
                        let break_time = last_session.total_break_time();
                        if break_time.num_minutes() > 0 {
                            println!(
                                "  Break time: {} ({} breaks)",
                                DurationParser::format(break_time),
                                last_session.breaks.len()
                            );
                        }
                    }
                }
            }
        }

        println!("\nToday's Summary:");
        println!(
            "  Work time: {}",
            DurationParser::format(status.today_work_time)
        );
        if status.today_break_time.num_minutes() > 0 {
            println!(
                "  Break time: {}",
                DurationParser::format(status.today_break_time)
            );
        }
        println!("  Sessions: {}", status.today_sessions);
        if status.today_breaks > 0 {
            println!("  Breaks: {}", status.today_breaks);
        }

        println!("\n Total (all time):");
        println!(
            "  Work time: {}",
            DurationParser::format(status.total_work_time)
        );
        if status.total_break_time.num_minutes() > 0 {
            println!(
                "  Break time: {}",
                DurationParser::format(status.total_break_time)
            );
        }
        println!("  Sessions: {}", status.total_sessions);

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        Ok(())
    }
}
