# scaling-octo-lamp
~~~ mermaid
erDiagram
    PARTNER {
        int partner_id PK
        string partner_name
    }

    PARTNERHANDLER {
        int partner_handler_id PK
        int partner_id FK
        string name
        text contact_info
    }

    HANDLER {
        int handler_id PK
        string name
        string department
        text contact_info
    }

    PROJECT {
        int project_id PK
        string project_name
    }

    INQUIRY {
        int inquiry_id PK
        int partner_id FK
        datetime inquiry_date
        string inquiry_type
        text content
        string status
        datetime completion_date
        int handler_id FK
        int project_id FK
        int partner_handler_id FK
    }

    REMARK {
        int remark_id PK
        int inquiry_id FK
        text remark_text
    }
~~~

    PARTNER ||--o{ INQUIRY : has
    HANDLER ||--o{ INQUIRY : assigned_to
    PARTNER ||--o{ PARTNERHANDLER : employs
    PARTNERHANDLER ||--o{ INQUIRY : contacts
    PROJECT ||--o{ INQUIRY : includes
    INQUIRY ||--o{ REMARK : contains
