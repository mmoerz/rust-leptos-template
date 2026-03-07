# Architecture
```
UI (Leptos components/pages)
        │
        ▼
Business Services
        │
        ▼
Database Layer
        │
        ▼
Database
```

# Key Design Ideas
1) ``components/`` reusable widgets
2) ``pages/`` each represents a _route_
3) ``services/`` core application logic
4) ``db/`` database abstraction
5) ``server/`` server-specific logic
5) ``state/`` shared state injected into the app

# Example Flow
```
Route (/dashboard)
     │
     ▼
Page Component
     │
     ▼
Service Layer
     │
     ▼
Database Query
```

