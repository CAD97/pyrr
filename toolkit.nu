# Toolkit module containing scripts for building and testing the Pyrr project.

export def main [--help] {
    try {
        help toolkit
    } catch {
        error make {
            msg: $"cannot run toolkit module"
            label: {
                text: "invoked here"
                span: (metadata $help).span
            }
            help: $"you probably want `use ($env.PROCESS_PATH)`"
        }
    }
}

# Get metadata about the project workspace.
export def workspace [] {
    let metadata = ^cargo metadata --format-version 1 --no-deps | from json
    {
        members: $metadata.packages
        default_members: $metadata.workspace_default_members
        root: $metadata.workspace_root
        target: $metadata.target_directory
        metadata: $metadata.metadata
    }
}

export def `workspace members` [] { workspace | get members }
export def `workspace default-members` [] { workspace | get default_members }
export def `workspace root` [] { workspace | get root }
export def `workspace target` [] { workspace | get target }
export def `workspace metadata` [] { workspace | get metadata }

export def `workspace member` [member: string, key?: string] {
    let out = workspace members | where name == $member | get 0?
    if ($out == null) {
        error make {
            msg: "failed to find workspace member"
            label: {
                text: "not a workspace member"
                span: (metadata $member).span
            }
            help: $"expected one of (workspace members | get name)"
        }
    }
    $out
}

def ensure-cargo-tool [tool: string, package?: string] {
    if not (which $tool | is-empty) { return }
    let package = $package | default $tool;

    if $nu.is-interactive and (which cargo-binstall) {
        ^cargo binstall $package
    }

    error make {
        msg: $"($tool) not found"
        help: $"available via cargo install ($'https://crates.io/crates/($package)' | ansi link --text $package)"
    }
}

def ? --wrapped [command: string, ...args] {
    print ([$"(ansi attr_dimmed)#" $command ...$args (ansi reset)] | str join " ")
}

def > --wrapped [command: string, ...args] {
    print ([$"(ansi attr_dimmed)>" $command ...$args (ansi reset)] | str join " ")
    run-external $command $args
}

# Build components of the pyrr project.
export def build [] {
    return (help build)
}

# Build the wasm components of the pyrr project.
export def `build wasm` [
    --all       # build all wasm components
    --math      # build pyrr-math
    --keep-temp # keep transient artifacts
] {
    if (not $all or $math) {
        return (help build wasm)
    }

    ensure-cargo-tool wasm-tools
    ensure-cargo-tool wasm-opt

    if $math or $all {
        let core_wasm = [(workspace target) "wasm32-unknown-unknown" "wasm-component" "pyrr_math.wasm"] | path join
        let temp_wasm = [(mktemp --directory) "pyrr-math.wasm"] | path join
        let comp_wasm = [(workspace target) "wasm-component" "pyrr-math.wasm"] | path join

        ? toolkit build wasm --math
        > cargo build --manifest-path (workspace member pyrr-math).manifest_path --target wasm32-unknown-unknown --profile wasm-component
        > wasm-opt $core_wasm -o $temp_wasm --ignore-implicit-traps -O3
        > wasm-tools component new $temp_wasm -o $comp_wasm

        if not $keep_temp { rm -r ($temp_wasm | path dirname) }
    }
}

