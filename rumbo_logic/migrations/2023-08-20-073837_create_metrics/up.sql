-- Your SQL goes here
CREATE TABLE metrics (
    id BIGSERIAL PRIMARY KEY,
    instance_id BIGSERIAL NOT NULL,
    metric_type TEXT NOT NULL,
    creating_date TIMESTAMP NOT NULL,
    metric_value JSON NOT NULL,

    CONSTRAINT fk_instance
        FOREIGN KEY(instance_id)
            REFERENCES instances(id)
)