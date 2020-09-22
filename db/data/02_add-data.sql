\c todolist;

CREATE TABLE todolist.todos (
    content text NOT NULL,
    done boolean NOT NULL DEFAULT false,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    completed_at timestamp,
    PRIMARY KEY(created_at)
);

INSERT INTO todolist.todos(content) VALUES ('hello');