export def rust-target [target: string, --toolchain: string] {
    let flags = if ($toolchain | is-empty) { [] } else { [--toolchain $toolchain] }
    let install_args = [target add $target ...$flags]
    let targets = ^rustup target list --installed ...$flags | lines
    if $target in $targets { return }

    if $nu.is-interactive {
        if ([no yes] | input list $'rustup target ($target) missing; install it?') =~ 'y' {
            > rustup ...$install_args
            return
        }
    }

    error make {
        msg: $'rustup target ($target) is not installed'
        help: $'available via ($"\(rustup ($install_args | str join ` `)\)" | nu-highlight)'
    }
}

export def rust-toolchain [toolchain: string] {
    let install_args = [toolchain add $toolchain]
    if (do { rustup run $toolchain rustup -V } | complete).exit_code == 0 { return }

    if $nu.is-interactive {
        if ([no yes] | input list $'rustup toolchain ($toolchain) missing; install it') =~ 'y' {
            > rustup ...$install_args
            return
        }
    }

    error make {
        msg: $'rustup toolchain ($toolchain) is not installed'
        help: $'available via ($"\(rustup ($install_args | str join ` `))" | nu-highlight)'
    }
}

export def cargo-tool [tool: string, package?: string] {
    if not (which $tool | is-empty) { return }
    let package = $package | default $tool;

    if $nu.is-interactive and (not (which cargo-binstall | is-empty)) {
        > cargo binstall $package
        return
    }

    error make {
        msg: $"($tool) not found"
        help: $'available via ($"\(cargo install ($package))" | nu-highlight)'
    }
}
