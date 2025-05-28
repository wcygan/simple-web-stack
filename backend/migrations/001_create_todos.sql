-- Create todos table
CREATE TABLE todos (
    id CHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create index for filtering by completion status
CREATE INDEX idx_todos_completed ON todos(completed);

-- Insert sample data for testing
INSERT INTO todos (id, title, completed) VALUES
    ('550e8400-e29b-41d4-a716-446655440001', 'Learn Rust programming', false),
    ('550e8400-e29b-41d4-a716-446655440002', 'Build a todo app', false),
    ('550e8400-e29b-41d4-a716-446655440003', 'Write comprehensive tests', false),
    ('550e8400-e29b-41d4-a716-446655440004', 'Deploy to production', false),
    ('550e8400-e29b-41d4-a716-446655440005', 'Set up CI/CD pipeline', true); 