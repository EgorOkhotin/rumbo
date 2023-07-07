# Project overview
## rumbo_ui
    Angular frontend part
## rumbo_web
    Binary file. It will contains only configuration of the web server. It should be as less as possible
## rumbo_logic
    This project will contain all business logic for our project.

The profit of this separation that rumbo_logic could be covered fully by tests and by default rust lib projects contains test samples.

# Commands for development
Useful command for Dockerfile:
 - docker run --name rumbo -p 8080:8080 -it rumbo
 - docker build . -t rumbo

# Models

## General metric info
```JSON
{
    "instance_name":"",
    "timestamp":"",

    "metric_value":{}
}
```
## Metric types
```JSON
[
    {
        "metric_type":"RAM_SPACE_INFO",
        "free_amount": 0,
        "total_amount": 0
    },
    {
        "metric_type":"CPU_USAGE",
        "core":"1",
        "load_percents": 120
    },
    {
        "metric_type":"DISK_USAGE",
        "name":"Disk1",
        "load_percents": 120,
        "reading_speed": 2000,
        "writing_speed": 2000
    },
    {
        "metric_type":"NETWORK_USAGE",
        "sending_speed":100,
        "receiving_speed":100
    },
    {
        "metric_type":"DISK_SPACE_INFO",
        "name":"Disk1",
        "free_amount":0,
        "total_amount":0
    }
]
```