-- Identity
create table cy_tech_student_numbers
( id integer primary key generated always as identity
, cy_tech_student_number text not null unique
, created_at timestamptz not null default (now())
);

-- 'users', 'cy_tech_student_numbers' matrix
create table users_cy_tech_student_numbers
( user_id integer not null references users(id)
, cy_tech_student_number_id integer not null references cy_tech_student_numbers(id)
, primary key (user_id, cy_tech_student_number_id)
, updated_at timestamptz not null default (now())
, unique (user_id) -- One person may only have one student number.
, unique (cy_tech_student_number_id) -- One student number may only be associated with one person.
);
