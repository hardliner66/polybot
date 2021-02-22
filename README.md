# PolyBot
I want to create the most polyglot twitch-chat bot out there.

This will basically be my training ground for all languages and technologies.

## Planned Features
- Microservice architecture
- Web-Interface:
    - OAuth Login
- Use many languages and technologies:
    - at least one imperative language
    - at least one functional langauge
    - at least one relational database (PostgreSQL)
    - at least one nosql database (not PostgreSQL)
- Use modern deployment (docker, kubernetes)
- Bot features:
    - Plugin System (WASM or script engine)
    - Point System
- Client:
    - sqlite

## Database
- Library:
    - diesel (i intuitively prefer this one for use behind an API)
        - not async
        - orm
    - sqlx
        - async
        - no orm
        - type checked SQL
- Database access is managed through an API server
    - Services communicate with API, API communicates with DB
- Communication:
    - https://www.grpc.io/
        - https://crates.io/crates/tonic
    - graphql

## Non-Features
- I don't want to have every language technologie there is.
    - So probably no cobol, no php and a few others. The list might change in the future.
- Be usable for the general public.
    - If the project works for you, use it.
    - If it doesn't, I probably wont change it.
    - I try to keep everything as generic and adaptable as possible and I will accept feature and change requests
      if I think they are a good fit.
