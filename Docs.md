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
        +main()
        +parse_args()
    }
    
    class TimeTracker {
        -project_path: PathBuf
        +init(project_name: String)
        +clock_in()
        +clock_out()
        +status()
    }
    
    class Project {
        +name: String
        +current_session: Option~Session~
        +sessions: Vec~Session~
        +new(name: String)
        +save_to_file(path: PathBuf)
        +load_from_file(path: PathBuf)
        +start_session()
        +end_session()
    }
    
    class Session {
        +start: DateTime
        +end: Option~DateTime~
        +new()
        +finish(end_time: DateTime)
        +duration() Duration
        +is_active() bool
    }
    
    CLI --> TimeTracker : uses
    TimeTracker --> Project : manages
    Project --> Session : contains
```
