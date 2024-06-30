# flowchart of the processes
graph TD
    A[Client Request] --> B[main.rs]
    B --> C[api.rs]
    C --> D[task_controller.rs]
    D --> E{data_validator.rs}
    D --> F[database.rs]
    F --> G[(SQLite DB)]
    D --> H[error_handler.rs]
    
    E -->|Valid| F
    E -->|Invalid| H
    F -->|Success| D
    F -->|Error| H
    H --> I[Error Response]
    D -->|Success| J[JSON Response]
    