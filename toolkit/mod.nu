# Toolkit module containing scripts for building and testing the Pyrr project.
#
# You must use one of the subcommands. Using as a command will only produce this help message.
export def main [] {
    try {
        help toolkit | ignore
        use std
        std help toolkit
    } catch {
        error make {
            msg: 'cannot run toolkit module'
            help: $'you probably want ("(use toolkit)" | nu-highlight)'
        }
    }
}

alias core-help = help
def help [...args] {
    try { use std; std help toolkit ...$args } catch { core-help ...$args }
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

# Get metadata about a specific workspace member.
export def "workspace member" [member: string, key?: string] {
    let out = workspace | get members | where name == $member | get 0?
    if ($out == null) {
        error make {
            msg: "failed to find workspace member"
            label: {
                text: $"($member) is not known workspace member"
                span: (metadata $member).span
            }
            help: $"expected one of (workspace | get members | get name)"
        }
    }
    $out
}

def ? --wrapped [command: string, ...args] {
    print ([$"(ansi attr_dimmed)#" $command ...$args (ansi reset)] | str join " ")
}

def > --wrapped [command: string, ...args] {
    print ([$"(ansi attr_dimmed)>" $command ...$args (ansi reset)] | str join " ")
    run-external $command ...$args
}

use ensure.nu

# Build components of the pyrr project.
export def build [] {
    return (help build)
}

# Build wasm components of the pyrr project.
export def "build wasm" [
    --all       # build all wasm components
    --demo      # build demo-component
    --math      # build pyrr-math
    --keep-temp # keep transient artifacts
] {
    if (not $all or $math or $demo) {
        return (help build wasm)
    }

    ensure rust-toolchain nightly
    ensure rust-target wasm32-unknown-unknown --toolchain nightly
    ensure cargo-tool wasm-tools
    ensure cargo-tool wasm-opt

    if $math or $all {
        ? toolkit build wasm --math

        let core_wasm = [(workspace | get target) wasm32-unknown-unknown wasm-component pyrr_math.wasm] | path join
        let temp_wasm = [(mktemp --directory) pyrr-math.wasm] | path join
        let comp_wasm = [(workspace | get target) wasm-component pyrr-math.wasm] | path join

        > cargo +nightly build --manifest-path (workspace member pyrr-math).manifest_path --target wasm32-unknown-unknown --profile wasm-component --features libm/unstable
        > wasm-opt $core_wasm -o $temp_wasm --ignore-implicit-traps -O3
        > wasm-tools component new $temp_wasm -o $comp_wasm

        if not $keep_temp { rm -r ($temp_wasm | path dirname) }
    }

    if $demo or $all {
        ? toolkit build wasm --demo

        let core_wasm = [(workspace | get target) wasm32-unknown-unknown wasm-component pyrr_demo_component.wasm] | path join
        let temp_wasm = [(mktemp --directory) pyrr-demo.wasm] | path join
        let comp_wasm = [(workspace | get target) wasm-component pyrr-demo.wasm] | path join
    
        > cargo +nightly build --manifest-path (workspace member pyrr-demo-component).manifest_path --target wasm32-unknown-unknown --profile wasm-component
        > wasm-opt $core_wasm -o $temp_wasm --ignore-implicit-traps -O3
        > wasm-tools component new $temp_wasm -o $comp_wasm

        if not $keep_temp { rm -r ($temp_wasm | path dirname) }

        let math_wasm = [(workspace | get target) wasm-component pyrr-math.wasm] | path join
        print $"(ansi yellow)Note:(ansi reset) currently, the demo requires you to compose the wasm component manually, i.e."
        print ($"    \(wasm-tools compose `($comp_wasm)`\n        -d `($math_wasm)` -o test.wasm)" | nu-highlight)
    }
}
