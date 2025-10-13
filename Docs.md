# Dependencies

- clap for arguments
- serde for JSON
- chrono for time

## Commands

- clock-me init
  creates a .clockme directory with a data.json file.
  asks for a project name.

- clock-me now
  Record current timestamp.
  Error if already clocked in.

- clock-me out
  Record current ending timestamp.

- clock-me status
  Shows current project, and if you're clocked in or not.

## Data structure

```json
{
  "project_name": "my-app",
  "current_session": {
    "start": "some-dateTime"
  } or null
  "session_list": [
    {"start": "...", "end": "...", "duration_minutes": 2.5}

  ]
}
```

## Next steps

- Breaks
- Reports
- Tags/TODO notices.
- Formatting?

```mermaid
classDiagram
    class CLI {
        <<entry point>>
        -commands: CommandHandler
        +main()
    }
    
    class CommandHandler {
        -session_service: SessionService
        +handle_init(name: String)
        +handle_clock_in()
        +handle_clock_out()
        +handle_status()
    }
    
    class SessionService {
        -repository: Box~dyn Repository~
        -clock: Box~dyn Clock~
        +init_project(name: String)
        +start_session()
        +end_session()
        +get_status()
    }
    
    class Repository {
        <<trait>>
        +load() Result~Project~
        +save(project: Project) Result
    }
    
    class FileRepository {
        -path: PathBuf
        +new(path: PathBuf)
    }
    
    class Clock {
        <<trait>>
        +now() DateTime
    }
    
    class SystemClock {
        +now() DateTime
    }
    
    class Project {
        +name: String
        +current_session: Option~Session~
        +sessions: Vec~Session~
    }
    
    class Session {
        +start: DateTime
        +end: Option~DateTime~
        +duration() Option~Duration~
    }
    
    CLI --> CommandHandler
    CommandHandler --> SessionService
    SessionService --> Repository
    SessionService --> Clock
    Repository <|.. FileRepository : implements
    Clock <|.. SystemClock : implements
    SessionService ..> Project : uses
    Project --> Session
```
