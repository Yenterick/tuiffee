-- Your SQL goes here

CREATE TABLE IF NOT EXISTS method (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS variety (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS coffee (
  id INTEGER PRIMARY KEY,
  brand VARCHAR(255) NOT NULL,
  name VARCHAR(255),
  tostion_level VARCHAR(255) CHECK ( tostion_level IN ('light', 'medium', 'medium-dark', 'dark') ) NOT NULL,
  process_type VARCHAR(255) CHECK ( process_type IN ('washed', 'honey', 'natural', 'culturing') ) NOT NULL,
  descriptive_notes VARCHAR(255),
  location VARCHAR(255),
  variety_id INTEGER NOT NULL,
  created_at DATETIME DEFAULT current_timestamp, 
  updated_at DATETIME DEFAULT current_timestamp,
  FOREIGN KEY (variety_id) REFERENCES variety (id)
);

CREATE TABLE IF NOT EXISTS preparation_technique (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  grind_level VARCHAR(255) CHECK ( grind_level IN ('extra-coarse', 'coarse', 'medium-coarse', 'medium', 'medium-fine', 'fine', 'extra-fine') ) NOT NULL,
  ratio INTEGER NOT NULL,
  people_target INTEGER NOT NULL,
  coffee_g REAL NOT NULL,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS preparation_record (
  id INTEGER PRIMARY KEY,
  method_id INTEGER NOT NULL,
  coffee_id INTEGER NOT NULL,
  preparation_technique_id INTEGER NOT NULL,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp,
  FOREIGN KEY (method_id) REFERENCES method (id),
  FOREIGN KEY (coffee_id) REFERENCES coffee (id),
  FOREIGN KEY (preparation_technique_id) REFERENCES preparation_technique (id)
);

CREATE TABLE IF NOT EXISTS blooming (
  id INTEGER PRIMARY KEY,
  water_g REAL NOT NULL,
  time_s INTEGER NOT NULL,
  sequency INTEGER NOT NULL,
  preparation_technique_id INTEGER NOT NULL,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp,
  UNIQUE (sequency, preparation_technique_id),
  FOREIGN KEY (preparation_technique_id) REFERENCES preparation_technique (id)
);

CREATE TABLE IF NOT EXISTS pouring (
  id INTEGER PRIMARY KEY,
  water_g REAL NOT NULL,
  time_s INTEGER NOT NULL,
  sequency INTEGER NOT NULL,
  preparation_technique_id INTEGER NOT NULL,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp,
  UNIQUE (sequency, preparation_technique_id),
  FOREIGN KEY (preparation_technique_id) REFERENCES preparation_technique (id)
);

CREATE TABLE IF NOT EXISTS cupping (
  id INTEGER PRIMARY KEY,
  author VARCHAR(255) NOT NULL,
  aroma INTEGER,
  flavor INTEGER,
  acidity INTEGER,
  sweetness INTEGER,
  body INTEGER,
  aftertaste INTEGER,
  balance INTEGER,
  observations VARCHAR(255),
  preparation_record_id INTEGER NOT NULL,
  created_at DATETIME DEFAULT current_timestamp,
  updated_at DATETIME DEFAULT current_timestamp,
  FOREIGN KEY (preparation_record_id) REFERENCES preparation_record (id)
);
