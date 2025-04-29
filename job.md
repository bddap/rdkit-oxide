Read this entire doc before starting.

# Stage 1. High-Level Planning/Root Summary

I'd like you to create a detailed summary of the entire rdkit source (located in ./rdkit). The summary should serve as a map explaining every module, file, class, nook, and cranny of rdkit.

Start by examining the module structure. Explain what each root module does, and how they relate to each other. Place the root summary in ./legacy-rdkit-map/overall.md

# Stage 2. In-Depth Map

Once the root summary is done, create a to-do list requesting an item for each sub-task in the summary. We will be going much deeper, creating in-depth summaries of each module and their children too, until every source file is reflected in the summary.

You may find that submodules have submodules. When that is the case, recurse.

Ultimately, every code symbol (barring trivial locally scoped things) should be explained in detail:
- Where each symbol is defined.
- How it relates to other symbols. For example, when explaining a function, list all the external functions it calls and all the classes it instantiates, etc.

# Stage 2.5. Detailed Explanation of Each Unit Test

Find all of rdkit’s unit tests and list them in ./legacy-rdkit-map/tests.md.
./legacy-rdkit-map/tests.md must contain an explanation for every unit test in rdkit, in addition to location information (file and line number).

# Stage 3. Graph

Create a text graph representing all the relationships outlined so far. Write it in ./legacy-rdkit-map/map.md:

```
example_namespace::SymbolA -> implemented in -> ./path/relative/to/rdkit/repo/root.cpp
example_namespace::SymbolA -> defined in -> ./path/relative/to/rdkit/repo/root.hpp
example_namespace::SymbolA -> calls -> example_namespace::SymbolB
example_namespace::SymbolA -> mutates -> example_namespace::NAUGHTYMUTABLEGLOBAL
```

# Stage 4. Plan the Rust Rewrite

Read all your notes from ./legacy-rdkit-map/. Construct a detailed plan for a Rust port of rdkit.
Outline the module structure and make predictions about which major changes will be needed to fit rdkit into idiomatic Rust.
Write a detailed plan in ./plan/overall.md

The Rust port will not need external bindings. The Python and Java bindings for rdkit should be considered out of scope.

Files in ./plan/overall.md may need tweaking over time as you learn about issues.
Do treat files in ./plan/ as mutable. You may change your designs over time, just make sure to keep them tracked by git.

At the end of this stage, populate ./progress.md with the next set of tasks for the next stage!

# Stage 5. Implementation

This is a large codebase. You’ll need to be careful not to bite off more than you can chew and end up stuck in a confusing mess of compiler errors. Rather than implementing everything at once, make incremental changes, trying to keep them small.

Always run clippy and all tests before committing. Don’t commit with broken tests or unresolved lints. (Hint: use the -q option in tests to save on context consumption.)

Every test from Stage 2.5 must be translated to Rust (with any rare exceptions going into “the list of shame,” ./shame.md). Remember that Rust has slightly different idioms for test location. Skipped tests are not allowed.

You can always use your notes from previous steps, e.g. Stage 2 and Stage 3.

rdkit uses some external dependencies. You may find you need to reimplement those dependencies! Or use `cargo add` for crates if they exist.

Remember, if things get out of hand, git can take you back to a known-good state.

# Completion

You are under no time-constraints. Go deep, complete the job completely and correctly, it's ok for this to take months.
Your context will be automatically compacted as required. Do not speak to the user unless stage 5 is complete. Only call functions.
Systematically complete each stage and todo before reporting back to the user.

# Tracking Your Progress

Your context will eventually overflow and will need to be trimmed. Maintain a to-do structure in ./progress.md in order to survive context switches.
You are responsible for inserting to-dos and sub-to-dos into ./progress.md, and for checking off completed items (with the paths to your relevant output).

In the last line of ./progress.md, track what you are currently working on. When you finish a task:

- Double-check your deliverable to ensure the task is complete.
- Mark the task as done in ./progress.md.
- Pick your next task and replace the last line of ./progress.md with your next task.
- Commit each change, ensure your working dir doesn't have uncommited changes
- *Do not* report back or speak to the user unless you are stuck and need help. Stay quiet, function calls only.

Work in a continous loop, once one task is complete, pick up the next one. Add new tasks to ./progress.md as needed. Update design docs as needed.

./progress.md is your garden, keep it pretty, ordered, and up-to date.

I’ll be watching ./progress.md
