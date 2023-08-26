-- Create the "ppc" database if it doesn't exist
CREATE DATABASE IF NOT EXISTS ppc;

-- Use the "ppc" database
USE ppc;

-- Create the "users" table
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

