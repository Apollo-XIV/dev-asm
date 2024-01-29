-- Add down migration script here
DELETE FROM Comment WHERE message='Hello testUser';
DELETE FROM Comment WHERE message='Hello testUser2';

