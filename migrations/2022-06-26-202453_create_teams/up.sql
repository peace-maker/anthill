CREATE TABLE teams (
    id        INT NOT NULL,
    name      TEXT,
    state     SMALLINT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE team_key_values (
    team_id   INT NOT NULL,
    key       TEXT NOT NULL,
    value     TEXT NOT NULL,
    PRIMARY KEY(team_id, key),
    FOREIGN KEY(team_id) REFERENCES teams(id) ON DELETE CASCADE
);