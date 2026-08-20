# CPM Shell Tab Auto-Completion Script
Register-ArgumentCompleter -Native -CommandName cpm -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $subcommands = @('doctor', 'benchmark', 'generate-stubs', 'bundle', 'repl', 'alias', 'dockerfile', 'search', 'rollback', 'licenses', 'scan-secrets', 'diff', 'resolve', 'cost', 'helm', 'policy', 'flamegraph', 'operator', 'cache', 'logs', 'sccache', 'verify-sig', 'audit-log', 'trace', 'graph', 'completion')
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
