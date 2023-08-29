
Rust Monitoring Board(Rumbo) - is a dashboard for collecting metrics like a server info(CPU, RAM, health checks etc.), logs, and error aggregation.

## Architecture
![Architecture overview](./docs/assets/architecture_overview.png)

# Models
## General metric info
```JSON
{
    "instance_name":"INSTANCE_ID",
    "timestamp":"CREATING_DATE",

    "metric_value":{}
}
```
## Metric types
```JSON
[
    {
        "metric_type":"RamSpace",
        "free_amount": 0,
        "total_amount": 0
    },
    {
        "metric_type":"CpuUsage",
        "core":"1",
        "load_percents": 120
    },
    {
        "metric_type":"DiskUsage",
        "name":"Disk1",
        "load_percents": 120,
        "reading_speed": 2000,
        "writing_speed": 2000
    },
    {
        "metric_type":"NetworkUsage",
        "sending_speed":100,
        "receiving_speed":100,
        "name": "some_adapter"
    },
    {
        "metric_type":"DiskSpace",
        "name":"Disk1",
        "free_amount":0,
        "total_amount":0
    }
]
```

# Development Info
## Project overview
### rumbo_ui
    Angular frontend part
### rumbo_web
    Binary file. It will contains only configuration of the web server. It should be as less as possible
### rumbo_logic
    This project will contain all business logic for our project.

The profit of this separation that rumbo_logic could be covered fully by tests and by default rust lib projects contains test samples.

## MIGRATIONS
In case you don't want to install diesel, postgres, rust locally you should do the next steps:
1. docker-compose build
2. Run the DB and Adminer only in docker
3. Go to rumbo_logic/migrations
4. Run all *Up.sql* through the adminer console (localhost:8080) in docker
5. Run the webapp
In case of new version you need to drop db and repeat all these steps

### Postgres and Diesel installation steps
1. Install postgres libraries(libpq). You can do in on the official website of Postgres
2. Add to the PATH(if you are windows user) the following folders(your path or version of Postgres could be different):
   - C:\Program Files\PostgreSQL\15\bin
   - C:\Program Files\PostgreSQL\15\lib
3. Add new environment variable PQ_LIB_DIR = C:\Program Files\PostgreSQL\15\lib
4. Run: cargo install diesel_cli --no-default-features --features postgres
5. Now create .env file in the root of the repo
6. Write there: DATABASE_URL=postgres://postgres:example@localhost/rumbo_app
7. Run: diesel setup
8. Run: diesel migration run

## Keeping GIT history
In case of moving or renaming files is better to keep the history of previous changes in this section you could find a guide how to do it.

Move files with git and keep file history
Be sure you don't have files uncommitted, if not commit them before next step.

    git status

In project-directory create FOLDER subfolder

    mkdir FOLDER

Move files with git mv except FOLDER subfolder to avoid errors

    for file in $(ls | grep -v 'FOLDER'); do git mv $file FOLDER; done;

Move specific files...

    git mv FILE FOLDER/

Commit changes

    git commit -m 'Moved files to FOLDER/'

That's all !

    git log -M summary

## Commands for docker
Commands for Dockerfile:
 - docker run --name rumbo -p 8080:8080 -it rumbo
 - docker build . -t rumbo

Commands for Docker Compose:
 - docker-compose build
 - docker-compose up
