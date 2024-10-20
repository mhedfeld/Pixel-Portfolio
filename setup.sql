CREATE TABLE contacts ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL, 
    email TEXT NOT NULL,
    message TEXT NOT NULL
);

CREATE TABLE projects ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT, 
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    link TEXT NOT NULL
);
