-- Your SQL goes here
CREATE TABLE access_rules(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    use_case_id BIGINT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id BIGINT NULL,

    CONSTRAINT fk_users
        FOREIGN KEY(user_id)
            REFERENCES users(id),
    
    CONSTRAINT fk_use_cases
        FOREIGN KEY(use_case_id)
            REFERENCES use_cases(id)
);

CREATE INDEX index_access_rules ON access_rules (user_id, use_case_id, resource_type);