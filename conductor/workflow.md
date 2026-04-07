# Project Workflow

## Guiding Principles

1. **The Plan is the Source of Truth:** All work must be tracked in `plan.md`
2. **The Tech Stack is Deliberate:** Changes to the tech stack must be documented in `tech-stack.md` *before* implementation
3. **Test-Driven Development:** Write unit tests before implementing functionality
4. **High Code Coverage:** Aim for >80% code coverage for all modules
5. **User Experience First:** Every decision should prioritize user experience
6. **Non-Interactive & CI-Aware:** Prefer non-interactive commands. Use `CI=true` for watch-mode tools (tests, linters) to ensure single execution.

## Task Workflow

All tasks follow a strict lifecycle:

### Standard Task Workflow

1. **Select Task:** Choose the next available task from `plan.md` in sequential order
2. **Mark In Progress:** Before beginning work, edit `plan.md` and change the task from `[ ]` to `[~]`
3. **Write Failing Tests (Red Phase):** Write one or more unit tests that define expected behavior
4. **Implement to Pass Tests (Green Phase):** Write minimum code to make tests pass
5. **Refactor:** Improve code quality with passing tests as safety net
6. **Verify Coverage:** Run coverage reports. Target >80% for new code
7. **Document Deviations:** If implementation differs from tech stack, update `tech-stack.md`
8. **Commit Code Changes:** Clear, concise commit message (e.g., `feat(ui): Create server card component`)
9. **Attach Task Summary with Git Notes:** Attach detailed summary to commit via `git notes`
10. **Update plan.md:** Mark task `[x]` with commit SHA
11. **Commit Plan Update:** Stage and commit `plan.md` changes

### Phase Completion Verification and Checkpointing Protocol

Executed after completing a phase:

1. Announce phase completion
2. Verify test coverage for phase changes (compare to previous checkpoint)
3. Execute automated tests
4. Propose manual verification plan
5. Await explicit user feedback
6. Create checkpoint commit
7. Attach verification report as git note
8. Record phase checkpoint SHA in `plan.md`
9. Commit plan update
10. Announce completion

## Quality Gates

Before marking any task complete:
- All tests pass
- Code coverage >80%
- Follows code style guidelines
- No linting or static analysis errors
- Documentation updated if needed
- No security vulnerabilities introduced

## Testing Requirements

| Type | Coverage Goal | Location |
|------|---------------|----------|
| Unit Tests | >80% | `src-tauri/tests/`, `src/components/__tests__/` |
| Integration Tests | Core flows covered | `tests/integration/` |

## Development Commands

```bash
# Install dependencies
npm install && cargo build

# Daily development
npm run dev          # Start React dev server
cargo tauri dev      # Start Tauri dev

# Before committing
npm run check        # lint + type check
cargo clippy         # Rust linting
npm test             # Unit tests
npm run test:e2e     # E2E tests

# Build for production
npm run build        # React build
cargo tauri build    # Tauri build
```

## Commit Guidelines

Format: `<type>(<scope>): <description>`

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example: `feat(server): Add server start command with status tracking`

## Definition of Done

A task is complete when:
1. All code implemented to specification
2. Unit tests written and passing
3. Code coverage meets requirements (>80%)
4. Code passes linting
5. Works correctly on target OS
6. Implementation notes added to `plan.md`
7. Changes committed with proper message
8. Git note with task summary attached

## Emergency Procedures

### Critical Bug
1. Create hotfix branch
2. Write failing test for bug
3. Implement minimal fix
4. Test thoroughly
5. Deploy immediately

### Data Loss
1. Stop all write operations
2. Restore from latest backup
3. Verify data integrity
4. Document incident
