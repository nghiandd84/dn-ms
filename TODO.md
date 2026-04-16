1. docs/agents/ (The AI Control Center)
As we discussed, this is for your "AI-as-a-Collaborator" instructions.

prompts/: Reusable complex prompt templates for your Gemini CLI.

personas/: Specific instructions for different roles (e.g., security-auditor.md, rust-expert.md).

context/: Snapshots of project state for the AI to "read" to get up to speed quickly.

2. docs/architecture/ (The Big Picture)
Essential for keeping the AI from making structural mistakes.

adr/: (Architecture Decision Records) Short files explaining why you chose a specific library or AWS service.

diagrams/: Mermaid or SVG files showing data flow (AI can read Mermaid code very well).

schema/: Definitions for databases or JSON structures.

3. docs/api/ (The Interface)
If your project exposes any services or has internal modules.

endpoints.md: Definition of your API routes or CLI commands.

contracts.md: Expected inputs and outputs for your main Rust traits or AWS Lambda functions.

4. docs/dev/ (The "How-To")
Specifically for your local environment quirks.

wsl-setup.md: Guide for setting up the specific Linux environment needed for this code.

aws-deploy.md: Steps for invalidating CloudFront caches or updating IAM roles.

testing.md: How to run your Rust test suites and mock AWS services.

5. docs/research/ (The Knowledge Base)
A place to dump information from your Google searches or Gemini chats.

benchmarks/: Performance comparisons (crucial for Rust projects).

whitepapers/: Notes on algorithms or AWS best practices you are implementing.