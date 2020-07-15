complete -c sibling -n "__fish_use_subcommand" -s t -l type -d 'specifies the traversing type of siblings.' -r -f -a "next previous random"
complete -c sibling -n "__fish_use_subcommand" -s a -l absolute -d 'print the directory name in the absolute path.'
complete -c sibling -n "__fish_use_subcommand" -s p -l progress -d 'print the progress traversing directories.'
complete -c sibling -n "__fish_use_subcommand" -s P -l parent -d 'print parent directory, when no more sibling directories (available on no-console mode).'
complete -c sibling -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c sibling -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
