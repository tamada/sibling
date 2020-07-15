
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'sibling' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'sibling'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'sibling' {
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'specifies the traversing type of siblings.')
            [CompletionResult]::new('--type', 'type', [CompletionResultType]::ParameterName, 'specifies the traversing type of siblings.')
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'print the directory name in the absolute path.')
            [CompletionResult]::new('--absolute', 'absolute', [CompletionResultType]::ParameterName, 'print the directory name in the absolute path.')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'print the progress traversing directories.')
            [CompletionResult]::new('--progress', 'progress', [CompletionResultType]::ParameterName, 'print the progress traversing directories.')
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'print parent directory, when no more sibling directories (available on no-console mode).')
            [CompletionResult]::new('--parent', 'parent', [CompletionResultType]::ParameterName, 'print parent directory, when no more sibling directories (available on no-console mode).')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
