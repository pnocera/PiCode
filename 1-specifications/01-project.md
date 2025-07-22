# Project name PiCode
## Goal : 
The project's goal is to replicate a subset of the features of Claude Code in a new application developped in rust, 
that will work with any OpenAPI compatible provider.

Claude Code documentation can be found at https://docs.anthropic.com/en/docs/claude-code/overview . context7 library id docs_anthropic_com-en-docs-claude-code-overview.

The root path of the project is at /home/pierre/Apps/pi-code/

Features to be implemented, following Claude Code definitions (note : the Claude Code documentation can be found at https://docs.anthropic.com/en/docs/claude-code/overview ) :
- cli reference https://docs.anthropic.com/en/docs/claude-code/cli-reference
- Interactive mode : https://docs.anthropic.com/en/docs/claude-code/interactive-mode
- Slash commands : https://docs.anthropic.com/en/docs/claude-code/slash-commands
- Hooks reference https://docs.anthropic.com/en/docs/claude-code/hooks
- Configurable llm options ( OpenAPI )

## Tech stack : 
- rust
- use zellij ( source available in /home/pierre/Apps/pi-code/0-github-repos/zellij ) as the foundation to create the cli.
- wasm : the cli will be compiled to wasm to provide an MCP server for other LLMs to use it as MCP tools, or to be run in interactive mode in a terminal or browser.


## Plan :

### Phase 1 : Research
Find all documentation for the Claude Code and do a deep analysis of the features it provides. Inspect the zellij docs in /home/pierre/Apps/pi-code/0-github-repos/zellij/docs and /home/pierre/Apps/pi-code/0-github-repos/zellij/examples.
Output the research results as markdown in the /home/pierre/Apps/pi-code/2-analysis folder. use web tools search, context7 MCP tools, github search to do any relevant research.
use a 5 agents swarm in parallel.


### Phase 2 : Human review
Let a human review the research results before proceeding to Phase 3

### Phase 3 : Conception - no coding yet
Analyse and create a proposal for the software architecture that will be implemented. Provide a detailed conception of each component that should be implemented.
output the results in the as markdown in the /home/pierre/Apps/pi-code/2-analysis folder.
use a 5 agents swarm in parallel.

### Phase 4 : Human review
Let a human review the conception results before proceeding to Phase 5

### Phase 5 : Implementation



