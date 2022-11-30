# Database homework – Workplace Information System DB

Design and a part of a database layer of a workspace information system. The requirements are:

- Information system has data about employees:
  - `personal information`: **name, address, email address**, and other information you deem fit
  - `work attendance`: an employee can be **working, have sick days, have days off**, etc.
    - Note that employees can take "hours" off, for example, half a day off.
    - Every work day for an employee should be represented as one or more attendance records.
  - `branch affiliation`: what branch the employee works under. The affiliation also keeps track of the time that the employee has worked for the branch.
- Data about branches:
  - Branches should include information like the **budget**, **description** and any other information you deem fit for storing.

## Tasks

- Create a physical ERD for the system.
- Create a schema migration for the database.
- Implement the `database-layer` library.
- Show us a working example of the library in the `database-showcase` project.
- Bonus: Implement `database-showcase` as a fully fleshed-out CLI app.

The whole assignment can be found on [this issue page](https://gitlab.fi.muni.cz/pv281/pv281-iterations-2022/-/issues/1).

> Note: please, remove the comments and markdown files (**except this readme**) that are here as hints and guides. You will make your tutor's life MUCH easier. Thank you <3
