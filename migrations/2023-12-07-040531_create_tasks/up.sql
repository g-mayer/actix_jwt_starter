CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- noqa

CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    task VARCHAR NOT NULL,
    done BOOLEAN NOT NULL DEFAULT false,
    status VARCHAR NOT NULL DEFAULT 'active',
    task_type VARCHAR NOT NULL DEFAULT 'task',
    details TEXT NOT NULL,
    priority VARCHAR,
    progress FLOAT,
    tags TEXT [] NOT NULL DEFAULT '{}',
    theme VARCHAR,
    due_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE lists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    name VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'active', 
    date TIMESTAMP NOT NULL,
    theme VARCHAR,
    due_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE subtask_mapping (
    task_id UUID NOT NULL,
    dependent_id UUID NOT NULL,
    PRIMARY KEY (task_id, dependent_id),
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
    FOREIGN KEY (dependent_id) REFERENCES tasks (id) ON DELETE CASCADE
);

CREATE TABLE task_list_mapping (
    task_id UUID NOT NULL,
    list_id UUID NOT NULL,
    PRIMARY KEY (task_id, list_id),
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
    FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE
);


CREATE INDEX idx_task_user_id ON tasks (user_id);
CREATE INDEX idx_task_type ON tasks (task_type);
CREATE INDEX idx_task_due_date ON tasks (due_date);

CREATE INDEX idx_lists_user_id ON lists (user_id);

CREATE INDEX idx_subtask_mapping_dependent_id ON subtask_mapping (dependent_id);
CREATE INDEX idx_subtask_mapping_task_id ON subtask_mapping (task_id);

CREATE INDEX idx_task_list_mapping_task_id ON task_list_mapping (task_id);
CREATE INDEX idx_task_list_mapping_list_id ON task_list_mapping (list_id);
