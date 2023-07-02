# Project overview
## rumbo-ui
    Angular frontend part
## rumbo-web
    Binary file. It will contains only configuration of the web server. It should be as less as possible
## rumbo-logic
    This project will contain all business logic for our project.

The profit of this separation that rumbo-logic could be covered fully by tests and by default rust lib projects contains test samples.

# Commands for development
Useful command for Dockerfile:
 - docker run --name rumbo -p 8080:8080 -it rumbo
 - docker build . -t rumbo