FROM node:latest as uiBuild
WORKDIR /rumbo_ui
COPY ./rumbo_ui/ /rumbo_ui
RUN npm install
RUN npm install -g @angular/cli
RUN ng build -c production

FROM rust:latest as build
WORKDIR /rumbo

# copy your source tree
COPY ./ /rumbo

# Copy angular buil artifacts
COPY --from=uiBuild /rumbo_ui/dist/rumbo_ui/ /rumbo/target/release/static

RUN cargo build --release

# our final base
FROM rust:slim-buster

# copy the build artifact from the build stage
COPY --from=build /rumbo/target/release/ ./rumbo

# set the startup command to run your binary
EXPOSE 8080
WORKDIR /rumbo
RUN chmod +x rumbo_web
ENV IS_PRODUCTION=true
CMD ["./rumbo_web"]